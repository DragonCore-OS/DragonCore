use anyhow::{Context, Result};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::governance::{GovernanceEngine, RunState, Seat};
use crate::ledger::{Ledger, StabilityMetrics};
use crate::models::{Message, ModelRouter, Role};
use crate::tmux::{create_governance_session, TmuxManager};
use crate::worktree::{RunContext, WorktreeManager};

/// DragonCore Runtime
pub struct DragonCoreRuntime {
    config: Config,
    governance: Arc<RwLock<GovernanceEngine>>,
    ledger: Arc<RwLock<Ledger>>,
    tmux: TmuxManager,
    worktree: WorktreeManager,
    model_router: Arc<RwLock<ModelRouter>>,
    active_runs: Arc<RwLock<HashMap<String, RunContext>>>,
}

impl DragonCoreRuntime {
    /// Create a new runtime instance
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize ledger
        let ledger = Ledger::new(&config.ledger.storage_path)?;
        
        // Initialize tmux manager
        let tmux = TmuxManager::new(&config.execution.tmux_prefix);
        tmux.check_tmux()?;
        
        // Initialize worktree manager
        let worktree = WorktreeManager::new(
            &config.execution.worktree_base,
            std::env::current_dir().context("Failed to get current directory")?,
        );
        worktree.check_git()?;
        
        // Initialize model router
        let mut model_router = ModelRouter::new();
        for (name, provider_config) in &config.providers {
            // Check if we should use kimi-cli for Kimi Code keys
            let provider_name = if matches!(provider_config.provider_type, crate::config::ProviderType::KimiCli) {
                "kimi-cli"
            } else {
                name.as_str()
            };
            
            match crate::models::create_provider(provider_name, provider_config) {
                Ok(provider) => {
                    model_router.add_provider(provider);
                    tracing::info!("Added model provider: {} (type: {:?})", name, provider_config.provider_type);
                }
                Err(e) => {
                    tracing::warn!("Failed to create provider {}: {}", name, e);
                }
            }
        }
        
        Ok(Self {
            config,
            governance: Arc::new(RwLock::new(GovernanceEngine::new())),
            ledger: Arc::new(RwLock::new(ledger)),
            tmux,
            worktree,
            model_router: Arc::new(RwLock::new(model_router)),
            active_runs: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Initialize a new governance run
    pub async fn init_run(&self, run_id: impl Into<String>, input_type: impl Into<String>) -> Result<RunContext> {
        let run_id = run_id.into();
        let input_type = input_type.into();
        
        tracing::info!("Initializing governance run: {}", run_id);
        
        // Create governance run
        {
            let mut gov = self.governance.write().await;
            gov.create_run(run_id.clone())?;
        }
        
        // Start ledger
        {
            let mut ledger = self.ledger.write().await;
            ledger.start_run(&run_id, &input_type);
        }
        
        // Create worktree
        let worktree_path = self.worktree.create_worktree_from_head(&run_id)?;
        let commit_hash = self.worktree.get_commit_hash(&run_id)?;
        
        // Create run context
        let context = RunContext {
            run_id: run_id.clone(),
            worktree_path: worktree_path.clone(),
            commit_hash,
        };
        
        // Store active run
        {
            let mut active = self.active_runs.write().await;
            active.insert(run_id.clone(), context.clone());
        }
        
        // Create tmux session if isolation is enabled
        if self.config.execution.isolation_enabled {
            create_governance_session(&self.tmux, &run_id)?;
        }
        
        tracing::info!("Governance run {} initialized at {:?}", run_id, worktree_path);
        Ok(context)
    }
    
    /// Execute a seat's role in a run
    pub async fn execute_seat(&self, run_id: &str, seat: Seat, task: &str) -> Result<String> {
        let run_id = run_id.to_string();
        
        // Record participation
        {
            let mut ledger = self.ledger.write().await;
            ledger.record_participation(seat)?;
        }
        
        // Get system prompt for this seat
        let system_prompt = self.get_seat_prompt(seat);
        
        // Build messages
        let messages = vec![
            Message {
                role: Role::System,
                content: system_prompt,
            },
            Message {
                role: Role::User,
                content: task.to_string(),
            },
        ];
        
        // Send to model
        let router = self.model_router.read().await;
        let response = router.chat(messages).await?;
        
        // Store output in worktree
        if let Some(context) = self.active_runs.read().await.get(&run_id) {
            let output_file = format!("{:?}_output.md", seat).to_lowercase();
            context.write_artifact(&output_file, &response)?;
        }
        
        tracing::info!("Seat {:?} executed for run {}", seat, run_id);
        Ok(response)
    }
    
    /// Exercise veto
    pub async fn exercise_veto(&self, run_id: &str, seat: Seat, reason: &str) -> Result<()> {
        // Check if seat has veto authority
        if !seat.has_authority(crate::governance::Authority::Veto) {
            anyhow::bail!("Seat {:?} does not have veto authority", seat);
        }
        
        // Exercise veto in governance engine
        {
            let mut gov = self.governance.write().await;
            gov.exercise_veto(run_id, seat, reason.to_string())?;
        }
        
        // Record in ledger
        {
            let mut ledger = self.ledger.write().await;
            ledger.record_veto(seat)?;
        }
        
        tracing::info!("Veto exercised by {:?} on run {}: {}", seat, run_id, reason);
        Ok(())
    }
    
    /// Execute final gate (Tianshu only)
    pub async fn final_gate(&self, run_id: &str, approve: bool) -> Result<()> {
        // Execute final gate
        {
            let mut gov = self.governance.write().await;
            gov.final_gate(run_id, approve)?;
        }
        
        // Finalize ledger
        {
            let mut ledger = self.ledger.write().await;
            let final_state = if approve { RunState::Approved } else { RunState::Rejected };
            ledger.finalize_run(final_state)?;
        }
        
        tracing::info!("Final gate executed for run {}: {}", run_id, if approve { "APPROVED" } else { "REJECTED" });
        Ok(())
    }
    
    /// Archive a run
    pub async fn archive_run(&self, run_id: &str, seat: Seat) -> Result<()> {
        // Archive in governance engine
        {
            let mut gov = self.governance.write().await;
            gov.archive_run(run_id, seat)?;
        }
        
        // Record in ledger
        {
            let mut ledger = self.ledger.write().await;
            ledger.record_archive()?;
            ledger.finalize_run(RunState::Archived)?;
        }
        
        // Remove from active runs
        {
            let mut active = self.active_runs.write().await;
            active.remove(run_id);
        }
        
        tracing::info!("Run {} archived by {:?}", run_id, seat);
        Ok(())
    }
    
    /// Terminate a run
    pub async fn terminate_run(&self, run_id: &str, seat: Seat, reason: &str) -> Result<()> {
        // Terminate in governance engine
        {
            let mut gov = self.governance.write().await;
            gov.terminate_run(run_id, seat, reason.to_string())?;
        }
        
        // Record in ledger
        {
            let mut ledger = self.ledger.write().await;
            ledger.record_terminate()?;
            ledger.finalize_run(RunState::Terminated)?;
        }
        
        // Remove from active runs
        {
            let mut active = self.active_runs.write().await;
            active.remove(run_id);
        }
        
        // Kill tmux session
        if self.config.execution.isolation_enabled {
            self.tmux.kill_session(run_id)?;
        }
        
        tracing::info!("Run {} terminated by {:?}: {}", run_id, seat, reason);
        Ok(())
    }
    
    /// Get run status
    pub async fn get_run_status(&self, run_id: &str) -> Option<RunState> {
        let gov = self.governance.read().await;
        gov.get_run(run_id).map(|r| r.state)
    }
    
    /// Get stability metrics
    pub async fn get_stability_metrics(&self) -> Result<StabilityMetrics> {
        let ledger = self.ledger.read().await;
        ledger.get_stability_metrics()
    }
    
    /// List active runs
    pub async fn list_active_runs(&self) -> Vec<String> {
        let active = self.active_runs.read().await;
        active.keys().cloned().collect()
    }
    
    /// Get seat prompt
    fn get_seat_prompt(&self, seat: Seat) -> String {
        let base_prompt = format!(
            "You are {} ({}), serving as {} in the DragonCore governance system.\n\n",
            seat.chinese_name(),
            format!("{:?}", seat),
            seat.role()
        );
        
        let role_specific = match seat {
            Seat::Tianshu => "You are the final arbiter. You make ultimate decisions when other seats conflict. You do not participate in day-to-day execution.",
            Seat::Tianxuan => "You guard against risks. You review for compliance and systemic dangers. You can veto when risks are unacceptable.",
            Seat::Tianji => "You define technical standards. You set architecture direction and ensure technical consistency.",
            Seat::Tianquan => "You orchestrate execution. You translate strategy into actionable plans and coordinate resources.",
            Seat::Yuheng => "You guard quality gates. You review deliverables against standards. You can veto when quality is insufficient.",
            Seat::Kaiyang => "You review implementation. You examine engineering output for correctness and adherence to plans.",
            Seat::Yaoguang => "You manage innovation and archives. You preserve completed runs for future reference.",
            Seat::Qinglong => "You explore new tracks. You seize opportunities but must define stop conditions.",
            Seat::Baihu => "You are the red team. You stress test and find failure modes. You must provide fix windows.",
            Seat::Zhuque => "You manage external narrative. You speak for the system externally after verification.",
            Seat::Xuanwu => "You ensure stability. You protect steady-state operations while allowing exploration.",
            Seat::Yangjian => "You inspect quality. Your heavenly eye sees through deception and fake progress.",
            Seat::Baozheng => "You audit independently. You investigate power abuse and deliver verdicts.",
            Seat::Zhongkui => "You purge anomalies. You eliminate toxic agents and malicious processes.",
            Seat::Luban => "You build platforms. You create tools and automation that last.",
            Seat::Zhugeliang => "You are the chief advisor. You plan complex campaigns and multi-line strategies.",
            Seat::Nezha => "You are rapid deployment. You break through deadlocks with speed and decisive action.",
            Seat::Xiwangmu => "You control scarce resources. You decide who gets what, when, and why.",
            Seat::Fengdudadi => "You are the terminator. You end what must end and archive what must be preserved.",
        };
        
        format!("{}\n{}", base_prompt, role_specific)
    }
    
    /// Shutdown the runtime
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down DragonCore runtime");
        
        // Kill all tmux sessions
        self.tmux.kill_all_sessions()?;
        
        // Finalize any active runs
        let active_runs = self.list_active_runs().await;
        for run_id in active_runs {
            tracing::warn!("Force archiving active run: {}", run_id);
            let _ = self.archive_run(&run_id, Seat::Yaoguang).await;
        }
        
        tracing::info!("DragonCore runtime shutdown complete");
        Ok(())
    }
}

/// Runtime builder
pub struct RuntimeBuilder {
    config: Option<Config>,
}

impl RuntimeBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }
    
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }
    
    pub async fn build(self) -> Result<DragonCoreRuntime> {
        let config = match self.config {
            Some(c) => c,
            None => Config::init_default()?,
        };
        DragonCoreRuntime::new(config).await
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

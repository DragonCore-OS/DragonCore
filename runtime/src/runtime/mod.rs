#![allow(dead_code)]

use anyhow::{Context, Result};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::{Config, NormalizedConfig};
use crate::events::{DiblManager, EventChannel, EventScope, GovernanceEvent, GovernanceEventType, JsonlEventStore, Severity};
use crate::governance::{GovernanceEngine, Seat};
use crate::ledger::{Ledger, StabilityMetrics};
use crate::models::{Message, ModelRouter, Role};
use crate::persistence::{JsonFileStore, RunStore};
use crate::tmux::{create_governance_session, TmuxManager};
use crate::worktree::{RunContext, WorktreeManager};

/// DragonCore Runtime
pub struct DragonCoreRuntime {
    config: Config,
    normalized_config: NormalizedConfig,
    governance: Arc<RwLock<GovernanceEngine>>,
    ledger: Arc<RwLock<Ledger>>,
    store: Arc<JsonFileStore>,
    tmux: TmuxManager,
    worktree: WorktreeManager,
    model_router: Arc<RwLock<ModelRouter>>,
    active_runs: Arc<RwLock<HashMap<String, RunContext>>>,
    dible: Arc<DiblManager>,
}

impl DragonCoreRuntime {
    /// Create a new runtime instance
    pub async fn new(config: Config) -> Result<Self> {
        let normalized_config = config.normalize()?;

        // Initialize persistence store
        let storage_path = config.execution.worktree_base.join("../runtime_state/runs");
        let store = Arc::new(JsonFileStore::new(&storage_path)?);
        tracing::info!("Initialized persistence store at: {:?}", storage_path);
        
        // Initialize governance engine with store
        let store_clone: Box<dyn crate::persistence::RunStore> = Box::new(JsonFileStore::new(&storage_path)?);
        let governance = GovernanceEngine::new(store_clone)?;
        tracing::info!("Loaded {} runs from persistence", governance.list_runs().len());
        
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
        
        // Initialize model router from normalized provider registry
        let mut model_router = ModelRouter::new();
        for (name, provider_config) in &normalized_config.provider_registry {
            // Check if we should use kimi-cli for Kimi Code keys
            let provider_name = if matches!(provider_config.provider_type, crate::config::ProviderType::KimiCli) {
                "kimi-cli"
            } else {
                name.as_str()
            };
            
            match crate::models::create_provider(provider_name, provider_config) {
                Ok(provider) => {
                    model_router.add_provider(name.clone(), provider);
                    tracing::info!("Added model provider: {} (type: {:?})", name, provider_config.provider_type);
                }
                Err(e) => {
                    tracing::warn!("Failed to create provider {}: {}", name, e);
                }
            }
        }
        
        // Configure seat routing from normalized seat policies
        let mut seat_mapping = HashMap::new();
        for (seat, policy) in &normalized_config.seat_policies {
            if let Some(model_cfg) = normalized_config.model_registry.get(&policy.primary_model) {
                seat_mapping.insert(seat.clone(), model_cfg.provider.clone());
            }
        }
        if !seat_mapping.is_empty() {
            model_router.configure_seat_mappings(seat_mapping);
            tracing::info!("Configured normalized seat policies for routing");
        }

        if let Some(default_provider) = normalized_config.provider_for_seat(Some(&normalized_config.brains.sovereign_seat)) {
            model_router.set_default_provider(default_provider.to_string());
        } else if let Some(first_provider) = normalized_config.provider_registry.keys().next() {
            model_router.set_default_provider(first_provider.clone());
        }
        
        // Initialize DIBL event store
        let events_path = config.execution.worktree_base.join("../runtime_state/events");
        let event_store = Arc::new(JsonlEventStore::new(&events_path)?);
        let dible = Arc::new(DiblManager::new(event_store));
        tracing::info!("Initialized DIBL event store at: {:?}", events_path);
        
        Ok(Self {
            config,
            normalized_config,
            governance: Arc::new(RwLock::new(governance)),
            ledger: Arc::new(RwLock::new(ledger)),
            store,
            tmux,
            worktree,
            model_router: Arc::new(RwLock::new(model_router)),
            active_runs: Arc::new(RwLock::new(HashMap::new())),
            dible,
        })
    }
    
    /// Initialize a new governance run
    pub async fn init_run(&self, run_id: impl Into<String>, input_type: impl Into<String>, task: impl Into<String>) -> Result<RunContext> {
        let run_id = run_id.into();
        let input_type = input_type.into();
        let task = task.into();
        
        // Check if providers are configured
        if !self.normalized_config.has_providers() {
            anyhow::bail!(
                "No model providers configured.\n\n\
                To use DragonCore, you need to configure at least one AI provider.\n\
                Edit dragoncore.toml and add your API keys:\n\n\
                [providers.kimi]\n\
                provider_type = \"kimi\"\n\
                api_key = \"your-api-key\"\n\
                base_url = \"https://api.moonshot.cn/v1\"\n\
                model = \"kimi-for-coding\"\n\
                timeout = 60\n\n\
                Or for Kimi CLI (if you have Kimi Code membership):\n\
                [providers.kimi-cli]\n\
                provider_type = \"kimi_cli\"\n\
                api_key = \"your-api-key\"\n\
                base_url = \"https://api.kimi.com/coding/v1\"\n\
                model = \"kimi-for-coding\"\n\
                timeout = 60\n\n\
                Get your API key from: https://platform.moonshot.cn/"
            );
        }
        
        tracing::info!("Initializing governance run: {}", run_id);
        
        // Start ledger first (persist immediately)
        {
            let mut ledger = self.ledger.write().await;
            ledger.start_run(&run_id, &input_type)?;
        }
        
        // Create worktree and get paths
        let worktree_path = self.worktree.create_worktree_from_head(&run_id)?;
        let commit_hash = self.worktree.get_commit_hash(&run_id)?;
        let tmux_session = format!("dragoncore_{}", run_id);
        
        // Create governance run with all required info
        {
            let mut gov = self.governance.write().await;
            let run_state = gov.create_run(
                run_id.clone(),
                task.clone(),
                input_type.clone(),
                worktree_path.clone(),
                tmux_session,
            )?;
            
            // Persist run state immediately
            self.store.save_run(run_state)?;
        }
        
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
        
        // EMIT DIBL EVENT: RunCreated -> Control channel
        let event = GovernanceEvent::new(&run_id, GovernanceEventType::RunCreated, EventChannel::Control, "system")
            .with_scope(EventScope::OperatorVisible)
            .with_summary(format!("Task: {}", task))
            .with_artifact(format!("worktree: {:?}", worktree_path))
            .with_trigger_context("runtime.init_run");
        
        if let Err(e) = self.dible.emit(event) {
            tracing::error!("Failed to emit run_created event: {}", e);
        }
        
        tracing::info!("Governance run {} initialized at {:?}", run_id, worktree_path);
        Ok(context)
    }
    
    /// Execute a seat's role in a run
    pub async fn execute_seat(&self, run_id: &str, seat: Seat, task: &str) -> Result<String> {
        let seat_str = format!("{:?}", seat);
        
        // Get provider name for this seat (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some(&seat_str)).to_string()
        };
        
        // EMIT DIBL EVENT: SeatStarted -> Control channel
        let start_event = GovernanceEvent::new(run_id, GovernanceEventType::SeatStarted, EventChannel::Control, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::Internal)
            .with_summary(format!("Seat {} started execution", seat_str))
            .with_trigger_context("runtime.execute_seat")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(start_event) {
            tracing::error!("Failed to emit seat_started event: {}", e);
        }
        
        // Record participation
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            ledger.record_participation(run_id, seat)?;
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
        
        // Send to model with seat-based provider selection
        let router = self.model_router.read().await;
        let response = router.chat(messages, Some(&seat_str)).await?;
        
        // Store output in worktree
        if let Some(context) = self.active_runs.read().await.get(run_id) {
            let output_file = format!("{:?}_output.md", seat).to_lowercase();
            context.write_artifact(&output_file, &response)?;
        }
        
        // EMIT DIBL EVENT: SeatCompleted -> Research channel
        let complete_event = GovernanceEvent::new(run_id, GovernanceEventType::SeatCompleted, EventChannel::Research, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::Internal)
            .with_summary(format!("Seat {} completed (output: {} chars)", seat_str, response.len()))
            .with_artifact(format!("{:?}_output.md", seat).to_lowercase())
            .with_trigger_context("runtime.execute_seat")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(complete_event) {
            tracing::error!("Failed to emit seat_completed event: {}", e);
        }
        
        tracing::info!("Seat {:?} executed with provider {} for run {}", seat, provider_name, run_id);
        Ok(response)
    }
    
    /// Raise a risk (RiskRaised event)
    /// This allows seats to flag risks without exercising veto
    pub async fn raise_risk(&self, run_id: &str, seat: Seat, risk_type: &str, description: &str) -> Result<()> {
        // Record risk in ledger
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            ledger.record_risk(run_id)?;
        }
        
        // Get provider for this seat (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some(&format!("{:?}", seat))).to_string()
        };
        
        // EMIT DIBL EVENT: RiskRaised -> Security channel
        let seat_str = format!("{:?}", seat);
        let event = GovernanceEvent::new(run_id, GovernanceEventType::RiskRaised, EventChannel::Security, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::OperatorVisible)
            .with_severity(Severity::Warn)
            .with_summary(format!("[{}] {}", risk_type, description))
            .with_artifact(format!("risk_type: {}", risk_type))
            .with_trigger_context("runtime.raise_risk")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(event) {
            tracing::error!("Failed to emit risk_raised event: {}", e);
        }
        
        tracing::info!("Risk raised by {:?} on run {}: [{}] {}", seat, run_id, risk_type, description);
        Ok(())
    }
    
    /// Exercise veto
    pub async fn exercise_veto(&self, run_id: &str, seat: Seat, reason: &str) -> Result<()> {
        // Check if seat has veto authority
        if !seat.has_authority(crate::governance::Authority::Veto) {
            anyhow::bail!("Seat {:?} does not have veto authority", seat);
        }
        
        // Exercise veto in governance engine and persist
        {
            let mut gov = self.governance.write().await;
            let run = gov.exercise_veto(run_id, seat, reason.to_string())?;
            self.store.save_run(&run)?;
        }
        
        // Record in ledger (load first for cross-CLI continuity)
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            ledger.record_veto(run_id, seat)?;
        }
        
        // Get provider for this seat (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some(&format!("{:?}", seat))).to_string()
        };
        
        // EMIT DIBL EVENT: VetoExercised -> Security channel
        // Rule: JSON/ledger must succeed before broadcast
        let seat_str = format!("{:?}", seat);
        let event = GovernanceEvent::new(run_id, GovernanceEventType::VetoExercised, EventChannel::Security, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::OperatorVisible)
            .with_severity(Severity::Warn)
            .with_summary(reason.to_string())
            .with_trigger_context("runtime.exercise_veto")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(event) {
            tracing::error!("Failed to emit veto event: {}", e);
            // Note: We don't fail the operation if event emission fails
            // The veto itself is already persisted in JSON/ledger
        }
        
        tracing::info!("Veto exercised by {:?} on run {}: {}", seat, run_id, reason);
        Ok(())
    }
    
    /// Execute final gate (Tianshu only)
    pub async fn final_gate(&self, run_id: &str, approve: bool) -> Result<()> {
        // Execute final gate and persist
        {
            let mut gov = self.governance.write().await;
            let run = gov.final_gate(run_id, crate::governance::Seat::Tianshu, approve)?;
            self.store.save_run(&run)?;
        }
        
        // Finalize ledger (load first for cross-CLI continuity)
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            let final_state = if approve { crate::governance::RunState::Approved } else { crate::governance::RunState::Rejected };
            ledger.finalize_run(run_id, final_state)?;
        }
        
        // Get provider for Tianshu (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some("Tianshu")).to_string()
        };
        
        // EMIT DIBL EVENT: FinalGateOpened -> Control channel
        let fg_event = GovernanceEvent::new(run_id, GovernanceEventType::FinalGateOpened, EventChannel::Control, "Tianshu")
            .with_seat("Tianshu")
            .with_scope(EventScope::OperatorVisible)
            .with_summary(format!("Final gate: {}", if approve { "APPROVED" } else { "REJECTED" }))
            .with_trigger_context("runtime.final_gate")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(fg_event) {
            tracing::error!("Failed to emit final_gate event: {}", e);
        }
        
        // Get provider for Tianshu (for tracking) - reuse from above
        let provider_name_decision = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some("Tianshu")).to_string()
        };
        
        // EMIT DIBL EVENT: DecisionCommitted -> Control channel
        let decision_event = GovernanceEvent::new(run_id, GovernanceEventType::DecisionCommitted, EventChannel::Control, "Tianshu")
            .with_seat("Tianshu")
            .with_scope(EventScope::Exportable)
            .with_summary(if approve { "APPROVED".to_string() } else { "REJECTED".to_string() })
            .with_trigger_context("runtime.final_gate")
            .with_provider(&provider_name_decision);
        
        if let Err(e) = self.dible.emit(decision_event) {
            tracing::error!("Failed to emit decision event: {}", e);
        }
        
        tracing::info!("Final gate executed for run {}: {}", run_id, if approve { "APPROVED" } else { "REJECTED" });
        Ok(())
    }
    
    /// Archive a run
    pub async fn archive_run(&self, run_id: &str, seat: Seat) -> Result<()> {
        // Archive in governance engine and persist
        {
            let mut gov = self.governance.write().await;
            let run = gov.archive_run(run_id, seat)?;
            self.store.save_run(&run)?;
        }
        
        // Record in ledger (load first for cross-CLI continuity)
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            ledger.record_archive(run_id)?;
            ledger.finalize_run(run_id, crate::governance::RunState::Archived)?;
        }
        
        // Remove from active runs
        {
            let mut active = self.active_runs.write().await;
            active.remove(run_id);
        }
        
        // Get provider for this seat (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some(&format!("{:?}", seat))).to_string()
        };
        
        // EMIT DIBL EVENT: ArchiveCompleted -> Ops channel
        let seat_str = format!("{:?}", seat);
        let event = GovernanceEvent::new(run_id, GovernanceEventType::ArchiveCompleted, EventChannel::Ops, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::OperatorVisible)
            .with_summary("Run archived")
            .with_trigger_context("runtime.archive_run")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(event) {
            tracing::error!("Failed to emit archive event: {}", e);
        }
        
        tracing::info!("Run {} archived by {:?}", run_id, seat);
        Ok(())
    }
    
    /// Terminate a run
    pub async fn terminate_run(&self, run_id: &str, seat: Seat, reason: &str) -> Result<()> {
        // Terminate in governance engine and persist
        {
            let mut gov = self.governance.write().await;
            let run = gov.terminate_run(run_id, seat, reason.to_string())?;
            self.store.save_run(&run)?;
        }
        
        // Record in ledger (load first for cross-CLI continuity)
        {
            let mut ledger = self.ledger.write().await;
            ledger.load_run(run_id);
            ledger.record_terminate(run_id)?;
            ledger.finalize_run(run_id, crate::governance::RunState::Terminated)?;
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
        
        // Get provider for this seat (for tracking)
        let provider_name = {
            let router = self.model_router.read().await;
            router.get_provider_name_for_seat(Some(&format!("{:?}", seat))).to_string()
        };
        
        // EMIT DIBL EVENT: TerminateTriggered -> Security channel
        let seat_str = format!("{:?}", seat);
        let event = GovernanceEvent::new(run_id, GovernanceEventType::TerminateTriggered, EventChannel::Security, &seat_str)
            .with_seat(&seat_str)
            .with_scope(EventScope::OperatorVisible)
            .with_severity(Severity::Critical)
            .with_summary(reason.to_string())
            .with_trigger_context("runtime.terminate_run")
            .with_provider(&provider_name);
        
        if let Err(e) = self.dible.emit(event) {
            tracing::error!("Failed to emit terminate event: {}", e);
        }
        
        tracing::info!("Run {} terminated by {:?}: {}", run_id, seat, reason);
        Ok(())
    }
    
    /// Get run status
    pub async fn get_run_status(&self, run_id: &str) -> Result<Option<crate::persistence::PersistedRunStatus>> {
        let mut gov = self.governance.write().await;
        match gov.get_run(run_id) {
            Ok(run) => Ok(Some(run.status.clone())),
            Err(_) => Ok(None),
        }
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
    
    /// List all runs from persistence
    pub async fn list_all_runs(&self) -> Vec<String> {
        let gov = self.governance.read().await;
        gov.list_runs().iter().map(|r| r.run_id.clone()).collect()
    }
    
    /// Load events for a run (DIBL)
    pub fn load_run_events(&self, run_id: &str) -> Result<Vec<crate::events::GovernanceEvent>> {
        self.dible.load_run_events(run_id)
    }
    
    /// Replay events for a run (DIBL)
    pub fn replay_run_events(&self, run_id: &str) -> Result<Vec<crate::events::GovernanceEvent>> {
        self.dible.replay_run(run_id)
    }
    
    /// Get operator projection for a run (DIBL)
    pub fn get_operator_projection(&self, run_id: &str) -> Result<crate::events::OperatorProjection> {
        let events = self.dible.load_run_events(run_id)?;
        Ok(crate::events::OperatorProjection::from_events(run_id, &events))
    }
    
    /// Check if run has events (DIBL)
    pub fn run_has_events(&self, run_id: &str) -> bool {
        self.dible.run_has_events(run_id)
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

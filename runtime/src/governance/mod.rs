use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::persistence::{PersistedRun, PersistedRunStatus, RunEvent, RunMetrics, RunStore, VetoRecord};

/// Governance authority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Authority {
    Suggest,
    Review,
    Veto,
    Approve,
    Execute,
    Orchestrate,
    FinalGate,
    Archive,
    Terminate,
}

/// The 19 governance seats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Seat {
    Tianshu, Tianxuan, Tianji, Tianquan, Yuheng, Kaiyang, Yaoguang,
    Qinglong, Baihu, Zhuque, Xuanwu,
    Yangjian, Baozheng, Zhongkui, Luban, Zhugeliang, Nezha, Xiwangmu, Fengdudadi,
}

impl Seat {
    pub fn layer(&self) -> Layer {
        match self {
            Seat::Tianshu | Seat::Tianxuan | Seat::Tianji | 
            Seat::Tianquan | Seat::Yuheng | Seat::Kaiyang | Seat::Yaoguang => Layer::SevenStars,
            Seat::Qinglong | Seat::Baihu | Seat::Zhuque | Seat::Xuanwu => Layer::FourSymbols,
            Seat::Yangjian | Seat::Baozheng | Seat::Zhongkui | Seat::Luban |
            Seat::Zhugeliang | Seat::Nezha | Seat::Xiwangmu | Seat::Fengdudadi => Layer::EightImmortals,
        }
    }
    
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Seat::Tianshu => "天枢", Seat::Tianxuan => "天璇", Seat::Tianji => "天玑",
            Seat::Tianquan => "天权", Seat::Yuheng => "玉衡", Seat::Kaiyang => "开阳",
            Seat::Yaoguang => "瑶光", Seat::Qinglong => "青龙", Seat::Baihu => "白虎",
            Seat::Zhuque => "朱雀", Seat::Xuanwu => "玄武", Seat::Yangjian => "杨戬",
            Seat::Baozheng => "包拯", Seat::Zhongkui => "钟馗", Seat::Luban => "鲁班",
            Seat::Zhugeliang => "诸葛亮", Seat::Nezha => "哪吒", Seat::Xiwangmu => "西王母",
            Seat::Fengdudadi => "丰都大帝",
        }
    }
    
    pub fn role(&self) -> &'static str {
        match self {
            Seat::Tianshu => "CEO / Final Arbiter",
            Seat::Tianxuan => "COO / Risk Guardian",
            Seat::Tianji => "CTO / Technical Lead",
            Seat::Tianquan => "CSO / Strategy Definition",
            Seat::Yuheng => "CRO / Quality Gate",
            Seat::Kaiyang => "Implementation Review",
            Seat::Yaoguang => "Innovation & Archive",
            Seat::Qinglong => "New Track Exploration",
            Seat::Baihu => "Red Team / Stress Test",
            Seat::Zhuque => "External Narrative",
            Seat::Xuanwu => "Stability Assurance",
            Seat::Yangjian => "Quality Inspection",
            Seat::Baozheng => "Independent Audit",
            Seat::Zhongkui => "Anomaly Purge",
            Seat::Luban => "Engineering Platform",
            Seat::Zhugeliang => "Chief Advisor",
            Seat::Nezha => "Rapid Deployment",
            Seat::Xiwangmu => "Scarce Resources",
            Seat::Fengdudadi => "Termination & Archive",
        }
    }
    
    pub fn authorities(&self) -> Vec<Authority> {
        match self {
            Seat::Tianshu => vec![Authority::Approve, Authority::Veto, Authority::FinalGate],
            Seat::Tianxuan => vec![Authority::Review, Authority::Veto, Authority::Suggest],
            Seat::Tianji => vec![Authority::Review, Authority::Suggest],
            Seat::Tianquan => vec![Authority::Execute, Authority::Orchestrate, Authority::Approve],
            Seat::Yuheng => vec![Authority::Review, Authority::Veto, Authority::Suggest],
            Seat::Kaiyang => vec![Authority::Execute, Authority::Review, Authority::Suggest],
            Seat::Yaoguang => vec![Authority::Archive, Authority::Suggest, Authority::Review],
            Seat::Qinglong => vec![Authority::Suggest, Authority::Execute],
            Seat::Baihu => vec![Authority::Review, Authority::Veto],
            Seat::Zhuque => vec![Authority::Suggest, Authority::Review],
            Seat::Xuanwu => vec![Authority::Review, Authority::Suggest],
            Seat::Yangjian => vec![Authority::Review],
            Seat::Baozheng => vec![Authority::Review, Authority::Veto],
            Seat::Zhongkui => vec![Authority::Terminate, Authority::Review],
            Seat::Luban => vec![Authority::Execute, Authority::Review],
            Seat::Zhugeliang => vec![Authority::Suggest, Authority::Review],
            Seat::Nezha => vec![Authority::Execute],
            Seat::Xiwangmu => vec![Authority::Approve],
            Seat::Fengdudadi => vec![Authority::Terminate, Authority::Archive, Authority::Review],
        }
    }
    
    pub fn has_authority(&self, authority: Authority) -> bool {
        self.authorities().contains(&authority)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layer {
    SevenStars, FourSymbols, EightImmortals,
}

impl Layer {
    pub fn name(&self) -> &'static str {
        match self {
            Layer::SevenStars => "北斗七星",
            Layer::FourSymbols => "四象",
            Layer::EightImmortals => "八仙护法",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Pending, InProgress, Reviewing, Escalated,
    Approved, Rejected, RolledBack, Archived, Terminated,
}

/// Governance engine with durable persistence
/// 
/// Core rule: Every state transition = memory update + durable write
pub struct GovernanceEngine {
    cache: HashMap<String, PersistedRun>,
    store: Box<dyn RunStore>,
}

impl GovernanceEngine {
    /// Create new engine, loading all runs from persistent storage
    pub fn new(store: Box<dyn RunStore>) -> Result<Self> {
        let runs = store.load_all_runs()
            .context("Failed to load runs from storage")?;
        
        tracing::info!("Loaded {} runs from persistent storage", runs.len());
        
        Ok(Self { cache: runs, store })
    }
    
    /// Create a new governance run with persistence
    pub fn create_run(&mut self, run_id: String, task: String, input_type: String, 
                      worktree_path: std::path::PathBuf, tmux_session: String) -> Result<&PersistedRun> {
        let run = PersistedRun::new(
            run_id.clone(),
            task,
            input_type,
            worktree_path,
            tmux_session,
        );
        
        // Persist FIRST (source of truth)
        self.store.create_run(&run)
            .with_context(|| format!("Failed to persist run {}", run_id))?;
        
        // Then update cache
        self.cache.insert(run_id.clone(), run);
        
        self.cache.get(&run_id)
            .context("Run was just inserted but not found in cache")
    }
    
    /// Get run by ID - loads from storage if not in cache
    pub fn get_run(&mut self, run_id: &str) -> Result<&PersistedRun> {
        use std::collections::hash_map::Entry;
        
        match self.cache.entry(run_id.to_string()) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                // Load from storage
                let run = self.store.load_run(run_id)
                    .with_context(|| format!("Failed to load run {}", run_id))?
                    .ok_or_else(|| anyhow::anyhow!("Run {} not found in storage", run_id))?;
                Ok(entry.insert(run))
            }
        }
    }
    
    /// Get mutable reference to run - requires reload from storage
    pub fn get_run_mut(&mut self, run_id: &str) -> Result<&mut PersistedRun> {
        // Ensure it's in cache
        if !self.cache.contains_key(run_id) {
            let run = self.store.load_run(run_id)
                .with_context(|| format!("Failed to load run {}", run_id))?
                .ok_or_else(|| anyhow::anyhow!("Run {} not found in storage", run_id))?;
            self.cache.insert(run_id.to_string(), run);
        }
        
        self.cache.get_mut(run_id)
            .context("Run not found in cache")
    }
    
    /// Save run state to persistent storage
    fn persist_run(&self, run: &PersistedRun) -> Result<()> {
        self.store.save_run(run)
            .with_context(|| format!("Failed to persist run {}", run.run_id))
    }
    
    /// Record seat participation with persistence
    pub fn record_participation(&mut self, run_id: &str, seat: Seat) -> Result<()> {
        let run = self.get_run_mut(run_id)?;
        
        let seat_str = format!("{:?}", seat);
        if !run.seats_participated.contains(&seat_str) {
            run.seats_participated.push(seat_str);
        }
        
        run.add_event(seat, "participate", None);
        
        // Persist
        let run_clone = run.clone();
        self.persist_run(&run_clone)?;
        
        Ok(())
    }
    
    /// Execute seat action with persistence
    pub fn execute_seat(&mut self, run_id: &str, seat: Seat, output: &str) -> Result<()> {
        let run = self.get_run_mut(run_id)?;
        
        run.current_seat = Some(format!("{:?}", seat));
        run.add_event(seat, "execute", Some(&format!("output length: {} chars", output.len())));
        run.artifacts.push(format!("{:?}_output.md", seat).to_lowercase());
        
        // Persist
        let run_clone = run.clone();
        self.persist_run(&run_clone)?;
        
        Ok(())
    }
    
    /// Exercise veto - returns modified run for persistence by caller
    pub fn exercise_veto(&mut self, run_id: &str, seat: Seat, reason: String) -> Result<&PersistedRun> {
        // Check authority
        if !seat.has_authority(Authority::Veto) {
            anyhow::bail!("Seat {:?} does not have veto authority", seat);
        }
        
        let run = self.get_run_mut(run_id)?;
        
        // Update state
        run.status = PersistedRunStatus::Vetoed;
        run.veto = Some(VetoRecord {
            seat: format!("{:?}", seat),
            reason: reason.clone(),
            timestamp: Utc::now(),
        });
        run.add_event(seat, "veto", Some(&reason));
        
        let run_id_owned = run.run_id.clone();
        tracing::info!("Veto exercised by {:?} on run {}: {}", seat, run_id_owned, reason);
        
        self.cache.get(&run_id_owned)
            .context("Run was just modified but not found in cache")
    }
    
    /// Execute final gate - returns modified run for persistence by caller
    pub fn final_gate(&mut self, run_id: &str, seat: Seat, approve: bool) -> Result<&PersistedRun> {
        // Check authority (only Tianshu)
        if seat != Seat::Tianshu {
            anyhow::bail!("Only Tianshu can execute final gate");
        }
        
        let run = self.get_run_mut(run_id)?;
        
        // Update state
        run.status = if approve { PersistedRunStatus::Approved } else { PersistedRunStatus::Rejected };
        run.final_gate = Some(crate::persistence::FinalGateRecord {
            seat: format!("{:?}", seat),
            approved: approve,
            timestamp: Utc::now(),
        });
        run.add_event(seat, "final_gate", Some(&format!("approved: {}", approve)));
        
        let run_id_owned = run.run_id.clone();
        tracing::info!("Final gate executed for run {}: {}", run_id_owned, 
            if approve { "APPROVED" } else { "REJECTED" });
        
        self.cache.get(&run_id_owned)
            .context("Run was just modified but not found in cache")
    }
    
    /// Archive run - returns modified run for persistence by caller
    pub fn archive_run(&mut self, run_id: &str, seat: Seat) -> Result<&PersistedRun> {
        // Check authority
        if !seat.has_authority(Authority::Archive) {
            anyhow::bail!("Seat {:?} does not have archive authority", seat);
        }
        
        let run = self.get_run_mut(run_id)?;
        
        run.status = PersistedRunStatus::Archived;
        run.add_event(seat, "archive", None);
        
        let run_id_owned = run.run_id.clone();
        tracing::info!("Run {} archived by {:?}", run_id_owned, seat);
        
        self.cache.get(&run_id_owned)
            .context("Run was just modified but not found in cache")
    }
    
    /// Terminate run - returns modified run for persistence by caller
    pub fn terminate_run(&mut self, run_id: &str, seat: Seat, reason: String) -> Result<&PersistedRun> {
        // Check authority
        if !seat.has_authority(Authority::Terminate) {
            anyhow::bail!("Seat {:?} does not have terminate authority", seat);
        }
        
        let run = self.get_run_mut(run_id)?;
        
        run.status = PersistedRunStatus::Terminated;
        run.add_event(seat, "terminate", Some(&reason));
        
        let run_id_owned = run.run_id.clone();
        tracing::info!("Run {} terminated by {:?}: {}", run_id_owned, seat, reason);
        
        self.cache.get(&run_id_owned)
            .context("Run was just modified but not found in cache")
    }
    
    /// List all runs from cache (which is loaded from storage)
    pub fn list_runs(&self) -> Vec<&PersistedRun> {
        self.cache.values().collect()
    }
    
    /// List active runs
    pub fn list_active_runs(&self) -> Vec<&PersistedRun> {
        self.cache.values()
            .filter(|r| matches!(r.status, 
                PersistedRunStatus::Created | 
                PersistedRunStatus::Running
            ))
            .collect()
    }
    
    /// Get run status
    pub fn get_run_status(&self, run_id: &str) -> Option<PersistedRunStatus> {
        self.cache.get(run_id).map(|r| r.status.clone())
    }
    
    /// Check if run exists
    pub fn run_exists(&self, run_id: &str) -> bool {
        if self.cache.contains_key(run_id) {
            return true;
        }
        self.store.run_exists(run_id).unwrap_or(false)
    }
}

/// Get all 19 seats
pub fn all_seats() -> Vec<Seat> {
    vec![
        Seat::Tianshu, Seat::Tianxuan, Seat::Tianji, Seat::Tianquan,
        Seat::Yuheng, Seat::Kaiyang, Seat::Yaoguang,
        Seat::Qinglong, Seat::Baihu, Seat::Zhuque, Seat::Xuanwu,
        Seat::Yangjian, Seat::Baozheng, Seat::Zhongkui, Seat::Luban,
        Seat::Zhugeliang, Seat::Nezha, Seat::Xiwangmu, Seat::Fengdudadi,
    ]
}

/// Get seats by layer
pub fn seats_by_layer(layer: Layer) -> Vec<Seat> {
    all_seats().into_iter()
        .filter(|s| s.layer() == layer)
        .collect()
}

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Governance authority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Authority {
    /// Can recommend, no binding power
    Suggest,
    /// Can examine and comment
    Review,
    /// Can block with documented reason
    Veto,
    /// Can authorize passage
    Approve,
    /// Can implement directly
    Execute,
    /// Can orchestrate execution
    Orchestrate,
    /// Ultimate decision, no appeal
    FinalGate,
    /// Can preserve to institutional memory
    Archive,
    /// Can end permanently
    Terminate,
}

/// The 19 governance seats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Seat {
    // 北斗七星 - Seven Northern Stars
    Tianshu,    // CEO / Final Arbiter
    Tianxuan,   // COO / Risk Guardian
    Tianji,     // CTO / Technical Lead
    Tianquan,   // CSO / Strategy Definition
    Yuheng,     // CRO / Quality Gate
    Kaiyang,    // Implementation Review
    Yaoguang,   // Innovation & Archive
    
    // 四象 - Four Symbols
    Qinglong,   // East - New Track Exploration
    Baihu,      // West - Red Team / Stress Test
    Zhuque,     // South - External Narrative
    Xuanwu,     // North - Stability Assurance
    
    // 八仙护法 - Eight Guardian Immortals
    Yangjian,   // Quality Inspection
    Baozheng,   // Independent Audit
    Zhongkui,   // Anomaly Purge
    Luban,      // Engineering Platform
    Zhugeliang, // Chief Advisor
    Nezha,      // Rapid Deployment
    Xiwangmu,   // Scarce Resources
    Fengdudadi, // Termination & Archive
}

impl Seat {
    /// Get the layer this seat belongs to
    pub fn layer(&self) -> Layer {
        match self {
            Seat::Tianshu | Seat::Tianxuan | Seat::Tianji | 
            Seat::Tianquan | Seat::Yuheng | Seat::Kaiyang | Seat::Yaoguang => Layer::SevenStars,
            Seat::Qinglong | Seat::Baihu | Seat::Zhuque | Seat::Xuanwu => Layer::FourSymbols,
            Seat::Yangjian | Seat::Baozheng | Seat::Zhongkui | Seat::Luban |
            Seat::Zhugeliang | Seat::Nezha | Seat::Xiwangmu | Seat::Fengdudadi => Layer::EightImmortals,
        }
    }
    
    /// Get the Chinese name for this seat
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Seat::Tianshu => "天枢",
            Seat::Tianxuan => "天璇",
            Seat::Tianji => "天玑",
            Seat::Tianquan => "天权",
            Seat::Yuheng => "玉衡",
            Seat::Kaiyang => "开阳",
            Seat::Yaoguang => "瑶光",
            Seat::Qinglong => "青龙",
            Seat::Baihu => "白虎",
            Seat::Zhuque => "朱雀",
            Seat::Xuanwu => "玄武",
            Seat::Yangjian => "杨戬",
            Seat::Baozheng => "包拯",
            Seat::Zhongkui => "钟馗",
            Seat::Luban => "鲁班",
            Seat::Zhugeliang => "诸葛亮",
            Seat::Nezha => "哪吒",
            Seat::Xiwangmu => "西王母",
            Seat::Fengdudadi => "丰都大帝",
        }
    }
    
    /// Get the role description for this seat
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
    
    /// Get authorities for this seat
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
    
    /// Check if this seat has a specific authority
    pub fn has_authority(&self, authority: Authority) -> bool {
        self.authorities().contains(&authority)
    }
}

/// Governance layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layer {
    SevenStars,      // 北斗七星
    FourSymbols,     // 四象
    EightImmortals,  // 八仙护法
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

/// Governance run state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunState {
    Pending,
    InProgress,
    Reviewing,
    Escalated,
    Approved,
    Rejected,
    RolledBack,
    Archived,
    Terminated,
}

/// A governance run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRun {
    pub run_id: String,
    pub state: RunState,
    pub seats: HashMap<Seat, SeatState>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatState {
    pub status: SeatStatus,
    pub outputs: Vec<String>,
    pub veto_reason: Option<String>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeatStatus {
    Waiting,
    Active,
    Completed,
    Vetoed,
    Escalated,
}

/// Governance engine
pub struct GovernanceEngine {
    runs: HashMap<String, GovernanceRun>,
}

impl GovernanceEngine {
    pub fn new() -> Self {
        Self {
            runs: HashMap::new(),
        }
    }
    
    /// Create a new governance run
    pub fn create_run(&mut self, run_id: String) -> Result<&GovernanceRun> {
        let now = chrono::Utc::now();
        
        let mut seats = HashMap::new();
        for seat in all_seats() {
            seats.insert(seat, SeatState {
                status: SeatStatus::Waiting,
                outputs: Vec::new(),
                veto_reason: None,
                approved_at: None,
            });
        }
        
        let run = GovernanceRun {
            run_id: run_id.clone(),
            state: RunState::Pending,
            seats,
            created_at: now,
            updated_at: now,
        };
        
        self.runs.insert(run_id.clone(), run);
        
        self.runs.get(&run_id)
            .context("Run was just inserted but not found")
    }
    
    /// Get a run by ID
    pub fn get_run(&self, run_id: &str) -> Option<&GovernanceRun> {
        self.runs.get(run_id)
    }
    
    /// Get mutable reference to a run
    pub fn get_run_mut(&mut self, run_id: &str) -> Option<&mut GovernanceRun> {
        self.runs.get_mut(run_id)
    }
    
    /// Check if a seat can exercise veto
    pub fn can_veto(&self, run_id: &str, seat: Seat) -> Result<bool> {
        let run = self.get_run(run_id)
            .context("Run not found")?;
        
        Ok(seat.has_authority(Authority::Veto) && 
           run.state == RunState::Reviewing)
    }
    
    /// Exercise veto
    pub fn exercise_veto(&mut self, run_id: &str, seat: Seat, reason: String) -> Result<()> {
        let run = self.get_run_mut(run_id)
            .context("Run not found")?;
        
        if !seat.has_authority(Authority::Veto) {
            anyhow::bail!("Seat {:?} does not have veto authority", seat);
        }
        
        let seat_state = run.seats.get_mut(&seat)
            .context("Seat not found in run")?;
        
        seat_state.status = SeatStatus::Vetoed;
        seat_state.veto_reason = Some(reason);
        
        run.state = RunState::Rejected;
        run.updated_at = chrono::Utc::now();
        
        Ok(())
    }
    
    /// Execute final gate (Tianshu only)
    pub fn final_gate(&mut self, run_id: &str, approve: bool) -> Result<()> {
        let run = self.get_run_mut(run_id)
            .context("Run not found")?;
        
        if approve {
            run.state = RunState::Approved;
        } else {
            run.state = RunState::Rejected;
        }
        
        run.updated_at = chrono::Utc::now();
        
        Ok(())
    }
    
    /// Archive a run (Yaoguang or Fengdudadi)
    pub fn archive_run(&mut self, run_id: &str, seat: Seat) -> Result<()> {
        if !seat.has_authority(Authority::Archive) {
            anyhow::bail!("Seat {:?} does not have archive authority", seat);
        }
        
        let run = self.get_run_mut(run_id)
            .context("Run not found")?;
        
        run.state = RunState::Archived;
        run.updated_at = chrono::Utc::now();
        
        Ok(())
    }
    
    /// Terminate a run (Fengdudadi or Zhongkui)
    pub fn terminate_run(&mut self, run_id: &str, seat: Seat, reason: String) -> Result<()> {
        if !seat.has_authority(Authority::Terminate) {
            anyhow::bail!("Seat {:?} does not have terminate authority", seat);
        }
        
        let run = self.get_run_mut(run_id)
            .context("Run not found")?;
        
        run.state = RunState::Terminated;
        run.updated_at = chrono::Utc::now();
        
        // Record termination reason in Yaoguang's outputs
        if let Some(yaoguang) = run.seats.get_mut(&Seat::Yaoguang) {
            yaoguang.outputs.push(format!("Termination by {:?}: {}", seat, reason));
        }
        
        Ok(())
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

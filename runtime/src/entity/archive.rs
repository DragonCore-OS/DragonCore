use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::fmt;

/// 职级
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityRank {
    /// 入门级
    Entry,
    /// 初级
    Junior,
    /// 中级
    Intermediate,
    /// 高级
    Senior,
    /// 主管
    Lead,
    /// 负责人
    Principal,
    /// 总监
    Director,
}

impl EntityRank {
    /// 获取职级描述
    pub fn description(&self) -> &'static str {
        match self {
            EntityRank::Entry => "入门级",
            EntityRank::Junior => "初级",
            EntityRank::Intermediate => "中级",
            EntityRank::Senior => "高级",
            EntityRank::Lead => "主管",
            EntityRank::Principal => "负责人",
            EntityRank::Director => "总监",
        }
    }
    
    /// 获取英文名称
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityRank::Entry => "Entry",
            EntityRank::Junior => "Junior",
            EntityRank::Intermediate => "Intermediate",
            EntityRank::Senior => "Senior",
            EntityRank::Lead => "Lead",
            EntityRank::Principal => "Principal",
            EntityRank::Director => "Director",
        }
    }
    
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "entry" => Some(EntityRank::Entry),
            "junior" => Some(EntityRank::Junior),
            "intermediate" => Some(EntityRank::Intermediate),
            "senior" => Some(EntityRank::Senior),
            "lead" => Some(EntityRank::Lead),
            "principal" => Some(EntityRank::Principal),
            "director" => Some(EntityRank::Director),
            _ => None,
        }
    }
    
    /// 获取数值等级 (用于比较)
    pub fn level(&self) -> u8 {
        match self {
            EntityRank::Entry => 1,
            EntityRank::Junior => 2,
            EntityRank::Intermediate => 3,
            EntityRank::Senior => 4,
            EntityRank::Lead => 5,
            EntityRank::Principal => 6,
            EntityRank::Director => 7,
        }
    }
}

impl fmt::Display for EntityRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 部门
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Department {
    /// 治理部
    Governance,
    /// 执行部
    Execution,
    /// 审计部
    Audit,
    /// 研究部
    Research,
    /// 协调部
    Coordination,
    /// 其他
    Other,
}

impl Department {
    /// 获取部门描述
    pub fn description(&self) -> &'static str {
        match self {
            Department::Governance => "治理部",
            Department::Execution => "执行部",
            Department::Audit => "审计部",
            Department::Research => "研究部",
            Department::Coordination => "协调部",
            Department::Other => "其他",
        }
    }
    
    /// 获取英文名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Department::Governance => "Governance",
            Department::Execution => "Execution",
            Department::Audit => "Audit",
            Department::Research => "Research",
            Department::Coordination => "Coordination",
            Department::Other => "Other",
        }
    }
    
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "governance" => Some(Department::Governance),
            "execution" => Some(Department::Execution),
            "audit" => Some(Department::Audit),
            "research" => Some(Department::Research),
            "coordination" => Some(Department::Coordination),
            "other" => Some(Department::Other),
            _ => None,
        }
    }
}

/// 权限
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Authority {
    /// 发言
    Speak,
    /// 投票
    Vote,
    /// 提案
    Propose,
    /// 审查
    Review,
    /// 否决
    Veto,
    /// 最终门控
    FinalGate,
    /// 终止
    Terminate,
    /// 归档
    Archive,
    /// 任命
    Appoint,
    /// 降职
    Demote,
}

impl Authority {
    /// 获取权限描述
    pub fn description(&self) -> &'static str {
        match self {
            Authority::Speak => "发言",
            Authority::Vote => "投票",
            Authority::Propose => "提案",
            Authority::Review => "审查",
            Authority::Veto => "否决",
            Authority::FinalGate => "最终门控",
            Authority::Terminate => "终止",
            Authority::Archive => "归档",
            Authority::Appoint => "任命",
            Authority::Demote => "降职",
        }
    }
    
    /// 获取英文名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Authority::Speak => "Speak",
            Authority::Vote => "Vote",
            Authority::Propose => "Propose",
            Authority::Review => "Review",
            Authority::Veto => "Veto",
            Authority::FinalGate => "FinalGate",
            Authority::Terminate => "Terminate",
            Authority::Archive => "Archive",
            Authority::Appoint => "Appoint",
            Authority::Demote => "Demote",
        }
    }
    
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "speak" => Some(Authority::Speak),
            "vote" => Some(Authority::Vote),
            "propose" => Some(Authority::Propose),
            "review" => Some(Authority::Review),
            "veto" => Some(Authority::Veto),
            "finalgate" | "final_gate" => Some(Authority::FinalGate),
            "terminate" => Some(Authority::Terminate),
            "archive" => Some(Authority::Archive),
            "appoint" => Some(Authority::Appoint),
            "demote" => Some(Authority::Demote),
            _ => None,
        }
    }
}

/// 基础档案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityBaseArchive {
    /// 当前职级
    pub current_rank: EntityRank,
    /// 任期开始时间
    pub term_start: DateTime<Utc>,
    /// 最近一次升职时间
    pub last_promotion: Option<DateTime<Utc>>,
    /// 最近一次降职时间
    pub last_demotion: Option<DateTime<Utc>>,
    /// 权限集合
    pub authority_set: Vec<Authority>,
    /// 部门
    pub department: Department,
    /// 汇报对象
    pub reports_to: Option<String>,
    /// 备注
    pub notes: String,
}

impl Default for EntityBaseArchive {
    fn default() -> Self {
        Self {
            current_rank: EntityRank::Entry,
            term_start: Utc::now(),
            last_promotion: None,
            last_demotion: None,
            authority_set: Vec::new(),
            department: Department::Other,
            reports_to: None,
            notes: String::new(),
        }
    }
}

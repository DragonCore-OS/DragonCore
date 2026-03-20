use crate::entity::status::EntityStatus;
use crate::entity::archive::{EntityRank, Department, Authority};
use crate::governance::Seat;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// AI生命体唯一身份
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AIEntityIdentity {
    /// 唯一实体ID (UUID v4)
    pub entity_id: Uuid,
    
    /// 实体名称 (如 "Yuheng-V2", "Nezha-Prod")
    pub name: String,
    
    /// 当前角色 (映射到19席)
    pub role: Seat,
    
    /// 当前职级
    pub rank: EntityRank,
    
    /// 所属部门
    pub department: Department,
    
    /// 权限集合
    pub authority_set: Vec<Authority>,
    
    /// 激活状态 (8状态机)
    pub status: EntityStatus,
    
    /// 任期开始时间
    pub term_start: DateTime<Utc>,
    
    /// 最近一次状态变更时间
    pub last_status_change: DateTime<Utc>,
    
    /// 记忆根目录
    pub memory_root: PathBuf,
    
    /// 绩效根目录
    pub performance_root: PathBuf,
    
    /// 纪律根目录
    pub discipline_root: PathBuf,
    
    /// 扩展元数据
    pub metadata: HashMap<String, String>,
}

impl AIEntityIdentity {
    /// 创建新实体 (默认 Candidate 状态)
    pub fn new(name: String, role: Seat, department: Department) -> Self {
        let entity_id = Uuid::new_v4();
        let now = Utc::now();
        let base_path = format!("./runtime_state/entities/{}", entity_id);
        
        Self {
            entity_id,
            name: name.clone(),
            role,
            rank: EntityRank::Entry,
            department,
            authority_set: Vec::new(),
            status: EntityStatus::Candidate,
            term_start: now,
            last_status_change: now,
            memory_root: format!("{}/memory", base_path).into(),
            performance_root: format!("{}/performance", base_path).into(),
            discipline_root: format!("{}/discipline", base_path).into(),
            metadata: HashMap::new(),
        }
    }
    
    /// 检查是否"活着" (有持续身份，可能有限制)
    /// 包括: Candidate, Active, Limited, UnderReview, Demoted
    pub fn is_alive(&self) -> bool {
        matches!(self.status, 
            EntityStatus::Candidate |
            EntityStatus::Active | 
            EntityStatus::Limited | 
            EntityStatus::Demoted | 
            EntityStatus::UnderReview
        )
    }
    
    /// 检查是否具有活跃治理权力
    /// 只有 Active 状态才有完整治理权力
    pub fn is_governance_active(&self) -> bool {
        self.status == EntityStatus::Active
    }
    
    /// 检查是否拥有特定权限
    pub fn has_authority(&self, authority: &Authority) -> bool {
        self.authority_set.contains(authority)
    }
    
    /// 检查是否可以自动恢复 (从 Limited/Demoted 恢复)
    pub fn can_auto_recover(&self) -> bool {
        matches!(self.status, EntityStatus::Limited | EntityStatus::Demoted)
    }
    
    /// 获取状态描述
    pub fn status_description(&self) -> &'static str {
        self.status.description()
    }
    
    /// 更新状态时间戳
    pub fn touch_status_change(&mut self) {
        self.last_status_change = Utc::now();
    }
    
    /// 获取任期时长 (天)
    pub fn tenure_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.term_start).num_days()
    }
    
    /// 获取当前状态持续时长 (天)
    pub fn status_duration_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.last_status_change).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_creation() {
        let entity = AIEntityIdentity::new(
            "TestEntity".to_string(),
            Seat::Tianshu,
            Department::Governance,
        );
        
        assert_eq!(entity.name, "TestEntity");
        assert_eq!(entity.status, EntityStatus::Candidate);
        assert_eq!(entity.rank, EntityRank::Entry);
        assert!(entity.authority_set.is_empty());
        assert!(entity.is_alive()); // Candidate 也算活着
        assert!(!entity.is_governance_active()); // 但没有治理权力
    }
    
    #[test]
    fn test_is_alive_states() {
        let mut entity = AIEntityIdentity::new(
            "Test".to_string(),
            Seat::Yuheng,
            Department::Audit,
        );
        
        // Active - 活着且有治理权力
        entity.status = EntityStatus::Active;
        assert!(entity.is_alive());
        assert!(entity.is_governance_active());
        
        // Limited - 活着但没有治理权力
        entity.status = EntityStatus::Limited;
        assert!(entity.is_alive());
        assert!(!entity.is_governance_active());
        
        // Suspended - 不算活着，也没有治理权力
        entity.status = EntityStatus::Suspended;
        assert!(!entity.is_alive());
        assert!(!entity.is_governance_active());
        
        // Terminated - 不算活着
        entity.status = EntityStatus::Terminated;
        assert!(!entity.is_alive());
        assert!(!entity.is_governance_active());
    }
    
    #[test]
    fn test_can_auto_recover() {
        let mut entity = AIEntityIdentity::new(
            "Test".to_string(),
            Seat::Nezha,
            Department::Execution,
        );
        
        entity.status = EntityStatus::Limited;
        assert!(entity.can_auto_recover());
        
        entity.status = EntityStatus::Demoted;
        assert!(entity.can_auto_recover());
        
        entity.status = EntityStatus::Suspended;
        assert!(!entity.can_auto_recover());
        
        entity.status = EntityStatus::Terminated;
        assert!(!entity.can_auto_recover());
    }
}

use crate::entity::identity::AIEntityIdentity;
use crate::entity::status::{EntityStatus, StateTransitionRequest, StateTransitionValidator, StateTransitionRecord};
use crate::entity::storage::{EntityStorage, StorageError};
use crate::entity::archive::{Authority, EntityRank};
use crate::entity::attribution::DecisionAttribution;
use crate::entity::kpi::PeriodKPI;
use crate::events::{GovernanceEvent, GovernanceEventType};
use crate::governance::Seat;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// 实体管理错误
#[derive(Debug, Error)]
pub enum EntityError {
    #[error("存储错误: {0}")]
    Storage(#[from] StorageError),
    
    #[error("实体未找到: {0}")]
    EntityNotFound(Uuid),
    
    #[error("状态转移错误: {0}")]
    StateTransition(#[from] crate::entity::status::StateTransitionError),
    
    #[error("事件发送失败")]
    EventSendFailed,
    
    #[error("实体已存在: {0}")]
    EntityAlreadyExists(String),
    
    #[error("权限不足")]
    InsufficientAuthority,
}

/// AI实体管理器
pub struct EntityManager {
    /// 内存缓存 (entity_id -> entity)
    entities: Arc<RwLock<HashMap<Uuid, AIEntityIdentity>>>,
    /// 存储后端
    storage: Arc<dyn EntityStorage>,
    /// 事件发送器
    event_tx: mpsc::Sender<GovernanceEvent>,
    /// 状态转移历史
    transition_history: Arc<RwLock<HashMap<Uuid, Vec<StateTransitionRecord>>>>,
    /// 决策归因缓存 (decision_id -> attribution)
    attributions: Arc<RwLock<HashMap<Uuid, DecisionAttribution>>>,
    /// KPI 缓存 (entity_id -> period -> kpi)
    kpis: Arc<RwLock<HashMap<Uuid, HashMap<String, PeriodKPI>>>>,
}

impl EntityManager {
    /// 创建新的实体管理器
    pub fn new(
        storage: Arc<dyn EntityStorage>,
        event_tx: mpsc::Sender<GovernanceEvent>,
    ) -> Self {
        Self {
            entities: Arc::new(RwLock::new(HashMap::new())),
            storage,
            event_tx,
            transition_history: Arc::new(RwLock::new(HashMap::new())),
            attributions: Arc::new(RwLock::new(HashMap::new())),
            kpis: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 从存储加载所有实体到内存
    pub async fn load_all(&self) -> Result<(), EntityError> {
        let entities = self.storage.list_all().await?;
        let mut cache = self.entities.write().await;
        
        for entity in entities {
            cache.insert(entity.entity_id, entity);
        }
        
        Ok(())
    }
    
    /// 创建新实体
    pub async fn create_entity(
        &self,
        name: String,
        role: Seat,
        department: crate::entity::archive::Department,
    ) -> Result<AIEntityIdentity, EntityError> {
        // 检查名称是否已存在
        let cache = self.entities.read().await;
        if cache.values().any(|e| e.name == name) {
            return Err(EntityError::EntityAlreadyExists(name));
        }
        drop(cache);
        
        let entity = AIEntityIdentity::new(name.clone(), role, department);
        
        // 持久化
        self.storage.save(&entity).await?;
        
        // 发送事件
        self.emit_event(
            GovernanceEventType::EntityCreated,
            format!("AI实体创建: {} (ID: {})", entity.name, entity.entity_id),
            Some(entity.entity_id),
        ).await?;
        
        // 缓存
        let mut cache = self.entities.write().await;
        cache.insert(entity.entity_id, entity.clone());
        
        Ok(entity)
    }
    
    /// 获取实体
    pub async fn get_entity(&self, entity_id: Uuid) -> Result<AIEntityIdentity, EntityError> {
        // 先查缓存
        let cache = self.entities.read().await;
        if let Some(entity) = cache.get(&entity_id) {
            return Ok(entity.clone());
        }
        drop(cache);
        
        // 再查存储
        let entity = self.storage.load(entity_id).await?;
        
        // 加入缓存
        let mut cache = self.entities.write().await;
        cache.insert(entity_id, entity.clone());
        
        Ok(entity)
    }
    
    /// 状态转移
    pub async fn transition_status(
        &self,
        entity_id: Uuid,
        request: StateTransitionRequest,
    ) -> Result<(), EntityError> {
        let mut entity = self.get_entity(entity_id).await?;
        let old_status = entity.status;
        
        // 验证转移
        StateTransitionValidator::validate(old_status, &request)?;
        
        // 执行转移
        let new_status = request.to_status;
        entity.status = new_status;
        entity.touch_status_change();
        
        // 更新职级 (如果是降职)
        if new_status == EntityStatus::Demoted {
            entity.rank = EntityRank::Entry;
        }
        
        // 更新权限
        self.update_authority(&mut entity);
        
        // 持久化
        self.storage.save(&entity).await?;
        
        // 记录转移历史
        let record = StateTransitionRecord {
            from_status: old_status,
            to_status: new_status,
            reason: request.reason.clone(),
            initiated_by: request.initiated_by.clone(),
            approved_by: request.approved_by.clone(),
            evidence: request.evidence.clone(),
            timestamp: Utc::now(),
        };
        
        let mut history = self.transition_history.write().await;
        history.entry(entity_id).or_default().push(record);
        
        // 更新缓存
        let mut cache = self.entities.write().await;
        cache.insert(entity_id, entity);
        
        // 发送事件
        let event_type = match new_status {
            EntityStatus::Active => GovernanceEventType::EntityActivated,
            EntityStatus::Limited => GovernanceEventType::EntityLimited,
            EntityStatus::UnderReview => GovernanceEventType::EntityUnderReview,
            EntityStatus::Demoted => GovernanceEventType::EntityDemoted,
            EntityStatus::Suspended => GovernanceEventType::EntitySuspended,
            EntityStatus::Archived => GovernanceEventType::EntityArchived,
            EntityStatus::Terminated => GovernanceEventType::EntityTerminated,
            _ => GovernanceEventType::EntityStatusChanged,
        };
        
        self.emit_event(
            event_type,
            format!(
                "实体 {} 状态转移: {} → {}, 原因: {}, 发起人: {}",
                entity_id,
                old_status,
                new_status,
                request.reason,
                request.initiated_by
            ),
            Some(entity_id),
        ).await?;
        
        Ok(())
    }
    
    /// 升职
    pub async fn promote_entity(
        &self,
        entity_id: Uuid,
        to_rank: EntityRank,
        reason: String,
        initiated_by: String,
    ) -> Result<(), EntityError> {
        let mut entity = self.get_entity(entity_id).await?;
        
        if entity.rank >= to_rank {
            return Err(EntityError::InsufficientAuthority);
        }
        
        let old_rank = entity.rank;
        entity.rank = to_rank;
        
        // 更新权限
        self.update_authority(&mut entity);
        
        // 持久化
        self.storage.save(&entity).await?;
        
        // 更新缓存
        let mut cache = self.entities.write().await;
        cache.insert(entity_id, entity);
        
        // 发送事件
        self.emit_event(
            GovernanceEventType::EntityPromoted,
            format!(
                "实体 {} 升职: {} → {}, 原因: {}, 发起人: {}",
                entity_id, old_rank, to_rank, reason, initiated_by
            ),
            Some(entity_id),
        ).await?;
        
        Ok(())
    }
    
    /// 降职
    pub async fn demote_entity(
        &self,
        entity_id: Uuid,
        reason: String,
        initiated_by: String,
    ) -> Result<(), EntityError> {
        let request = StateTransitionRequest::new(
            EntityStatus::Demoted,
            reason,
            initiated_by,
        )?;
        
        self.transition_status(entity_id, request).await
    }
    
    /// 暂停
    pub async fn suspend_entity(
        &self,
        entity_id: Uuid,
        reason: String,
        initiated_by: String,
    ) -> Result<(), EntityError> {
        let request = StateTransitionRequest::new(
            EntityStatus::Suspended,
            reason,
            initiated_by,
        )?;
        
        self.transition_status(entity_id, request).await
    }
    
    /// 终止
    pub async fn terminate_entity(
        &self,
        entity_id: Uuid,
        reason: String,
        initiated_by: String,
        approved_by: String,
    ) -> Result<(), EntityError> {
        let request = StateTransitionRequest::new(
            EntityStatus::Terminated,
            reason,
            initiated_by,
        )?.with_approval(approved_by);
        
        self.transition_status(entity_id, request).await
    }
    
    /// 列出所有活跃实体 (有治理权力)
    pub async fn list_governance_active(&self) -> Vec<AIEntityIdentity> {
        let cache = self.entities.read().await;
        cache
            .values()
            .filter(|e| e.is_governance_active())
            .cloned()
            .collect()
    }
    
    /// 列出所有"活着"的实体
    pub async fn list_alive(&self) -> Vec<AIEntityIdentity> {
        let cache = self.entities.read().await;
        cache
            .values()
            .filter(|e| e.is_alive())
            .cloned()
            .collect()
    }
    
    /// 按状态列出实体
    pub async fn list_by_status(&self, status: EntityStatus) -> Vec<AIEntityIdentity> {
        let cache = self.entities.read().await;
        cache
            .values()
            .filter(|e| e.status == status)
            .cloned()
            .collect()
    }
    
    /// 获取状态转移历史
    pub async fn get_transition_history(&self, entity_id: Uuid) -> Vec<StateTransitionRecord> {
        let history = self.transition_history.read().await;
        history.get(&entity_id).cloned().unwrap_or_default()
    }
    
    /// 更新权限
    fn update_authority(&self, entity: &mut AIEntityIdentity) {
        match entity.status {
            EntityStatus::Active => {
                entity.authority_set = self.calculate_authority(&entity.role, &entity.rank);
            }
            EntityStatus::Limited => {
                // 限制高风险权限
                entity.authority_set.retain(|a| {
                    !matches!(a, Authority::Veto | Authority::FinalGate | Authority::Terminate)
                });
                if entity.authority_set.is_empty() {
                    entity.authority_set.push(Authority::Speak);
                }
            }
            EntityStatus::Demoted | EntityStatus::Suspended | EntityStatus::UnderReview => {
                // 仅保留基础权限
                entity.authority_set = vec![Authority::Speak];
            }
            _ => {
                entity.authority_set.clear();
            }
        }
    }
    
    /// 计算权限
    fn calculate_authority(&self, role: &Seat, rank: &EntityRank) -> Vec<Authority> {
        let mut authorities = vec![Authority::Speak];
        
        // 根据角色赋予权限
        match role {
            Seat::Tianshu => {
                authorities.push(Authority::FinalGate);
                authorities.push(Authority::Archive);
                authorities.push(Authority::Appoint);
            }
            Seat::Yuheng | Seat::Baihu => {
                authorities.push(Authority::Veto);
                authorities.push(Authority::Review);
            }
            _ => {
                authorities.push(Authority::Vote);
                authorities.push(Authority::Propose);
            }
        }
        
        // 根据职级调整
        if rank.level() >= EntityRank::Lead.level() {
            authorities.push(Authority::Demote);
        }
        
        authorities
    }
    
    /// 发送事件
    async fn emit_event(
        &self,
        event_type: GovernanceEventType,
        summary: String,
        entity_id: Option<Uuid>,
    ) -> Result<(), EntityError> {
        let event = GovernanceEvent::entity_event(
            event_type,
            summary,
            entity_id,
        );
        
        self.event_tx
            .send(event)
            .await
            .map_err(|_| EntityError::EventSendFailed)
    }
    
    // ===== PR-2: 归因与 KPI 方法 =====
    
    /// 记录决策归因
    pub async fn record_attribution(&self, attribution: DecisionAttribution) -> Result<(), EntityError> {
        let decision_id = attribution.decision_id;
        
        // 缓存归因
        {
            let mut attrs = self.attributions.write().await;
            attrs.insert(decision_id, attribution.clone());
        }
        
        // 发送 DIBL 事件
        let event = attribution.to_event();
        self.event_tx.send(event).await.map_err(|_| EntityError::EventSendFailed)?;
        
        Ok(())
    }
    
    /// 获取决策归因
    pub async fn get_attribution(&self, decision_id: Uuid) -> Option<DecisionAttribution> {
        let attrs = self.attributions.read().await;
        attrs.get(&decision_id).cloned()
    }
    
    /// 列出实体的所有归因
    pub async fn list_entity_attributions(&self, entity_id: Uuid) -> Vec<DecisionAttribution> {
        let attrs = self.attributions.read().await;
        attrs.values()
            .filter(|a| {
                a.primary_owner == entity_id 
                || a.approving_authority == entity_id
                || a.supporting.contains(&entity_id)
                || a.challenging.contains(&entity_id)
            })
            .cloned()
            .collect()
    }
    
    /// 更新 KPI
    pub async fn update_kpi(&self, kpi: PeriodKPI) -> Result<(), EntityError> {
        let entity_id = kpi.entity_id;
        let period = kpi.period.clone();
        
        // 缓存 KPI
        {
            let mut kpis = self.kpis.write().await;
            kpis.entry(entity_id).or_default().insert(period, kpi.clone());
        }
        
        // 发送 DIBL 事件
        let event = kpi.to_event();
        self.event_tx.send(event).await.map_err(|_| EntityError::EventSendFailed)?;
        
        Ok(())
    }
    
    /// 获取实体 KPI
    pub async fn get_kpi(&self, entity_id: Uuid, period: &str) -> Option<PeriodKPI> {
        let kpis = self.kpis.read().await;
        kpis.get(&entity_id)?.get(period).cloned()
    }
    
    /// 列出实体的所有 KPI
    pub async fn list_entity_kpis(&self, entity_id: Uuid) -> Vec<PeriodKPI> {
        let kpis = self.kpis.read().await;
        kpis.get(&entity_id)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::storage::MemoryStorage;
    use crate::entity::archive::Department;
    
    async fn create_test_manager() -> (EntityManager, mpsc::Receiver<GovernanceEvent>) {
        let (tx, rx) = mpsc::channel(100);
        let storage = Arc::new(MemoryStorage::new());
        let manager = EntityManager::new(storage, tx);
        (manager, rx)
    }
    
    #[tokio::test]
    async fn test_create_entity() {
        let (manager, _rx) = create_test_manager().await;
        
        let entity = manager.create_entity(
            "TestEntity".to_string(),
            Seat::Tianshu,
            Department::Governance,
        ).await.unwrap();
        
        assert_eq!(entity.name, "TestEntity");
        assert_eq!(entity.status, EntityStatus::Candidate);
        assert!(entity.is_alive());
        assert!(!entity.is_governance_active());
    }
    
    #[tokio::test]
    async fn test_status_transition() {
        let (manager, mut rx) = create_test_manager().await;
        
        let entity = manager.create_entity(
            "Test".to_string(),
            Seat::Yuheng,
            Department::Audit,
        ).await.unwrap();
        
        // Candidate → Active
        let request = StateTransitionRequest::new(
            EntityStatus::Active,
            "通过考核".to_string(),
            "System".to_string(),
        ).unwrap();
        
        manager.transition_status(entity.entity_id, request).await.unwrap();
        
        let updated = manager.get_entity(entity.entity_id).await.unwrap();
        assert_eq!(updated.status, EntityStatus::Active);
        assert!(updated.is_governance_active());
        assert!(updated.has_authority(&Authority::Review));
    }
}

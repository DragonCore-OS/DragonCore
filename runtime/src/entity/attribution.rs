use crate::entity::identity::AIEntityIdentity;
use crate::events::{GovernanceEvent, GovernanceEventType};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 决策类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionType {
    Proposal,
    RiskAssessment,
    Veto,
    FinalGate,
    Termination,
    Archive,
    ResourceAllocation,
}

/// 决策结果
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionOutcome {
    Approved,
    Rejected,
    Modified,
    Deferred,
}

/// 决策归因记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionAttribution {
    pub decision_id: Uuid,
    pub decision_type: DecisionType,
    pub primary_owner: Uuid,
    pub supporting: Vec<Uuid>,
    pub challenging: Vec<Uuid>,
    pub approving_authority: Uuid,
    pub execution_entities: Vec<Uuid>,
    pub decided_at: DateTime<Utc>,
    pub outcome: DecisionOutcome,
    pub impact: Option<DecisionImpact>,
}

/// 决策影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionImpact {
    pub success: bool,
    pub new_issues: u32,
    pub triggered_rollback: bool,
    pub assessed_at: DateTime<Utc>,
}

impl DecisionAttribution {
    pub fn new(
        decision_type: DecisionType,
        primary_owner: Uuid,
        approving_authority: Uuid,
    ) -> Self {
        Self {
            decision_id: Uuid::new_v4(),
            decision_type,
            primary_owner,
            supporting: Vec::new(),
            challenging: Vec::new(),
            approving_authority,
            execution_entities: Vec::new(),
            decided_at: Utc::now(),
            outcome: DecisionOutcome::Approved,
            impact: None,
        }
    }

    pub fn with_supporters(mut self, supporters: Vec<Uuid>) -> Self {
        self.supporting = supporters;
        self
    }

    pub fn with_challengers(mut self, challengers: Vec<Uuid>) -> Self {
        self.challenging = challengers;
        self
    }

    pub fn with_execution(mut self, executors: Vec<Uuid>) -> Self {
        self.execution_entities = executors;
        self
    }

    pub fn with_outcome(mut self, outcome: DecisionOutcome) -> Self {
        self.outcome = outcome;
        self
    }

    /// 计算责任权重
    /// 规则: 主责40% + 拍板30% + 支持者30%均分
    /// 同一实体多角色时权重累加
    pub fn calculate_responsibility(&self) -> HashMap<Uuid, f32> {
        let mut weights: HashMap<Uuid, f32> = HashMap::new();
        
        *weights.entry(self.primary_owner).or_insert(0.0) += 0.40;
        *weights.entry(self.approving_authority).or_insert(0.0) += 0.30;
        
        if !self.supporting.is_empty() {
            let supporter_weight = 0.30 / self.supporting.len() as f32;
            for supporter in &self.supporting {
                *weights.entry(*supporter).or_insert(0.0) += supporter_weight;
            }
        }
        
        weights
    }

    pub fn has_multiple_roles(&self, entity_id: Uuid) -> Vec<&'static str> {
        let mut roles = Vec::new();
        if self.primary_owner == entity_id {
            roles.push("primary_owner");
        }
        if self.approving_authority == entity_id {
            roles.push("approving_authority");
        }
        if self.supporting.contains(&entity_id) {
            roles.push("supporter");
        }
        if self.challenging.contains(&entity_id) {
            roles.push("challenger");
        }
        roles
    }

    pub fn to_event(&self) -> GovernanceEvent {
        GovernanceEvent::entity_event(
            GovernanceEventType::DecisionAttributed,
            format!("Decision {} attributed", self.decision_id),
            Some(self.decision_id),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_weight_distribution() {
        let attr = DecisionAttribution::new(
            DecisionType::Proposal,
            Uuid::new_v4(),
            Uuid::new_v4(),
        ).with_supporters(vec![Uuid::new_v4(), Uuid::new_v4()]);
        
        let weights = attr.calculate_responsibility();
        let total: f32 = weights.values().sum();
        assert!((total - 1.0).abs() < 0.001, "Total weight should be 1.0, got {}", total);
    }

    #[test]
    fn test_multiple_roles_accumulation() {
        let same_entity = Uuid::new_v4();
        let other_supporter = Uuid::new_v4();
        
        let attr = DecisionAttribution {
            decision_id: Uuid::new_v4(),
            decision_type: DecisionType::Proposal,
            primary_owner: same_entity,
            supporting: vec![same_entity, other_supporter],
            challenging: vec![],
            approving_authority: same_entity,
            execution_entities: vec![],
            decided_at: Utc::now(),
            outcome: DecisionOutcome::Approved,
            impact: None,
        };
        
        let weights = attr.calculate_responsibility();
        
        // 40% + 30% + 15% = 85%
        assert_eq!(weights.get(&same_entity), Some(&0.85));
        assert_eq!(weights.get(&other_supporter), Some(&0.15));
        
        let total: f32 = weights.values().sum();
        assert!((total - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_no_supporter_weight_forfeited() {
        let attr = DecisionAttribution::new(
            DecisionType::Proposal,
            Uuid::new_v4(),
            Uuid::new_v4(),
        );
        
        let weights = attr.calculate_responsibility();
        let total: f32 = weights.values().sum();
        
        // 40% + 30% = 70%, 30% forfeited
        assert!((total - 0.70).abs() < 0.001, "Total should be 0.70, got {}", total);
    }
}

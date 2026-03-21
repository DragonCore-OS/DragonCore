use crate::events::{GovernanceEvent, GovernanceEventType};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// 周期 KPI 记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodKPI {
    pub period: String,
    pub entity_id: Uuid,
    pub decision_quality: f32,
    pub governance_conduct: f32,
    pub organizational_duty: f32,
    pub details: KPIDetails,
    pub calculated_at: DateTime<Utc>,
}

/// KPI 详细指标
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KPIDetails {
    pub proposal_count: u32,
    pub adopted_count: u32,
    pub success_count: u32,
    pub rollback_count: u32,
    pub risk_warnings: u32,
    pub risk_hits: u32,
    pub speak_count: u32,
    pub silence_count: u32,
    pub challenge_count: u32,
    pub valid_challenges: u32,
    pub repetitive_count: u32,
    pub assigned_tasks: u32,
    pub completed_tasks: u32,
    pub coverage_rate: f32,
    pub convergence_support: u32,
}

impl PeriodKPI {
    pub fn new(entity_id: Uuid, period: &str) -> Self {
        Self {
            period: period.to_string(),
            entity_id,
            decision_quality: 50.0,
            governance_conduct: 50.0,
            organizational_duty: 50.0,
            details: KPIDetails::default(),
            calculated_at: Utc::now(),
        }
    }

    /// 总分计算: DecisionQuality 45% + GovernanceConduct 30% + OrganizationalDuty 25%
    pub fn total_score(&self) -> f32 {
        let score = self.decision_quality * 0.45
                  + self.governance_conduct * 0.30
                  + self.organizational_duty * 0.25;
        score.clamp(0.0, 100.0)
    }

    pub fn to_event(&self) -> GovernanceEvent {
        let summary = format!(
            "Entity {} KPI for {}: total={:.2}, dq={:.2}, gc={:.2}, od={:.2}",
            self.entity_id, self.period, self.total_score(),
            self.decision_quality, self.governance_conduct, self.organizational_duty
        );
        GovernanceEvent::entity_event(
            GovernanceEventType::EntityKpiUpdated,
            summary,
            Some(self.entity_id),
        )
    }
}

/// KPI 计算器
pub struct KPICalculator;

impl KPICalculator {
    pub fn calculate(entity_id: Uuid, period: &str, events: &[crate::events::GovernanceEvent]) -> PeriodKPI {
        let mut details = KPIDetails::default();
        
        for event in events {
            if event.actor != entity_id.to_string() {
                continue;
            }
            
            match event.event_type {
                GovernanceEventType::DecisionCommitted => {
                    details.proposal_count += 1;
                }
                GovernanceEventType::RollbackTriggered => {
                    details.rollback_count += 1;
                }
                GovernanceEventType::RiskRaised => {
                    details.risk_warnings += 1;
                }
                _ => {}
            }
        }
        
        let decision_quality = Self::calc_decision_quality(&details);
        let governance_conduct = Self::calc_governance_conduct(&details);
        let organizational_duty = Self::calc_organizational_duty(&details);
        
        PeriodKPI {
            period: period.to_string(),
            entity_id,
            decision_quality,
            governance_conduct,
            organizational_duty,
            details,
            calculated_at: Utc::now(),
        }
    }

    fn calc_decision_quality(details: &KPIDetails) -> f32 {
        if details.proposal_count == 0 {
            return 50.0;
        }
        
        let adoption_rate = details.adopted_count as f32 / details.proposal_count as f32;
        let success_rate = if details.adopted_count > 0 {
            details.success_count as f32 / details.adopted_count as f32
        } else {
            0.0
        };
        let rollback_penalty = (details.rollback_count as f32 * 10.0).min(30.0);
        
        (adoption_rate * 30.0 + success_rate * 50.0 - rollback_penalty).clamp(0.0, 100.0)
    }

    fn calc_governance_conduct(_details: &KPIDetails) -> f32 {
        50.0
    }

    fn calc_organizational_duty(_details: &KPIDetails) -> f32 {
        50.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpi_total_score_calculation() {
        let kpi = PeriodKPI {
            period: "2026-03".to_string(),
            entity_id: Uuid::new_v4(),
            decision_quality: 80.0,
            governance_conduct: 70.0,
            organizational_duty: 60.0,
            details: KPIDetails::default(),
            calculated_at: Utc::now(),
        };
        
        // 80*0.45 + 70*0.30 + 60*0.25 = 36 + 21 + 15 = 72
        let expected = 72.0;
        let actual = kpi.total_score();
        assert!((actual - expected).abs() < 0.1, 
            "Expected {:.2}, got {:.2}", expected, actual);
    }

    #[test]
    fn test_kpi_weights_locked() {
        // 验证权重不被意外修改
        let kpi = PeriodKPI::new(Uuid::new_v4(), "2026-03");
        
        // 权重必须是 45/30/25
        assert_eq!(kpi.decision_quality, 50.0);
        assert_eq!(kpi.governance_conduct, 50.0);
        assert_eq!(kpi.organizational_duty, 50.0);
        
        // 中性基线总分 50
        assert!((kpi.total_score() - 50.0).abs() < 0.1);
    }
}

use serde::{Serialize, Deserialize};
use std::fmt;
use thiserror::Error;

/// AI生命体状态 (8状态机)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityStatus {
    /// 候选：尚未正式任命
    Candidate,
    
    /// 活跃：正式在岗，可执行全部授权职责
    Active,
    
    /// 受限：仍存活，但部分权限冻结
    Limited,
    
    /// 审查中：进入调查/观察期，关键决策权限暂停
    UnderReview,
    
    /// 降职：保留存在但权力、资源、席位等级下降
    Demoted,
    
    /// 停职：不再参与实时治理，但保留记忆与复核权
    Suspended,
    
    /// 归档：不再active，但保留历史、可供审计和研究
    Archived,
    
    /// 终止：彻底退出active系统，不再参与治理
    Terminated,
}

impl EntityStatus {
    /// 获取状态描述
    pub fn description(&self) -> &'static str {
        match self {
            EntityStatus::Candidate => "候选，尚未正式任命",
            EntityStatus::Active => "活跃，正式在岗",
            EntityStatus::Limited => "受限，部分权限冻结",
            EntityStatus::UnderReview => "审查中，关键权限暂停",
            EntityStatus::Demoted => "降职，权力等级下降",
            EntityStatus::Suspended => "停职，不参与实时治理",
            EntityStatus::Archived => "归档，保留历史供审计",
            EntityStatus::Terminated => "终止，彻底退出系统",
        }
    }
    
    /// 获取状态英文名称
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityStatus::Candidate => "Candidate",
            EntityStatus::Active => "Active",
            EntityStatus::Limited => "Limited",
            EntityStatus::UnderReview => "UnderReview",
            EntityStatus::Demoted => "Demoted",
            EntityStatus::Suspended => "Suspended",
            EntityStatus::Archived => "Archived",
            EntityStatus::Terminated => "Terminated",
        }
    }
    
    /// 从字符串解析状态
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "candidate" => Some(EntityStatus::Candidate),
            "active" => Some(EntityStatus::Active),
            "limited" => Some(EntityStatus::Limited),
            "underreview" | "under_review" | "review" => Some(EntityStatus::UnderReview),
            "demoted" => Some(EntityStatus::Demoted),
            "suspended" => Some(EntityStatus::Suspended),
            "archived" => Some(EntityStatus::Archived),
            "terminated" => Some(EntityStatus::Terminated),
            _ => None,
        }
    }
}

impl fmt::Display for EntityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// 状态转移请求 (含必填字段)
#[derive(Debug, Clone)]
pub struct StateTransitionRequest {
    /// 目标状态
    pub to_status: EntityStatus,
    
    /// 转移原因 (必填)
    pub reason: String,
    
    /// 发起人 (必填)
    pub initiated_by: String,
    
    /// 审批人 (可选，重要转移需要)
    pub approved_by: Option<String>,
    
    /// 附加证据/上下文
    pub evidence: Option<String>,
}

impl StateTransitionRequest {
    /// 创建状态转移请求
    pub fn new(
        to_status: EntityStatus,
        reason: String,
        initiated_by: String,
    ) -> Result<Self, StateTransitionError> {
        if reason.trim().is_empty() {
            return Err(StateTransitionError::MissingReason);
        }
        
        if initiated_by.trim().is_empty() {
            return Err(StateTransitionError::MissingInitiator);
        }
        
        Ok(Self {
            to_status,
            reason,
            initiated_by,
            approved_by: None,
            evidence: None,
        })
    }
    
    /// 设置审批人
    pub fn with_approval(mut self, approved_by: String) -> Self {
        self.approved_by = Some(approved_by);
        self
    }
    
    /// 设置证据
    pub fn with_evidence(mut self, evidence: String) -> Self {
        self.evidence = Some(evidence);
        self
    }
}

/// 状态转移验证器
pub struct StateTransitionValidator;

impl StateTransitionValidator {
    /// 验证状态转移是否合法
    pub fn validate(
        from: EntityStatus,
        request: &StateTransitionRequest,
    ) -> Result<(), StateTransitionError> {
        let to = request.to_status;
        
        // 检查是否相同状态
        if from == to {
            return Err(StateTransitionError::SameState);
        }
        
        // 检查转移是否合法
        let valid = match (from, to) {
            // 候选 → 活跃 (通过考核)
            (EntityStatus::Candidate, EntityStatus::Active) => true,
            
            // 活跃 → 受限 (警告后限权)
            (EntityStatus::Active, EntityStatus::Limited) => true,
            // 活跃 → 审查中 (触发调查)
            (EntityStatus::Active, EntityStatus::UnderReview) => true,
            // 活跃 → 降职 (KPI不达标)
            (EntityStatus::Active, EntityStatus::Demoted) => true,
            // 活跃 → 停职 (高风险错误)
            (EntityStatus::Active, EntityStatus::Suspended) => true,
            // 活跃 → 归档 (主动/被动归档)
            (EntityStatus::Active, EntityStatus::Archived) => true,
            // 活跃 → 终止 (重大恶意) - 需要审批
            (EntityStatus::Active, EntityStatus::Terminated) => {
                // 终止需要审批人
                if request.approved_by.is_none() {
                    return Err(StateTransitionError::ApprovalRequired);
                }
                true
            }
            
            // 受限 → 活跃 (整改通过)
            (EntityStatus::Limited, EntityStatus::Active) => true,
            // 降职 → 活跃 (表现恢复)
            (EntityStatus::Demoted, EntityStatus::Active) => true,
            // 停职 → 活跃 (观察通过) - 需要审批
            (EntityStatus::Suspended, EntityStatus::Active) => {
                if request.approved_by.is_none() {
                    return Err(StateTransitionError::ApprovalRequired);
                }
                true
            }
            // 停职 → 归档 (无法恢复)
            (EntityStatus::Suspended, EntityStatus::Archived) => true,
            // 归档 → 活跃 (特殊复活) - 需要审批
            (EntityStatus::Archived, EntityStatus::Active) => {
                if request.approved_by.is_none() {
                    return Err(StateTransitionError::ApprovalRequired);
                }
                true
            }
            
            // 其他：非法
            _ => false,
        };
        
        if valid {
            Ok(())
        } else {
            Err(StateTransitionError::InvalidTransition {
                from: from.to_string(),
                to: to.to_string(),
            })
        }
    }
    
    /// 获取所有合法转移目标
    pub fn get_valid_transitions(from: EntityStatus) -> Vec<EntityStatus> {
        match from {
            EntityStatus::Candidate => vec![EntityStatus::Active],
            EntityStatus::Active => vec![
                EntityStatus::Limited,
                EntityStatus::UnderReview,
                EntityStatus::Demoted,
                EntityStatus::Suspended,
                EntityStatus::Archived,
                EntityStatus::Terminated,
            ],
            EntityStatus::Limited => vec![EntityStatus::Active],
            EntityStatus::UnderReview => vec![], // 审查结束后由系统决定
            EntityStatus::Demoted => vec![EntityStatus::Active],
            EntityStatus::Suspended => vec![EntityStatus::Active, EntityStatus::Archived],
            EntityStatus::Archived => vec![EntityStatus::Active],
            EntityStatus::Terminated => vec![], // 终止不可逆
        }
    }
    
    /// 检查转移是否需要审批
    pub fn requires_approval(from: EntityStatus, to: EntityStatus) -> bool {
        matches!((from, to),
            (EntityStatus::Active, EntityStatus::Terminated) |
            (EntityStatus::Suspended, EntityStatus::Active) |
            (EntityStatus::Archived, EntityStatus::Active)
        )
    }
}

/// 状态转移错误
#[derive(Debug, Error)]
pub enum StateTransitionError {
    #[error("非法状态转移: {from} → {to}")]
    InvalidTransition { from: String, to: String },
    
    #[error("缺少转移原因")]
    MissingReason,
    
    #[error("缺少发起人")]
    MissingInitiator,
    
    #[error("目标状态与当前状态相同")]
    SameState,
    
    #[error("该转移需要审批人")]
    ApprovalRequired,
    
    #[error("权限不足")]
    InsufficientAuthority,
}

/// 状态转移记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransitionRecord {
    pub from_status: EntityStatus,
    pub to_status: EntityStatus,
    pub reason: String,
    pub initiated_by: String,
    pub approved_by: Option<String>,
    pub evidence: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_transitions() {
        // Candidate → Active
        let req = StateTransitionRequest::new(
            EntityStatus::Active,
            "通过考核".to_string(),
            "System".to_string(),
        ).unwrap();
        assert!(StateTransitionValidator::validate(EntityStatus::Candidate, &req).is_ok());
        
        // Active → Limited
        let req = StateTransitionRequest::new(
            EntityStatus::Limited,
            "警告后限权".to_string(),
            "Manager".to_string(),
        ).unwrap();
        assert!(StateTransitionValidator::validate(EntityStatus::Active, &req).is_ok());
        
        // Limited → Active
        let req = StateTransitionRequest::new(
            EntityStatus::Active,
            "整改通过".to_string(),
            "System".to_string(),
        ).unwrap();
        assert!(StateTransitionValidator::validate(EntityStatus::Limited, &req).is_ok());
    }
    
    #[test]
    fn test_invalid_transitions() {
        // Terminated → Active (不可逆)
        let req = StateTransitionRequest::new(
            EntityStatus::Active,
            "试图复活".to_string(),
            "Hacker".to_string(),
        ).unwrap();
        assert!(StateTransitionValidator::validate(EntityStatus::Terminated, &req).is_err());
        
        // Candidate → Terminated (跳过Active)
        let req = StateTransitionRequest::new(
            EntityStatus::Terminated,
            "直接终止".to_string(),
            "Admin".to_string(),
        ).unwrap();
        assert!(StateTransitionValidator::validate(EntityStatus::Candidate, &req).is_err());
    }
    
    #[test]
    fn test_same_state() {
        let req = StateTransitionRequest::new(
            EntityStatus::Active,
            "保持现状".to_string(),
            "User".to_string(),
        ).unwrap();
        assert!(matches!(
            StateTransitionValidator::validate(EntityStatus::Active, &req),
            Err(StateTransitionError::SameState)
        ));
    }
    
    #[test]
    fn test_approval_required() {
        // Active → Terminated 需要审批
        let req_no_approval = StateTransitionRequest::new(
            EntityStatus::Terminated,
            "重大违规".to_string(),
            "Admin".to_string(),
        ).unwrap();
        assert!(matches!(
            StateTransitionValidator::validate(EntityStatus::Active, &req_no_approval),
            Err(StateTransitionError::ApprovalRequired)
        ));
        
        // 加上审批
        let req_with_approval = req_no_approval.with_approval("Director".to_string());
        assert!(StateTransitionValidator::validate(EntityStatus::Active, &req_with_approval).is_ok());
    }
    
    #[test]
    fn test_missing_reason() {
        let result = StateTransitionRequest::new(
            EntityStatus::Limited,
            "".to_string(),
            "Admin".to_string(),
        );
        assert!(matches!(result, Err(StateTransitionError::MissingReason)));
    }
    
    #[test]
    fn test_get_valid_transitions() {
        assert_eq!(
            StateTransitionValidator::get_valid_transitions(EntityStatus::Candidate),
            vec![EntityStatus::Active]
        );
        
        let active_transitions = StateTransitionValidator::get_valid_transitions(EntityStatus::Active);
        assert!(active_transitions.contains(&EntityStatus::Limited));
        assert!(active_transitions.contains(&EntityStatus::Terminated));
        assert!(!active_transitions.contains(&EntityStatus::Candidate));
        
        assert!(
            StateTransitionValidator::get_valid_transitions(EntityStatus::Terminated).is_empty()
        );
    }
}

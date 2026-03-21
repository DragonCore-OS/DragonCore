//! AI 生命体管理模块
//!
//! 提供 AI 实体的身份、状态机、存储、管理功能。
//!
//! 核心原则：
//! - 没有后果，就没有责任
//! - 没有责任，就没有生命体
//!
//! 8 状态机：
//! Candidate → Active → Limited/UnderReview/Demoted/Suspended → Archived/Terminated

pub mod attribution;
pub mod identity;
pub mod kpi;
pub mod status;
pub mod storage;
pub mod archive;
pub mod manager;

// 重新导出主要类型
pub use attribution::{DecisionAttribution, DecisionType, DecisionOutcome, DecisionImpact};
pub use identity::AIEntityIdentity;
pub use kpi::{PeriodKPI, KPIDetails, KPICalculator};
pub use status::{EntityStatus, StateTransitionRequest, StateTransitionValidator, StateTransitionRecord};
pub use storage::{EntityStorage, FileSystemStorage, MemoryStorage, StorageError};
pub use archive::{EntityRank, Department, Authority, EntityBaseArchive};
pub use manager::{EntityManager, EntityError};

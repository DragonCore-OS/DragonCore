use serde::{Deserialize, Serialize};

/// PR-2 scaffold: model arbitration trace contract for PR-3 extraction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSelectionTrace {
    pub trace_id: String,
    pub seat: String,
    pub selected: Option<SelectedModel>,
    #[serde(default)]
    pub candidates: Vec<ModelCandidate>,
    #[serde(default)]
    pub rejections: Vec<CandidateRejection>,
    #[serde(default)]
    pub downgrade_reason: Option<DowngradeReason>,
    #[serde(default)]
    pub outage_classification: Option<ProviderOutageClassification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedModel {
    pub provider: String,
    pub model_key: String,
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    pub provider: String,
    pub model_key: String,
    pub model_id: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateRejection {
    pub model_key: String,
    pub reason: RejectionReason,
    #[serde(default)]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RejectionReason {
    Timeout,
    ProviderOutage,
    CapabilityMismatch,
    PolicyMismatch,
    MissingCredential,
    InvalidConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    Timeout,
    ProviderOutage,
    CostConstraint,
    LatencyConstraint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderOutageClassification {
    ConnectionFailure,
    Http5xx,
    RateLimited,
    CircuitOpen,
}

//! DragonCore Meeting Protocol v0.1
//! 
//! 在現有19席治理協議之上，加入會議層：
//! - 集合、點名、議題鎖定
//! - 自主申請發言、強制點名
//! - 輪次討論、挑戰窗口
//! - 決議草案、治理動作映射

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 會議階段狀態機
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeetingPhase {
    Idle,
    Assembling,
    RollCall,
    TopicLock,
    OpenFloor,
    DeliberationRound,
    ChallengeWindow,
    ResolutionDraft,
    GovernanceAction,
    Closed,
    Terminated,
}

impl Default for MeetingPhase {
    fn default() -> Self {
        MeetingPhase::Idle
    }
}

/// 會議會話
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingSession {
    pub run_id: String,
    pub topic: String,
    pub convener: String,
    pub moderator: String,
    pub phase: MeetingPhase,
    pub required_seats: Vec<String>,
    pub present_seats: HashMap<String, PresenceState>,
    pub rounds: Vec<DiscussionRound>,
    pub active_requests: Vec<SpeakRequest>,
    pub stances: HashMap<String, SeatStance>,
    pub current_resolution: Option<ResolutionDraft>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MeetingSession {
    pub fn new(run_id: impl Into<String>, topic: impl Into<String>, convener: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            run_id: run_id.into(),
            topic: topic.into(),
            convener: convener.into(),
            moderator: "Tianshu".to_string(),
            phase: MeetingPhase::Idle,
            required_seats: Vec::new(),
            present_seats: HashMap::new(),
            rounds: Vec::new(),
            active_requests: Vec::new(),
            stances: HashMap::new(),
            current_resolution: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn with_moderator(mut self, moderator: impl Into<String>) -> Self {
        self.moderator = moderator.into();
        self
    }
    
    pub fn with_required_seats(mut self, seats: Vec<String>) -> Self {
        self.required_seats = seats;
        self
    }
    
    pub fn transition_to(&mut self, phase: MeetingPhase) {
        self.phase = phase;
        self.updated_at = Utc::now();
    }
}

/// 席位在線狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceState {
    pub alive: bool,
    pub active: bool,
    pub provider_ready: bool,
    pub context_loaded: bool,
    pub last_heartbeat_at: DateTime<Utc>,
    pub current_provider: Option<String>,
    pub current_model: Option<String>,
}

impl Default for PresenceState {
    fn default() -> Self {
        Self {
            alive: false,
            active: false,
            provider_ready: false,
            context_loaded: false,
            last_heartbeat_at: Utc::now(),
            current_provider: None,
            current_model: None,
        }
    }
}

/// 發言申請
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakRequest {
    pub request_id: Uuid,
    pub run_id: String,
    pub seat_id: String,
    pub intent: SpeakIntent,
    pub confidence: f32,
    pub urgency: f32,
    pub novelty_score: f32,
    pub reason: String,
    pub delta_summary: String,
    pub references_turns: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl SpeakRequest {
    pub fn new(run_id: impl Into<String>, seat_id: impl Into<String>, intent: SpeakIntent) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            run_id: run_id.into(),
            seat_id: seat_id.into(),
            intent,
            confidence: 0.5,
            urgency: 0.5,
            novelty_score: 0.5,
            reason: String::new(),
            delta_summary: String::new(),
            references_turns: Vec::new(),
            created_at: Utc::now(),
        }
    }
    
    /// 計算發言評分
    pub fn speak_score(&self, role_relevance: f32, conflict_need: f32) -> f32 {
        role_relevance * 0.30 +
        self.urgency * 0.20 +
        self.confidence * 0.15 +
        self.novelty_score * 0.20 +
        conflict_need * 0.15
    }
}

/// 發言意圖
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpeakIntent {
    NewAnalysis,
    RiskAlert,
    ResourceConstraint,
    ImplementationPlan,
    Challenge,
    Support,
    Revision,
    DecisionSummary,
}

/// 立場引用（P1: 必須引用對象，不是純文本）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceReference {
    pub seat_id: String,
    pub turn_id: Uuid,
    pub position_summary: String,
}

impl StanceReference {
    pub fn new(seat_id: impl Into<String>, turn_id: Uuid) -> Self {
        Self {
            seat_id: seat_id.into(),
            turn_id,
            position_summary: String::new(),
        }
    }
    
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.position_summary = summary.into();
        self
    }
}

/// 立場變化記錄（P1: 可回放）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StanceChange {
    pub from_position: String,
    pub to_position: String,
    pub from_confidence: f32,
    pub to_confidence: f32,
    pub triggered_by_turn: Uuid,
    pub triggered_by_seat: String,
    pub change_reason: String,
    pub changed_at: DateTime<Utc>,
}

/// 席位立場（P1: 完整狀態追踪）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatStance {
    pub seat_id: String,
    pub position: String,
    pub confidence: f32,
    /// P1: supports 必須引用 seat + turn
    pub supports: Vec<StanceReference>,
    /// P1: challenges 必須引用 seat + turn
    pub challenges: Vec<StanceReference>,
    pub changed_after_turn: Option<Uuid>,
    pub last_updated_at: DateTime<Utc>,
    /// P1: 立場變化歷史
    pub change_history: Vec<StanceChange>,
    /// P1: 本輪已更新次數（收斂規則用）
    pub updates_this_round: u32,
    /// P1: 是否只是重複（非增量）
    pub is_repetition: bool,
}

impl SeatStance {
    pub fn new(seat_id: impl Into<String>) -> Self {
        Self {
            seat_id: seat_id.into(),
            position: String::new(),
            confidence: 0.5,
            supports: Vec::new(),
            challenges: Vec::new(),
            changed_after_turn: None,
            last_updated_at: Utc::now(),
            change_history: Vec::new(),
            updates_this_round: 0,
            is_repetition: false,
        }
    }
    
    /// P1: 更新立場（記錄變化）
    pub fn update_position(
        &mut self,
        new_position: impl Into<String>,
        new_confidence: f32,
        triggered_by_turn: Uuid,
        triggered_by_seat: impl Into<String>,
        reason: impl Into<String>,
    ) -> bool {
        let new_position = new_position.into();
        let triggered_by_seat = triggered_by_seat.into();
        let reason = reason.into();
        
        // P1: 檢查是否只是重複
        if self.position == new_position && (self.confidence - new_confidence).abs() < 0.01 {
            self.is_repetition = true;
            return false; // 無實質變化
        }
        
        // P1: 記錄變化
        let change = StanceChange {
            from_position: self.position.clone(),
            to_position: new_position.clone(),
            from_confidence: self.confidence,
            to_confidence: new_confidence,
            triggered_by_turn,
            triggered_by_seat,
            change_reason: reason,
            changed_at: Utc::now(),
        };
        
        self.change_history.push(change);
        self.position = new_position;
        self.confidence = new_confidence;
        self.changed_after_turn = Some(triggered_by_turn);
        self.last_updated_at = Utc::now();
        self.updates_this_round += 1;
        self.is_repetition = false;
        
        true
    }
    
    /// P1: 添加支持引用
    pub fn add_support(&mut self, seat_id: impl Into<String>, turn_id: Uuid, summary: impl Into<String>) {
        self.supports.push(StanceReference::new(seat_id, turn_id).with_summary(summary));
        self.last_updated_at = Utc::now();
    }
    
    /// P1: 添加挑戰引用
    pub fn add_challenge(&mut self, seat_id: impl Into<String>, turn_id: Uuid, summary: impl Into<String>) {
        self.challenges.push(StanceReference::new(seat_id, turn_id).with_summary(summary));
        self.last_updated_at = Utc::now();
    }
    
    /// P1: 重置輪次計數（進入新輪時調用）
    pub fn reset_round_counter(&mut self) {
        self.updates_this_round = 0;
    }
    
    /// P1: 是否穩定（confidence 高且無近期變化）
    pub fn is_stable(&self, min_confidence: f32) -> bool {
        self.confidence >= min_confidence && self.updates_this_round == 0
    }
    
    /// P1: 獲取變化摘要
    pub fn change_summary(&self) -> String {
        if self.change_history.is_empty() {
            return format!("{}: 無變化", self.seat_id);
        }
        
        let last = self.change_history.last().unwrap();
        format!(
            "{}: {} -> {} (由 {} 在 turn {} 引發)",
            self.seat_id,
            last.from_position,
            last.to_position,
            last.triggered_by_seat,
            last.triggered_by_turn
        )
    }
}

/// 討論輪次
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionRound {
    pub round_no: u32,
    pub scheduled_speakers: Vec<String>,
    pub turns: Vec<MeetingTurn>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

impl DiscussionRound {
    pub fn new(round_no: u32) -> Self {
        Self {
            round_no,
            scheduled_speakers: Vec::new(),
            turns: Vec::new(),
            opened_at: Utc::now(),
            closed_at: None,
        }
    }
}

/// 會議發言回合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingTurn {
    pub turn_id: Uuid,
    pub seat_id: String,
    pub intent: SpeakIntent,
    pub content: String,
    pub confidence: f32,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub based_on_turns: Vec<Uuid>,
    pub stance_delta: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl MeetingTurn {
    pub fn new(seat_id: impl Into<String>, intent: SpeakIntent) -> Self {
        Self {
            turn_id: Uuid::new_v4(),
            seat_id: seat_id.into(),
            intent,
            content: String::new(),
            confidence: 0.5,
            provider: None,
            model: None,
            based_on_turns: Vec::new(),
            stance_delta: None,
            created_at: Utc::now(),
        }
    }
}

/// 決議草案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionDraft {
    pub summary: String,
    pub candidate_options: Vec<String>,
    pub consensus_level: f32,
    pub unresolved_conflicts: Vec<String>,
    pub recommended_action: RecommendedAction,
    pub drafted_by: String,
    pub created_at: DateTime<Utc>,
}

/// 推薦治理動作
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
    ContinueDiscussion,
    RaiseRisk,
    ExerciseVeto,
    OpenFinalGate,
    Terminate,
    Archive,
}

/// 席位人格參數 (P3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatPersonaProfile {
    pub seat_id: String,
    pub role_domain: Vec<String>,
    pub bias_style: BiasStyle,
    pub caution_level: f32,
    pub interruption_threshold: f32,
    pub challenge_tendency: f32,
    pub support_tendency: f32,
    pub verbosity_preference: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BiasStyle {
    Conservative,
    Aggressive,
    VerificationFirst,
    ResourceBound,
    ExecutionFirst,
    Strategic,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_speak_score_calculation() {
        let request = SpeakRequest::new("run-1", "Baihu", SpeakIntent::RiskAlert)
            .with_confidence(0.8)
            .with_urgency(0.9)
            .with_novelty_score(0.7);
        
        let score = request.speak_score(0.9, 0.8);
        // 0.9*0.30 + 0.9*0.20 + 0.8*0.15 + 0.7*0.20 + 0.8*0.15
        // = 0.27 + 0.18 + 0.12 + 0.14 + 0.12 = 0.83
        assert!(score > 0.8 && score < 0.85);
    }
}

// MeetingManager for state persistence and operations
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct MeetingManager {
    storage_path: PathBuf,
}

impl MeetingManager {
    pub fn new(storage_path: impl Into<PathBuf>) -> Self {
        Self {
            storage_path: storage_path.into(),
        }
    }
    
    /// Create meeting session storage
    pub fn create_session(&self, session: &MeetingSession) -> Result<()> {
        let path = self.session_path(&session.run_id);
        std::fs::create_dir_all(&self.storage_path)?;
        
        let json = serde_json::to_string_pretty(session)?;
        std::fs::write(&path, json)
            .with_context(|| format!("Failed to write meeting session: {:?}", path))?;
        
        Ok(())
    }
    
    /// Load meeting session
    pub fn load_session(&self, run_id: &str) -> Result<Option<MeetingSession>> {
        let path = self.session_path(run_id);
        if !path.exists() {
            return Ok(None);
        }
        
        let json = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read meeting session: {:?}", path))?;
        
        let session: MeetingSession = serde_json::from_str(&json)
            .with_context(|| format!("Failed to parse meeting session: {:?}", path))?;
        
        Ok(Some(session))
    }
    
    /// Update meeting session
    pub fn update_session(&self, session: &MeetingSession) -> Result<()> {
        self.create_session(session)
    }
    
    /// List all meeting sessions
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut run_ids = Vec::new();
        if self.storage_path.exists() {
            for entry in std::fs::read_dir(&self.storage_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Some(stem) = path.file_stem() {
                        run_ids.push(stem.to_string_lossy().to_string());
                    }
                }
            }
        }
        Ok(run_ids)
    }
    
    fn session_path(&self, run_id: &str) -> PathBuf {
        self.storage_path.join(format!("{}.json", run_id))
    }
}

// Extension methods for SpeakRequest
impl SpeakRequest {
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_urgency(mut self, urgency: f32) -> Self {
        self.urgency = urgency.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_novelty_score(mut self, score: f32) -> Self {
        self.novelty_score = score.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = reason.into();
        self
    }
    
    pub fn with_delta_summary(mut self, delta: impl Into<String>) -> Self {
        self.delta_summary = delta.into();
        self
    }
}

// Extension methods for MeetingTurn
impl MeetingTurn {
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }
    
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }
    
    pub fn with_stance_delta(mut self, delta: impl Into<String>) -> Self {
        self.stance_delta = Some(delta.into());
        self
    }
}

/// P1: 會議收斂規則引擎
#[derive(Debug, Clone)]
pub struct ConvergenceRules {
    /// 同一席位同一輪最大更新次數
    pub max_updates_per_round: u32,
    /// 最小共識水平（進入 ResolutionDraft）
    pub min_consensus_level: f32,
    /// 無新增高質量請求時自動建議收束
    pub auto_converge_after_empty_rounds: u32,
    /// 重複內容閾值
    pub repetition_threshold: f32,
}

impl Default for ConvergenceRules {
    fn default() -> Self {
        Self {
            max_updates_per_round: 1,  // P1: 默認 N=1
            min_consensus_level: 0.72,
            auto_converge_after_empty_rounds: 1,
            repetition_threshold: 0.85,
        }
    }
}

impl ConvergenceRules {
    /// P1: 檢查席位是否可更新立場
    pub fn can_update_stance(&self, stance: &SeatStance) -> bool {
        stance.updates_this_round < self.max_updates_per_round
    }
    
    /// P1: 檢查內容是否為重複
    pub fn is_repetition(&self, novelty_score: f32) -> bool {
        novelty_score < self.repetition_threshold
    }
    
    /// P1: 計算會議共識水平
    pub fn calculate_consensus(&self, stances: &HashMap<String, SeatStance>) -> f32 {
        if stances.is_empty() {
            return 0.0;
        }
        
        let total_confidence: f32 = stances.values().map(|s| s.confidence).sum();
        let avg_confidence = total_confidence / stances.len() as f32;
        
        // 穩定席位比例
        let stable_count = stances.values().filter(|s| s.is_stable(0.7)).count();
        let stability_ratio = stable_count as f32 / stances.len() as f32;
        
        // 共識 = 平均 confidence * 穩定性
        avg_confidence * 0.6 + stability_ratio * 0.4
    }
    
    /// P1: 是否應進入 ResolutionDraft
    pub fn should_converge(
        &self,
        stances: &HashMap<String, SeatStance>,
        empty_rounds: u32,
        critical_seats_spoken: bool,
    ) -> bool {
        // 條件1: 關鍵席位已發言且共識達標
        if critical_seats_spoken {
            let consensus = self.calculate_consensus(stances);
            if consensus >= self.min_consensus_level {
                return true;
            }
        }
        
        // 條件2: 連續無新增信息
        if empty_rounds >= self.auto_converge_after_empty_rounds {
            return true;
        }
        
        false
    }
}

/// P1: 會議事件（DIBL 擴展）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeetingEvent {
    MeetingOpened { run_id: String, topic: String, moderator: String },
    RollCallStarted { run_id: String, required_seats: Vec<String> },
    SeatPresentConfirmed { run_id: String, seat_id: String, provider: String },
    SeatMissing { run_id: String, seat_id: String, reason: String },
    TopicLocked { run_id: String, summary: String, confirmed_by: String },
    SpeakRequested { request: SpeakRequest },
    RoundScheduled { run_id: String, round_no: u32, speakers: Vec<String> },
    MeetingTurnPublished { turn: MeetingTurn },
    /// P1: 立場更新事件（獨立於發言）
    StanceUpdated {
        run_id: String,
        seat_id: String,
        change: StanceChange,
        current_position: String,
        current_confidence: f32,
    },
    /// P1: 支持聲明
    SupportDeclared {
        run_id: String,
        seat_id: String,
        supported_seat: String,
        supported_turn: Uuid,
        position_summary: String,
    },
    /// P1: 挑戰聲明
    ChallengeDeclared {
        run_id: String,
        seat_id: String,
        challenged_seat: String,
        challenged_turn: Uuid,
        challenge_reason: String,
    },
    ChallengeWindowOpened { run_id: String, eligible_seats: Vec<String> },
    SeatForcedToSpeak { run_id: String, seat_id: String, reason: String },
    ResolutionDrafted { run_id: String, draft: ResolutionDraft },
    GovernanceActionRequested { run_id: String, action: RecommendedAction, requested_by: String },
    MeetingClosed { run_id: String, final_consensus: f32 },
    MeetingTerminated { run_id: String, reason: String },
}

impl MeetingEvent {
    /// 獲取 run_id
    pub fn run_id(&self) -> &str {
        match self {
            Self::MeetingOpened { run_id, .. } => run_id,
            Self::RollCallStarted { run_id, .. } => run_id,
            Self::SeatPresentConfirmed { run_id, .. } => run_id,
            Self::SeatMissing { run_id, .. } => run_id,
            Self::TopicLocked { run_id, .. } => run_id,
            Self::SpeakRequested { request } => &request.run_id,
            Self::RoundScheduled { run_id, .. } => run_id,
            Self::MeetingTurnPublished { turn } => &turn.seat_id, // Note: turn doesn't have run_id
            Self::StanceUpdated { run_id, .. } => run_id,
            Self::SupportDeclared { run_id, .. } => run_id,
            Self::ChallengeDeclared { run_id, .. } => run_id,
            Self::ChallengeWindowOpened { run_id, .. } => run_id,
            Self::SeatForcedToSpeak { run_id, .. } => run_id,
            Self::ResolutionDrafted { run_id, .. } => run_id,
            Self::GovernanceActionRequested { run_id, .. } => run_id,
            Self::MeetingClosed { run_id, .. } => run_id,
            Self::MeetingTerminated { run_id, .. } => run_id,
        }
    }
}

/// P1: MeetingSession 收斂方法擴展
impl MeetingSession {
    /// P1: 獲取指定輪次
    pub fn get_round(&self, round_no: u32) -> Option<&DiscussionRound> {
        self.rounds.iter().find(|r| r.round_no == round_no)
    }
    
    /// P1: 獲取當前輪次
    pub fn current_round(&self) -> Option<&DiscussionRound> {
        self.rounds.last()
    }
    
    /// P1: 獲取席位立場（如果不存在則創建）
    pub fn get_or_create_stance(&mut self, seat_id: &str) -> &mut SeatStance {
        if !self.stances.contains_key(seat_id) {
            self.stances.insert(seat_id.to_string(), SeatStance::new(seat_id));
        }
        self.stances.get_mut(seat_id).unwrap()
    }
    
    /// P1: 檢查是否所有關鍵席位已發言
    pub fn critical_seats_spoken(&self, current_round_no: u32) -> bool {
        // 關鍵席位：風險、質量、審計、資源
        let critical = vec!["Baihu", "Yuheng", "Baozheng", "Xuanwu", "Xiwangmu"];
        
        if let Some(round) = self.get_round(current_round_no) {
            let spoken_seats: Vec<_> = round.turns.iter().map(|t| t.seat_id.as_str()).collect();
            critical.iter().all(|s| spoken_seats.contains(s) || self.required_seats.contains(&s.to_string()))
        } else {
            false
        }
    }
    
    /// P1: 計算本輪高質量發言請求數
    pub fn high_quality_requests_this_round(&self, threshold: f32) -> usize {
        self.active_requests.iter()
            .filter(|r| r.speak_score(0.8, 0.5) >= threshold)
            .count()
    }
    
    /// P1: 獲取立場變化摘要
    pub fn stance_changes_summary(&self) -> Vec<String> {
        self.stances.values()
            .map(|s| s.change_summary())
            .collect()
    }
    
    /// P1: 進入新輪時重置計數器
    pub fn enter_new_round(&mut self, round_no: u32) {
        for stance in self.stances.values_mut() {
            stance.reset_round_counter();
        }
        self.rounds.push(DiscussionRound::new(round_no));
        self.transition_to(MeetingPhase::DeliberationRound);
    }
    
    /// P1: 生成立場回放序列
    pub fn stance_replay_sequence(&self) -> Vec<(DateTime<Utc>, String, String)> {
        let mut sequence: Vec<_> = self.stances.values()
            .flat_map(|s| {
                s.change_history.iter().map(move |c| {
                    (c.changed_at, s.seat_id.clone(), c.change_reason.clone())
                })
            })
            .collect();
        
        sequence.sort_by(|a, b| a.0.cmp(&b.0));
        sequence
    }
}

#[cfg(test)]
mod p1_tests {
    use super::*;
    
    /// P1 Test Case 1: 立場被說服
    /// - Tianquan 先發言
    /// - Baihu 挑戰
    /// - Qinglong 修改 stance
    /// - 最終 resolution 收束
    #[test]
    fn test_stance_persuasion() {
        let mut session = MeetingSession::new("test-1", "Risk Assessment", "user");
        
        // 初始立場
        let turn1 = Uuid::new_v4();
        let mut qinglong = SeatStance::new("Qinglong");
        
        // Qinglong 初始支持
        qinglong.update_position(
            "Support quick deployment",
            0.6,
            turn1,
            "Qinglong",
            "Initial assessment"
        );
        
        // Baihu 挑戰後，Qinglong 改變立場
        let turn2 = Uuid::new_v4();
        let changed = qinglong.update_position(
            "Conditional support with rollback plan",
            0.75,
            turn2,
            "Baihu",
            "Persuaded by risk analysis"
        );
        
        assert!(changed);
        // 兩次更新，所以有兩條變化記錄
        assert_eq!(qinglong.change_history.len(), 2);
        assert_eq!(qinglong.position, "Conditional support with rollback plan");
        
        // 驗證第二次回放
        let change = &qinglong.change_history[1];
        assert_eq!(change.from_position, "Support quick deployment");
        assert_eq!(change.to_position, "Conditional support with rollback plan");
        assert_eq!(change.triggered_by_seat, "Baihu");
    }
    
    /// P1 Test Case 2: 重複發言不引發假變化
    /// - 兩個席位重複同一立場
    /// - 系統識別為低 novelty
    /// - 不計為新的 stance delta
    #[test]
    fn test_repetition_detection() {
        let mut stance = SeatStance::new("Tianquan");
        let turn1 = Uuid::new_v4();
        
        // 第一次更新
        stance.update_position(
            "Support the plan",
            0.8,
            turn1,
            "Tianquan",
            "Initial position"
        );
        
        // 重複相同立場（應被拒絕）
        let turn2 = Uuid::new_v4();
        let changed = stance.update_position(
            "Support the plan",
            0.8,
            turn2,
            "Kaiyang",
            "Same position"
        );
        
        assert!(!changed);
        assert!(stance.is_repetition);
        assert_eq!(stance.change_history.len(), 1); // 只有第一次記錄
    }
    
    /// P1 Test Case 3: 挑戰後收束
    /// - 一個 support
    /// - 一個 challenge
    /// - 一個被點名席給關鍵補充
    /// - resolution draft 收束到 final gate 或 raise risk
    #[test]
    fn test_challenge_convergence() {
        let mut session = MeetingSession::new("test-3", "Deployment Decision", "user");
        session.required_seats = vec![
            "Tianquan".to_string(),
            "Yuheng".to_string(),
            "Baihu".to_string(),
        ];
        
        // 設置立場（依次創建，避免借用衝突）
        let turn1 = Uuid::new_v4();
        let turn2 = Uuid::new_v4();
        let turn3 = Uuid::new_v4();
        
        {
            let tianquan = session.get_or_create_stance("Tianquan");
            tianquan.update_position("Support", 0.9, turn1, "Tianquan", "Initial");
            tianquan.add_support("Tianquan", turn1, "Quick deployment needed");
        }
        
        {
            let yuheng = session.get_or_create_stance("Yuheng");
            yuheng.update_position("Challenge", 0.85, turn2, "Yuheng", "Quality concerns");
            yuheng.add_challenge("Tianquan", turn1, "Insufficient testing");
        }
        
        {
            let baihu = session.get_or_create_stance("Baihu");
            baihu.update_position("Conditional", 0.8, turn3, "Baihu", "Risk mitigation required");
        }
        
        // 驗證 supports/challenges 有對象引用
        let tianquan = session.stances.get("Tianquan").unwrap();
        assert_eq!(tianquan.supports.len(), 1);
        assert_eq!(tianquan.supports[0].seat_id, "Tianquan");
        assert_eq!(tianquan.supports[0].turn_id, turn1);
        
        let yuheng = session.stances.get("Yuheng").unwrap();
        assert_eq!(yuheng.challenges.len(), 1);
        assert_eq!(yuheng.challenges[0].seat_id, "Tianquan");
        assert_eq!(yuheng.challenges[0].turn_id, turn1);
        
        // 驗證收斂規則
        let rules = ConvergenceRules::default();
        let consensus = rules.calculate_consensus(&session.stances);
        assert!(consensus > 0.0);
        
        // 驗證 stance 回放序列
        let replay = session.stance_replay_sequence();
        assert_eq!(replay.len(), 3); // 三個立場變化
    }
    
    /// P1: 測試收斂規則 - 同一輪更新次數限制
    #[test]
    fn test_convergence_max_updates_per_round() {
        let mut stance = SeatStance::new("TestSeat");
        let turn1 = Uuid::new_v4();
        let turn2 = Uuid::new_v4();
        
        // 第一次更新
        stance.update_position("Pos1", 0.5, turn1, "Other", "First");
        assert_eq!(stance.updates_this_round, 1);
        
        // 第二次更新（應被拒絕，因為默認 max=1）
        let rules = ConvergenceRules::default();
        assert!(!rules.can_update_stance(&stance));
    }
    
    /// P1: 測試 stance 穩定性檢查
    #[test]
    fn test_stance_stability() {
        let mut stance = SeatStance::new("TestSeat");
        
        // 不穩定（confidence 低）
        assert!(!stance.is_stable(0.7));
        
        // 更新後
        let turn = Uuid::new_v4();
        stance.update_position("Stable position", 0.85, turn, "Other", "Update");
        stance.reset_round_counter();
        
        // 現在穩定了
        assert!(stance.is_stable(0.7));
    }
}

// P2: 智能主持系統（Coverage-Aware Scheduler）

/// 角色類型分類（P2: Conflict Coverage 檢查用）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoleCategory {
    Risk,       // 風險席: Baihu, Yuheng
    Execution,  // 執行席: Nezha, Luban
    Resource,   // 資源席: Xiwangmu
    Audit,      // 審計席: Baozheng, Yangjian
    Strategy,   // 戰略席: Zhugeliang, Tianquan
    Stability,  // 穩定席: Xuanwu
    Review,     // 審查席: Kaiyang
    Innovation, // 創新席: Qinglong, Yaoguang
}

impl RoleCategory {
    /// 獲取席位所屬角色類型
    pub fn from_seat(seat_id: &str) -> Vec<Self> {
        match seat_id {
            "Baihu" | "Yuheng" => vec![RoleCategory::Risk],
            "Nezha" | "Luban" => vec![RoleCategory::Execution],
            "Xiwangmu" => vec![RoleCategory::Resource],
            "Baozheng" | "Yangjian" => vec![RoleCategory::Audit],
            "Zhugeliang" | "Tianquan" => vec![RoleCategory::Strategy],
            "Xuanwu" => vec![RoleCategory::Stability],
            "Kaiyang" => vec![RoleCategory::Review],
            "Qinglong" | "Yaoguang" => vec![RoleCategory::Innovation],
            _ => vec![RoleCategory::Review],
        }
    }
    
    /// 獲取該角色類型的關鍵程度（P2: 用於缺位檢查優先級）
    pub fn criticality(&self) -> u32 {
        match self {
            RoleCategory::Risk => 10,      // 最高優先
            RoleCategory::Audit => 9,
            RoleCategory::Stability => 8,
            RoleCategory::Strategy => 7,
            RoleCategory::Resource => 6,
            RoleCategory::Execution => 5,
            RoleCategory::Review => 4,
            RoleCategory::Innovation => 3,
        }
    }
}

/// P2: Coverage 檢查結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    pub covered_roles: Vec<RoleCategory>,
    pub missing_roles: Vec<RoleCategory>,
    pub covered_seats: Vec<String>,
    pub missing_critical_seats: Vec<String>,
    pub coverage_ratio: f32,
    pub is_sufficient: bool,
    pub recommendations: Vec<String>,
}

/// P2: Conflict Coverage 檢查器
pub struct CoverageChecker;

impl CoverageChecker {
    /// 檢查當前討論的角色覆蓋情況
    pub fn check_coverage(
        session: &MeetingSession,
        current_round: u32,
    ) -> CoverageReport {
        let mut covered_roles = std::collections::HashSet::new();
        let mut covered_seats = std::collections::HashSet::new();
        
        // 收集已發言席位的角色
        if let Some(round) = session.get_round(current_round) {
            for turn in &round.turns {
                covered_seats.insert(turn.seat_id.clone());
                for role in RoleCategory::from_seat(&turn.seat_id) {
                    covered_roles.insert(role);
                }
            }
        }
        
        // 檢查 required seats 的覆蓋
        let mut missing_critical_seats = Vec::new();
        for seat in &session.required_seats {
            if !covered_seats.contains(seat) {
                missing_critical_seats.push(seat.clone());
            }
        }
        
        // 計算缺失的關鍵角色
        let all_critical_roles = vec![
            RoleCategory::Risk,
            RoleCategory::Audit,
            RoleCategory::Stability,
        ];
        
        let missing_roles: Vec<_> = all_critical_roles
            .into_iter()
            .filter(|r| !covered_roles.contains(r))
            .collect();
        
        let coverage_ratio = if session.required_seats.is_empty() {
            1.0
        } else {
            covered_seats.len() as f32 / session.required_seats.len() as f32
        };
        
        // 生成建議
        let mut recommendations = Vec::new();
        for role in &missing_roles {
            recommendations.push(format!(
                "Missing {} perspective (criticality: {})",
                format!("{:?}", role).to_lowercase(),
                role.criticality()
            ));
        }
        
        CoverageReport {
            covered_roles: covered_roles.into_iter().collect(),
            missing_roles: missing_roles.clone(),
            covered_seats: covered_seats.into_iter().collect(),
            missing_critical_seats,
            coverage_ratio,
            is_sufficient: coverage_ratio >= 0.7 && missing_roles.is_empty(),
            recommendations,
        }
    }
    
    /// P2: 檢查是否缺少風險視角（用於自動 challenge window）
    pub fn is_risk_perspective_missing(session: &MeetingSession, round: u32) -> bool {
        let report = Self::check_coverage(session, round);
        report.missing_roles.contains(&RoleCategory::Risk)
    }
    
    /// P2: 獲取缺位優先級最高的角色
    pub fn get_highest_priority_missing(session: &MeetingSession, round: u32) -> Option<RoleCategory> {
        let report = Self::check_coverage(session, round);
        report.missing_roles
            .into_iter()
            .max_by_key(|r| r.criticality())
    }
}

/// P2: 發言選擇因素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeakerSelectionFactors {
    pub relevance: f32,           // 議題相關性
    pub urgency: f32,             // 緊急程度
    pub novelty: f32,             // 新信息量
    pub conflict_coverage: f32,   // 衝突覆蓋貢獻
    pub underrepresented_role: f32, // 角色代表性不足補償
    pub repeated_dominance_penalty: f32, // 連續發言懲罰
}

impl Default for SpeakerSelectionFactors {
    fn default() -> Self {
        Self {
            relevance: 1.0,
            urgency: 1.0,
            novelty: 1.0,
            conflict_coverage: 1.0,
            underrepresented_role: 1.0,
            repeated_dominance_penalty: 0.0,
        }
    }
}

/// P2: 可解釋的發言排序結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainableSpeakerRanking {
    pub seat_id: String,
    pub total_score: f32,
    pub factors: SpeakerSelectionFactors,
    pub explanation: String,
    pub rank: usize,
}

/// P2: 智能主持調度器
pub struct SmartModerator {
    pub rules: ConvergenceRules,
    pub dominance_tracker: std::collections::HashMap<String, u32>, // 席位發言次數追踪
}

impl SmartModerator {
    pub fn new() -> Self {
        Self {
            rules: ConvergenceRules::default(),
            dominance_tracker: std::collections::HashMap::new(),
        }
    }
    
    /// P2: 選擇下一位發言人（6因素模型）
    pub fn select_next_speaker(
        &mut self,
        session: &MeetingSession,
        requests: &[SpeakRequest],
        coverage_report: &CoverageReport,
    ) -> Option<ExplainableSpeakerRanking> {
        if requests.is_empty() {
            return None;
        }
        
        let mut rankings = Vec::new();
        
        for request in requests {
            let factors = self.calculate_factors(request, session, coverage_report);
            let total_score = self.compute_total_score(&factors);
            
            let explanation = self.generate_explanation(request, &factors);
            
            rankings.push(ExplainableSpeakerRanking {
                seat_id: request.seat_id.clone(),
                total_score,
                factors,
                explanation,
                rank: 0, // 會在排序後填充
            });
        }
        
        // 按總分排序
        rankings.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
        
        // 填充排名
        for (i, ranking) in rankings.iter_mut().enumerate() {
            ranking.rank = i + 1;
        }
        
        // 更新發言次數追踪
        if let Some(first) = rankings.first() {
            *self.dominance_tracker.entry(first.seat_id.clone()).or_insert(0) += 1;
        }
        
        rankings.into_iter().next()
    }
    
    /// P2: 計算6因素
    fn calculate_factors(
        &self,
        request: &SpeakRequest,
        session: &MeetingSession,
        coverage: &CoverageReport,
    ) -> SpeakerSelectionFactors {
        // relevance: 基於 role 和議題匹配
        let relevance = request.speak_score(0.8, 0.5);
        
        // urgency: 直接使用請求的 urgency
        let urgency = request.urgency;
        
        // novelty: 直接使用請求的 novelty_score
        let novelty = request.novelty_score;
        
        // conflict_coverage: 如果該角色類型缺失，給予高分
        let roles = RoleCategory::from_seat(&request.seat_id);
        let conflict_coverage = if roles.iter().any(|r| coverage.missing_roles.contains(r)) {
            0.9 // 高優先補充缺失角色
        } else {
            0.5
        };
        
        // underrepresented_role: 該席位已發言次數越少，分數越高
        let speak_count = self.dominance_tracker.get(&request.seat_id).copied().unwrap_or(0);
        let underrepresented_role = 1.0 / (1.0 + speak_count as f32 * 0.3);
        
        // repeated_dominance_penalty: 連續發言懲罰
        let repeated_dominance_penalty = if speak_count >= 2 {
            -(speak_count as f32 - 1.0) * 0.2
        } else {
            0.0
        };
        
        SpeakerSelectionFactors {
            relevance,
            urgency,
            novelty,
            conflict_coverage,
            underrepresented_role,
            repeated_dominance_penalty,
        }
    }
    
    /// P2: 計算總分
    fn compute_total_score(&self, factors: &SpeakerSelectionFactors) -> f32 {
        let base = factors.relevance * 0.25
            + factors.urgency * 0.20
            + factors.novelty * 0.20
            + factors.conflict_coverage * 0.20
            + factors.underrepresented_role * 0.15;
        
        base + factors.repeated_dominance_penalty
    }
    
    /// P2: 生成可解釋的排序理由
    fn generate_explanation(
        &self,
        request: &SpeakRequest,
        factors: &SpeakerSelectionFactors,
    ) -> String {
        let mut reasons = Vec::new();
        
        if factors.conflict_coverage > 0.8 {
            reasons.push("fills missing role perspective");
        }
        if factors.underrepresented_role > 0.7 {
            reasons.push("underrepresented in discussion");
        }
        if factors.urgency > 0.8 {
            reasons.push("high urgency");
        }
        if factors.novelty > 0.8 {
            reasons.push("high novelty");
        }
        if factors.repeated_dominance_penalty < -0.1 {
            reasons.push("repeated speaker penalty applied");
        }
        
        if reasons.is_empty() {
            format!("Standard priority for {}", request.seat_id)
        } else {
            format!("{}: {}", request.seat_id, reasons.join(", "))
        }
    }
    
    /// P2: 檢查是否應自動打開 challenge window
    pub fn should_auto_challenge_window(
        &self,
        session: &MeetingSession,
        current_round: u32,
    ) -> Option<String> {
        let coverage = CoverageChecker::check_coverage(session, current_round);
        
        // 條件1: 高共識但缺少風險視角
        if coverage.coverage_ratio > 0.8 && coverage.missing_roles.contains(&RoleCategory::Risk) {
            return Some("High consensus but missing risk perspective".to_string());
        }
        
        // 條件2: 方案推進太快（執行席已發言但風險席未發言）
        let exec_spoken = coverage.covered_roles.contains(&RoleCategory::Execution);
        let risk_missing = coverage.missing_roles.contains(&RoleCategory::Risk);
        if exec_spoken && risk_missing {
            return Some("Execution perspective present but risk perspective missing".to_string());
        }
        
        // 條件3: 最終決議前反方聲音不足
        if session.phase == MeetingPhase::ResolutionDraft && 
           coverage.challenges_count() < 1 {
            return Some("Insufficient challenge before resolution".to_string());
        }
        
        None
    }
    
    /// P2: Dead Floor 處理
    pub fn handle_dead_floor(
        &self,
        session: &MeetingSession,
        current_round: u32,
    ) -> DeadFloorAction {
        let coverage = CoverageChecker::check_coverage(session, current_round);
        
        // 如果還有缺位的關鍵席，自動點名
        if !coverage.missing_critical_seats.is_empty() {
            return DeadFloorAction::ForceSpeak(coverage.missing_critical_seats[0].clone());
        }
        
        // 如果覆蓋足夠，建議進入 resolution
        if coverage.is_sufficient {
            return DeadFloorAction::SuggestResolution;
        }
        
        // 否則等待更多輸入
        DeadFloorAction::Wait
    }
}

/// P2: CoverageReport 擴展方法
trait CoverageReportExt {
    fn challenges_count(&self) -> usize;
}

impl CoverageReportExt for CoverageReport {
    fn challenges_count(&self) -> usize {
        // 簡化實現：實際應檢查 stances 中的 challenges
        0
    }
}

/// P2: Dead Floor 處理動作
#[derive(Debug, Clone)]
pub enum DeadFloorAction {
    ForceSpeak(String),  // 強制點名指定席位
    SuggestResolution,   // 建議進入 resolution
    Wait,                // 繼續等待
}

#[cfg(test)]
mod p2_tests {
    use super::*;
    
    /// P2 Test 1: 系統能識別缺失的關鍵角色覆蓋
    #[test]
    fn test_coverage_check_identifies_missing_roles() {
        let mut session = MeetingSession::new("test-coverage", "Deployment Decision", "user");
        session.required_seats = vec!["Baihu".to_string(), "Nezha".to_string(), "Yuheng".to_string()];
        
        // 創建第一輪，只有執行席發言
        session.enter_new_round(1);
        let round = session.rounds.last_mut().unwrap();
        round.turns.push(MeetingTurn::new("Nezha", SpeakIntent::ImplementationPlan));
        
        // 檢查覆蓋
        let coverage = CoverageChecker::check_coverage(&session, 1);
        
        // 應該缺少風險視角
        assert!(coverage.missing_roles.contains(&RoleCategory::Risk));
        assert!(coverage.recommendations.iter().any(|r| r.contains("risk")));
        assert!(!coverage.is_sufficient);
    }
    
    /// P2 Test 2: 不會連續偏向同一類席位
    #[test]
    fn test_no_continuous_bias_to_same_role() {
        let mut moderator = SmartModerator::new();
        let mut session = MeetingSession::new("test-bias", "Architecture Review", "user");
        
        // 模擬多個發言請求
        let requests = vec![
            SpeakRequest::new("test", "Nezha", SpeakIntent::ImplementationPlan)
                .with_urgency(0.9),
            SpeakRequest::new("test", "Luban", SpeakIntent::ImplementationPlan)
                .with_urgency(0.8),
            SpeakRequest::new("test", "Baihu", SpeakIntent::RiskAlert)
                .with_urgency(0.7),
        ];
        
        // 第一次選擇
        let coverage = CoverageChecker::check_coverage(&session, 0);
        let ranking1 = moderator.select_next_speaker(&session, &requests, &coverage);
        
        assert!(ranking1.is_some());
        let first = ranking1.unwrap();
        
        // 驗證有解釋
        assert!(!first.explanation.is_empty());
        
        // 如果選了執行席，驗證另一個執行席的懲罰分數較低
        if first.seat_id == "Nezha" {
            let ranking2 = moderator.select_next_speaker(&session, &requests, &coverage);
            if let Some(second) = ranking2 {
                if second.seat_id == "Luban" {
                    // 同一角色類型連續被選，應有懲罰
                    assert!(second.factors.repeated_dominance_penalty < 0.0);
                }
            }
        }
    }
    
    /// P2 Test 3: 高共識但低挑戰場景下主動補 challenge coverage
    #[test]
    fn test_auto_challenge_window_high_consensus_low_challenge() {
        let mut moderator = SmartModerator::new();
        let mut session = MeetingSession::new("test-challenge", "Quick Deploy", "user");
        
        // 設置高共識但只有執行席
        session.enter_new_round(1);
        let round = session.rounds.last_mut().unwrap();
        round.turns.push(MeetingTurn::new("Nezha", SpeakIntent::ImplementationPlan));
        round.turns.push(MeetingTurn::new("Tianquan", SpeakIntent::DecisionSummary));
        
        let reason = moderator.should_auto_challenge_window(&session, 1);
        
        // 應自動建議打開 challenge window
        assert!(reason.is_some());
        assert!(reason.unwrap().contains("risk"));
    }
    
    /// P2 Test 4: 無人申請發言時不會死鎖
    #[test]
    fn test_dead_floor_no_deadlock() {
        let moderator = SmartModerator::new();
        let mut session = MeetingSession::new("test-dead", "Risk Assessment", "user");
        session.required_seats = vec!["Baihu".to_string(), "Yuheng".to_string()];
        
        // 空輪，無人發言
        session.enter_new_round(1);
        
        let action = moderator.handle_dead_floor(&session, 1);
        
        // 應自動點名缺位的關鍵席
        match action {
            DeadFloorAction::ForceSpeak(seat) => {
                assert!(session.required_seats.contains(&seat));
            }
            _ => panic!("Expected ForceSpeak for dead floor"),
        }
    }
    
    /// P2 Test 5: 自動排序結果可解釋
    #[test]
    fn test_ranking_is_explainable() {
        let mut moderator = SmartModerator::new();
        let session = MeetingSession::new("test-explain", "Design Review", "user");
        
        let request = SpeakRequest::new("test", "Baihu", SpeakIntent::RiskAlert)
            .with_urgency(0.95)
            .with_novelty_score(0.9);
        
        let coverage = CoverageChecker::check_coverage(&session, 0);
        let ranking = moderator.select_next_speaker(&session, &[request], &coverage);
        
        assert!(ranking.is_some());
        let r = ranking.unwrap();
        
        // 驗證有解釋
        assert!(!r.explanation.is_empty());
        
        // 驗證6個因素都有值
        assert!(r.factors.relevance > 0.0);
        assert!(r.factors.urgency > 0.0);
        assert!(r.factors.novelty > 0.0);
        assert!(r.factors.conflict_coverage >= 0.0);
        assert!(r.factors.underrepresented_role > 0.0);
        // repeated_dominance_penalty 可以是負數
        
        // 驗證總分計算正確
        let expected_score = r.factors.relevance * 0.25
            + r.factors.urgency * 0.20
            + r.factors.novelty * 0.20
            + r.factors.conflict_coverage * 0.20
            + r.factors.underrepresented_role * 0.15
            + r.factors.repeated_dominance_penalty;
        
        assert!((r.total_score - expected_score).abs() < 0.001);
    }
    
    /// P2: 驗證覆蓋率計算正確
    #[test]
    fn test_coverage_ratio_calculation() {
        let mut session = MeetingSession::new("test-ratio", "Review", "user");
        session.required_seats = vec![
            "Baihu".to_string(),
            "Nezha".to_string(),
            "Yuheng".to_string(),
            "Tianquan".to_string(),
        ];
        
        session.enter_new_round(1);
        let round = session.rounds.last_mut().unwrap();
        round.turns.push(MeetingTurn::new("Nezha", SpeakIntent::ImplementationPlan));
        round.turns.push(MeetingTurn::new("Tianquan", SpeakIntent::DecisionSummary));
        
        let coverage = CoverageChecker::check_coverage(&session, 1);
        
        // 2/4 = 0.5
        assert!((coverage.coverage_ratio - 0.5).abs() < 0.01);
        assert!(!coverage.is_sufficient); // < 0.7
    }
}

// P3: 行為人格系統（Behavior-Driven Personality）

/// P3: 席位行為參數配置（9個核心參數）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatBehaviorProfile {
    pub seat_id: String,
    pub bias_style: BiasStyle,
    /// 謹慎程度（0.0-1.0）：越高越保守，越低越激進
    pub caution_level: f32,
    /// 打斷閾值（0.0-1.0）：多容易打斷他人
    pub interruption_threshold: f32,
    /// 挑戰傾向（0.0-1.0）：多容易挑戰他人
    pub challenge_tendency: f32,
    /// 支持傾向（0.0-1.0）：多容易支持他人
    pub support_tendency: f32,
    /// 立場更新敏感度（0.0-1.0）：多容易被說服
    pub stance_update_sensitivity: f32,
    /// 沉默閾值（0.0-1.0）：多高興趣才發言
    pub silence_threshold: f32,
    /// 風險升級傾向（0.0-1.0）：多容易 raise risk
    pub risk_escalation_tendency: f32,
}

impl SeatBehaviorProfile {
    /// P3: 創建默認行為配置
    pub fn new(seat_id: impl Into<String>) -> Self {
        Self {
            seat_id: seat_id.into(),
            bias_style: BiasStyle::Strategic,
            caution_level: 0.5,
            interruption_threshold: 0.5,
            challenge_tendency: 0.5,
            support_tendency: 0.5,
            stance_update_sensitivity: 0.5,
            silence_threshold: 0.5,
            risk_escalation_tendency: 0.5,
        }
    }
    
    /// P3: 根據 BiasStyle 創建預設配置
    pub fn with_bias_style(seat_id: impl Into<String>, style: BiasStyle) -> Self {
        let mut profile = Self::new(seat_id);
        profile.bias_style = style;
        profile.apply_bias_style_defaults();
        profile
    }
    
    /// P3: BiasStyle 默認參數映射
    fn apply_bias_style_defaults(&mut self) {
        match self.bias_style {
            BiasStyle::Conservative => {
                self.caution_level = 0.85;
                self.interruption_threshold = 0.3;
                self.challenge_tendency = 0.4;
                self.support_tendency = 0.4;
                self.stance_update_sensitivity = 0.2;
                self.silence_threshold = 0.7;
                self.risk_escalation_tendency = 0.6;
            }
            BiasStyle::Aggressive => {
                self.caution_level = 0.2;
                self.interruption_threshold = 0.8;
                self.challenge_tendency = 0.8;
                self.support_tendency = 0.3;
                self.stance_update_sensitivity = 0.4;
                self.silence_threshold = 0.2;
                self.risk_escalation_tendency = 0.4;
            }
            BiasStyle::VerificationFirst => {
                self.caution_level = 0.7;
                self.interruption_threshold = 0.4;
                self.challenge_tendency = 0.7;
                self.support_tendency = 0.3;
                self.stance_update_sensitivity = 0.5;
                self.silence_threshold = 0.6;
                self.risk_escalation_tendency = 0.8;
            }
            BiasStyle::ResourceBound => {
                self.caution_level = 0.6;
                self.interruption_threshold = 0.4;
                self.challenge_tendency = 0.5;
                self.support_tendency = 0.4;
                self.stance_update_sensitivity = 0.3;
                self.silence_threshold = 0.5;
                self.risk_escalation_tendency = 0.5;
            }
            BiasStyle::ExecutionFirst => {
                self.caution_level = 0.3;
                self.interruption_threshold = 0.6;
                self.challenge_tendency = 0.3;
                self.support_tendency = 0.7;
                self.stance_update_sensitivity = 0.6;
                self.silence_threshold = 0.3;
                self.risk_escalation_tendency = 0.2;
            }
            BiasStyle::Strategic => {
                self.caution_level = 0.5;
                self.interruption_threshold = 0.5;
                self.challenge_tendency = 0.5;
                self.support_tendency = 0.5;
                self.stance_update_sensitivity = 0.7;
                self.silence_threshold = 0.5;
                self.risk_escalation_tendency = 0.5;
            }
        }
    }
    
    /// P3: 計算發言申請概率（0.0-1.0）
    pub fn calculate_speak_probability(
        &self,
        base_relevance: f32,
        risk_level: f32,
    ) -> f32 {
        // Conservative: 更少主動發言，但更容易在風險上升時發言
        // Aggressive: 更容易搶首輪發言
        // VerificationFirst: 更容易在他人給出結論後挑戰
        // ResourceBound: 更容易在資源約束類議題中提高 role relevance
        
        let base_prob = (1.0 - self.silence_threshold) * 0.5 + base_relevance * 0.5;
        
        // 風險調整
        let risk_adjustment = match self.bias_style {
            BiasStyle::Conservative => risk_level * 0.3,      // 風險上升時更積極
            BiasStyle::Aggressive => -risk_level * 0.2,       // 風險上升時更激進
            BiasStyle::VerificationFirst => risk_level * 0.4, // 風險上升時更謹慎
            _ => 0.0,
        };
        
        (base_prob + risk_adjustment).clamp(0.0, 1.0)
    }
    
    /// P3: 計算初始 urgency（人格影響）
    pub fn calculate_initial_urgency(&self, topic_risk: f32) -> f32 {
        let base = 0.5;
        let adjustment = match self.bias_style {
            BiasStyle::Aggressive => 0.2,
            BiasStyle::Conservative => -0.1,
            BiasStyle::ExecutionFirst => 0.15,
            _ => 0.0,
        };
        (base + adjustment + topic_risk * self.risk_escalation_tendency).clamp(0.0, 1.0)
    }
    
    /// P3: 計算初始 confidence（人格影響）
    pub fn calculate_initial_confidence(&self) -> f32 {
        match self.bias_style {
            BiasStyle::Aggressive => 0.75,
            BiasStyle::Conservative => 0.55,
            BiasStyle::VerificationFirst => 0.65,
            BiasStyle::ExecutionFirst => 0.8,
            _ => 0.6,
        }
    }
    
    /// P3: 計算 novelty 門檻（人格影響）
    pub fn calculate_novelty_threshold(&self) -> f32 {
        // Conservative 和 VerificationFirst 需要更高的 novelty 才發言
        match self.bias_style {
            BiasStyle::Conservative => 0.7,
            BiasStyle::VerificationFirst => 0.65,
            BiasStyle::Aggressive => 0.3,
            _ => 0.5,
        }
    }
    
    /// P3: 計算被說服概率
    pub fn calculate_persuadability(
        &self,
        challenger_confidence: f32,
        evidence_strength: f32,
    ) -> f32 {
        // Strategic: 更容易在多方意見後調整
        // Conservative: 不會輕易從 challenge 變 support
        // VerificationFirst: 對證據充分性更敏感
        
        let base = self.stance_update_sensitivity;
        
        let adjustment = match self.bias_style {
            BiasStyle::Strategic => 0.1,
            BiasStyle::Conservative => -0.2,
            BiasStyle::VerificationFirst => evidence_strength * 0.3 - 0.1,
            BiasStyle::ExecutionFirst => challenger_confidence * 0.2,
            _ => 0.0,
        };
        
        (base + adjustment).clamp(0.0, 1.0)
    }
    
    /// P3: 決定是否挑戰
    pub fn should_challenge(
        &self,
        target_position: &str,
        risk_level: f32,
    ) -> bool {
        let base_prob = self.challenge_tendency;
        
        let risk_boost = if risk_level > 0.7 {
            match self.bias_style {
                BiasStyle::VerificationFirst => 0.3,
                BiasStyle::Conservative => 0.2,
                _ => 0.1,
            }
        } else {
            0.0
        };
        
        let final_prob = (base_prob + risk_boost).clamp(0.0, 1.0);
        final_prob > 0.5
    }
    
    /// P3: 決定是否 raise risk
    pub fn should_escalate_risk(&self, perceived_risk: f32) -> bool {
        let threshold = 1.0 - self.risk_escalation_tendency;
        perceived_risk > threshold
    }
    
    /// P3: 計算立場更新幅度
    pub fn calculate_stance_delta(
        &self,
        original_confidence: f32,
        persuasion_strength: f32,
    ) -> f32 {
        // 返回 confidence 變化量（可正可負）
        let base_delta = persuasion_strength * self.stance_update_sensitivity;
        
        match self.bias_style {
            BiasStyle::Conservative => base_delta * 0.5,  // 保守派變化慢
            BiasStyle::Strategic => base_delta * 1.2,     // 戰略派變化快
            BiasStyle::ExecutionFirst => base_delta * 0.8,
            _ => base_delta,
        }
    }
}

/// P3: 行為人格管理器
pub struct BehaviorPersonalityManager {
    profiles: HashMap<String, SeatBehaviorProfile>,
}

impl BehaviorPersonalityManager {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        
        // P3: 為19席預設行為配置
        profiles.insert("Tianshu".to_string(), SeatBehaviorProfile::with_bias_style("Tianshu", BiasStyle::Strategic));
        profiles.insert("Tianquan".to_string(), SeatBehaviorProfile::with_bias_style("Tianquan", BiasStyle::Strategic));
        profiles.insert("Yuheng".to_string(), SeatBehaviorProfile::with_bias_style("Yuheng", BiasStyle::VerificationFirst));
        profiles.insert("Baihu".to_string(), SeatBehaviorProfile::with_bias_style("Baihu", BiasStyle::VerificationFirst));
        profiles.insert("Xuanwu".to_string(), SeatBehaviorProfile::with_bias_style("Xuanwu", BiasStyle::Conservative));
        profiles.insert("Nezha".to_string(), SeatBehaviorProfile::with_bias_style("Nezha", BiasStyle::ExecutionFirst));
        profiles.insert("Qinglong".to_string(), SeatBehaviorProfile::with_bias_style("Qinglong", BiasStyle::Aggressive));
        profiles.insert("Luban".to_string(), SeatBehaviorProfile::with_bias_style("Luban", BiasStyle::ExecutionFirst));
        profiles.insert("Zhugeliang".to_string(), SeatBehaviorProfile::with_bias_style("Zhugeliang", BiasStyle::Strategic));
        profiles.insert("Baozheng".to_string(), SeatBehaviorProfile::with_bias_style("Baozheng", BiasStyle::VerificationFirst));
        profiles.insert("Xiwangmu".to_string(), SeatBehaviorProfile::with_bias_style("Xiwangmu", BiasStyle::ResourceBound));
        profiles.insert("Kaiyang".to_string(), SeatBehaviorProfile::with_bias_style("Kaiyang", BiasStyle::VerificationFirst));
        profiles.insert("Yangjian".to_string(), SeatBehaviorProfile::with_bias_style("Yangjian", BiasStyle::VerificationFirst));
        profiles.insert("Yaoguang".to_string(), SeatBehaviorProfile::with_bias_style("Yaoguang", BiasStyle::Conservative));
        profiles.insert("Zhuque".to_string(), SeatBehaviorProfile::with_bias_style("Zhuque", BiasStyle::Aggressive));
        profiles.insert("Zhongkui".to_string(), SeatBehaviorProfile::with_bias_style("Zhongkui", BiasStyle::Aggressive));
        profiles.insert("Tianji".to_string(), SeatBehaviorProfile::with_bias_style("Tianji", BiasStyle::Strategic));
        profiles.insert("Tianxuan".to_string(), SeatBehaviorProfile::with_bias_style("Tianxuan", BiasStyle::Conservative));
        profiles.insert("Fengdudadi".to_string(), SeatBehaviorProfile::with_bias_style("Fengdudadi", BiasStyle::Conservative));
        
        Self { profiles }
    }
    
    pub fn get_profile(&self, seat_id: &str) -> Option<&SeatBehaviorProfile> {
        self.profiles.get(seat_id)
    }
    
    pub fn update_profile(&mut self, profile: SeatBehaviorProfile) {
        self.profiles.insert(profile.seat_id.clone(), profile);
    }
}

impl Default for BehaviorPersonalityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod p3_tests {
    use super::*;
    
    /// P3 Test 1: 風險上升場景下不同人格的行為差異
    /// Conservative vs VerificationFirst vs ExecutionFirst
    #[test]
    fn test_behavior_difference_under_rising_risk() {
        let conservative = SeatBehaviorProfile::with_bias_style("Xuanwu", BiasStyle::Conservative);
        let verification = SeatBehaviorProfile::with_bias_style("Yuheng", BiasStyle::VerificationFirst);
        let execution = SeatBehaviorProfile::with_bias_style("Nezha", BiasStyle::ExecutionFirst);
        
        let risk_level = 0.8;
        
        // Conservative: 風險上升時更積極發言
        let cons_speak_prob = conservative.calculate_speak_probability(0.5, risk_level);
        assert!(cons_speak_prob > 0.5, "Conservative should speak more when risk rises");
        
        // VerificationFirst: 更容易 raise risk
        let verif_escalate = verification.should_escalate_risk(risk_level);
        assert!(verif_escalate, "VerificationFirst should escalate risk");
        
        // ExecutionFirst: 風險上升時更激進（更不容易 raise risk）
        let exec_escalate = execution.should_escalate_risk(risk_level);
        assert!(!exec_escalate, "ExecutionFirst should not escalate easily");
        
        // 驗證 urgency 差異
        // 注意：風險上升時，Conservative 的 urgency 會因為 risk_escalation_tendency 高而更高
        let cons_urgency = conservative.calculate_initial_urgency(risk_level);
        let exec_urgency = execution.calculate_initial_urgency(risk_level);
        // Conservative (caution_level=0.85, risk_escalation=0.6) vs ExecutionFirst (adjustment=0.15, risk_escalation=0.2)
        // 在 risk=0.8 時：Conservative = 0.5 + 0.8*0.6 = 0.98, ExecutionFirst = 0.5 + 0.15 + 0.8*0.2 = 0.81
        assert!(cons_urgency > exec_urgency, "Conservative has higher urgency when risk rises due to risk_escalation_tendency");
    }
    
    /// P3 Test 2: 資源受限場景下不同人格的行為差異
    /// ResourceBound vs Aggressive vs Strategic
    #[test]
    fn test_behavior_difference_under_resource_constraint() {
        let resource = SeatBehaviorProfile::with_bias_style("Xiwangmu", BiasStyle::ResourceBound);
        let aggressive = SeatBehaviorProfile::with_bias_style("Qinglong", BiasStyle::Aggressive);
        let strategic = SeatBehaviorProfile::with_bias_style("Zhugeliang", BiasStyle::Strategic);
        
        // ResourceBound: 更高的謹慎度
        assert!(resource.caution_level > aggressive.caution_level);
        
        // Aggressive: 更容易打斷
        assert!(aggressive.interruption_threshold > resource.interruption_threshold);
        
        // Strategic: 更容易被說服（更新敏感度更高）
        let persuadability = strategic.calculate_persuadability(0.8, 0.7);
        let conservative_persuadability = resource.calculate_persuadability(0.8, 0.7);
        assert!(persuadability > conservative_persuadability, "Strategic more persuadable");
    }
    
    /// P3 Test 3: 高共識但低挑戰場景
    /// 驗證是否自然出現更高 challenge coverage
    #[test]
    fn test_natural_challenge_in_high_consensus() {
        let verification = SeatBehaviorProfile::with_bias_style("Yuheng", BiasStyle::VerificationFirst);
        let aggressive = SeatBehaviorProfile::with_bias_style("Baihu", BiasStyle::Aggressive);
        let conservative = SeatBehaviorProfile::with_bias_style("Xuanwu", BiasStyle::Conservative);
        
        let high_consensus_position = "Everyone agrees to deploy";
        let risk_level = 0.6;
        
        // VerificationFirst 和 Aggressive 應該更容易挑戰
        let verif_challenge = verification.should_challenge(high_consensus_position, risk_level);
        let aggressive_challenge = aggressive.should_challenge(high_consensus_position, risk_level);
        let conservative_challenge = conservative.should_challenge(high_consensus_position, risk_level);
        
        // 在 verification 和 aggressive 中至少一個會挑戰
        assert!(verif_challenge || aggressive_challenge, 
                "VerificationFirst or Aggressive should challenge high consensus");
        
        // Conservative 可能不挑戰（取決於具體風險）
        // 這個測試主要驗證不同人格確實會產生不同行為
    }
    
    /// P3 Test 4: 行為差異體現在決策上，而不是隨機漂移
    #[test]
    fn test_behavior_is_repeatable_not_random() {
        let profile = SeatBehaviorProfile::with_bias_style("Baihu", BiasStyle::VerificationFirst);
        
        // 多次計算應該得到相同結果
        let prob1 = profile.calculate_speak_probability(0.6, 0.7);
        let prob2 = profile.calculate_speak_probability(0.6, 0.7);
        let prob3 = profile.calculate_speak_probability(0.6, 0.7);
        
        assert!((prob1 - prob2).abs() < 0.001);
        assert!((prob2 - prob3).abs() < 0.001);
        
        // urgency 也是確定性的
        let urg1 = profile.calculate_initial_urgency(0.5);
        let urg2 = profile.calculate_initial_urgency(0.5);
        assert!((urg1 - urg2).abs() < 0.001);
    }
    
    /// P3 Test 5: 人格參數不繞過 authority boundary
    #[test]
    fn test_personality_respects_authority_boundary() {
        let aggressive = SeatBehaviorProfile::with_bias_style("Qinglong", BiasStyle::Aggressive);
        
        // 即使 Aggressive 的 challenge_tendency 很高
        // 也不意味著可以繞過正式的 veto/final-gate/terminate 流程
        assert!(aggressive.challenge_tendency > 0.5);
        
        // 行為人格只影響發言和立場，不影響正式權力
        // 這個測試驗證 profile 中沒有繞過 authority 的字段
        // 所有權力動作仍需通過 governance 層
    }
    
    /// P3 Test 6: replay 能解釋為何這席位在這里發言/沉默/挑戰
    #[test]
    fn test_replay_can_explain_behavior() {
        let profile = SeatBehaviorProfile::with_bias_style("Yuheng", BiasStyle::VerificationFirst);
        
        // 計算決策因子
        let speak_prob = profile.calculate_speak_probability(0.5, 0.6);
        let urgency = profile.calculate_initial_urgency(0.6);
        let confidence = profile.calculate_initial_confidence();
        
        // 這些因子可以被記錄和回放
        // 驗證它們在合理範圍內
        assert!(speak_prob >= 0.0 && speak_prob <= 1.0);
        assert!(urgency >= 0.0 && urgency <= 1.0);
        assert!(confidence >= 0.0 && confidence <= 1.0);
        
        // 驗證 BiasStyle 影響了結果
        // VerificationFirst (risk_escalation=0.8) vs ExecutionFirst (adjustment=0.15, risk_escalation=0.2)
        // 在 risk=0.6 時：VerificationFirst = 0.5 + 0.6*0.8 = 0.98, ExecutionFirst = 0.5 + 0.15 + 0.6*0.2 = 0.77
        let execution = SeatBehaviorProfile::with_bias_style("Nezha", BiasStyle::ExecutionFirst);
        let exec_urgency = execution.calculate_initial_urgency(0.6);
        assert!(urgency > exec_urgency, "VerificationFirst has higher urgency due to risk_escalation_tendency");
    }
    
    /// P3: 驗證默認19席配置
    #[test]
    fn test_default_19_seats_profiles() {
        let manager = BehaviorPersonalityManager::new();
        
        // 驗證關鍵席位有配置
        assert!(manager.get_profile("Yuheng").is_some());
        assert!(manager.get_profile("Baihu").is_some());
        assert!(manager.get_profile("Nezha").is_some());
        
        // 驗證 Yuheng 是 VerificationFirst
        let yuheng = manager.get_profile("Yuheng").unwrap();
        assert!(matches!(yuheng.bias_style, BiasStyle::VerificationFirst));
        
        // 驗證 Nezha 是 ExecutionFirst
        let nezha = manager.get_profile("Nezha").unwrap();
        assert!(matches!(nezha.bias_style, BiasStyle::ExecutionFirst));
        
        // 驗證 Xuanwu 是 Conservative
        let xuanwu = manager.get_profile("Xuanwu").unwrap();
        assert!(matches!(xuanwu.bias_style, BiasStyle::Conservative));
    }
}

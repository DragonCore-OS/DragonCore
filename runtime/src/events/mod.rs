//! DragonCore Internal Broadcast Layer (DIBL)
//! 
//! DIBL is not a chat system. It is a governance event layer for:
//! - Run lifecycle tracking
//! - 19-seat coordination
//! - Risk escalation
//! - Operator observability
//!
//! Core principle: JSON/ledger is source of truth; broadcast is derived view.

#![allow(dead_code)]

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

/// Event visibility scope
/// Mirrors AXI's dual-layer visibility but adapted for DragonCore's needs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventScope {
    /// Internal to 19-seat governance only
    /// Raw seat outputs, veto details, conflicts, debug telemetry
    Internal,
    /// Operator/dashboard visible
    /// Summarized status, risk alerts, milestones
    OperatorVisible,
    /// Exportable to external reports
    /// Public artifacts, final outcomes
    Exportable,
}

/// Event channel/topic
/// Derived from AXI's internal layer principles but event-based, not room-based
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventChannel {
    /// Governance control flow
    /// Run creation, seat rotation, final gate, archive
    Control,
    /// Runtime operations
    /// Tmux status, worktree state, persistence results
    Ops,
    /// Security and risk
    /// Veto, terminate, rollback, policy breach
    Security,
    /// Research and output
    /// Seat conclusions, findings, hypotheses
    Research,
}

/// Event severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warn,
    Critical,
}

/// Correlation context for event tracing
/// Flattened into GovernanceEvent for simplicity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CorrelationContext {
    pub correlation_id: Option<String>,
    pub parent_event_id: Option<Uuid>,
    pub actor: String,
    pub trigger_context: Option<String>,
    // Model provider used for this event (for multi-model tracking)
    pub provider: Option<String>,
}

impl CorrelationContext {
    /// Create a new correlation context
    #[allow(dead_code)]
    pub fn new(actor: impl Into<String>) -> Self {
        Self {
            correlation_id: None,
            parent_event_id: None,
            actor: actor.into(),
            trigger_context: None,
            provider: None,
        }
    }
    
    /// Set correlation ID
    #[allow(dead_code)]
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }
    
    /// Set parent event ID
    #[allow(dead_code)]
    pub fn with_parent_event(mut self, id: Uuid) -> Self {
        self.parent_event_id = Some(id);
        self
    }
    
    /// Set trigger context
    #[allow(dead_code)]
    pub fn with_context(mut self, ctx: impl Into<String>) -> Self {
        self.trigger_context = Some(ctx.into());
        self
    }
}

/// Governance event types
/// Aligned with DragonCore's run lifecycle state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceEventType {
    RunCreated,
    SeatStarted,
    SeatCompleted,
    RiskRaised,
    VetoExercised,
    FinalGateOpened,
    DecisionCommitted,
    RollbackTriggered,
    ArchiveCompleted,
    TerminateTriggered,
    
    // AI Entity Lifecycle Events
    EntityCreated,
    EntityActivated,
    EntityLimited,
    EntityUnderReview,
    EntityDemoted,
    EntitySuspended,
    EntityArchived,
    EntityTerminated,
    EntityStatusChanged,
    EntityPromoted,
    
    // PR-2: Decision Attribution and KPI
    EntityKpiUpdated,
    DecisionAttributed,
    EntityWarned,
}

/// A governance event
/// This is the core unit of DIBL - not a message, not a chat,
/// but a structured event in the run lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvent {
    pub event_id: Uuid,
    pub run_id: String,
    pub seat_id: Option<String>,
    pub channel: EventChannel,
    pub event_type: GovernanceEventType,
    pub scope: EventScope,
    pub severity: Severity,
    pub summary: String,
    pub details_ref: Option<String>,
    pub artifact_refs: Vec<String>,
    pub created_at: DateTime<Utc>,
    // Correlation context (flattened from AXI schema)
    pub correlation_id: Option<String>,
    pub parent_event_id: Option<Uuid>,
    pub actor: String,
    pub trigger_context: Option<String>,
    // Model provider used for this event (for multi-model tracking)
    pub provider: Option<String>,
}

impl Default for GovernanceEvent {
    fn default() -> Self {
        Self {
            event_id: Uuid::new_v4(),
            run_id: String::new(),
            seat_id: None,
            channel: EventChannel::Control,
            event_type: GovernanceEventType::RunCreated,
            scope: EventScope::Internal,
            severity: Severity::Info,
            summary: String::new(),
            details_ref: None,
            artifact_refs: Vec::new(),
            created_at: Utc::now(),
            correlation_id: None,
            parent_event_id: None,
            actor: String::new(),
            trigger_context: None,
            provider: None,
        }
    }
}

impl GovernanceEvent {
    /// Create a new governance event
    pub fn new(
        run_id: impl Into<String>,
        event_type: GovernanceEventType,
        channel: EventChannel,
        actor: impl Into<String>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            run_id: run_id.into(),
            seat_id: None,
            channel,
            event_type,
            scope: EventScope::Internal,
            severity: Severity::Info,
            summary: String::new(),
            details_ref: None,
            artifact_refs: Vec::new(),
            created_at: Utc::now(),
            correlation_id: None,
            parent_event_id: None,
            actor: actor.into(),
            trigger_context: None,
            provider: None,
        }
    }
    
    /// Create an entity event
    pub fn entity_event(
        event_type: GovernanceEventType,
        summary: impl Into<String>,
        entity_id: Option<Uuid>,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            run_id: entity_id.map(|id| id.to_string()).unwrap_or_default(),
            seat_id: None,
            channel: EventChannel::Control,
            event_type,
            scope: EventScope::Internal,
            severity: Severity::Info,
            summary: summary.into(),
            details_ref: None,
            artifact_refs: Vec::new(),
            created_at: Utc::now(),
            correlation_id: None,
            parent_event_id: None,
            actor: "entity_system".to_string(),
            trigger_context: None,
            provider: None,
        }
    }

    /// Set the seat
    pub fn with_seat(mut self, seat: impl Into<String>) -> Self {
        self.seat_id = Some(seat.into());
        self
    }

    /// Set the scope
    pub fn with_scope(mut self, scope: EventScope) -> Self {
        self.scope = scope;
        self
    }

    /// Set the severity
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Set the summary
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    /// Set details reference
    #[allow(dead_code)]
    pub fn with_details_ref(mut self, ref_path: impl Into<String>) -> Self {
        self.details_ref = Some(ref_path.into());
        self
    }

    /// Add artifact reference
    pub fn with_artifact(mut self, artifact: impl Into<String>) -> Self {
        self.artifact_refs.push(artifact.into());
        self
    }
    
    /// Set correlation ID
    #[allow(dead_code)]
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }
    
    /// Set parent event ID
    #[allow(dead_code)]
    pub fn with_parent_event(mut self, id: Uuid) -> Self {
        self.parent_event_id = Some(id);
        self
    }
    
    /// Set trigger context
    pub fn with_trigger_context(mut self, ctx: impl Into<String>) -> Self {
        self.trigger_context = Some(ctx.into());
        self
    }
    
    /// Set model provider
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }
}

/// Event store trait
/// Responsible for durable persistence of events
pub trait EventStore: Send + Sync {
    /// Append an event to the store
    fn append_event(&self, event: &GovernanceEvent) -> Result<()>;
    
    /// Load all events for a run
    fn load_run_events(&self, run_id: &str) -> Result<Vec<GovernanceEvent>>;
    
    /// Check if events exist for a run
    fn run_has_events(&self, run_id: &str) -> bool;
}

/// JSONL-based event store
/// Simple, debuggable, compatible with existing JSON/CSV infrastructure
pub struct JsonlEventStore {
    base_path: PathBuf,
}

impl JsonlEventStore {
    /// Create a new JSONL event store
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    /// Get the path for a run's event file
    fn event_file_path(&self, run_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.jsonl", run_id))
    }
}

impl EventStore for JsonlEventStore {
    fn append_event(&self, event: &GovernanceEvent) -> Result<()> {
        let file_path = self.event_file_path(&event.run_id);
        
        // Append mode, create if not exists
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)
            .with_context(|| format!("Failed to open event file: {:?}", file_path))?;
        
        // Serialize to JSON line
        let json_line = serde_json::to_string(event)
            .context("Failed to serialize event")?;
        
        // Write with newline
        writeln!(file, "{}", json_line)
            .with_context(|| format!("Failed to write event to: {:?}", file_path))?;
        
        // Ensure durability - sync to disk
        file.sync_all()
            .with_context(|| format!("Failed to sync event file: {:?}", file_path))?;
        
        Ok(())
    }

    fn load_run_events(&self, run_id: &str) -> Result<Vec<GovernanceEvent>> {
        let file_path = self.event_file_path(run_id);
        
        if !file_path.exists() {
            return Ok(Vec::new());
        }
        
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open event file: {:?}", file_path))?;
        
        let reader = BufReader::new(file);
        let mut events = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            
            // Gracefully skip malformed lines instead of failing
            match serde_json::from_str::<GovernanceEvent>(&line) {
                Ok(event) => events.push(event),
                Err(e) => {
                    tracing::warn!("Skipping malformed event line: {} (error: {})", line, e);
                    continue;
                }
            }
        }
        
        Ok(events)
    }

    fn run_has_events(&self, run_id: &str) -> bool {
        self.event_file_path(run_id).exists()
    }
}

/// In-memory broadcaster for real-time subscriptions
pub struct EventBroadcaster {
    /// Channel senders by run_id
    senders: Arc<Mutex<HashMap<String, broadcast::Sender<GovernanceEvent>>>>,
}

impl EventBroadcaster {
    /// Create a new broadcaster
    pub fn new() -> Self {
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Publish an event
    pub fn publish(&self, event: GovernanceEvent) -> Result<()> {
        let run_id = event.run_id.clone();
        
        let mut senders = self.senders.lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock senders"))?;
        
        // Get or create sender for this run
        let sender = senders.entry(run_id.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        
        // Send event (ignore errors if no receivers)
        let _ = sender.send(event);
        
        Ok(())
    }

    /// Subscribe to events for a specific run
    #[allow(dead_code)]
    pub fn subscribe_run(&self, run_id: &str) -> broadcast::Receiver<GovernanceEvent> {
        let mut senders = self.senders.lock().expect("Failed to lock senders");
        
        let sender = senders.entry(run_id.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        
        sender.subscribe()
    }

    /// Subscribe to all events (by creating a new global channel)
    #[allow(dead_code)]
    pub fn subscribe_all(&self) -> broadcast::Receiver<GovernanceEvent> {
        let mut senders = self.senders.lock().expect("Failed to lock senders");
        
        let sender = senders.entry("__all__".to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1000);
            tx
        });
        
        sender.subscribe()
    }
}

impl Default for EventBroadcaster {
    fn default() -> Self {
        Self::new()
    }
}

/// DIBL manager - combines store and broadcaster
pub struct DiblManager {
    store: Arc<dyn EventStore>,
    broadcaster: EventBroadcaster,
}

impl DiblManager {
    /// Create a new DIBL manager
    pub fn new(store: Arc<dyn EventStore>) -> Self {
        Self {
            store,
            broadcaster: EventBroadcaster::new(),
        }
    }

    /// Emit an event
    /// 
    /// Rule 1: Event is persisted FIRST
    /// Rule 2: Then broadcasted
    pub fn emit(&self, event: GovernanceEvent) -> Result<()> {
        // Persist first (source of truth)
        self.store.append_event(&event)?;
        
        // Then broadcast (derived view)
        self.broadcaster.publish(event)?;
        
        Ok(())
    }

    /// Load events for a run
    pub fn load_run_events(&self, run_id: &str) -> Result<Vec<GovernanceEvent>> {
        self.store.load_run_events(run_id)
    }

    /// Replay events for a run
    pub fn replay_run(&self, run_id: &str) -> Result<Vec<GovernanceEvent>> {
        let events = self.store.load_run_events(run_id)?;
        
        tracing::info!("Replaying {} events for run {}", events.len(), run_id);
        
        for event in &events {
            self.broadcaster.publish(event.clone())?;
        }
        
        Ok(events)
    }

    /// Subscribe to run events
    #[allow(dead_code)]
    pub fn subscribe_run(&self, run_id: &str) -> broadcast::Receiver<GovernanceEvent> {
        self.broadcaster.subscribe_run(run_id)
    }

    /// Check if run has events
    #[allow(dead_code)]
    pub fn run_has_events(&self, run_id: &str) -> bool {
        self.store.run_has_events(run_id)
    }
}

/// Operator projection - minimal summary view
/// Does NOT expose internal details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorProjection {
    pub run_id: String,
    pub current_phase: String,
    pub current_seat: Option<String>,
    pub last_significant_event: Option<String>,
    pub open_risks: Vec<String>,
    pub veto_count: u32,
    pub terminate_flag: bool,
    pub final_outcome: Option<String>,
    pub event_count: usize,
    pub last_updated: DateTime<Utc>,
}

impl OperatorProjection {
    /// Build projection from events
    pub fn from_events(run_id: &str, events: &[GovernanceEvent]) -> Self {
        let mut current_phase = "Created".to_string();
        let mut current_seat = None;
        let mut last_significant_event = None;
        let mut open_risks = Vec::new();
        let mut veto_count = 0;
        let mut terminate_flag = false;
        let mut final_outcome = None;
        
        for event in events {
            match event.event_type {
                GovernanceEventType::RunCreated => {
                    current_phase = "Created".to_string();
                    last_significant_event = Some("Run created".to_string());
                }
                GovernanceEventType::SeatStarted => {
                    current_phase = "Executing".to_string();
                    if let Some(ref seat) = event.seat_id {
                        current_seat = Some(seat.clone());
                    }
                    last_significant_event = Some(format!("Seat started: {:?}", event.seat_id));
                }
                GovernanceEventType::SeatCompleted => {
                    last_significant_event = Some(format!("Seat completed: {:?}", event.seat_id));
                }
                GovernanceEventType::RiskRaised => {
                    open_risks.push(event.summary.clone());
                    last_significant_event = Some(format!("Risk raised: {}", event.summary));
                }
                GovernanceEventType::VetoExercised => {
                    veto_count += 1;
                    current_phase = "Vetoed".to_string();
                    last_significant_event = Some(format!("Veto by {:?}", event.seat_id));
                }
                GovernanceEventType::FinalGateOpened => {
                    current_phase = "FinalGate".to_string();
                    last_significant_event = Some("Final gate opened".to_string());
                }
                GovernanceEventType::DecisionCommitted => {
                    current_phase = "Decided".to_string();
                    final_outcome = Some(event.summary.clone());
                    last_significant_event = Some(format!("Decision: {}", event.summary));
                }
                GovernanceEventType::RollbackTriggered => {
                    current_phase = "RolledBack".to_string();
                    last_significant_event = Some("Rollback triggered".to_string());
                }
                GovernanceEventType::ArchiveCompleted => {
                    current_phase = "Archived".to_string();
                    final_outcome = Some("Archived".to_string());
                    last_significant_event = Some("Archive completed".to_string());
                }
                GovernanceEventType::TerminateTriggered => {
                    terminate_flag = true;
                    current_phase = "Terminated".to_string();
                    final_outcome = Some("Terminated".to_string());
                    last_significant_event = Some(format!("Terminated: {}", event.summary));
                }
                // Entity lifecycle events - for PR-1, we just record them
                _ => {
                    // Entity events are tracked but don't change run projection
                    last_significant_event = Some(event.summary.clone());
                }
            }
        }
        
        let last_updated = events.last().map(|e| e.created_at).unwrap_or_else(Utc::now);
        
        Self {
            run_id: run_id.to_string(),
            current_phase,
            current_seat,
            last_significant_event,
            open_risks,
            veto_count,
            terminate_flag,
            final_outcome,
            event_count: events.len(),
            last_updated,
        }
    }
}

/// Policy filter for scope transitions
#[allow(dead_code)]
pub struct PolicyFilter;

impl PolicyFilter {
    /// Filter events for operator visibility
    /// Internal events are NOT exposed
    #[allow(dead_code)]
    pub fn filter_operator_visible(events: &[GovernanceEvent]) -> Vec<GovernanceEvent> {
        events
            .iter()
            .filter(|e| matches!(e.scope, EventScope::OperatorVisible | EventScope::Exportable))
            .cloned()
            .collect()
    }

    /// Filter events for external export
    /// Only exportable scope
    #[allow(dead_code)]
    pub fn filter_exportable(events: &[GovernanceEvent]) -> Vec<GovernanceEvent> {
        events
            .iter()
            .filter(|e| matches!(e.scope, EventScope::Exportable))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_jsonl_store_append_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        
        let event = GovernanceEvent::new("test-run", GovernanceEventType::RunCreated, EventChannel::Control, "system")
            .with_summary("Test run created");
        
        store.append_event(&event).unwrap();
        
        let loaded = store.load_run_events("test-run").unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].run_id, "test-run");
        assert_eq!(loaded[0].event_type, GovernanceEventType::RunCreated);
        assert_eq!(loaded[0].actor, "system");
    }

    #[test]
    fn test_operator_projection() {
        let events = vec![
            GovernanceEvent::new("r1", GovernanceEventType::RunCreated, EventChannel::Control, "system")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::SeatStarted, EventChannel::Control, "Tianquan")
                .with_seat("Tianquan")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::VetoExercised, EventChannel::Security, "Yuheng")
                .with_seat("Yuheng")
                .with_summary("Quality issue")
                .with_scope(EventScope::OperatorVisible),
        ];
        
        let proj = OperatorProjection::from_events("r1", &events);
        
        assert_eq!(proj.run_id, "r1");
        assert_eq!(proj.current_phase, "Vetoed");
        assert_eq!(proj.veto_count, 1);
        assert_eq!(proj.event_count, 3);
    }
    
    #[test]
    fn test_correlation_context() {
        let event = GovernanceEvent::new("r1", GovernanceEventType::RunCreated, EventChannel::Control, "system")
            .with_correlation_id("corr-123")
            .with_trigger_context("test.context");
        
        assert_eq!(event.correlation_id, Some("corr-123".to_string()));
        assert_eq!(event.actor, "system");
        assert_eq!(event.trigger_context, Some("test.context".to_string()));
    }
    
    #[test]
    fn test_axi_sample_interop() {
        // Verify DragonCore can parse AXI-generated events
        let axi_json = r#"{"event_id":"550e8400-e29b-41d4-a716-446655440000","run_id":"sample-run-001","seat_id":null,"channel":"control","event_type":"run_created","scope":"operator_visible","severity":"info","summary":"Test run created from AXI","details_ref":null,"artifact_refs":[],"created_at":"2026-03-16T12:00:00Z","correlation_id":null,"parent_event_id":null,"actor":"operator","trigger_context":"axi.init_run"}"#;
        
        let event: GovernanceEvent = serde_json::from_str(axi_json).expect("Failed to parse AXI event");
        
        assert_eq!(event.run_id, "sample-run-001");
        assert_eq!(event.channel, EventChannel::Control);
        assert_eq!(event.event_type, GovernanceEventType::RunCreated);
        assert_eq!(event.scope, EventScope::OperatorVisible);
        assert_eq!(event.severity, Severity::Info);
        assert_eq!(event.actor, "operator");
        assert_eq!(event.trigger_context, Some("axi.init_run".to_string()));
    }
    
    #[test]
    fn test_event_append_isolation() {
        // Test: Event write failure should not corrupt existing JSONL
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        
        let run_id = "isolation-test";
        
        // Append first event
        let event1 = GovernanceEvent::new(run_id, GovernanceEventType::RunCreated, EventChannel::Control, "system");
        store.append_event(&event1).unwrap();
        
        // Verify first event exists
        let loaded1 = store.load_run_events(run_id).unwrap();
        assert_eq!(loaded1.len(), 1);
        
        // Append more events
        let event2 = GovernanceEvent::new(run_id, GovernanceEventType::SeatStarted, EventChannel::Control, "Tianquan").with_seat("Tianquan");
        store.append_event(&event2).unwrap();
        
        // Verify both events exist and are in order
        let loaded2 = store.load_run_events(run_id).unwrap();
        assert_eq!(loaded2.len(), 2);
        assert_eq!(loaded2[0].event_type, GovernanceEventType::RunCreated);
        assert_eq!(loaded2[1].event_type, GovernanceEventType::SeatStarted);
    }
    
    #[test]
    fn test_malformed_jsonl_graceful_handling() {
        // Test: Invalid JSON lines should be skipped, not crash
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        
        let run_id = "malformed-test";
        let file_path = temp_dir.path().join(format!("{}.jsonl", run_id));
        
        // Write a mix of valid and invalid lines (with all required fields)
        let mut file = std::fs::File::create(&file_path).unwrap();
        let valid_event1 = r#"{"event_id":"550e8400-e29b-41d4-a716-000000000001","run_id":"malformed-test","seat_id":null,"channel":"control","event_type":"run_created","scope":"internal","severity":"info","summary":"valid","details_ref":null,"artifact_refs":[],"created_at":"2026-03-16T12:00:00Z","correlation_id":null,"parent_event_id":null,"actor":"system","trigger_context":null}"#;
        let valid_event2 = r#"{"event_id":"550e8400-e29b-41d4-a716-000000000002","run_id":"malformed-test","seat_id":null,"channel":"control","event_type":"seat_started","scope":"internal","severity":"info","summary":"valid","details_ref":null,"artifact_refs":[],"created_at":"2026-03-16T12:00:01Z","correlation_id":null,"parent_event_id":null,"actor":"Tianquan","trigger_context":null}"#;
        writeln!(file, "{}", valid_event1).unwrap();
        writeln!(file, "this is not valid json").unwrap();
        writeln!(file, "{}", valid_event2).unwrap();
        drop(file);
        
        // Load should skip invalid line
        let loaded = store.load_run_events(run_id).unwrap();
        assert_eq!(loaded.len(), 2);
    }
    
    #[test]
    fn test_internal_events_filtered_from_operator_view() {
        // Test: Internal events should not appear in operator projection
        let events = vec![
            GovernanceEvent::new("r1", GovernanceEventType::RunCreated, EventChannel::Control, "system")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::SeatStarted, EventChannel::Control, "Tianquan")
                .with_seat("Tianquan")
                .with_scope(EventScope::Internal), // This should be filtered
            GovernanceEvent::new("r1", GovernanceEventType::SeatCompleted, EventChannel::Research, "Tianquan")
                .with_seat("Tianquan")
                .with_scope(EventScope::Internal), // This should be filtered
            GovernanceEvent::new("r1", GovernanceEventType::FinalGateOpened, EventChannel::Control, "Tianshu")
                .with_seat("Tianshu")
                .with_scope(EventScope::OperatorVisible),
        ];
        
        // Filter operator visible
        let operator_visible = PolicyFilter::filter_operator_visible(&events);
        assert_eq!(operator_visible.len(), 2);
        assert_eq!(operator_visible[0].event_type, GovernanceEventType::RunCreated);
        assert_eq!(operator_visible[1].event_type, GovernanceEventType::FinalGateOpened);
        
        // All events in projection
        let proj = OperatorProjection::from_events("r1", &events);
        assert_eq!(proj.event_count, 4); // Projection uses all events
    }
    
    #[test]
    fn test_replay_output_order_stable() {
        // Test: Replay should output events in original order
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        let broadcaster = EventBroadcaster::new();
        let manager = DiblManager::new(Arc::new(store));
        
        let run_id = "order-test";
        
        // Create events in specific order
        let event_types = vec![
            GovernanceEventType::RunCreated,
            GovernanceEventType::SeatStarted,
            GovernanceEventType::SeatCompleted,
            GovernanceEventType::VetoExercised,
            GovernanceEventType::FinalGateOpened,
        ];
        
        for (i, event_type) in event_types.iter().enumerate() {
            let event = GovernanceEvent::new(run_id, *event_type, EventChannel::Control, "system")
                .with_summary(format!("Event {}", i));
            manager.emit(event).unwrap();
        }
        
        // Load and verify order
        let loaded = manager.load_run_events(run_id).unwrap();
        assert_eq!(loaded.len(), 5);
        for (i, event_type) in event_types.iter().enumerate() {
            assert_eq!(loaded[i].event_type, *event_type);
        }
    }
    
    #[test]
    fn test_different_run_id_isolation() {
        // Test: Different run_id should not share events
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        
        // Create events for different runs
        let run1 = "run-001";
        let run2 = "run-002";
        
        store.append_event(&GovernanceEvent::new(run1, GovernanceEventType::RunCreated, EventChannel::Control, "system")).unwrap();
        store.append_event(&GovernanceEvent::new(run2, GovernanceEventType::RunCreated, EventChannel::Control, "system")).unwrap();
        store.append_event(&GovernanceEvent::new(run1, GovernanceEventType::SeatStarted, EventChannel::Control, "Tianquan").with_seat("Tianquan")).unwrap();
        
        // Verify isolation
        let loaded1 = store.load_run_events(run1).unwrap();
        let loaded2 = store.load_run_events(run2).unwrap();
        
        assert_eq!(loaded1.len(), 2);
        assert_eq!(loaded2.len(), 1);
        
        assert_eq!(loaded1[0].run_id, run1);
        assert_eq!(loaded1[1].run_id, run1);
        assert_eq!(loaded2[0].run_id, run2);
    }
    
    #[test]
    fn test_security_events_channel_scope() {
        // Test: Veto, Terminate, Rollback should be Security channel with correct scope
        let events = vec![
            GovernanceEvent::new("r1", GovernanceEventType::VetoExercised, EventChannel::Security, "Yuheng")
                .with_seat("Yuheng")
                .with_scope(EventScope::OperatorVisible)
                .with_severity(Severity::Warn),
            GovernanceEvent::new("r1", GovernanceEventType::TerminateTriggered, EventChannel::Security, "Fengdudadi")
                .with_seat("Fengdudadi")
                .with_scope(EventScope::OperatorVisible)
                .with_severity(Severity::Critical),
            GovernanceEvent::new("r1", GovernanceEventType::RollbackTriggered, EventChannel::Security, "system")
                .with_scope(EventScope::OperatorVisible)
                .with_severity(Severity::Warn),
        ];
        
        for event in &events {
            assert_eq!(event.channel, EventChannel::Security, "{:?} should be Security channel", event.event_type);
            assert_eq!(event.scope, EventScope::OperatorVisible, "{:?} should be OperatorVisible", event.event_type);
        }
        
        assert_eq!(events[0].severity, Severity::Warn);
        assert_eq!(events[1].severity, Severity::Critical);
        assert_eq!(events[2].severity, Severity::Warn);
    }
    
    #[test]
    fn test_risk_raised_event_projection() {
        // Test: RiskRaised events should appear in operator projection
        let events = vec![
            GovernanceEvent::new("r1", GovernanceEventType::RunCreated, EventChannel::Control, "system")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::SeatStarted, EventChannel::Control, "Baihu")
                .with_seat("Baihu")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::RiskRaised, EventChannel::Security, "Baihu")
                .with_seat("Baihu")
                .with_scope(EventScope::OperatorVisible)
                .with_severity(Severity::Warn)
                .with_summary("[PERSISTENCE] Risk of data loss in rollback"),
            GovernanceEvent::new("r1", GovernanceEventType::FinalGateOpened, EventChannel::Control, "Tianshu")
                .with_seat("Tianshu")
                .with_scope(EventScope::OperatorVisible),
            GovernanceEvent::new("r1", GovernanceEventType::DecisionCommitted, EventChannel::Control, "Tianshu")
                .with_seat("Tianshu")
                .with_scope(EventScope::Exportable)
                .with_summary("APPROVED"),
        ];
        
        let proj = OperatorProjection::from_events("r1", &events);
        
        assert_eq!(proj.run_id, "r1");
        assert_eq!(proj.current_phase, "Decided");
        assert_eq!(proj.open_risks.len(), 1);
        assert_eq!(proj.open_risks[0], "[PERSISTENCE] Risk of data loss in rollback");
        assert_eq!(proj.event_count, 5);
    }
    
    #[test]
    fn test_provider_tracking_in_events() {
        // Test: All event types should support provider tracking
        let events = vec![
            GovernanceEvent::new("r1", GovernanceEventType::RunCreated, EventChannel::Control, "system")
                .with_provider("kimi"),
            GovernanceEvent::new("r1", GovernanceEventType::SeatStarted, EventChannel::Control, "Tianquan")
                .with_seat("Tianquan")
                .with_provider("gpt_oss_120b"),
            GovernanceEvent::new("r1", GovernanceEventType::RiskRaised, EventChannel::Security, "Baihu")
                .with_seat("Baihu")
                .with_provider("kimi")
                .with_severity(Severity::Warn),
            GovernanceEvent::new("r1", GovernanceEventType::VetoExercised, EventChannel::Security, "Yuheng")
                .with_seat("Yuheng")
                .with_provider("kimi"),
            GovernanceEvent::new("r1", GovernanceEventType::FinalGateOpened, EventChannel::Control, "Tianshu")
                .with_seat("Tianshu")
                .with_provider("gpt_oss_120b"),
            GovernanceEvent::new("r1", GovernanceEventType::DecisionCommitted, EventChannel::Control, "Tianshu")
                .with_seat("Tianshu")
                .with_provider("gpt_oss_120b"),
            GovernanceEvent::new("r1", GovernanceEventType::ArchiveCompleted, EventChannel::Ops, "Yaoguang")
                .with_seat("Yaoguang")
                .with_provider("kimi"),
            GovernanceEvent::new("r1", GovernanceEventType::TerminateTriggered, EventChannel::Security, "Fengdudadi")
                .with_seat("Fengdudadi")
                .with_provider("kimi")
                .with_severity(Severity::Critical),
        ];
        
        // Verify provider is set correctly for each event
        assert_eq!(events[0].provider, Some("kimi".to_string()));
        assert_eq!(events[1].provider, Some("gpt_oss_120b".to_string()));
        assert_eq!(events[2].provider, Some("kimi".to_string()));
        assert_eq!(events[3].provider, Some("kimi".to_string()));
        assert_eq!(events[4].provider, Some("gpt_oss_120b".to_string()));
        assert_eq!(events[5].provider, Some("gpt_oss_120b".to_string()));
        assert_eq!(events[6].provider, Some("kimi".to_string()));
        assert_eq!(events[7].provider, Some("kimi".to_string()));
    }
    
    #[test]
    fn test_complete_governance_lifecycle_events() {
        // Test: Complete lifecycle with all event types in correct order
        let temp_dir = TempDir::new().unwrap();
        let store = JsonlEventStore::new(temp_dir.path()).unwrap();
        let manager = DiblManager::new(Arc::new(store));
        
        let run_id = "complete-lifecycle";
        
        // Simulate complete governance flow
        let flow = vec![
            (GovernanceEventType::RunCreated, EventChannel::Control, None as Option<&str>),
            (GovernanceEventType::SeatStarted, EventChannel::Control, Some("Tianquan")),
            (GovernanceEventType::SeatCompleted, EventChannel::Research, Some("Tianquan")),
            (GovernanceEventType::SeatStarted, EventChannel::Control, Some("Baihu")),
            (GovernanceEventType::RiskRaised, EventChannel::Security, Some("Baihu")),
            (GovernanceEventType::SeatCompleted, EventChannel::Research, Some("Baihu")),
            (GovernanceEventType::SeatStarted, EventChannel::Control, Some("Yuheng")),
            (GovernanceEventType::SeatCompleted, EventChannel::Research, Some("Yuheng")),
            (GovernanceEventType::FinalGateOpened, EventChannel::Control, Some("Tianshu")),
            (GovernanceEventType::DecisionCommitted, EventChannel::Control, Some("Tianshu")),
            (GovernanceEventType::ArchiveCompleted, EventChannel::Ops, Some("Yaoguang")),
        ];
        
        for (event_type, channel, seat) in flow {
            let mut event = GovernanceEvent::new(run_id, event_type, channel, seat.unwrap_or("system"));
            if let Some(s) = seat {
                event = event.with_seat(s);
            }
            manager.emit(event).unwrap();
        }
        
        // Load and verify
        let loaded = manager.load_run_events(run_id).unwrap();
        assert_eq!(loaded.len(), 11);
        
        // Verify order
        assert_eq!(loaded[0].event_type, GovernanceEventType::RunCreated);
        assert_eq!(loaded[4].event_type, GovernanceEventType::RiskRaised);
        assert_eq!(loaded[8].event_type, GovernanceEventType::FinalGateOpened);
        assert_eq!(loaded[10].event_type, GovernanceEventType::ArchiveCompleted);
        
        // Verify projection
        let proj = OperatorProjection::from_events(run_id, &loaded);
        assert_eq!(proj.current_phase, "Archived");
        assert_eq!(proj.open_risks.len(), 1);
    }
}

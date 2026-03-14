//! Persistence layer for DragonCore Runtime
//! 
//! Provides durable storage for governance runs using JSON files.
//! Every state transition is immediately persisted to disk.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Trait for run storage backends
pub trait RunStore: Send + Sync {
    /// Create a new run in storage
    fn create_run(&self, run: &PersistedRun) -> Result<()>;
    
    /// Load a run from storage
    fn load_run(&self, run_id: &str) -> Result<Option<PersistedRun>>;
    
    /// Save (update) a run in storage
    fn save_run(&self, run: &PersistedRun) -> Result<()>;
    
    /// List all run IDs in storage
    fn list_runs(&self) -> Result<Vec<String>>;
    
    /// Load all runs from storage
    fn load_all_runs(&self) -> Result<HashMap<String, PersistedRun>>;
    
    /// Check if a run exists in storage
    fn run_exists(&self, run_id: &str) -> Result<bool> {
        Ok(self.load_run(run_id)?.is_some())
    }
}

/// JSON file-based run storage
pub struct JsonFileStore {
    base_path: PathBuf,
}

impl JsonFileStore {
    /// Create a new JSON file store
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)
            .with_context(|| format!("Failed to create storage directory: {:?}", base_path))?;
        Ok(Self { base_path })
    }
    
    /// Get the file path for a run
    fn run_file_path(&self, run_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", run_id))
    }
}

impl RunStore for JsonFileStore {
    fn create_run(&self, run: &PersistedRun) -> Result<()> {
        let path = self.run_file_path(&run.run_id);
        
        // Check if file already exists
        if path.exists() {
            anyhow::bail!("Run {} already exists", run.run_id);
        }
        
        // Write with atomic rename pattern
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(run)
            .context("Failed to serialize run")?;
        fs::write(&temp_path, json)
            .with_context(|| format!("Failed to write temp file: {:?}", temp_path))?;
        fs::rename(&temp_path, &path)
            .with_context(|| format!("Failed to rename temp file to: {:?}", path))?;
        
        Ok(())
    }
    
    fn load_run(&self, run_id: &str) -> Result<Option<PersistedRun>> {
        let path = self.run_file_path(run_id);
        
        if !path.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read run file: {:?}", path))?;
        let run: PersistedRun = serde_json::from_str(&json)
            .with_context(|| format!("Failed to parse run file: {:?}", path))?;
        
        Ok(Some(run))
    }
    
    fn save_run(&self, run: &PersistedRun) -> Result<()> {
        let path = self.run_file_path(&run.run_id);
        
        // Write with atomic rename pattern
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_string_pretty(run)
            .context("Failed to serialize run")?;
        fs::write(&temp_path, json)
            .with_context(|| format!("Failed to write temp file: {:?}", temp_path))?;
        fs::rename(&temp_path, &path)
            .with_context(|| format!("Failed to rename temp file to: {:?}", path))?;
        
        Ok(())
    }
    
    fn list_runs(&self) -> Result<Vec<String>> {
        let mut runs = Vec::new();
        
        for entry in fs::read_dir(&self.base_path)
            .with_context(|| format!("Failed to read storage directory: {:?}", self.base_path))? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(stem) = path.file_stem() {
                    runs.push(stem.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(runs)
    }
    
    fn load_all_runs(&self) -> Result<HashMap<String, PersistedRun>> {
        let mut runs = HashMap::new();
        
        for run_id in self.list_runs()? {
            if let Some(run) = self.load_run(&run_id)? {
                runs.insert(run_id, run);
            }
        }
        
        Ok(runs)
    }
}

/// Persisted run state - the source of truth stored on disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedRun {
    pub run_id: String,
    pub status: PersistedRunStatus,
    pub task: String,
    pub input_type: String,
    pub worktree_path: PathBuf,
    pub tmux_session: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub events: Vec<RunEvent>,
    pub metrics: RunMetrics,
    pub veto: Option<VetoRecord>,
    pub final_gate: Option<FinalGateRecord>,
    pub current_seat: Option<String>,
    pub seats_participated: Vec<String>,
    pub artifacts: Vec<String>,
}

impl PersistedRun {
    /// Create a new persisted run
    pub fn new(
        run_id: String,
        task: String,
        input_type: String,
        worktree_path: PathBuf,
        tmux_session: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            run_id,
            status: PersistedRunStatus::Created,
            task,
            input_type,
            worktree_path,
            tmux_session,
            created_at: now,
            updated_at: now,
            events: Vec::new(),
            metrics: RunMetrics::default(),
            veto: None,
            final_gate: None,
            current_seat: None,
            seats_participated: Vec::new(),
            artifacts: Vec::new(),
        }
    }
    
    /// Add an event to the run
    pub fn add_event(&mut self, seat: crate::governance::Seat, action: &str, details: Option<&str>) {
        self.events.push(RunEvent {
            timestamp: Utc::now(),
            seat: format!("{:?}", seat),
            action: action.to_string(),
            details: details.map(|s| s.to_string()),
        });
        self.updated_at = Utc::now();
    }
}

/// Run status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistedRunStatus {
    Created,
    Running,
    Executing,
    Completed,
    Vetoed,
    Approved,
    Rejected,
    Archived,
    Terminated,
}

/// Run event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEvent {
    pub timestamp: DateTime<Utc>,
    pub seat: String,
    pub action: String,
    pub details: Option<String>,
}

/// Run metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunMetrics {
    pub seat_participation: HashMap<String, u32>,
    pub veto_count: u32,
    pub revision_count: u32,
}

/// Veto record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VetoRecord {
    pub seat: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

/// Final gate record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalGateRecord {
    pub seat: String,
    pub approved: bool,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_json_file_store() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonFileStore::new(temp_dir.path()).unwrap();
        
        // Create a run
        let run = PersistedRun::new(
            "test-run-001".to_string(),
            "Test task".to_string(),
            "code".to_string(),
            PathBuf::from("/tmp/test"),
            "dragoncore_test".to_string(),
        );
        
        // Save it
        store.create_run(&run).unwrap();
        
        // Load it back
        let loaded = store.load_run("test-run-001").unwrap().unwrap();
        assert_eq!(loaded.run_id, "test-run-001");
        assert_eq!(loaded.task, "Test task");
        
        // List runs
        let runs = store.list_runs().unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0], "test-run-001");
        
        // Update and save
        let mut modified = loaded.clone();
        modified.status = PersistedRunStatus::Executing;
        store.save_run(&modified).unwrap();
        
        // Verify update
        let reloaded = store.load_run("test-run-001").unwrap().unwrap();
        assert!(matches!(reloaded.status, PersistedRunStatus::Executing));
    }
}

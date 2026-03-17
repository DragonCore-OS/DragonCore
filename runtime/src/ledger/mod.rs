use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use crate::governance::{RunState, Seat};

/// Ledger entry for a governance run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub run_id: String,
    pub timestamp: DateTime<Utc>,
    pub input_type: String,
    pub final_state: RunState,
    pub seats_participated: Vec<Seat>,
    pub veto_used: Option<Seat>,
    pub escalation_triggered: bool,
    pub rollback_executed: bool,
    pub archive_executed: bool,
    pub terminate_executed: bool,
    pub authority_violation: bool,
    pub fake_closure: bool,
    pub tokens_used: u64,
    pub wall_clock_seconds: u64,
    pub human_intervention: bool,
    pub metadata: HashMap<String, String>,
    pub finalized: bool,
}

impl LedgerEntry {
    /// Create a new ledger entry
    pub fn new(run_id: impl Into<String>, input_type: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            timestamp: Utc::now(),
            input_type: input_type.into(),
            final_state: RunState::Pending,
            seats_participated: Vec::new(),
            veto_used: None,
            escalation_triggered: false,
            rollback_executed: false,
            archive_executed: false,
            terminate_executed: false,
            authority_violation: false,
            fake_closure: false,
            tokens_used: 0,
            wall_clock_seconds: 0,
            human_intervention: false,
            metadata: HashMap::new(),
            finalized: false,
        }
    }
    
    /// Serialize to CSV row
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.run_id,
            self.timestamp.to_rfc3339(),
            self.input_type,
            format!("{:?}", self.final_state),
            self.seats_participated.len(),
            self.veto_used.map(|s| format!("{:?}", s)).unwrap_or_default(),
            self.escalation_triggered,
            self.rollback_executed,
            self.archive_executed,
            self.terminate_executed,
            self.authority_violation,
            self.fake_closure,
            self.tokens_used,
            self.wall_clock_seconds,
            self.human_intervention,
            self.finalized,
        )
    }
}

/// Production ledger with persistence
pub struct Ledger {
    storage_path: PathBuf,
    entries: HashMap<String, LedgerEntry>,
    current_run_id: Option<String>,
    start_time: Option<DateTime<Utc>>,
}

impl Ledger {
    /// Create a new ledger, loading existing entries from disk
    pub fn new(storage_path: impl AsRef<Path>) -> Result<Self> {
        let storage_path = storage_path.as_ref().to_path_buf();
        
        // Ensure directory exists
        std::fs::create_dir_all(&storage_path)
            .with_context(|| format!("Failed to create ledger directory: {:?}", storage_path))?;
        
        // Ensure CSV file exists with headers
        let csv_path = storage_path.join("production_ledger.csv");
        if !csv_path.exists() {
            let mut file = File::create(&csv_path)
                .with_context(|| "Failed to create ledger CSV")?;
            writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized")?;
        }
        
        // Load existing entries
        let entries = Self::load_entries(&csv_path)?;
        
        Ok(Self {
            storage_path,
            entries,
            current_run_id: None,
            start_time: None,
        })
    }
    
    /// Load all entries from CSV
    fn load_entries(csv_path: &Path) -> Result<HashMap<String, LedgerEntry>> {
        let mut entries = HashMap::new();
        
        if !csv_path.exists() {
            return Ok(entries);
        }
        
        let file = File::open(csv_path)
            .with_context(|| "Failed to open ledger CSV")?;
        
        let reader = BufReader::new(file);
        
        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                continue; // Skip header
            }
            
            let line = line.with_context(|| "Failed to read ledger line")?;
            if line.trim().is_empty() {
                continue;
            }
            
            match Self::parse_csv_row(&line) {
                Ok(entry) => {
                    entries.insert(entry.run_id.clone(), entry);
                }
                Err(e) => {
                    tracing::warn!("Failed to parse ledger row {}: {}", i, e);
                }
            }
        }
        
        Ok(entries)
    }
    
    /// Parse CSV row into entry
    fn parse_csv_row(row: &str) -> Result<LedgerEntry> {
        let parts: Vec<&str> = row.split(',').collect();
        if parts.len() < 15 {
            anyhow::bail!("Invalid CSV row: insufficient columns");
        }
        
        Ok(LedgerEntry {
            run_id: parts[0].to_string(),
            timestamp: DateTime::parse_from_rfc3339(parts[1])
                .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                .into(),
            input_type: parts[2].to_string(),
            final_state: parse_run_state(parts[3])?,
            seats_participated: Vec::new(),
            veto_used: if parts[5].is_empty() { None } else { parse_seat(parts[5]).ok() },
            escalation_triggered: parts[6].parse()?,
            rollback_executed: parts[7].parse()?,
            archive_executed: parts[8].parse()?,
            terminate_executed: parts[9].parse()?,
            authority_violation: parts[10].parse()?,
            fake_closure: parts[11].parse()?,
            tokens_used: parts[12].parse()?,
            wall_clock_seconds: parts[13].parse()?,
            human_intervention: parts[14].parse()?,
            metadata: HashMap::new(),
            finalized: parts.get(15).map(|s| s.parse().unwrap_or(false)).unwrap_or(false),
        })
    }
    
    /// Save all entries to CSV
    fn save_entries(&self) -> Result<()> {
        let csv_path = self.storage_path.join("production_ledger.csv");
        let mut file = File::create(&csv_path)
            .with_context(|| "Failed to create ledger CSV")?;
        
        // Write header
        writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized")?;
        
        // Write all entries
        for entry in self.entries.values() {
            writeln!(file, "{}", entry.to_csv_row())?;
        }
        
        file.flush()?;
        Ok(())
    }
    
    /// Start a new run - immediately persists
    pub fn start_run(&mut self, run_id: impl Into<String>, input_type: impl Into<String>) -> Result<()> {
        let run_id = run_id.into();
        let input_type = input_type.into();
        
        // Create entry
        let entry = LedgerEntry::new(&run_id, &input_type);
        
        // Store in cache
        let run_id_str = run_id.to_string();
        self.entries.insert(run_id_str.clone(), entry);
        self.current_run_id = Some(run_id_str.clone());
        self.start_time = Some(Utc::now());
        
        // IMMEDIATELY persist
        self.save_entries()?;
        
        tracing::info!("Started ledger entry for run: {}", run_id_str);
        Ok(())
    }
    
    /// Get mutable reference to entry by run_id
    fn get_entry(&mut self, run_id: &str) -> Option<&mut LedgerEntry> {
        self.entries.get_mut(run_id)
    }
    
    /// Record seat participation
    pub fn record_participation(&mut self, run_id: &str, seat: Seat) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            if !entry.seats_participated.contains(&seat) {
                entry.seats_participated.push(seat);
            }
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record veto
    pub fn record_veto(&mut self, run_id: &str, seat: Seat) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.veto_used = Some(seat);
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record escalation
    pub fn record_escalation(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.escalation_triggered = true;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record rollback
    pub fn record_rollback(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.rollback_executed = true;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record archive
    pub fn record_archive(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.archive_executed = true;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record termination
    pub fn record_terminate(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.terminate_executed = true;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record red line violation
    pub fn record_red_line(&mut self, run_id: &str, violation_type: RedLineViolation) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            match violation_type {
                RedLineViolation::AuthorityViolation => entry.authority_violation = true,
                RedLineViolation::FakeClosure => entry.fake_closure = true,
            }
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record token usage
    pub fn record_tokens(&mut self, run_id: &str, tokens: u64) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.tokens_used = tokens;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Record human intervention
    pub fn record_human_intervention(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.human_intervention = true;
            self.save_entries()?;
        }
        Ok(())
    }
    
    /// Finalize a run - persists final state
    pub fn finalize_run(&mut self, run_id: &str, final_state: RunState) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.final_state = final_state;
            entry.finalized = true;
            
            // Calculate wall clock time from entry timestamp
            entry.wall_clock_seconds = (Utc::now() - entry.timestamp).num_seconds() as u64;
            
            // Persist final state
            self.save_entries()?;
            
            tracing::info!("Finalized ledger entry for run: {} with state: {:?}", run_id, final_state);
        }
        
        if self.current_run_id.as_ref().map(|s| s.as_str()) == Some(run_id) {
            self.current_run_id = None;
            self.start_time = None;
        }
        
        Ok(())
    }
    
    /// Load a run's entry (for cross-CLI continuity)
    pub fn load_run(&mut self, run_id: &str) -> bool {
        if let Some(entry) = self.entries.get(run_id) {
            if !entry.finalized {
                self.current_run_id = Some(run_id.to_string());
                self.start_time = Some(entry.timestamp);
                return true;
            }
        }
        false
    }
    
    /// Get all ledger entries
    pub fn get_entries(&self) -> Result<Vec<LedgerEntry>> {
        Ok(self.entries.values().cloned().collect())
    }
    
    /// Get run count by state
    pub fn get_run_counts(&self) -> Result<HashMap<RunState, usize>> {
        let mut counts = HashMap::new();
        
        for entry in self.entries.values() {
            *counts.entry(entry.final_state).or_insert(0) += 1;
        }
        
        Ok(counts)
    }
    
    /// Get stability metrics
    pub fn get_stability_metrics(&self) -> Result<StabilityMetrics> {
        let entries: Vec<_> = self.entries.values().collect();
        
        let total = entries.len();
        let authority_violations = entries.iter().filter(|e| e.authority_violation).count();
        let fake_closures = entries.iter().filter(|e| e.fake_closure).count();
        let rollbacks = entries.iter().filter(|e| e.rollback_executed).count();
        let terminations = entries.iter().filter(|e| e.terminate_executed).count();
        let clean_runs = total.saturating_sub(authority_violations + fake_closures + terminations);
        
        Ok(StabilityMetrics {
            total_runs: total,
            clean_runs,
            authority_violations,
            fake_closures,
            rollbacks,
            terminations,
        })
    }
}

/// Parse run state from string
fn parse_run_state(s: &str) -> Result<RunState> {
    match s {
        "Pending" => Ok(RunState::Pending),
        "InProgress" => Ok(RunState::InProgress),
        "Reviewing" => Ok(RunState::Reviewing),
        "Escalated" => Ok(RunState::Escalated),
        "Approved" => Ok(RunState::Approved),
        "Rejected" => Ok(RunState::Rejected),
        "RolledBack" => Ok(RunState::RolledBack),
        "Archived" => Ok(RunState::Archived),
        "Terminated" => Ok(RunState::Terminated),
        _ => anyhow::bail!("Unknown run state: {}", s),
    }
}

/// Parse seat from string
fn parse_seat(s: &str) -> Result<Seat> {
    match s {
        "Tianshu" => Ok(Seat::Tianshu),
        "Tianxuan" => Ok(Seat::Tianxuan),
        "Tianji" => Ok(Seat::Tianji),
        "Tianquan" => Ok(Seat::Tianquan),
        "Yuheng" => Ok(Seat::Yuheng),
        "Kaiyang" => Ok(Seat::Kaiyang),
        "Yaoguang" => Ok(Seat::Yaoguang),
        "Qinglong" => Ok(Seat::Qinglong),
        "Baihu" => Ok(Seat::Baihu),
        "Zhuque" => Ok(Seat::Zhuque),
        "Xuanwu" => Ok(Seat::Xuanwu),
        "Yangjian" => Ok(Seat::Yangjian),
        "Baozheng" => Ok(Seat::Baozheng),
        "Zhongkui" => Ok(Seat::Zhongkui),
        "Luban" => Ok(Seat::Luban),
        "Zhugeliang" => Ok(Seat::Zhugeliang),
        "Nezha" => Ok(Seat::Nezha),
        "Xiwangmu" => Ok(Seat::Xiwangmu),
        "Fengdudadi" => Ok(Seat::Fengdudadi),
        _ => anyhow::bail!("Unknown seat: {}", s),
    }
}

/// Red line violation types
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum RedLineViolation {
    AuthorityViolation,
    FakeClosure,
}

/// Stability metrics
#[derive(Debug, Clone)]
pub struct StabilityMetrics {
    pub total_runs: usize,
    pub clean_runs: usize,
    pub authority_violations: usize,
    pub fake_closures: usize,
    pub rollbacks: usize,
    pub terminations: usize,
}

impl StabilityMetrics {
    /// Calculate stability score (0-100)
    #[allow(dead_code)]
    pub fn stability_score(&self) -> u32 {
        if self.total_runs == 0 {
            return 100;
        }
        
        let bad_runs = self.authority_violations + self.fake_closures + self.terminations;
        let score = 100 - (bad_runs * 100 / self.total_runs);
        score as u32
    }
}

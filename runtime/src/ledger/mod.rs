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
    pub risk_raised_count: u32,
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
            risk_raised_count: 0,
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
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
            self.risk_raised_count,
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
            writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,risk_raised_count,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized")?;
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
    /// 
    /// Backward compatibility:
    /// - Format v1 (15 columns): Old format without risk_raised_count
    /// - Format v2 (17 columns): Current format with risk_raised_count
    fn parse_csv_row(row: &str) -> Result<LedgerEntry> {
        let parts: Vec<&str> = row.split(',').collect();
        
        // Support both old (15-col) and new (17-col) formats
        let column_count = parts.len();
        if column_count < 15 {
            anyhow::bail!("Invalid CSV row: expected 15 or 17 columns, got {}", column_count);
        }
        
        // Detect format version based on column count
        let is_v2_format = column_count >= 17;
        
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
            // risk_raised_count is at index 10 in v2, defaults to 0 in v1
            risk_raised_count: if is_v2_format { 
                parts[10].parse().unwrap_or(0) 
            } else { 
                0 
            },
            // In v1, authority_violation is at index 10; in v2, it's at index 11
            authority_violation: if is_v2_format { 
                parts[11].parse()? 
            } else { 
                parts[10].parse()? 
            },
            // In v1, fake_closure is at index 11; in v2, it's at index 12
            fake_closure: if is_v2_format { 
                parts[12].parse()? 
            } else { 
                parts[11].parse()? 
            },
            // In v1, tokens_used is at index 12; in v2, it's at index 13
            tokens_used: if is_v2_format { 
                parts[13].parse()? 
            } else { 
                parts[12].parse()? 
            },
            // In v1, wall_clock_seconds is at index 13; in v2, it's at index 14
            wall_clock_seconds: if is_v2_format { 
                parts[14].parse()? 
            } else { 
                parts[13].parse()? 
            },
            // In v1, human_intervention is at index 14; in v2, it's at index 15
            human_intervention: if is_v2_format { 
                parts[15].parse()? 
            } else { 
                parts[14].parse()? 
            },
            metadata: HashMap::new(),
            // finalized is at index 15 in v1, index 16 in v2
            finalized: if is_v2_format { 
                parts.get(16).map(|s| s.parse().unwrap_or(false)).unwrap_or(false) 
            } else { 
                parts.get(15).map(|s| s.parse().unwrap_or(false)).unwrap_or(false) 
            },
        })
    }
    
    /// Save all entries to CSV
    fn save_entries(&self) -> Result<()> {
        let csv_path = self.storage_path.join("production_ledger.csv");
        let mut file = File::create(&csv_path)
            .with_context(|| "Failed to create ledger CSV")?;
        
        // Write header
        writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,risk_raised_count,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized")?;
        
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
    
    /// Record risk raised
    pub fn record_risk(&mut self, run_id: &str) -> Result<()> {
        if let Some(entry) = self.get_entry(run_id) {
            entry.risk_raised_count += 1;
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    /// Test parsing old format (15 columns) without risk_raised_count
    #[test]
    fn test_parse_csv_row_v1_format() {
        // Old format: 15 columns (no risk_raised_count at index 10)
        let row = "test-run-001,2026-03-17T15:43:01.484781592+00:00,code,Pending,1,,false,false,false,false,false,false,0,0,false,false";
        
        let entry = Ledger::parse_csv_row(row).expect("Should parse v1 format");
        
        assert_eq!(entry.run_id, "test-run-001");
        assert_eq!(entry.input_type, "code");
        assert_eq!(entry.final_state, RunState::Pending);
        assert_eq!(entry.seats_participated.len(), 0); // Reconstructed from count
        assert_eq!(entry.veto_used, None);
        assert_eq!(entry.escalation_triggered, false);
        assert_eq!(entry.rollback_executed, false);
        assert_eq!(entry.archive_executed, false);
        assert_eq!(entry.terminate_executed, false);
        assert_eq!(entry.risk_raised_count, 0); // Default value for v1
        assert_eq!(entry.authority_violation, false);
        assert_eq!(entry.fake_closure, false);
        assert_eq!(entry.tokens_used, 0);
        assert_eq!(entry.wall_clock_seconds, 0);
        assert_eq!(entry.human_intervention, false);
        assert_eq!(entry.finalized, false);
    }

    /// Test parsing new format (17 columns) with risk_raised_count
    #[test]
    fn test_parse_csv_row_v2_format() {
        // New format: 17 columns (with risk_raised_count at index 10)
        let row = "test-run-002,2026-03-17T15:43:01.484781592+00:00,code,Approved,2,Tianquan,false,false,false,false,5,false,false,100,30,false,true";
        
        let entry = Ledger::parse_csv_row(row).expect("Should parse v2 format");
        
        assert_eq!(entry.run_id, "test-run-002");
        assert_eq!(entry.input_type, "code");
        assert_eq!(entry.final_state, RunState::Approved);
        assert_eq!(entry.veto_used, Some(Seat::Tianquan));
        assert_eq!(entry.escalation_triggered, false);
        assert_eq!(entry.rollback_executed, false);
        assert_eq!(entry.archive_executed, false);
        assert_eq!(entry.terminate_executed, false);
        assert_eq!(entry.risk_raised_count, 5); // Properly parsed from v2
        assert_eq!(entry.authority_violation, false);
        assert_eq!(entry.fake_closure, false);
        assert_eq!(entry.tokens_used, 100);
        assert_eq!(entry.wall_clock_seconds, 30);
        assert_eq!(entry.human_intervention, false);
        assert_eq!(entry.finalized, true);
    }

    /// Test parsing v1 format with all boolean flags set to true
    #[test]
    fn test_parse_csv_row_v1_all_flags_true() {
        let row = "test-run-003,2026-03-17T15:43:01.484781592+00:00,code,Terminated,3,Tianquan,true,true,true,true,true,true,50,60,true,true";
        
        let entry = Ledger::parse_csv_row(row).expect("Should parse v1 with flags");
        
        assert_eq!(entry.run_id, "test-run-003");
        assert_eq!(entry.final_state, RunState::Terminated);
        assert_eq!(entry.escalation_triggered, true);
        assert_eq!(entry.rollback_executed, true);
        assert_eq!(entry.archive_executed, true);
        assert_eq!(entry.terminate_executed, true);
        // v1 format doesn't have risk_raised_count, so it defaults to 0
        // But in this test, the v1 parser would interpret the authority_violation value (true) as risk_raised_count
        // Actually with our fix, v1 is detected as < 17 columns, so risk_raised_count = 0
        assert_eq!(entry.risk_raised_count, 0);
        assert_eq!(entry.authority_violation, true);
        assert_eq!(entry.fake_closure, true);
        assert_eq!(entry.tokens_used, 50);
        assert_eq!(entry.wall_clock_seconds, 60);
        assert_eq!(entry.human_intervention, true);
        assert_eq!(entry.finalized, true);
    }

    /// Test parsing v2 format with risk_raised_count = 0 explicitly
    #[test]
    fn test_parse_csv_row_v2_zero_risk_count() {
        let row = "test-run-004,2026-03-17T15:43:01.484781592+00:00,code,Approved,1,,false,false,false,false,0,false,false,0,0,false,true";
        
        let entry = Ledger::parse_csv_row(row).expect("Should parse v2 with zero risk count");
        
        assert_eq!(entry.risk_raised_count, 0);
        assert_eq!(entry.finalized, true);
    }

    /// Test error on insufficient columns (< 15)
    #[test]
    fn test_parse_csv_row_insufficient_columns() {
        // Only 10 columns - should fail
        let row = "test-run-005,2026-03-17T15:43:01.484781592+00:00,code,Pending,1,,false,false,false,false";
        
        let result = Ledger::parse_csv_row(row);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Invalid CSV row"));
        assert!(err_msg.contains("expected 15 or 17 columns"));
    }

    /// Test parsing 16 columns (edge case - should be treated as v1)
    #[test]
    fn test_parse_csv_row_16_columns() {
        // 16 columns is between v1 and v2 - should be treated as v1 (no risk_raised_count)
        // The 16th column would be "extra" which should be parsed as the finalized boolean
        // So we use "true" as the 16th column to make it valid
        let row = "test-run-006,2026-03-17T15:43:01.484781592+00:00,code,Pending,1,,false,false,false,false,false,false,0,0,false,true";
        
        let entry = Ledger::parse_csv_row(row).expect("Should parse 16-col format");
        
        assert_eq!(entry.run_id, "test-run-006");
        // 16 columns is < 17, so treated as v1
        assert_eq!(entry.risk_raised_count, 0);
        assert_eq!(entry.finalized, true);
    }

    /// Test loading ledger with mixed format entries
    #[test]
    fn test_load_entries_mixed_formats() {
        let temp_dir = TempDir::new().unwrap();
        let csv_path = temp_dir.path().join("production_ledger.csv");
        
        // Create CSV with mixed formats
        {
            let mut file = File::create(&csv_path).unwrap();
            writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,risk_raised_count,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized").unwrap();
            // v2 format entry
            writeln!(file, "run-001,2026-03-17T15:43:01.484781592+00:00,code,Approved,1,,false,false,false,false,3,false,false,100,30,false,true").unwrap();
            // v1 format entry (no risk_raised_count)
            writeln!(file, "run-002,2026-03-17T15:43:02.484781592+00:00,code,Pending,1,,false,false,false,false,false,false,0,0,false,false").unwrap();
            // v2 format entry with high risk count
            writeln!(file, "run-003,2026-03-17T15:43:03.484781592+00:00,code,Escalated,2,Tianquan,true,false,false,false,10,true,false,500,120,true,true").unwrap();
        }
        
        let entries = Ledger::load_entries(&csv_path).expect("Should load mixed formats");
        
        assert_eq!(entries.len(), 3);
        
        // Check v2 entry
        let entry1 = entries.get("run-001").expect("run-001 should exist");
        assert_eq!(entry1.risk_raised_count, 3);
        assert_eq!(entry1.final_state, RunState::Approved);
        
        // Check v1 entry (default risk_raised_count = 0)
        let entry2 = entries.get("run-002").expect("run-002 should exist");
        assert_eq!(entry2.risk_raised_count, 0);
        assert_eq!(entry2.final_state, RunState::Pending);
        
        // Check v2 entry with high risk count
        let entry3 = entries.get("run-003").expect("run-003 should exist");
        assert_eq!(entry3.risk_raised_count, 10);
        assert_eq!(entry3.escalation_triggered, true);
        assert_eq!(entry3.authority_violation, true);
    }

    /// Test that saved entries can be loaded back (round-trip)
    #[test]
    fn test_ledger_round_trip() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create a new ledger
        let mut ledger = Ledger::new(&temp_dir.path()).expect("Should create ledger");
        
        // Start and finalize a run
        ledger.start_run("round-trip-test", "code").expect("Should start run");
        ledger.finalize_run("round-trip-test", RunState::Approved).expect("Should finalize");
        
        // Create a new ledger instance pointing to the same path
        let ledger2 = Ledger::new(&temp_dir.path()).expect("Should load ledger");
        let entries = ledger2.get_entries().expect("Should get entries");
        
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];
        assert_eq!(entry.run_id, "round-trip-test");
        assert_eq!(entry.input_type, "code");
        assert_eq!(entry.final_state, RunState::Approved);
        assert_eq!(entry.finalized, true);
    }

    /// Test parsing all possible RunState values
    #[test]
    fn test_parse_all_run_states() {
        let states = vec![
            ("Pending", RunState::Pending),
            ("InProgress", RunState::InProgress),
            ("Reviewing", RunState::Reviewing),
            ("Escalated", RunState::Escalated),
            ("Approved", RunState::Approved),
            ("Rejected", RunState::Rejected),
            ("RolledBack", RunState::RolledBack),
            ("Archived", RunState::Archived),
            ("Terminated", RunState::Terminated),
        ];
        
        for (state_str, expected_state) in states {
            let row = format!(
                "test-run,2026-03-17T15:43:01.484781592+00:00,code,{},1,,false,false,false,false,0,false,false,0,0,false,false",
                state_str
            );
            let entry = Ledger::parse_csv_row(&row).expect(&format!("Should parse state {}", state_str));
            assert_eq!(entry.final_state, expected_state, "State {} should parse correctly", state_str);
        }
    }

    /// Test parsing various Seat values
    #[test]
    fn test_parse_various_seats() {
        let seats = vec![
            "Tianshu", "Tianxuan", "Tianji", "Tianquan", "Yuheng",
            "Kaiyang", "Yaoguang", "Qinglong", "Baihu", "Zhuque",
            "Xuanwu", "Yangjian", "Baozheng", "Zhongkui", "Luban",
            "Zhugeliang", "Nezha", "Xiwangmu", "Fengdudadi"
        ];
        
        for seat in seats {
            let row = format!(
                "test-run,2026-03-17T15:43:01.484781592+00:00,code,Approved,1,{},false,false,false,false,0,false,false,0,0,false,true",
                seat
            );
            let entry = Ledger::parse_csv_row(&row).expect(&format!("Should parse seat {}", seat));
            assert!(entry.veto_used.is_some(), "Seat {} should be parsed", seat);
        }
    }

    /// Test backward compatibility: v1 CSV file can be loaded and saved as v2
    #[test]
    fn test_v1_to_v2_migration() {
        let temp_dir = TempDir::new().unwrap();
        let csv_path = temp_dir.path().join("production_ledger.csv");
        
        // Create old-format CSV (v1 - 15 columns)
        {
            let mut file = File::create(&csv_path).unwrap();
            // Old header without risk_raised_count
            writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention,finalized").unwrap();
            // Old format row (15 columns)
            writeln!(file, "legacy-run,2026-03-17T15:43:01.484781592+00:00,code,Approved,1,Tianquan,false,false,false,false,false,false,100,30,false,true").unwrap();
        }
        
        // Load the old format
        let entries = Ledger::load_entries(&csv_path).expect("Should load v1 format");
        assert_eq!(entries.len(), 1);
        
        let entry = entries.get("legacy-run").expect("legacy-run should exist");
        assert_eq!(entry.risk_raised_count, 0); // Default value for v1
        assert_eq!(entry.final_state, RunState::Approved);
        
        // Create a new ledger (which will save in v2 format)
        let mut ledger = Ledger::new(&temp_dir.path()).expect("Should create ledger");
        ledger.start_run("new-run", "code").expect("Should start new run");
        ledger.finalize_run("new-run", RunState::Approved).expect("Should finalize");
        
        // Verify the file now has v2 format header
        let content = std::fs::read_to_string(&csv_path).expect("Should read file");
        assert!(content.contains("risk_raised_count"), "Header should include risk_raised_count");
        
        // Verify both entries are present after save
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 entries
    }
}

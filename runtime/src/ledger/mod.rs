use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
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
        }
    }
    
    /// Serialize to CSV row
    pub fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
        )
    }
    
    /// Deserialize from CSV row
    pub fn from_csv_row(row: &str) -> Result<Self> {
        let parts: Vec<&str> = row.split(',').collect();
        if parts.len() < 15 {
            anyhow::bail!("Invalid CSV row: insufficient columns");
        }
        
        Ok(Self {
            run_id: parts[0].to_string(),
            timestamp: DateTime::parse_from_rfc3339(parts[1])
                .map_err(|e| anyhow::anyhow!("Failed to parse timestamp: {}", e))?
                .into(),
            input_type: parts[2].to_string(),
            final_state: parse_run_state(parts[3])?,
            seats_participated: Vec::new(), // Simplified
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
        })
    }
}

/// Production ledger
pub struct Ledger {
    storage_path: PathBuf,
    current_run: Option<LedgerEntry>,
    start_time: Option<DateTime<Utc>>,
}

impl Ledger {
    /// Create a new ledger
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
            writeln!(file, "run_id,timestamp,input_type,final_state,seats_participated,veto_used,escalation_triggered,rollback_executed,archive_executed,terminate_executed,authority_violation,fake_closure,tokens_used,wall_clock_seconds,human_intervention")?;
        }
        
        Ok(Self {
            storage_path,
            current_run: None,
            start_time: None,
        })
    }
    
    /// Start a new run
    pub fn start_run(&mut self, run_id: impl Into<String>, input_type: impl Into<String>) {
        self.current_run = Some(LedgerEntry::new(run_id, input_type));
        self.start_time = Some(Utc::now());
    }
    
    /// Record seat participation
    pub fn record_participation(&mut self, seat: Seat) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            if !entry.seats_participated.contains(&seat) {
                entry.seats_participated.push(seat);
            }
        }
        Ok(())
    }
    
    /// Record veto
    pub fn record_veto(&mut self, seat: Seat) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.veto_used = Some(seat);
        }
        Ok(())
    }
    
    /// Record escalation
    pub fn record_escalation(&mut self) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.escalation_triggered = true;
        }
        Ok(())
    }
    
    /// Record rollback
    pub fn record_rollback(&mut self) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.rollback_executed = true;
        }
        Ok(())
    }
    
    /// Record archive
    pub fn record_archive(&mut self) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.archive_executed = true;
        }
        Ok(())
    }
    
    /// Record termination
    pub fn record_terminate(&mut self) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.terminate_executed = true;
        }
        Ok(())
    }
    
    /// Record red line violation
    pub fn record_red_line(&mut self, violation_type: RedLineViolation) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            match violation_type {
                RedLineViolation::AuthorityViolation => entry.authority_violation = true,
                RedLineViolation::FakeClosure => entry.fake_closure = true,
            }
        }
        Ok(())
    }
    
    /// Record token usage
    pub fn record_tokens(&mut self, tokens: u64) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.tokens_used = tokens;
        }
        Ok(())
    }
    
    /// Record human intervention
    pub fn record_human_intervention(&mut self) -> Result<()> {
        if let Some(entry) = &mut self.current_run {
            entry.human_intervention = true;
        }
        Ok(())
    }
    
    /// Finalize the current run
    pub fn finalize_run(&mut self, final_state: RunState) -> Result<()> {
        if let Some(mut entry) = self.current_run.take() {
            entry.final_state = final_state;
            
            // Calculate wall clock time
            if let Some(start) = self.start_time {
                entry.wall_clock_seconds = (Utc::now() - start).num_seconds() as u64;
            }
            
            // Append to CSV
            let csv_path = self.storage_path.join("production_ledger.csv");
            let mut file = OpenOptions::new()
                .append(true)
                .open(&csv_path)
                .with_context(|| "Failed to open ledger CSV for append")?;
            
            writeln!(file, "{}", entry.to_csv_row())
                .with_context(|| "Failed to write ledger entry")?;
            
            self.start_time = None;
        }
        
        Ok(())
    }
    
    /// Get all ledger entries
    pub fn get_entries(&self) -> Result<Vec<LedgerEntry>> {
        let csv_path = self.storage_path.join("production_ledger.csv");
        
        if !csv_path.exists() {
            return Ok(Vec::new());
        }
        
        let file = File::open(&csv_path)
            .with_context(|| "Failed to open ledger CSV")?;
        
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        
        for (i, line) in reader.lines().enumerate() {
            if i == 0 {
                // Skip header
                continue;
            }
            
            let line = line.with_context(|| "Failed to read ledger line")?;
            if line.trim().is_empty() {
                continue;
            }
            
            match LedgerEntry::from_csv_row(&line) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    tracing::warn!("Failed to parse ledger row {}: {}", i, e);
                }
            }
        }
        
        Ok(entries)
    }
    
    /// Get run count by state
    pub fn get_run_counts(&self) -> Result<HashMap<RunState, usize>> {
        let entries = self.get_entries()?;
        let mut counts = HashMap::new();
        
        for entry in entries {
            *counts.entry(entry.final_state).or_insert(0) += 1;
        }
        
        Ok(counts)
    }
    
    /// Get stability metrics
    pub fn get_stability_metrics(&self) -> Result<StabilityMetrics> {
        let entries = self.get_entries()?;
        
        let total = entries.len();
        let authority_violations = entries.iter().filter(|e| e.authority_violation).count();
        let fake_closures = entries.iter().filter(|e| e.fake_closure).count();
        let rollbacks = entries.iter().filter(|e| e.rollback_executed).count();
        let terminations = entries.iter().filter(|e| e.terminate_executed).count();
        
        Ok(StabilityMetrics {
            total_runs: total,
            authority_violations,
            fake_closures,
            rollbacks,
            terminations,
            clean_runs: total - authority_violations - fake_closures,
        })
    }
}

/// Red line violation types
#[derive(Debug, Clone, Copy)]
pub enum RedLineViolation {
    AuthorityViolation,
    FakeClosure,
}

/// Stability metrics
#[derive(Debug, Clone)]
pub struct StabilityMetrics {
    pub total_runs: usize,
    pub authority_violations: usize,
    pub fake_closures: usize,
    pub rollbacks: usize,
    pub terminations: usize,
    pub clean_runs: usize,
}

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

use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::timeout;

/// tmux session manager
pub struct TmuxManager {
    session_prefix: String,
}

impl TmuxManager {
    /// Create a new tmux manager
    pub fn new(session_prefix: impl Into<String>) -> Self {
        Self {
            session_prefix: session_prefix.into(),
        }
    }
    
    /// Check if tmux is available
    pub fn check_tmux(&self) -> Result<()> {
        let output = Command::new("which")
            .arg("tmux")
            .output()
            .context("Failed to check tmux availability")?;
        
        if !output.status.success() {
            anyhow::bail!("tmux is not installed or not in PATH");
        }
        
        Ok(())
    }
    
    /// Create a new tmux session
    pub fn create_session(&self, session_name: impl AsRef<str>) -> Result<()> {
        let full_name = format!("{}_{}", self.session_prefix, session_name.as_ref());
        
        // Check if session already exists
        if self.session_exists(&full_name)? {
            tracing::debug!("tmux session {} already exists", full_name);
            return Ok(());
        }
        
        let output = Command::new("tmux")
            .args(&["new-session", "-d", "-s", &full_name])
            .output()
            .with_context(|| format!("Failed to create tmux session {}", full_name))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to create tmux session: {}", stderr);
        }
        
        tracing::info!("Created tmux session: {}", full_name);
        Ok(())
    }
    
    /// Create a new window in a session
    pub fn create_window(&self, session_name: impl AsRef<str>, window_name: impl AsRef<str>) -> Result<()> {
        let full_session = format!("{}_{}", self.session_prefix, session_name.as_ref());
        let window_name = window_name.as_ref();
        
        let output = Command::new("tmux")
            .args(&[
                "new-window",
                "-t", &full_session,
                "-n", window_name,
            ])
            .output()
            .with_context(|| format!("Failed to create tmux window {}:{}", full_session, window_name))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Window might already exist, which is ok
            if !stderr.contains("duplicate") {
                anyhow::bail!("Failed to create tmux window: {}", stderr);
            }
        }
        
        tracing::debug!("Created tmux window {}:{}", full_session, window_name);
        Ok(())
    }
    
    /// Send a command to a tmux window
    #[allow(dead_code)]
    pub fn send_command(&self, session_name: impl AsRef<str>, window_name: impl AsRef<str>, command: impl AsRef<str>) -> Result<()> {
        let full_session = format!("{}_{}", self.session_prefix, session_name.as_ref());
        let target = format!("{}:{}", full_session, window_name.as_ref());
        let command = command.as_ref();
        
        let output = Command::new("tmux")
            .args(&[
                "send-keys",
                "-t", &target,
                command,
                "Enter",
            ])
            .output()
            .with_context(|| format!("Failed to send command to {}", target))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to send command: {}", stderr);
        }
        
        tracing::debug!("Sent command to {}: {}", target, command);
        Ok(())
    }
    
    /// Capture output from a tmux window
    #[allow(dead_code)]
    pub fn capture_output(&self, session_name: impl AsRef<str>, window_name: impl AsRef<str>) -> Result<String> {
        let full_session = format!("{}_{}", self.session_prefix, session_name.as_ref());
        let target = format!("{}:{}", full_session, window_name.as_ref());
        
        let output = Command::new("tmux")
            .args(&[
                "capture-pane",
                "-t", &target,
                "-p",
            ])
            .output()
            .with_context(|| format!("Failed to capture output from {}", target))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to capture output: {}", stderr);
        }
        
        String::from_utf8(output.stdout)
            .with_context(|| "Failed to decode tmux output as UTF-8")
    }
    
    /// Check if a session exists
    pub fn session_exists(&self, session_name: impl AsRef<str>) -> Result<bool> {
        let session_name = session_name.as_ref();
        
        let output = Command::new("tmux")
            .args(&["has-session", "-t", session_name])
            .output()
            .context("Failed to check tmux session")?;
        
        Ok(output.status.success())
    }
    
    /// List all sessions with our prefix
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let output = Command::new("tmux")
            .args(&["list-sessions", "-F", "#S"])
            .output()
            .context("Failed to list tmux sessions")?;
        
        if !output.status.success() {
            // No sessions is ok
            return Ok(Vec::new());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let sessions: Vec<String> = stdout
            .lines()
            .filter(|s| s.starts_with(&self.session_prefix))
            .map(|s| s.to_string())
            .collect();
        
        Ok(sessions)
    }
    
    /// Kill a session
    pub fn kill_session(&self, session_name: impl AsRef<str>) -> Result<()> {
        let full_name = format!("{}_{}", self.session_prefix, session_name.as_ref());
        
        let output = Command::new("tmux")
            .args(&["kill-session", "-t", &full_name])
            .output()
            .with_context(|| format!("Failed to kill tmux session {}", full_name))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to kill session: {}", stderr);
        }
        
        tracing::info!("Killed tmux session: {}", full_name);
        Ok(())
    }
    
    /// Kill all sessions with our prefix
    pub fn kill_all_sessions(&self) -> Result<()> {
        let sessions = self.list_sessions()?;
        
        for session in sessions {
            let output = Command::new("tmux")
                .args(&["kill-session", "-t", &session])
                .output()
                .with_context(|| format!("Failed to kill tmux session {}", session))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                tracing::warn!("Failed to kill session {}: {}", session, stderr);
            } else {
                tracing::info!("Killed tmux session: {}", session);
            }
        }
        
        Ok(())
    }
    
    /// Check if a process is running in a window
    #[allow(dead_code)]
    pub fn is_process_running(&self, session_name: impl AsRef<str>, window_name: impl AsRef<str>) -> Result<bool> {
        let full_session = format!("{}_{}", self.session_prefix, session_name.as_ref());
        let target = format!("{}:{}", full_session, window_name.as_ref());
        
        // Check if pane is active and has a process
        let output = Command::new("tmux")
            .args(&[
                "list-panes",
                "-t", &target,
                "-F", "#{pane_active} #{pane_pid}",
            ])
            .output()
            .context("Failed to check tmux pane status")?;
        
        if !output.status.success() {
            return Ok(false);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(!stdout.trim().is_empty())
    }
    
    /// Wait for a process to complete (with timeout)
    #[allow(dead_code)]
    pub async fn wait_for_process(&self, session_name: impl AsRef<str>, window_name: impl AsRef<str>, timeout_secs: u64) -> Result<bool> {
        let session_name = session_name.as_ref();
        let window_name = window_name.as_ref();
        
        let result = timeout(
            Duration::from_secs(timeout_secs),
            async {
                loop {
                    if !self.is_process_running(session_name, window_name)? {
                        return Ok::<_, anyhow::Error>(true);
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        ).await;
        
        match result {
            Ok(Ok(true)) => Ok(true),
            Ok(Ok(false)) => Ok(false),
            Ok(Err(e)) => Err(e),
            Err(_) => Ok(false), // Timeout
        }
    }
}

/// Create a governance session with all 19 seats
pub fn create_governance_session(manager: &TmuxManager, run_id: impl AsRef<str>) -> Result<()> {
    let run_id = run_id.as_ref();
    
    // Create main session
    manager.create_session(run_id)?;
    
    // Create windows for each of the 19 seats
    use crate::governance::all_seats;
    
    for seat in all_seats() {
        let seat_name = format!("{:?}", seat).to_lowercase();
        manager.create_window(run_id, &seat_name)?;
    }
    
    tracing::info!("Created governance session {} with 19 seats", run_id);
    Ok(())
}

/// Attach to a tmux session (for human operators)
pub fn attach_session(session_name: impl AsRef<str>) -> Result<()> {
    let session_name = session_name.as_ref();
    
    let status = Command::new("tmux")
        .args(&["attach", "-t", session_name])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to attach to tmux session {}", session_name))?;
    
    if !status.success() {
        anyhow::bail!("tmux attach failed");
    }
    
    Ok(())
}

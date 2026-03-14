use anyhow::{Context, Result};
use std::process::Command;

use crate::models::{Message, ModelProvider, Role};

/// Kimi CLI provider - uses the official kimi-cli tool
/// This is required for Kimi Code 699 membership API keys
pub struct KimiCliProvider {
    api_key: String,
    timeout_secs: u64,
}

impl KimiCliProvider {
    pub fn new(api_key: String, timeout_secs: u64) -> Self {
        Self {
            api_key,
            timeout_secs,
        }
    }
    
    /// Check if kimi CLI is installed
    pub fn check_cli() -> Result<()> {
        let output = Command::new("which")
            .arg("kimi")
            .output()
            .context("Failed to check if kimi CLI is installed")?;
        
        if !output.status.success() {
            anyhow::bail!("kimi CLI is not installed. Install with: uv tool install kimi-cli");
        }
        
        Ok(())
    }
    
    fn build_prompt(&self, messages: &[Message]) -> String {
        messages
            .iter()
            .map(|m| {
                let role_str = match m.role {
                    Role::System => "System",
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                };
                format!("{}: {}", role_str, m.content)
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

#[async_trait::async_trait]
impl ModelProvider for KimiCliProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let prompt = self.build_prompt(&messages);
        
        // Run kimi CLI with print mode for non-interactive output
        let api_key = self.api_key.clone();
        let timeout_secs = self.timeout_secs;
        
        let result = tokio::task::spawn_blocking(move || {
            Command::new("kimi")
                .env("KIMI_API_KEY", api_key)
                .args(&[
                    "--print",
                    "--prompt", &prompt,
                    "--output-format", "text",
                    "--final-message-only",
                    "--yolo", // Auto-approve actions
                ])
                .current_dir("/tmp") // Use temp dir to avoid file operations in project
                .output()
                .context("Failed to execute kimi CLI")
        }).await;
        
        match result {
            Ok(Ok(output)) => {
                if output.status.success() {
                    let response = String::from_utf8(output.stdout)
                        .context("Failed to decode kimi CLI output")?;
                    Ok(response.trim().to_string())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("kimi CLI failed: {}", stderr)
                }
            }
            Ok(Err(e)) => Err(e),
            Err(e) => anyhow::bail!("kimi CLI task failed: {}", e),
        }
    }
    
    fn name(&self) -> &str {
        "kimi-cli"
    }
    
    fn model(&self) -> &str {
        "kimi-for-coding"
    }
}

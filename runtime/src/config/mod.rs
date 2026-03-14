use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// DragonCore Runtime Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Runtime identity
    pub runtime: RuntimeConfig,
    
    /// Governance configuration
    pub governance: GovernanceConfig,
    
    /// Model providers
    pub providers: HashMap<String, ProviderConfig>,
    
    /// Execution environment
    pub execution: ExecutionConfig,
    
    /// Ledger configuration
    pub ledger: LedgerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub version: String,
    pub data_dir: PathBuf,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    /// Path to governance constitution files
    pub constitution_path: PathBuf,
    
    /// Default escalation timeout (seconds)
    pub escalation_timeout: u64,
    
    /// Enable strict mode (no bypass)
    pub strict_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: ProviderType,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    Kimi,
    KimiCli, // For Kimi Code 699 membership keys
    DeepSeek,
    Qwen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    /// tmux session name prefix
    pub tmux_prefix: String,
    
    /// Git worktree base path
    pub worktree_base: PathBuf,
    
    /// Maximum concurrent agents
    pub max_concurrent_agents: usize,
    
    /// Process isolation enabled
    pub isolation_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerConfig {
    /// Ledger storage path
    pub storage_path: PathBuf,
    
    /// Auto-archive after N runs
    pub auto_archive_threshold: usize,
    
    /// Retention days for archived runs
    pub retention_days: u64,
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = directories::ProjectDirs::from("org", "dragoncore", "runtime")
            .map(|d| d.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("./data"));
        
        Self {
            runtime: RuntimeConfig {
                name: "dragoncore".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                data_dir: data_dir.clone(),
                log_level: "info".to_string(),
            },
            governance: GovernanceConfig {
                constitution_path: data_dir.join("constitution"),
                escalation_timeout: 300,
                strict_mode: true,
            },
            providers: HashMap::new(),
            execution: ExecutionConfig {
                tmux_prefix: "dragoncore".to_string(),
                worktree_base: data_dir.join("worktrees"),
                max_concurrent_agents: 19,
                isolation_enabled: true,
            },
            ledger: LedgerConfig {
                storage_path: data_dir.join("ledger"),
                auto_archive_threshold: 100,
                retention_days: 365,
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config from {:?}", path.as_ref()))?;
        
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config TOML")?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .with_context(|| "Failed to serialize config")?;
        
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {:?}", path.as_ref()))?;
        
        Ok(())
    }
    
    /// Initialize default configuration with data directory
    pub fn init_default() -> Result<Self> {
        let mut config = Config::default();
        
        // Ensure data directories exist
        std::fs::create_dir_all(&config.runtime.data_dir)?;
        std::fs::create_dir_all(&config.governance.constitution_path)?;
        std::fs::create_dir_all(&config.execution.worktree_base)?;
        std::fs::create_dir_all(&config.ledger.storage_path)?;
        
        // Add default providers if API keys exist
        if let Ok(api_key) = std::env::var("KIMI_API_KEY") {
            // Detect if this is a Kimi Code key (kimi.com) vs Moonshot key
            // Kimi Code keys start with "sk-kimi-" and work with CLI
            let provider_type = if api_key.starts_with("sk-kimi-") {
                // Use kimi-cli for Kimi Code 699 membership keys
                ProviderType::KimiCli
            } else {
                ProviderType::Kimi
            };
            
            config.providers.insert("kimi".to_string(), ProviderConfig {
                provider_type,
                api_key,
                base_url: "https://api.kimi.com/coding/v1".to_string(),
                model: "kimi-for-coding".to_string(),
                timeout: 120, // CLI mode may take longer
            });
        }
        
        if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
            config.providers.insert("deepseek".to_string(), ProviderConfig {
                provider_type: ProviderType::DeepSeek,
                api_key,
                base_url: "https://api.deepseek.com/v1".to_string(),
                model: "deepseek-chat".to_string(),
                timeout: 60,
            });
        }
        
        if let Ok(api_key) = std::env::var("QWEN_API_KEY") {
            config.providers.insert("qwen".to_string(), ProviderConfig {
                provider_type: ProviderType::Qwen,
                api_key,
                base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
                model: "qwen-max".to_string(),
                timeout: 60,
            });
        }
        
        Ok(config)
    }
}

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const LEGACY_SCHEMA_VERSION: &str = "1.0";
const NORMALIZED_SCHEMA_VERSION: &str = "2.0";

/// DragonCore Runtime Configuration (dual-read: legacy + normalized)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Root config schema version marker.
    /// `1.x` = legacy-first, `2.x` = normalized-first.
    #[serde(default = "default_config_schema_version")]
    pub config_schema_version: String,

    /// Persisted state compatibility planning marker (no migration behavior in PR-2)
    #[serde(default)]
    pub state_schema: StateSchemaCompatibility,

    /// Runtime identity
    pub runtime: RuntimeConfig,

    /// Governance configuration
    pub governance: GovernanceConfig,

    /// Legacy model providers
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,

    /// Legacy seat to provider mapping (multi-model support precursor)
    #[serde(default)]
    pub seat_models: Option<SeatModelMapping>,

    /// Normalized brain topology
    #[serde(default)]
    pub brains: Option<BrainsConfig>,

    /// Normalized provider registry
    #[serde(default)]
    pub provider_registry: HashMap<String, ProviderConfig>,

    /// Normalized model registry
    #[serde(default)]
    pub model_registry: HashMap<String, ModelRegistryEntry>,

    /// Normalized seat-level policies
    #[serde(default)]
    pub seat_policies: HashMap<String, SeatPolicy>,

    /// Tool adapter extension point (stub-only in PR-2)
    #[serde(default)]
    pub tool_adapters: HashMap<String, ToolAdapterConfig>,

    /// Evolution/Forge extension point (disabled by default in PR-2)
    #[serde(default)]
    pub evolution: EvolutionConfig,

    /// Execution environment
    pub execution: ExecutionConfig,

    /// Ledger configuration
    pub ledger: LedgerConfig,
}

fn default_config_schema_version() -> String {
    LEGACY_SCHEMA_VERSION.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSchemaCompatibility {
    #[serde(default = "default_state_schema_version")]
    pub current_version: u32,
    #[serde(default = "default_state_schema_version")]
    pub min_compatible_version: u32,
    #[serde(default = "default_state_migration_strategy")]
    pub migration_strategy: String,
}

fn default_state_schema_version() -> u32 {
    1
}

fn default_state_migration_strategy() -> String {
    "deferred".to_string()
}

impl Default for StateSchemaCompatibility {
    fn default() -> Self {
        Self {
            current_version: default_state_schema_version(),
            min_compatible_version: default_state_schema_version(),
            migration_strategy: default_state_migration_strategy(),
        }
    }
}

/// Seat model mapping for legacy multi-model support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatModelMapping {
    /// Mapping from seat name to provider name
    #[serde(flatten)]
    pub mapping: HashMap<String, String>,
    /// Default provider for unspecified seats
    #[serde(default = "default_provider")]
    pub default: String,
}

fn default_provider() -> String {
    "default".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainsConfig {
    #[serde(default = "default_sovereign_seat")]
    pub sovereign_seat: String,
    #[serde(default)]
    pub advisory_seats: Vec<String>,
}

fn default_sovereign_seat() -> String {
    "Tianshu".to_string()
}

impl Default for BrainsConfig {
    fn default() -> Self {
        Self {
            sovereign_seat: default_sovereign_seat(),
            advisory_seats: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryEntry {
    /// Provider registry key
    pub provider: String,
    /// Concrete model ID used by provider
    pub model: String,
    /// Optional timeout override (seconds)
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatPolicy {
    /// Primary model registry key
    pub primary_model: String,
    /// Fallback model registry keys in priority order
    #[serde(default)]
    pub fallback_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolAdapterConfig {
    pub adapter_type: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_evolution_mode")]
    pub mode: String,
}

fn default_evolution_mode() -> String {
    "disabled".to_string()
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: default_evolution_mode(),
        }
    }
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
    // Multi-model support fields (optional)
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub cost_tier: Option<String>,
    #[serde(default)]
    pub speed: Option<String>,
    #[serde(default)]
    pub use_case: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    Kimi,
    KimiCli,
    DeepSeek,
    Qwen,
    OpenAiCompatible,
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

/// Canonical runtime-consumable representation (source-agnostic normalization target)
#[derive(Debug, Clone)]
pub struct NormalizedConfig {
    pub normalized_schema_version: String,
    pub state_schema: StateSchemaCompatibility,
    pub runtime: RuntimeConfig,
    pub governance: GovernanceConfig,
    pub brains: BrainsConfig,
    pub provider_registry: HashMap<String, ProviderConfig>,
    pub model_registry: HashMap<String, ModelRegistryEntry>,
    pub seat_policies: HashMap<String, SeatPolicy>,
    pub tool_adapters: HashMap<String, ToolAdapterConfig>,
    pub evolution: EvolutionConfig,
    pub default_provider_hint: Option<String>,
    pub execution: ExecutionConfig,
    pub ledger: LedgerConfig,
}

impl NormalizedConfig {
    /// Resolve seat -> provider selection outcome used by current runtime behavior.
    pub fn provider_for_seat(&self, seat: Option<&str>) -> Option<&str> {
        if let Some(seat_name) = seat {
            if let Some(policy) = self.seat_policies.get(seat_name) {
                if let Some(model) = self.model_registry.get(&policy.primary_model) {
                    return Some(model.provider.as_str());
                }
            }
        }

        let sovereign = self.brains.sovereign_seat.as_str();
        if let Some(policy) = self.seat_policies.get(sovereign) {
            if let Some(model) = self.model_registry.get(&policy.primary_model) {
                return Some(model.provider.as_str());
            }
        }

        if let Some(default_name) = &self.default_provider_hint {
            if self.provider_registry.contains_key(default_name) {
                return Some(default_name.as_str());
            }
        }

        self.provider_registry.keys().next().map(|k| k.as_str())
    }

    pub fn has_providers(&self) -> bool {
        !self.provider_registry.is_empty()
    }
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = directories::ProjectDirs::from("org", "dragoncore", "runtime")
            .map(|d| d.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("./data"));

        Self {
            config_schema_version: default_config_schema_version(),
            state_schema: StateSchemaCompatibility::default(),
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
            seat_models: None,
            brains: None,
            provider_registry: HashMap::new(),
            model_registry: HashMap::new(),
            seat_policies: HashMap::new(),
            tool_adapters: HashMap::new(),
            evolution: EvolutionConfig::default(),
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

        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config TOML")?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config to {:?}", path.as_ref()))?;

        Ok(())
    }

    /// Normalize legacy/new schemas into a canonical internal runtime representation.
    pub fn normalize(&self) -> Result<NormalizedConfig> {
        self.validate_schema_version()?;
        self.validate_state_schema()?;

        let has_new_schema = !self.provider_registry.is_empty()
            || !self.model_registry.is_empty()
            || !self.seat_policies.is_empty()
            || self.brains.is_some();

        let has_legacy_schema = !self.providers.is_empty() || self.seat_models.is_some();

        if has_new_schema && !self.providers.is_empty() {
            bail!("Config mixes legacy [providers] with normalized [provider_registry]; choose one schema mode");
        }

        if has_new_schema && self.seat_models.is_some() && !self.seat_policies.is_empty() {
            bail!("Config mixes legacy [seat_models] with normalized [seat_policies]; choose one schema mode");
        }

        if has_new_schema {
            return self.normalize_from_new_schema();
        }

        if has_legacy_schema {
            return self.normalize_from_legacy_schema();
        }

        // Empty providers are allowed (runtime will emit user-facing guidance when starting a run)
        self.normalize_from_legacy_schema()
    }

    fn validate_schema_version(&self) -> Result<()> {
        let version = self.config_schema_version.trim();
        if version.starts_with("1.") || version.starts_with("2.") {
            return Ok(());
        }

        bail!(
            "Unsupported config_schema_version '{}'. Supported major versions: 1.x (legacy), 2.x (normalized)",
            self.config_schema_version
        )
    }

    fn validate_state_schema(&self) -> Result<()> {
        if self.state_schema.min_compatible_version > self.state_schema.current_version {
            bail!(
                "Invalid state_schema: min_compatible_version ({}) cannot be greater than current_version ({})",
                self.state_schema.min_compatible_version,
                self.state_schema.current_version
            );
        }
        Ok(())
    }

    fn normalize_from_legacy_schema(&self) -> Result<NormalizedConfig> {
        let provider_registry = self.providers.clone();
        let mut model_registry = HashMap::new();

        for (provider_name, provider) in &provider_registry {
            model_registry.insert(
                format!("{provider_name}__default"),
                ModelRegistryEntry {
                    provider: provider_name.clone(),
                    model: provider.model.clone(),
                    timeout: Some(provider.timeout),
                },
            );
        }

        let mut seat_policies = HashMap::new();
        if let Some(seat_models) = &self.seat_models {
            for (seat, provider_name) in &seat_models.mapping {
                let model_key = format!("{provider_name}__default");
                if !model_registry.contains_key(&model_key) {
                    bail!(
                        "Legacy seat_models references unknown provider '{}' for seat '{}'",
                        provider_name,
                        seat
                    );
                }
                seat_policies.insert(
                    seat.clone(),
                    SeatPolicy {
                        primary_model: model_key,
                        fallback_models: vec![],
                    },
                );
            }

            let default_key = format!("{}__default", seat_models.default);
            if !seat_models.default.is_empty()
                && !provider_registry.is_empty()
                && !model_registry.contains_key(&default_key)
            {
                bail!(
                    "Legacy seat_models.default references unknown provider '{}'",
                    seat_models.default
                );
            }
        }

        let brains = self.brains.clone().unwrap_or_default();

        let default_provider_hint = self.seat_models.as_ref().map(|s| s.default.clone());

        Ok(NormalizedConfig {
            normalized_schema_version: NORMALIZED_SCHEMA_VERSION.to_string(),
            state_schema: self.state_schema.clone(),
            runtime: self.runtime.clone(),
            governance: self.governance.clone(),
            brains,
            provider_registry,
            model_registry,
            seat_policies,
            tool_adapters: self.tool_adapters.clone(),
            evolution: self.evolution.clone(),
            default_provider_hint,
            execution: self.execution.clone(),
            ledger: self.ledger.clone(),
        })
    }

    fn normalize_from_new_schema(&self) -> Result<NormalizedConfig> {
        if self.provider_registry.is_empty() && !self.model_registry.is_empty() {
            bail!("Normalized schema defines [model_registry] but [provider_registry] is empty");
        }

        for (model_key, model_entry) in &self.model_registry {
            if !self.provider_registry.contains_key(&model_entry.provider) {
                bail!(
                    "model_registry.{} references unknown provider '{}'",
                    model_key,
                    model_entry.provider
                );
            }
        }

        for (seat, policy) in &self.seat_policies {
            self.validate_seat_policy(seat, policy)?;
        }

        Ok(NormalizedConfig {
            normalized_schema_version: NORMALIZED_SCHEMA_VERSION.to_string(),
            state_schema: self.state_schema.clone(),
            runtime: self.runtime.clone(),
            governance: self.governance.clone(),
            brains: self.brains.clone().unwrap_or_default(),
            provider_registry: self.provider_registry.clone(),
            model_registry: self.model_registry.clone(),
            seat_policies: self.seat_policies.clone(),
            tool_adapters: self.tool_adapters.clone(),
            evolution: self.evolution.clone(),
            default_provider_hint: None,
            execution: self.execution.clone(),
            ledger: self.ledger.clone(),
        })
    }

    fn validate_seat_policy(&self, seat: &str, policy: &SeatPolicy) -> Result<()> {
        if !self.model_registry.contains_key(&policy.primary_model) {
            bail!(
                "seat_policies.{} primary_model '{}' not found in model_registry",
                seat,
                policy.primary_model
            );
        }

        let mut seen = HashSet::new();
        for fallback in &policy.fallback_models {
            if !self.model_registry.contains_key(fallback) {
                bail!(
                    "seat_policies.{} fallback model '{}' not found in model_registry",
                    seat,
                    fallback
                );
            }

            if fallback == &policy.primary_model {
                bail!(
                    "seat_policies.{} fallback model '{}' duplicates primary_model",
                    seat,
                    fallback
                );
            }

            if !seen.insert(fallback) {
                bail!(
                    "seat_policies.{} fallback model '{}' appears multiple times",
                    seat,
                    fallback
                );
            }
        }

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
            let provider_type = if api_key.starts_with("sk-kimi-") {
                ProviderType::KimiCli
            } else {
                ProviderType::Kimi
            };

            config.providers.insert(
                "kimi".to_string(),
                ProviderConfig {
                    provider_type,
                    api_key,
                    base_url: "https://api.kimi.com/coding/v1".to_string(),
                    model: "kimi-for-coding".to_string(),
                    timeout: 120,
                    capability: Some("high".to_string()),
                    cost_tier: Some("medium".to_string()),
                    speed: Some("fast".to_string()),
                    use_case: Some(vec!["coding".to_string(), "chat".to_string()]),
                },
            );
        }

        if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
            config.providers.insert(
                "deepseek".to_string(),
                ProviderConfig {
                    provider_type: ProviderType::DeepSeek,
                    api_key,
                    base_url: "https://api.deepseek.com/v1".to_string(),
                    model: "deepseek-chat".to_string(),
                    timeout: 60,
                    capability: Some("high".to_string()),
                    cost_tier: Some("low".to_string()),
                    speed: Some("fast".to_string()),
                    use_case: Some(vec!["coding".to_string(), "analysis".to_string()]),
                },
            );
        }

        if let Ok(api_key) = std::env::var("QWEN_API_KEY") {
            config.providers.insert(
                "qwen".to_string(),
                ProviderConfig {
                    provider_type: ProviderType::Qwen,
                    api_key,
                    base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
                    model: "qwen-max".to_string(),
                    timeout: 60,
                    capability: Some("high".to_string()),
                    cost_tier: Some("medium".to_string()),
                    speed: Some("fast".to_string()),
                    use_case: Some(vec!["general".to_string(), "coding".to_string()]),
                },
            );
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_toml() -> &'static str {
        r#"
config_schema_version = "1.0"

[runtime]
name = "dragoncore"
version = "0.3.0"
data_dir = "./runtime_state"
log_level = "info"

[governance]
constitution_path = "./runtime_state/constitution"
escalation_timeout = 300
strict_mode = true

[providers.kimi]
provider_type = "kimi"
api_key = "k"
base_url = "https://api.kimi.com/coding/v1"
model = "kimi-for-coding"
timeout = 60

[seat_models]
Tianshu = "kimi"
default = "kimi"

[execution]
tmux_prefix = "dragoncore"
worktree_base = "./runtime_state/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "./runtime_state/ledger"
auto_archive_threshold = 100
retention_days = 365
"#
    }

    fn normalized_toml() -> &'static str {
        r#"
config_schema_version = "2.0"

[state_schema]
current_version = 1
min_compatible_version = 1
migration_strategy = "deferred"

[runtime]
name = "dragoncore"
version = "0.3.0"
data_dir = "./runtime_state"
log_level = "info"

[governance]
constitution_path = "./runtime_state/constitution"
escalation_timeout = 300
strict_mode = true

[brains]
sovereign_seat = "Tianshu"
advisory_seats = ["Tianji"]

[provider_registry.kimi]
provider_type = "kimi"
api_key = "k"
base_url = "https://api.kimi.com/coding/v1"
model = "kimi-for-coding"
timeout = 60

[model_registry.kimi_primary]
provider = "kimi"
model = "kimi-for-coding"
timeout = 60

[seat_policies.Tianshu]
primary_model = "kimi_primary"
fallback_models = []

[tool_adapters.deerflow]
adapter_type = "deerflow_worker"
enabled = false
endpoint = "http://127.0.0.1:9000"

[evolution]
enabled = false
mode = "disabled"

[execution]
tmux_prefix = "dragoncore"
worktree_base = "./runtime_state/worktrees"
max_concurrent_agents = 19
isolation_enabled = true

[ledger]
storage_path = "./runtime_state/ledger"
auto_archive_threshold = 100
retention_days = 365
"#
    }

    #[test]
    fn legacy_config_parses_and_normalizes() {
        let cfg: Config = toml::from_str(legacy_toml()).expect("legacy config parse");
        let normalized = cfg.normalize().expect("legacy normalize");

        assert!(normalized.has_providers());
        assert_eq!(normalized.provider_for_seat(Some("Tianshu")), Some("kimi"));
    }

    #[test]
    fn normalized_config_parses_and_normalizes() {
        let cfg: Config = toml::from_str(normalized_toml()).expect("normalized config parse");
        let normalized = cfg.normalize().expect("normalized normalize");

        assert_eq!(normalized.provider_for_seat(Some("Tianshu")), Some("kimi"));
        assert_eq!(normalized.evolution.enabled, false);
        assert!(normalized.tool_adapters.contains_key("deerflow"));
    }

    #[test]
    fn representative_equivalence_legacy_vs_normalized() {
        let legacy: Config = toml::from_str(legacy_toml()).unwrap();
        let normalized_cfg: Config = toml::from_str(normalized_toml()).unwrap();

        let legacy_norm = legacy.normalize().unwrap();
        let new_norm = normalized_cfg.normalize().unwrap();

        assert_eq!(
            legacy_norm.provider_for_seat(Some("Tianshu")),
            new_norm.provider_for_seat(Some("Tianshu"))
        );
    }

    #[test]
    fn invalid_conflicting_seat_policy_definitions() {
        let bad = format!(
            "{}\n[seat_policies.Tianshu]\nprimary_model = \"kimi_primary\"\nfallback_models = []\n",
            legacy_toml()
        );

        let cfg: Config = toml::from_str(&bad).unwrap();
        let err = cfg.normalize().unwrap_err().to_string();
        assert!(
            err.contains("mixes legacy [seat_models] with normalized [seat_policies]")
                || err.contains("mixes legacy [providers] with normalized [provider_registry]")
        );
    }

    #[test]
    fn invalid_missing_provider_reference() {
        let bad =
            normalized_toml().replace("provider = \"kimi\"", "provider = \"missing_provider\"");
        let cfg: Config = toml::from_str(&bad).unwrap();
        let err = cfg.normalize().unwrap_err().to_string();
        assert!(err.contains("references unknown provider"));
    }

    #[test]
    fn invalid_fallback_chain_duplicate_or_missing_model() {
        let bad = normalized_toml().replace(
            "fallback_models = []",
            "fallback_models = [\"kimi_primary\", \"missing_model\"]",
        );
        let cfg: Config = toml::from_str(&bad).unwrap();
        let err = cfg.normalize().unwrap_err().to_string();
        assert!(
            err.contains("duplicates primary_model") || err.contains("not found in model_registry")
        );
    }

    #[test]
    fn invalid_schema_version_rejected() {
        let bad = legacy_toml().replace(
            "config_schema_version = \"1.0\"",
            "config_schema_version = \"9.0\"",
        );
        let cfg: Config = toml::from_str(&bad).unwrap();
        let err = cfg.normalize().unwrap_err().to_string();
        assert!(err.contains("Unsupported config_schema_version"));
    }
}

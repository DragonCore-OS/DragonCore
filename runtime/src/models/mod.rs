#![allow(dead_code)]

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::config::{ProviderConfig, ProviderType};

mod kimi_cli;
pub use kimi_cli::KimiCliProvider;

mod openai_compatible;
pub use openai_compatible::OpenAiCompatibleProvider;

/// Model provider trait
#[async_trait::async_trait]
pub trait ModelProvider: Send + Sync {
    /// Send a chat completion request
    async fn chat(&self, messages: Vec<Message>) -> Result<String>;
    
    /// Get provider name
    fn name(&self) -> &str {
        "unknown"
    }
    
    /// Get model name
    fn model(&self) -> &str {
        "unknown"
    }
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Kimi provider
#[allow(dead_code)]
pub struct KimiProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
    timeout: Duration,
}

impl KimiProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .context("Failed to create HTTP client")?;
        
        // Support both Kimi Code (api.kimi.com) and Moonshot (api.moonshot.cn)
        let base_url = if config.base_url.contains("kimi.com") {
            config.base_url.clone()
        } else if config.base_url.contains("moonshot") {
            config.base_url.clone()
        } else {
            // Default to Kimi Code endpoint
            "https://api.kimi.com/coding/v1".to_string()
        };
        
        let model = if config.model.is_empty() || config.model == "kimi-latest" {
            "kimi-for-coding".to_string()
        } else {
            config.model.clone()
        };
        
        Ok(Self {
            client,
            api_key: config.api_key.clone(),
            base_url,
            model,
            timeout: Duration::from_secs(config.timeout),
        })
    }
}

#[async_trait::async_trait]
impl ModelProvider for KimiProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let request = KimiRequest {
            model: self.model.clone(),
            messages: messages.into_iter().map(|m| KimiMessage {
                role: match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                }.to_string(),
                content: m.content,
            }).collect(),
            stream: false,
        };
        
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Kimi")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Kimi API error: {} - {}", status, text);
        }
        
        let response: KimiResponse = response
            .json()
            .await
            .context("Failed to parse Kimi response")?;
        
        response.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .context("No response from Kimi")
    }
    
    fn name(&self) -> &str {
        "kimi"
    }
    
    fn model(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct KimiRequest {
    model: String,
    messages: Vec<KimiMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct KimiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct KimiResponse {
    choices: Vec<KimiChoice>,
}

#[derive(Debug, Deserialize)]
struct KimiChoice {
    message: KimiMessage,
}

/// DeepSeek provider
pub struct DeepSeekProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl DeepSeekProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            client,
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
            model: config.model.clone(),
        })
    }
}

#[async_trait::async_trait]
impl ModelProvider for DeepSeekProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let request = DeepSeekRequest {
            model: self.model.clone(),
            messages: messages.into_iter().map(|m| DeepSeekMessage {
                role: match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                }.to_string(),
                content: m.content,
            }).collect(),
            stream: false,
        };
        
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("DeepSeek API error: {} - {}", status, text);
        }
        
        let response: DeepSeekResponse = response
            .json()
            .await
            .context("Failed to parse DeepSeek response")?;
        
        response.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .context("No response from DeepSeek")
    }
    
    fn name(&self) -> &str {
        "deepseek"
    }
    
    fn model(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekMessage,
}

/// Qwen provider
pub struct QwenProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl QwenProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout))
            .build()
            .context("Failed to create HTTP client")?;
        
        Ok(Self {
            client,
            api_key: config.api_key.clone(),
            base_url: config.base_url.clone(),
            model: config.model.clone(),
        })
    }
}

#[async_trait::async_trait]
impl ModelProvider for QwenProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let request = QwenRequest {
            model: self.model.clone(),
            input: QwenInput {
                messages: messages.into_iter().map(|m| QwenMessage {
                    role: match m.role {
                        Role::System => "system",
                        Role::User => "user",
                        Role::Assistant => "assistant",
                    }.to_string(),
                    content: m.content,
                }).collect(),
            },
            parameters: QwenParameters {
                result_format: "message".to_string(),
            },
        };
        
        let response = self.client
            .post(format!("{}/services/aigc/text-generation/generation", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Qwen")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("Qwen API error: {} - {}", status, text);
        }
        
        let response: QwenResponse = response
            .json()
            .await
            .context("Failed to parse Qwen response")?;
        
        response.output
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .context("No response from Qwen")
    }
    
    fn name(&self) -> &str {
        "qwen"
    }
    
    fn model(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct QwenRequest {
    model: String,
    input: QwenInput,
    parameters: QwenParameters,
}

#[derive(Debug, Serialize)]
struct QwenInput {
    messages: Vec<QwenMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QwenMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct QwenParameters {
    #[serde(rename = "result_format")]
    result_format: String,
}

#[derive(Debug, Deserialize)]
struct QwenResponse {
    output: QwenOutput,
}

#[derive(Debug, Deserialize)]
struct QwenOutput {
    choices: Vec<QwenChoice>,
}

#[derive(Debug, Deserialize)]
struct QwenChoice {
    message: QwenMessage,
}

/// Model provider factory
pub fn create_provider(name: &str, config: &ProviderConfig) -> Result<Box<dyn ModelProvider>> {
    match config.provider_type {
        ProviderType::Kimi => Ok(Box::new(KimiProvider::new(config)?)),
        ProviderType::KimiCli => {
            // Use Kimi CLI for Kimi Code 699 membership keys
            KimiCliProvider::check_cli()?;
            Ok(Box::new(KimiCliProvider::new(
                config.api_key.clone(),
                config.timeout,
            )))
        }
        ProviderType::DeepSeek => Ok(Box::new(DeepSeekProvider::new(config)?)),
        ProviderType::Qwen => Ok(Box::new(QwenProvider::new(config)?)),
        ProviderType::OpenAiCompatible => Ok(Box::new(OpenAiCompatibleProvider::new(config)?)),
    }
}

/// Multi-provider router with seat-based model selection
pub struct ModelRouter {
    /// Provider name -> provider instance
    providers: HashMap<String, Box<dyn ModelProvider>>,
    /// Default provider name
    default_provider: String,
    /// Seat name -> provider name mapping
    seat_mappings: HashMap<String, String>,
}

impl ModelRouter {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: "default".to_string(),
            seat_mappings: HashMap::new(),
        }
    }
    
    /// Add a provider with a name
    pub fn add_provider(&mut self, name: impl Into<String>, provider: Box<dyn ModelProvider>) {
        self.providers.insert(name.into(), provider);
    }
    
    /// Set the default provider
    pub fn set_default_provider(&mut self, name: impl Into<String>) {
        self.default_provider = name.into();
    }
    
    /// Configure seat-to-model mappings
    pub fn configure_seat_mappings(&mut self, mappings: HashMap<String, String>) {
        self.seat_mappings = mappings;
    }
    
    /// Get provider for a specific seat
    fn get_provider_for_seat(&self, seat: Option<&str>) -> Result<&Box<dyn ModelProvider>> {
        // If seat is specified and has a mapping, use it
        if let Some(seat_name) = seat {
            if let Some(provider_name) = self.seat_mappings.get(seat_name) {
                if let Some(provider) = self.providers.get(provider_name) {
                    return Ok(provider);
                }
            }
        }
        
        // Fall back to default provider
        self.providers.get(&self.default_provider)
            .or_else(|| self.providers.values().next())
            .ok_or_else(|| anyhow::anyhow!("No providers configured"))
    }
    
    /// Chat with automatic provider selection based on seat
    pub async fn chat(&self, messages: Vec<Message>, seat: Option<&str>) -> Result<String> {
        let provider = self.get_provider_for_seat(seat)?;
        provider.chat(messages).await
    }
    
    /// Chat with a specific provider by name
    pub async fn chat_with_provider(&self, provider_name: &str, messages: Vec<Message>) -> Result<String> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_name))?;
        provider.chat(messages).await
    }
    
    /// List available providers
    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(|s: &String| s.as_str()).collect()
    }
    
    /// Get seat mappings
    pub fn get_seat_mappings(&self) -> &HashMap<String, String> {
        &self.seat_mappings
    }
}

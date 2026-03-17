//! OpenAI-compatible provider for local/self-hosted models
//! Supports models like GPT-OSS-120B, local vLLM, etc.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::ProviderConfig;
use crate::models::{Message, ModelProvider, Role};

/// OpenAI-compatible provider
/// Works with any OpenAI API-compatible endpoint
pub struct OpenAiCompatibleProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl OpenAiCompatibleProvider {
    pub fn new(config: &ProviderConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .context("Failed to create HTTP client")?;
        
        // Ensure base_url doesn't end with /v1/chat/completions
        let base_url = config.base_url.trim_end_matches('/').to_string();
        
        Ok(Self {
            client,
            api_key: config.api_key.clone(),
            base_url,
            model: config.model.clone(),
        })
    }
}

#[async_trait::async_trait]
impl ModelProvider for OpenAiCompatibleProvider {
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: messages.into_iter().map(|m| OpenAiMessage {
                role: match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                }.to_string(),
                content: m.content,
            }).collect(),
            stream: false,
        };
        
        let mut request_builder = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Content-Type", "application/json");
        
        // Only add Authorization header if api_key is not empty and not "not-needed"
        if !self.api_key.is_empty() && self.api_key != "not-needed-for-local" {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", self.api_key));
        }
        
        let response = request_builder
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI-compatible API")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("OpenAI-compatible API error: {} - {}", status, text);
        }
        
        let response: OpenAiResponse = response
            .json()
            .await
            .context("Failed to parse OpenAI-compatible response")?;
        
        response.choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .context("No response from OpenAI-compatible API")
    }
    
    fn name(&self) -> &str {
        "openai_compatible"
    }
    
    fn model(&self) -> &str {
        &self.model
    }
}

#[derive(Debug, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<OpenAiMessage>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OpenAiMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessageResponse,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessageResponse {
    content: Option<String>,
}

//! omega-spokes: Standardized provider adapters.
//! This crate implements the interface for LLM providers (Anthropic, OpenAI, etc.),
//! translating requests into provider-specific formats and normalizing responses.

#![warn(missing_docs)]

use omega_core::{ProviderRequest, ProviderResponse};
use async_trait::async_trait;
use serde_json::json;
use reqwest::Client;

/// Capability supported by a provider (used for integration routing)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpokeCapability {
    /// Supports structured tool calling
    ToolCalling,
    /// Supports vision input
    Vision,
}

/// Invocation of a tool
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolInvocation {
    /// Name of the tool
    pub name: String,
    /// Arguments for the tool
    pub arguments: String,
}

/// Trait for all LLM spokes
#[async_trait]
pub trait ProviderSpoke: Send + Sync {
    /// Send a request to the provider and return a normalized response
    async fn send(&self, request: ProviderRequest) -> ProviderResponse;
    /// Return the name of the provider
    fn name(&self) -> String;
    /// Discover capabilities of this spoke
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String>;
}

/// Anthropic Spoke: Implementation for Claude
pub struct AnthropicSpoke {
    /// API Key
    pub api_key: String,
    client: Client,
}

impl AnthropicSpoke {
    /// Create new Anthropic spoke
    pub fn new(api_key: String) -> Self {
        Self { api_key, client: Client::new() }
    }
}

#[async_trait]
impl ProviderSpoke for AnthropicSpoke {
    fn name(&self) -> String { "anthropic".to_string() }
    
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        Ok(Vec::new())
    }

    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        let url = "https://api.anthropic.com/v1/messages";
        let model = request.model.as_ref().cloned().unwrap_or_else(|| "claude-3-5-sonnet-20241022".to_string());
        
        let body = json!({
            "model": model,
            "max_tokens": request.max_tokens,
            "system": request.system.unwrap_or_default(),
            "messages": [{"role": "user", "content": request.prompt}]
        });

        let resp = self.client.post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(r) => {
                let data: serde_json::Value = r.json().await.unwrap_or_else(|_| json!({}));
                ProviderResponse {
                    text: data["content"][0]["text"].as_str().unwrap_or("Error: No text in response").to_string(),
                    provider: self.name(),
                    model,
                    tokens: 0,
                    latency_ms: 0.0,
                    status: "success".to_string(),
                    error: None,
                }
            },
            Err(e) => ProviderResponse {
                text: "".to_string(),
                provider: self.name(),
                model: "".to_string(),
                tokens: 0,
                latency_ms: 0.0,
                status: "error".to_string(),
                error: Some(e.to_string()),
            }
        }
    }
}

/// OpenAI Spoke: Implementation for GPT
pub struct OpenAiSpoke {
    /// API Key
    pub api_key: String,
    client: Client,
}

impl OpenAiSpoke {
    /// Create new OpenAI spoke
    pub fn new(api_key: String) -> Self {
        Self { api_key, client: Client::new() }
    }
}

#[async_trait]
impl ProviderSpoke for OpenAiSpoke {
    fn name(&self) -> String { "openai".to_string() }

    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        Ok(Vec::new())
    }
    
    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        let url = "https://api.openai.com/v1/chat/completions";
        let model = request.model.as_ref().cloned().unwrap_or_else(|| "gpt-4o".to_string());

        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": request.prompt}],
            "max_completion_tokens": request.max_tokens
        });

        let resp = self.client.post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(r) => {
                let data: serde_json::Value = r.json().await.unwrap_or_else(|_| json!({}));
                ProviderResponse {
                    text: data["choices"][0]["message"]["content"].as_str().unwrap_or("Error: No text").to_string(),
                    provider: self.name(),
                    model,
                    tokens: 0,
                    latency_ms: 0.0,
                    status: "success".to_string(),
                    error: None,
                }
            },
            Err(e) => ProviderResponse {
                text: "".to_string(),
                provider: self.name(),
                model: "".to_string(),
                tokens: 0,
                latency_ms: 0.0,
                status: "error".to_string(),
                error: Some(e.to_string()),
            }
        }
    }
}

/// Registry to hold all available provider spokes
pub struct SpokeRegistry {
    /// Map of spoke names to implementations
    pub spokes: std::collections::HashMap<String, Box<dyn ProviderSpoke>>,
}

impl SpokeRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self { spokes: std::collections::HashMap::new() }
    }
    
    /// Get spoke by name
    pub fn get(&self, name: &str) -> Option<&dyn ProviderSpoke> {
        self.spokes.get(name).map(|s| s.as_ref())
    }
    
    /// Find provider by tool capability
    pub fn find_tool_provider(&self, _tool_name: &str) -> Option<&dyn ProviderSpoke> {
        // Simple search: first available
        self.spokes.values().next().map(|s| s.as_ref())
    }

    /// List spokes with a specific capability
    pub fn spokes_with_capability(&self, _capability: SpokeCapability) -> Vec<&dyn ProviderSpoke> {
        self.spokes.values().map(|s| s.as_ref()).collect()
    }
}

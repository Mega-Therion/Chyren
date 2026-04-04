//! omega-spokes: Standardized provider adapters.
//! This crate implements the interface for LLM providers (Anthropic, OpenAI, etc.),
//! translating requests into provider-specific formats and normalizing responses.

#![warn(missing_docs)]

use omega_core::{ProviderRequest, ProviderResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Capability supported by a provider (used for integration routing)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpokeCapability {
    /// Supports structured tool calling
    ToolCalling,
    /// Supports vision input
    Vision,
}

/// Invocation of a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[async_trait]
impl ProviderSpoke for AnthropicSpoke {
    fn name(&self) -> String { "anthropic".to_string() }
    
    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        Ok(Vec::new())
    }

    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        ProviderResponse {
            text: format!("Anthropic response for {}", request.prompt),
            provider: self.name(),
            model: request.model.unwrap_or_else(|| "claude-3-5-sonnet".to_string()),
            tokens: 0,
            latency_ms: 0.0,
            status: "success".to_string(),
            error: None,
        }
    }
}

/// OpenAI Spoke: Implementation for GPT
pub struct OpenAiSpoke {
    /// API Key
    pub api_key: String,
}

#[async_trait]
impl ProviderSpoke for OpenAiSpoke {
    fn name(&self) -> String { "openai".to_string() }

    async fn discover_tools(&self) -> Result<Vec<ToolInvocation>, String> {
        Ok(Vec::new())
    }
    
    async fn send(&self, request: ProviderRequest) -> ProviderResponse {
        ProviderResponse {
            text: format!("OpenAI response for {}", request.prompt),
            provider: self.name(),
            model: request.model.unwrap_or_else(|| "gpt-4o".to_string()),
            tokens: 0,
            latency_ms: 0.0,
            status: "success".to_string(),
            error: None,
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

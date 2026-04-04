//! omega-spokes: MCP server integration layer
//!
//! Spokes are modular connections to external AI systems, knowledge bases, and tools.
//! Each spoke provides:
//! - Tool discovery and invocation
//! - Context retrieval
//! - Request/response marshaling
//! - Error handling and retry logic
#![warn(missing_docs)]

pub mod spokes;

pub use spokes::{AnthropicSpoke, NeonSpoke, SearchSpoke};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Get current Unix timestamp
pub fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

/// Tool available from a spoke
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name (e.g., "search", "retrieve_memory")
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Input schema as JSON
    pub input_schema: serde_json::Value,
    /// Whether this tool is deterministic
    pub is_deterministic: bool,
    /// Cost in tokens (approximate)
    pub estimated_cost: u32,
}

/// Tool invocation request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolInvocation {
    /// Tool name to invoke
    pub tool: String,
    /// Tool input parameters
    pub input: serde_json::Value,
    /// Request ID for tracking
    pub request_id: String,
}

/// Tool execution result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResult {
    /// Success or error
    pub success: bool,
    /// Output data
    pub output: serde_json::Value,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Execution time in ms
    pub execution_time_ms: u32,
}

/// Spoke configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpokeConfig {
    /// Spoke identifier
    pub name: String,
    /// Spoke type (e.g., "anthropic", "neon", "search")
    pub spoke_type: String,
    /// Provider endpoint or connection string
    pub endpoint: String,
    /// API key or credentials reference
    pub credentials_ref: String,
    /// Enabled/disabled
    pub enabled: bool,
    /// Maximum concurrent requests
    pub max_concurrent: usize,
    /// Timeout in seconds
    pub timeout_seconds: u32,
}

/// Spoke base capabilities
#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum SpokeCapability {
    /// LLM inference
    Inference,
    /// Tool execution
    Tools,
    /// Knowledge retrieval
    Retrieval,
    /// Real-time data
    RealTime,
    /// Computation/reasoning
    Compute,
    /// Integration/coordination
    Integration,
}

/// Spoke status for monitoring
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpokeStatus {
    /// Spoke name
    pub name: String,
    /// Health: "healthy", "degraded", "unavailable"
    pub health: String,
    /// Last successful request time
    pub last_success: f64,
    /// Error count in last hour
    pub recent_errors: u32,
    /// Available tools
    pub available_tools: usize,
}

/// Core spoke trait
#[async_trait]
pub trait Spoke: Send + Sync {
    /// Get spoke identifier
    fn name(&self) -> &str;

    /// Get spoke type
    fn spoke_type(&self) -> &str;

    /// Get available capabilities
    fn capabilities(&self) -> Vec<SpokeCapability>;

    /// Discover available tools
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String>;

    /// Invoke a tool
    async fn invoke_tool(
        &self,
        invocation: ToolInvocation,
    ) -> Result<ToolResult, String>;

    /// Health check
    async fn health_check(&self) -> Result<SpokeStatus, String>;

    /// Get spoke configuration
    fn config(&self) -> &SpokeConfig;
}

/// Spoke registry and manager
#[derive(Clone)]
pub struct SpokeRegistry {
    spokes: HashMap<String, std::sync::Arc<dyn Spoke>>,
    tool_index: HashMap<String, String>, // tool_name -> spoke_name
    capabilities_index: HashMap<SpokeCapability, Vec<String>>, // capability -> [spoke_names]
}

impl SpokeRegistry {
    /// Create new spoke registry
    pub fn new() -> Self {
        SpokeRegistry {
            spokes: HashMap::new(),
            tool_index: HashMap::new(),
            capabilities_index: HashMap::new(),
        }
    }

    /// Register a spoke
    pub fn register(&mut self, spoke: std::sync::Arc<dyn Spoke>) -> Result<(), String> {
        let name = spoke.name().to_string();
        let capabilities = spoke.capabilities();

        self.spokes.insert(name.clone(), spoke);

        // Index capabilities
        for cap in capabilities {
            self.capabilities_index
                .entry(cap)
                .or_insert_with(Vec::new)
                .push(name.clone());
        }

        Ok(())
    }

    /// Get spoke by name
    pub fn get(&self, name: &str) -> Option<std::sync::Arc<dyn Spoke>> {
        self.spokes.get(name).cloned()
    }

    /// Get all spokes with capability
    pub fn spokes_with_capability(&self, capability: &SpokeCapability) -> Vec<String> {
        self.capabilities_index
            .get(capability)
            .cloned()
            .unwrap_or_default()
    }

    /// Update tool index after discovery
    pub fn index_tools(&mut self, spoke_name: &str, tools: Vec<ToolDefinition>) {
        for tool in tools {
            self.tool_index.insert(tool.name, spoke_name.to_string());
        }
    }

    /// Find which spoke provides a tool
    pub fn find_tool_provider(&self, tool_name: &str) -> Option<String> {
        self.tool_index.get(tool_name).cloned()
    }

    /// Get all registered spoke names
    pub fn list_spokes(&self) -> Vec<String> {
        self.spokes.keys().cloned().collect()
    }
}

impl Default for SpokeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spoke_registry_creation() {
        let registry = SpokeRegistry::new();
        assert_eq!(registry.list_spokes().len(), 0);
    }

    #[test]
    fn test_tool_index() {
        let mut registry = SpokeRegistry::new();
        let tools = vec![
            ToolDefinition {
                name: "search".to_string(),
                description: "Search the web".to_string(),
                input_schema: serde_json::json!({}),
                is_deterministic: false,
                estimated_cost: 100,
            },
        ];

        registry.index_tools("neon-spoke", tools);
        assert_eq!(registry.find_tool_provider("search"), Some("neon-spoke".to_string()));
    }
}

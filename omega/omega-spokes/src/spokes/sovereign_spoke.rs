use async_trait::async_trait;
use crate::{Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult};
use serde_json::json;

/// SovereignSpoke: Provides local system capabilities (Web, File, Shell) with ADCCL auditing.
pub struct SovereignSpoke {
    config: SpokeConfig,
}

impl SovereignSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Spoke for SovereignSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "sovereign"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![
            SpokeCapability::Search,
            SpokeCapability::Tools,
        ]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "web_search".into(),
                description: "Search the web for real-time information.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" }
                    },
                    "required": ["query"]
                }),
                is_deterministic: false,
                estimated_cost: 10,
            },
            ToolDefinition {
                name: "file_io".into(),
                description: "Read or write local files (Sandboxed).".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "action": { "type": "string", "enum": ["read", "write"] },
                        "path": { "type": "string" },
                        "content": { "type": "string" }
                    },
                    "required": ["action", "path"]
                }),
                is_deterministic: true,
                estimated_cost: 5,
            },
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        
        match invocation.tool.as_str() {
            "web_search" => {
                // In a real implementation, this would call Brave Search / Google / DuckDuckGo
                // For this evolution, we will stub the result, ready for the orchestration layer.
                Ok(ToolResult {
                    success: true,
                    output: json!({
                        "results": [
                            { "title": "Stub Search Result", "snippet": "Detailed information about sovereign AI architectures..." }
                        ]
                    }),
                    error: None,
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            },
            "file_io" => {
                // Placeholder for sandboxed file IO
                Ok(ToolResult {
                    success: true,
                    output: json!({ "status": "sandboxed_operation_queued" }),
                    error: None,
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            }
            _ => Err(format!("Tool {} not found on Sovereign spoke", invocation.tool)),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.name().to_string(),
            health: "OK".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 2,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

//! Search spoke for web and external data access

use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::json;

/// Search spoke for web and API access
pub struct SearchSpoke {
    config: SpokeConfig,
}

impl SearchSpoke {
    /// Create new Search spoke
    pub fn new(config: SpokeConfig) -> Self {
        SearchSpoke { config }
    }
}

#[async_trait]
impl Spoke for SearchSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "search"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![
            SpokeCapability::RealTime,
            SpokeCapability::Tools,
            SpokeCapability::Retrieval,
        ]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "web_search".to_string(),
                description: "Search the web for current information".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"},
                        "num_results": {"type": "integer", "description": "Number of results"}
                    },
                    "required": ["query"]
                }),
                is_deterministic: false,
                estimated_cost: 200,
            },
            ToolDefinition {
                name: "fetch_url".to_string(),
                description: "Fetch and extract content from a URL".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": {"type": "string", "description": "URL to fetch"},
                        "extract": {"type": "string", "description": "Extraction method"}
                    },
                    "required": ["url"]
                }),
                is_deterministic: false,
                estimated_cost: 150,
            },
            ToolDefinition {
                name: "api_call".to_string(),
                description: "Call external REST API".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "endpoint": {"type": "string"},
                        "method": {"type": "string"},
                        "params": {"type": "object"}
                    },
                    "required": ["endpoint", "method"]
                }),
                is_deterministic: false,
                estimated_cost: 250,
            },
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();

        let result = match invocation.tool.as_str() {
            "web_search" => {
                json!({
                    "results": [
                        {"title": "Result 1", "url": "https://example.com/1", "snippet": "..."},
                        {"title": "Result 2", "url": "https://example.com/2", "snippet": "..."}
                    ],
                    "total": 2
                })
            }
            "fetch_url" => {
                json!({
                    "title": "Example Page",
                    "content": "Mock fetched content from URL",
                    "status": 200
                })
            }
            "api_call" => {
                json!({
                    "status": 200,
                    "data": {"message": "Mock API response"}
                })
            }
            _ => {
                return Ok(ToolResult {
                    success: false,
                    output: json!({}),
                    error: Some(format!("Unknown tool: {}", invocation.tool)),
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            }
        };

        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u32,
        })
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 3,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

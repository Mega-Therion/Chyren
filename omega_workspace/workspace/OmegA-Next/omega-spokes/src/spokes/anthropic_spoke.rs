use tokio::sync::mpsc;
/// Anthropic API spoke for Claude inference

use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

/// Anthropic spoke for Claude model access
pub struct AnthropicSpoke {
    config: SpokeConfig,
}

impl AnthropicSpoke {
    /// Create new Anthropic spoke
    pub fn new(config: SpokeConfig) -> Self {
        AnthropicSpoke { config }
    }
}

#[async_trait]
impl Spoke for AnthropicSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "anthropic"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![
            SpokeCapability::Inference,
            SpokeCapability::Tools,
        ]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        // Anthropic provides tool_use capability
        Ok(vec![
            ToolDefinition {
                name: "chat_completion".to_string(),
                description: "Call Claude LLM for inference or reasoning".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model": {"type": "string", "description": "Claude model ID"},
                        "prompt": {"type": "string", "description": "User prompt"},
                        "max_tokens": {"type": "integer", "description": "Max response tokens"}
                    },
                    "required": ["prompt"]
                }),
                is_deterministic: false,
                estimated_cost: 1000,
            },
            ToolDefinition {
                name: "batch_process".to_string(),
                description: "Process multiple requests in batch mode".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "requests": {"type": "array", "description": "Batch requests"}
                    }
                }),
                is_deterministic: false,
                estimated_cost: 5000,
            },
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();

        let result = match invocation.tool.as_str() {
            "chat_completion" => {
                match self.chat_completion(&invocation.input).await {
                    Ok(response) => response,
                    Err(e) => {
                        return Ok(ToolResult {
                            success: false,
                            output: json!({}),
                            error: Some(e),
                            execution_time_ms: start.elapsed().as_millis() as u32,
                        })
                    }
                }
            }
            "batch_process" => {
                json!({
                    "batch_id": "batch-123456",
                    "status": "submitted",
                    "request_count": 10
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

    async fn invoke_tool_stream(&self, _invocation: ToolInvocation, _tx: mpsc::Sender<Value>) -> Result<(), String> {
         Err("Streaming not yet implemented for AnthropicSpoke".to_string())
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        // In real implementation, would ping Anthropic API
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 2,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

impl AnthropicSpoke {
    /// Invoke Claude model via Anthropic API
    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "ANTHROPIC_API_KEY environment variable not set".to_string())?;

        let prompt = input.get("prompt")
            .and_then(|p| p.as_str())
            .ok_or("Missing 'prompt' in input")?;

        let max_tokens = input.get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(1024) as i32;

        let model = input.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("claude-opus-4-6");

        // Make HTTP request to Anthropic API
        let client = reqwest::Client::new();
        let messages = input.get("messages")
            .and_then(|m| m.as_array())
            .map(|arr| {
                let start = arr.len().saturating_sub(15);
                arr[start..].to_vec()
            })
            .unwrap_or_else(|| vec![json!({"role": "user", "content": prompt})]);

        // Make HTTP request to Anthropic API
        let client = reqwest::Client::new();
        let request_body = json!({
            "model": model,
            "max_tokens": max_tokens,
            "messages": messages
        });

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("API error: {}", error_text));
        }

        let api_response: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(api_response)
    }
}

//! DeepSeek API spoke for chat model inference
//!
//! DeepSeek uses an OpenAI-compatible API structure but requires its own
//! endpoint and authentication key.

use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

/// DeepSeek spoke for chat model access
pub struct DeepSeekSpoke {
    config: SpokeConfig,
}

impl DeepSeekSpoke {
    /// Create a new DeepSeek spoke instance
    pub fn new(config: SpokeConfig) -> Self {
        DeepSeekSpoke { config }
    }
}

#[async_trait]
impl Spoke for DeepSeekSpoke {
    fn name(&self) -> &str { &self.config.name }
    fn spoke_type(&self) -> &str { "deepseek" }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference, SpokeCapability::Tools]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "chat_completion".to_string(),
                description: "Call DeepSeek for chat completion".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model": {"type": "string", "default": "deepseek-chat"},
                        "prompt": {"type": "string"},
                        "system": {"type": "string"},
                        "max_tokens": {"type": "integer"}
                    },
                    "required": ["prompt"]
                }),
                is_deterministic: false,
                estimated_cost: 10,
            }
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        let result = match invocation.tool.as_str() {
            "chat_completion" => self.chat_completion(&invocation.input).await?,
            _ => return Err(format!("Unknown tool: {}", invocation.tool)),
        };
        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u32,
        })
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        // Basic check for API key presence
        let key_exists = env::var("DEEPSEEK_API_KEY").is_ok();
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: if key_exists { "healthy".to_string() } else { "unauthenticated".to_string() },
            last_success: 0,
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig { &self.config }
}

impl DeepSeekSpoke {
    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| "DEEPSEEK_API_KEY not set".to_string())?;

        let prompt = input.get("prompt").and_then(|p| p.as_str()).ok_or("Missing prompt")?;
        let system = input.get("system").and_then(|s| s.as_str());
        let model = input.get("model").and_then(|m| m.as_str()).unwrap_or("deepseek-chat");
        let max_tokens = input.get("max_tokens").and_then(|t| t.as_u64()).unwrap_or(2048);

        let mut messages = Vec::new();
        if let Some(sys) = system {
            messages.push(json!({"role": "system", "content": sys}));
        }
        messages.push(json!({"role": "user", "content": prompt}));

        let client = reqwest::Client::new();
        let body = json!({
            "model": model,
            "messages": messages,
            "max_tokens": max_tokens,
            "stream": false
        });

        let resp = client.post("https://api.deepseek.com/v1/chat/completions")
            .bearer_auth(&api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("DeepSeek connection error: {}", e))?;

        if !resp.status().is_success() {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("DeepSeek API Error ({}): {}", resp.status(), error_text));
        }

        resp.json().await.map_err(|e| format!("DeepSeek JSON parse error: {}", e))
    }
}

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

const DEFAULT_MODEL: &str = "deepseek-chat";
const API_URL: &str = "https://api.deepseek.com/v1/chat/completions";

pub struct DeepSeekSpoke {
    config: SpokeConfig,
}

impl DeepSeekSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        DeepSeekSpoke { config }
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key =
            env::var("DEEPSEEK_API_KEY").map_err(|_| "DEEPSEEK_API_KEY not set".to_string())?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();
        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let system = input.get("system").and_then(|s| s.as_str()).unwrap_or("");
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or(DEFAULT_MODEL);
        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048);
        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.3);

        let mut messages = vec![];
        if !system.is_empty() {
            messages.push(json!({"role": "system", "content": system}));
        }
        messages.push(json!({"role": "user", "content": prompt}));

        let resp = client
            .post(API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": model,
                "messages": messages,
                "max_tokens": max_tokens,
                "temperature": temperature,
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err = resp.text().await.unwrap_or_default();
            return Err(format!(
                "DeepSeek HTTP {}: {}",
                status,
                &err[..err.len().min(300)]
            ));
        }

        resp.json().await.map_err(|e| e.to_string())
    }
}

#[async_trait]
impl Spoke for DeepSeekSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }
    fn spoke_type(&self) -> &str {
        "deepseek"
    }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "DeepSeek chat completion (deepseek-chat)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "prompt": {"type": "string"},
                    "system": {"type": "string"},
                    "model": {"type": "string"},
                    "max_tokens": {"type": "integer"},
                    "temperature": {"type": "number"}
                },
                "required": ["prompt"]
            }),
            is_deterministic: false,
            estimated_cost: 500,
        }])
    }
    async fn invoke_tool(&self, inv: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        match inv.tool.as_str() {
            "chat_completion" => match self.chat_completion(&inv.input).await {
                Ok(output) => Ok(ToolResult {
                    success: true,
                    output,
                    error: None,
                    execution_time_ms: start.elapsed().as_millis() as u32,
                }),
                Err(e) => Ok(ToolResult {
                    success: false,
                    output: json!({}),
                    error: Some(e),
                    execution_time_ms: start.elapsed().as_millis() as u32,
                }),
            },
            _ => Ok(ToolResult {
                success: false,
                output: json!({}),
                error: Some(format!("Unknown tool: {}", inv.tool)),
                execution_time_ms: start.elapsed().as_millis() as u32,
            }),
        }
    }
    async fn invoke_tool_stream(
        &self,
        _inv: ToolInvocation,
        _tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        Err("DeepSeek streaming not yet implemented".into())
    }
    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".into(),
            last_success: 0.0,
            recent_errors: 0,
            available_tools: 1,
        })
    }
    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

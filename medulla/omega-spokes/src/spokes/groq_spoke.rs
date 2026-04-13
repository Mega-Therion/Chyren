//! Groq inference spoke — uses the OpenAI-compatible API.
//!
//! Default model: `llama-3.3-70b-versatile` (override via `GROQ_MODEL` env var).
//! Endpoint: `https://api.groq.com/openai/v1/chat/completions`

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

const GROQ_ENDPOINT: &str = "https://api.groq.com/openai/v1/chat/completions";
const DEFAULT_MODEL: &str = "llama-3.3-70b-versatile";

pub struct GroqSpoke {
    config: SpokeConfig,
}

impl GroqSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        GroqSpoke { config }
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("GROQ_API_KEY")
            .map_err(|_| "GROQ_API_KEY environment variable not set".to_string())?;

        let model = env::var("GROQ_MODEL")
            .unwrap_or_else(|_| DEFAULT_MODEL.to_string());

        let prompt = input
            .get("prompt")
            .and_then(|p| p.as_str())
            .ok_or("Missing 'prompt' in input")?;

        let system = input
            .get("system")
            .and_then(|s| s.as_str())
            .unwrap_or("");

        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048);

        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.3);

        let mut messages: Vec<Value> = Vec::new();
        if !system.is_empty() {
            messages.push(json!({ "role": "system", "content": system }));
        }
        messages.push(json!({ "role": "user", "content": prompt }));

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .map_err(|e| e.to_string())?;

        let resp = client
            .post(GROQ_ENDPOINT)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": model,
                "messages": messages,
                "max_tokens": max_tokens,
                "temperature": temperature,
            }))
            .send()
            .await
            .map_err(|e| format!("Groq request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Groq HTTP {status}: {body}"));
        }

        resp.json::<Value>().await.map_err(|e| format!("Groq parse error: {e}"))
    }
}

#[async_trait]
impl Spoke for GroqSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "groq"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "Groq LPU inference — fast open-weights models".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "prompt":      { "type": "string" },
                    "system":      { "type": "string" },
                    "max_tokens":  { "type": "integer" },
                    "temperature": { "type": "number" }
                },
                "required": ["prompt"]
            }),
            is_deterministic: false,
            estimated_cost: 50,
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
        Err("Streaming not yet implemented for GroqSpoke".to_string())
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".into(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

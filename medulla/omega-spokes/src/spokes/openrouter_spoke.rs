//! OpenRouter spoke — OpenAI-compatible cloud routing gateway.
//!
//! Environment variables:
//!   OPENROUTER_BASE_URL  — API base (default: https://openrouter.ai/api/v1)
//!   OPENROUTER_API_KEY   — Bearer token (required for cloud calls)
//!   OPENROUTER_DEFAULT_MODEL — model slug (default: anthropic/claude-3.5-sonnet)

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

const DEFAULT_BASE_URL: &str = "https://openrouter.ai/api/v1";
const DEFAULT_MODEL: &str = "anthropic/claude-3.5-sonnet";

pub struct OpenRouterSpoke {
    config: SpokeConfig,
}

impl OpenRouterSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }

    fn base_url(&self) -> String {
        self.config
            .endpoint
            .clone()
            .unwrap_or_else(|| {
                env::var("OPENROUTER_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
            })
            .trim_end_matches('/')
            .to_string()
    }

    fn model(&self) -> String {
        env::var("OPENROUTER_DEFAULT_MODEL")
            .or_else(|_| env::var("OPENROUTER_MODEL"))
            .unwrap_or_else(|_| DEFAULT_MODEL.to_string())
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key =
            env::var("OPENROUTER_API_KEY").map_err(|_| "OPENROUTER_API_KEY not set".to_string())?;

        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let system = input.get("system").and_then(|s| s.as_str()).unwrap_or("");
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.model());
        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048);
        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.3);

        let mut messages = Vec::new();
        if !system.is_empty() {
            messages.push(json!({"role": "system", "content": system}));
        }
        messages.push(json!({"role": "user", "content": prompt}));

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .unwrap_or_default();
        let resp = client
            .post(format!("{}/chat/completions", self.base_url()))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("HTTP-Referer", "https://chyren.ai")
            .header("X-Title", "Chyren Sovereign Intelligence")
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
            let body = resp.text().await.unwrap_or_else(|_| "unknown".to_string());
            return Err(format!("OpenRouter error ({}): {}", status, body));
        }

        resp.json().await.map_err(|e| e.to_string())
    }
}

#[async_trait]
impl Spoke for OpenRouterSpoke {
    fn name(&self) -> &str {
        "openrouter"
    }

    fn spoke_type(&self) -> &str {
        "cloud_router"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "OpenRouter cloud inference (high-complexity tasks)".to_string(),
            input_schema: json!({"type": "object", "properties": {"prompt": {"type": "string"}}}),
            is_deterministic: false,
            estimated_cost: 10,
        }])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        match invocation.tool.as_str() {
            "chat_completion" => {
                let result = self.chat_completion(&invocation.input).await?;
                Ok(ToolResult {
                    success: true,
                    output: result,
                    error: None,
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            }
            other => Err(format!("OpenRouterSpoke: unknown tool '{}'", other)),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        let has_key = env::var("OPENROUTER_API_KEY").is_ok();
        Ok(SpokeStatus {
            name: "openrouter".to_string(),
            health: if has_key {
                "ok".to_string()
            } else {
                "no_key".to_string()
            },
            last_success: 0.0,
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }

    async fn invoke_tool_stream(
        &self,
        _invocation: ToolInvocation,
        _tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        Err("OpenRouterSpoke: streaming not yet implemented".to_string())
    }
}

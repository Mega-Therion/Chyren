//! Universal Adapter Spoke -- Configurable bridge for any custom API or SDK.
//!
//! Environment variables:
//!   UNIVERSAL_ENDPOINT     -- The full URL for chat completions (e.g. http://localhost:8000/v1/chat/completions)
//!   UNIVERSAL_AUTH_HEADER   -- The header name for authentication (default: Authorization)
//!   UNIVERSAL_AUTH_VALUE    -- The API key or token
//!   UNIVERSAL_MODEL         -- The model name to pass in the request
//!   UNIVERSAL_PROVIDER_NAME -- Display name for this spoke (default: universal)

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

pub struct UniversalSpoke {
    config: SpokeConfig,
}

impl UniversalSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }

    fn endpoint(&self) -> String {
        self.config
            .endpoint
            .clone()
            .or_else(|| env::var("UNIVERSAL_ENDPOINT").ok())
            .unwrap_or_else(|| "http://127.0.0.1:8888/v1/chat/completions".to_string())
    }

    fn auth_header(&self) -> String {
        env::var("UNIVERSAL_AUTH_HEADER").unwrap_or_else(|_| "Authorization".to_string())
    }

    fn auth_value(&self) -> String {
        env::var("UNIVERSAL_AUTH_VALUE").unwrap_or_default()
    }

    fn model(&self) -> String {
        env::var("UNIVERSAL_MODEL").unwrap_or_else(|_| "custom-model".to_string())
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        let url = self.endpoint();
        chyren_telemetry::info!("UniversalSpoke", "API_CALL", "Calling Universal endpoint: {}", url);
        let auth_header = self.auth_header();
        let auth_value = self.auth_value();
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.model());

        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let system = input.get("system").and_then(|s| s.as_str()).unwrap_or("");

        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(4096);
        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.7);

        // Standard OpenAI-style payload as the default 'Universal' format.
        // This is compatible with OpenLLM, vLLM, Ollama, and most custom wrappers.
        let body = json!({
            "model": model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt}
            ],
            "temperature": temperature,
            "max_tokens": max_tokens,
            "stream": false,
        });

        let mut request = client
            .post(url)
            .header("Content-Type", "application/json");

        if !auth_value.is_empty() {
            request = request.header(auth_header, auth_value);
        }

        let resp = request
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Universal API request failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!(
                "Universal API HTTP {} — {}",
                status,
                &err_body[..err_body.len().min(500)]
            ));
        }

        resp.json::<Value>()
            .await
            .map_err(|e| format!("Universal API JSON parse error: {}", e))
    }
}

#[async_trait]
impl Spoke for UniversalSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "universal"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: format!(
                "Universal Adapter chat completion (endpoint: {}).",
                self.endpoint()
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "model": { "type": "string" },
                    "prompt": { "type": "string" },
                    "system": { "type": "string" },
                    "max_tokens": { "type": "integer" },
                    "temperature": { "type": "number" }
                },
                "required": ["prompt"]
            }),
            is_deterministic: false,
            estimated_cost: 0,
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

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.name().to_string(),
            health: "OK".to_string(),
            last_success: 0.0,
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

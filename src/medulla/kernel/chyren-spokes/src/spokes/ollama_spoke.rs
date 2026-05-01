//! Ollama spoke -- OpenAI-compatible local inference.
//!
//! Environment variables:
//!   OLLAMA_BASE_URL  -- base URL for the Ollama server (default: http://localhost:11434/v1)
//!   OLLAMA_MODEL     -- model to use (default: llama3.2:3b)

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

const DEFAULT_BASE_URL: &str = "http://localhost:11434/v1";
const DEFAULT_MODEL: &str = "llama3.2:3b";

pub struct OllamaSpoke {
    config: SpokeConfig,
}

impl OllamaSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }

    fn base_url(&self) -> String {
        self.config
            .endpoint
            .clone()
            .unwrap_or_else(|| {
                env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
            })
            .trim_end_matches('/')
            .to_string()
    }

    fn model(&self) -> String {
        env::var("OLLAMA_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string())
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;
        let base = self.base_url();
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.model());

        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let system = input.get("system").and_then(|s| s.as_str()).unwrap_or("");

        // Gemma4 via Ollama returns empty content when given a `system` role message.
        // Inline the system content into the user turn.
        let user_content = if system.is_empty() {
            prompt.to_string()
        } else {
            format!("{}\n\n{}", system, prompt)
        };

        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(4096);
        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.7);

        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": user_content}],
            "temperature": temperature,
            "max_tokens": max_tokens,
            "stream": false,
        });

        let resp = client
            .post(format!("{}/chat/completions", base))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!(
                "Ollama HTTP {} — {}",
                status,
                &err_body[..err_body.len().min(300)]
            ));
        }

        resp.json::<Value>()
            .await
            .map_err(|e| format!("Ollama JSON parse error: {}", e))
    }

    /// Return `true` if the Ollama server is reachable and the configured model is loaded.
    async fn is_available(&self) -> bool {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .unwrap_or_default();
        let base = self.base_url();
        let model = self.model();

        match client.get(format!("{}/models", base)).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(data) = resp.json::<Value>().await {
                    data["data"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .any(|m| m.get("id").and_then(|id| id.as_str()) == Some(&model))
                        })
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[async_trait]
impl Spoke for OllamaSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "ollama"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: format!(
                "Local Ollama chat completion (model: {}). No network required.",
                self.model()
            ),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "model": {
                        "type": "string",
                        "description": "Ollama model override (default: gemma4:e2b)"
                    },
                    "prompt": {
                        "type": "string",
                        "description": "User prompt"
                    },
                    "system": {
                        "type": "string",
                        "description": "System context (prepended to user prompt for Gemma4 compatibility)"
                    },
                    "max_tokens": {
                        "type": "integer",
                        "description": "Maximum tokens to generate (default: 4096)"
                    },
                    "temperature": {
                        "type": "number",
                        "description": "Sampling temperature (default: 0.7)"
                    }
                },
                "required": ["prompt"]
            }),
            is_deterministic: false,
            estimated_cost: 0, // Local inference — no API cost.
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
        Err("Ollama streaming not yet implemented".into())
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        let available = self.is_available().await;
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: if available { "healthy" } else { "unavailable" }.to_string(),
            last_success: 0.0,
            recent_errors: 0,
            available_tools: if available { 1 } else { 0 },
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ollama_spoke() -> OllamaSpoke {
        OllamaSpoke::new(SpokeConfig {
            name: "ollama".to_string(),
            endpoint: None,
            priority: 50,
        })
    }

    #[test]
    fn spoke_name_and_type() {
        let spoke = ollama_spoke();
        assert_eq!(spoke.name(), "ollama");
        assert_eq!(spoke.spoke_type(), "ollama");
    }

    #[test]
    fn capabilities_include_inference() {
        let spoke = ollama_spoke();
        assert!(spoke.capabilities().contains(&SpokeCapability::Inference));
    }

    #[test]
    fn endpoint_override_via_config() {
        let spoke = OllamaSpoke::new(SpokeConfig {
            name: "ollama-custom".to_string(),
            endpoint: Some("http://my-ollama:11434/v1".to_string()),
            priority: 50,
        });
        assert_eq!(spoke.base_url(), "http://my-ollama:11434/v1");
    }

    #[test]
    fn default_model_is_set() {
        let spoke = ollama_spoke();
        assert!(!spoke.model().is_empty());
    }

    #[tokio::test]
    async fn discover_tools_returns_chat_completion() {
        let spoke = ollama_spoke();
        let tools = spoke.discover_tools().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "chat_completion");
        assert_eq!(tools[0].estimated_cost, 0); // local = free
    }

    #[tokio::test]
    async fn unknown_tool_returns_error_result() {
        let spoke = ollama_spoke();
        let result = spoke
            .invoke_tool(ToolInvocation {
                tool: "nonexistent_tool".to_string(),
                input: json!({}),
            })
            .await
            .unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }
}

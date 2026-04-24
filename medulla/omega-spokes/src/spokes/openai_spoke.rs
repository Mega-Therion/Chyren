use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use futures::StreamExt;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

pub struct OpenAISpoke {
    config: SpokeConfig,
}

impl OpenAISpoke {
    pub fn new(config: SpokeConfig) -> Self {
        OpenAISpoke { config }
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key =
            env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set".to_string())?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| e.to_string())?;
        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gpt-4");

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": model,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI Error ({}): {}", status, err_body));
        }

        resp.json().await.map_err(|e| e.to_string())
    }

    async fn chat_completion_stream(
        &self,
        input: &Value,
        tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        let api_key =
            env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set".to_string())?;
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| e.to_string())?;
        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        let system = input
            .get("system")
            .and_then(|s| s.as_str())
            .unwrap_or("You are a helpful assistant.");
        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gpt-4")
            .to_string();
        let max_tokens = input
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(1024);
        let temperature = input
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.7);

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": model,
                "messages": [
                    {"role": "system", "content": system},
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": max_tokens,
                "temperature": temperature,
                "stream": true
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI Stream Error ({}): {}", status, err_body));
        }

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&bytes);
            buffer.push_str(&text);

            while let Some(newline_pos) = buffer.find('\n') {
                let line = buffer[..newline_pos].trim().to_string();
                buffer = buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        break;
                    }

                    if let Ok(chunk_json) = serde_json::from_str::<Value>(data) {
                        if let Some(delta_content) = chunk_json
                            .get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|o| o.get("delta"))
                            .and_then(|d| d.get("content"))
                            .and_then(|ct| ct.as_str())
                        {
                            if !delta_content.is_empty() {
                                let frame = json!({
                                    "choices": [{"delta": {"content": delta_content}}]
                                });
                                if tx.send(frame).await.is_err() {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        let _ = tx.send(json!({"status": "done"})).await;

        Ok(())
    }
}

#[async_trait]
impl Spoke for OpenAISpoke {
    fn name(&self) -> &str {
        &self.config.name
    }
    fn spoke_type(&self) -> &str {
        "openai"
    }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "Call OpenAI chat completions API for inference".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "model": {"type": "string", "description": "Model ID (default: gpt-4)"},
                    "prompt": {"type": "string", "description": "User prompt"}
                },
                "required": ["prompt"]
            }),
            is_deterministic: false,
            estimated_cost: 1000,
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
        inv: ToolInvocation,
        tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        match inv.tool.as_str() {
            "chat_completion" => self.chat_completion_stream(&inv.input, tx).await,
            _ => Err(format!("Unknown tool for streaming: {}", inv.tool)),
        }
    }
    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".into(),
            last_success: 0.0,
            recent_errors: 0,
            available_tools: 0,
        })
    }
    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

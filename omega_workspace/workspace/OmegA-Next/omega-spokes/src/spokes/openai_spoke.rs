//! OpenAI API spoke for GPT model inference

use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;
use futures::StreamExt;

/// OpenAI spoke for GPT model access
pub struct OpenAISpoke {
    config: SpokeConfig,
}

impl OpenAISpoke {
    pub fn new(config: SpokeConfig) -> Self {
        OpenAISpoke { config }
    }
}

#[async_trait]
impl Spoke for OpenAISpoke {
    fn name(&self) -> &str { &self.config.name }
    fn spoke_type(&self) -> &str { "openai" }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference, SpokeCapability::Tools, SpokeCapability::Embeddings]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "chat_completion".to_string(),
                description: "Call OpenAI GPT for chat completion".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "model": {"type": "string"},
                        "prompt": {"type": "string"},
                        "max_tokens": {"type": "integer"}
                    },
                    "required": ["prompt"]
                }),
                is_deterministic: false,
                estimated_cost: 1000,
            },
            ToolDefinition {
                name: "create_embedding".to_string(),
                description: "Generate a semantic vector embedding for a text string".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "text": {"type": "string"},
                        "model": {"type": "string", "default": "text-embedding-3-small"}
                    },
                    "required": ["text"]
                }),
                is_deterministic: true,
                estimated_cost: 10,
            }
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        let result = match invocation.tool.as_str() {
            "chat_completion" => self.chat_completion(&invocation.input).await?,
            "create_embedding" => self.create_embedding(&invocation.input).await?,
            _ => return Err(format!("Unknown tool: {}", invocation.tool)),
        };
        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u32,
        })
    }

    async fn invoke_tool_stream(&self, invocation: ToolInvocation, tx: mpsc::Sender<Value>) -> Result<(), String> {
        match invocation.tool.as_str() {
            "chat_completion" => self.chat_completion_stream(&invocation.input, tx).await?,
            _ => return Err(format!("Streaming not supported for tool: {}", invocation.tool)),
        }
        Ok(())
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 2,
        })
    }

    fn config(&self) -> &SpokeConfig { &self.config }
}

impl OpenAISpoke {
    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set".to_string())?;

        let prompt = input.get("prompt").and_then(|p| p.as_str()).ok_or("Missing prompt")?;
        let model = input.get("model").and_then(|m| m.as_str()).unwrap_or("gpt-4o");

        let client = reqwest::Client::new();
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 1024
        });

        let resp = client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("OpenAI Error: {}", resp.status()));
        }

        resp.json().await.map_err(|e| e.to_string())
    }

    async fn chat_completion_stream(&self, input: &Value, tx: mpsc::Sender<Value>) -> Result<(), String> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set".to_string())?;

        let prompt = input.get("prompt").and_then(|p| p.as_str()).ok_or("Missing prompt")?;
        let model = input.get("model").and_then(|m| m.as_str()).unwrap_or("gpt-4o");

        let client = reqwest::Client::new();
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": true
        });

        let mut resp = client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("OpenAI Streaming Error: {}", resp.status()));
        }

        let mut stream = resp.bytes_stream();
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| e.to_string())?;
            let text = String::from_utf8_lossy(&chunk);
            
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line == "data: [DONE]" {
                    continue;
                }
                
                if let Some(json_str) = line.strip_prefix("data: ") {
                    if let Ok(val) = serde_json::from_str::<Value>(json_str) {
                         if tx.send(val).await.is_err() {
                             return Ok(()); // channel closed
                         }
                    }
                }
            }
        }

        Ok(())
    }

    async fn create_embedding(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY not set".to_string())?;

        let text = input.get("text").and_then(|t| t.as_str()).ok_or("Missing text")?;
        let model = input.get("model").and_then(|m| m.as_str()).unwrap_or("text-embedding-3-small");

        let client = reqwest::Client::new();
        let body = json!({
            "model": model,
            "input": text
        });

        let resp = client.post("https://api.openai.com/v1/embeddings")
            .bearer_auth(&api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let err_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI Embedding Error ({}): {}", resp.status(), err_body));
        }

        resp.json().await.map_err(|e| e.to_string())
    }
}

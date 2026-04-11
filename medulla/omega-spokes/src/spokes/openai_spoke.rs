use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;
use reqwest::Response;

pub struct OpenAISpoke {
    config: SpokeConfig,
}

impl OpenAISpoke {
    pub fn new(config: SpokeConfig) -> Self {
        OpenAISpoke { config }
    }

    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set".to_string())?;
        let client = reqwest::Client::new();
        let prompt = input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
        
        let resp = client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "gpt-4",
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("OpenAI Error ({}): {}", status, err_body));
        }
        
        resp.json().await.map_err(|e| e.to_string())
    }
}

#[async_trait]
impl Spoke for OpenAISpoke {
    fn name(&self) -> &str { &self.config.name }
    fn spoke_type(&self) -> &str { "openai" }
    fn capabilities(&self) -> Vec<SpokeCapability> { vec![SpokeCapability::Inference] }
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> { Ok(vec![]) }
    async fn invoke_tool(&self, _inv: ToolInvocation) -> Result<ToolResult, String> { 
        Ok(ToolResult { success: false, output: json!({}), error: Some("Not impl".into()), execution_time_ms: 0 }) 
    }
    async fn invoke_tool_stream(&self, _inv: ToolInvocation, _tx: mpsc::Sender<Value>) -> Result<(), String> { Err("Not impl".into()) }
    async fn health_check(&self) -> Result<SpokeStatus, String> { 
        Ok(SpokeStatus { name: self.config.name.clone(), health: "healthy".into(), last_success: 0.0, recent_errors: 0, available_tools: 0 }) 
    }
    fn config(&self) -> &SpokeConfig { &self.config }
}

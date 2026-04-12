//! Gemini API spoke for Google model inference

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct GeminiSpoke {
    config: SpokeConfig,
}

impl GeminiSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        GeminiSpoke { config }
    }
}

#[async_trait]
impl Spoke for GeminiSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }
    fn spoke_type(&self) -> &str {
        "gemini"
    }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "Call Google Gemini for content generation".to_string(),
            input_schema: json!({}),
            is_deterministic: false,
            estimated_cost: 500,
        }])
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
        Ok(SpokeStatus {
            name: self.config.name.clone(),
            health: "healthy".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

impl GeminiSpoke {
    async fn chat_completion(&self, input: &Value) -> Result<Value, String> {
        let api_key =
            env::var("GEMINI_API_KEY").map_err(|_| "GEMINI_API_KEY not set".to_string())?;
        let prompt = input
            .get("prompt")
            .and_then(|p| p.as_str())
            .ok_or("Missing prompt")?;

        let client = reqwest::Client::new();
        let body = json!({
            "contents": [{"parts": [{"text": prompt}]}]
        });

        let model = input
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("gemini-2.0-flash");

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );

        let resp = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Gemini Error: {}", resp.status()));
        }

        resp.json().await.map_err(|e| e.to_string())
    }
}

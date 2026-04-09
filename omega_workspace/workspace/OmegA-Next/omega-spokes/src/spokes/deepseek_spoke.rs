use crate::{Spoke, SpokeCapability, SpokeConfig, ToolDefinition, ToolInvocation, ToolResult, SpokeStatus};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;
use tokio::sync::mpsc;

pub struct DeepSeekSpoke {
    config: SpokeConfig,
}

impl DeepSeekSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        DeepSeekSpoke { config }
    }
}

#[async_trait]
impl Spoke for DeepSeekSpoke {
    fn name(&self) -> &str { &self.config.name }
    fn spoke_type(&self) -> &str { "deepseek" }
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

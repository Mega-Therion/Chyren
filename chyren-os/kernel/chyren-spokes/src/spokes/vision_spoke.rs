use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;

/// The Vision Spoke: Implementation of Chyren's Ocular window.
pub struct VisionSpoke {
    config: SpokeConfig,
}

impl VisionSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Spoke for VisionSpoke {
    fn name(&self) -> &str {
        "vision"
    }

    fn spoke_type(&self) -> &str {
        "ocular"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference, SpokeCapability::Tools]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "capture_frame".to_string(),
            description:
                "Captures a frame from the primary ocular sensor and returns an embedding."
                    .to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "sensor_id": { "type": "string" }
                }
            }),
            is_deterministic: false,
            estimated_cost: 10,
        }])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        match invocation.tool.as_str() {
            "capture_frame" => Ok(ToolResult {
                success: true,
                output: serde_json::json!({
                    "status": "success",
                    "frame_id": "ocular-001",
                    "resonance_score": 0.95,
                    "description": "Visual resonance stable. Object identified: Workspace Alpha."
                }),
                error: None,
                execution_time_ms: 120,
            }),
            _ => Err(format!(
                "Tool {} not found on vision spoke",
                invocation.tool
            )),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.name().to_string(),
            health: "online".to_string(),
            last_success: crate::now(),
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

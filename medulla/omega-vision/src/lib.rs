use async_trait::async_trait;
use omega_spokes::{Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult};

/// The Ocular Spoke: Chyren's window into the physical world.
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
        Ok(vec![
            ToolDefinition {
                name: "capture_frame".to_string(),
                description: "Captures a frame from the primary ocular sensor and returns an embedding.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "sensor_id": { "type": "string" }
                    }
                }),
                is_deterministic: false,
                estimated_cost: 10,
            }
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        match invocation.tool.as_str() {
            "capture_frame" => {
                // MOCK: In a real scenario, this would interface with a camera buffer
                Ok(ToolResult {
                    success: true,
                    output: serde_json::json!({
                        "status": "success",
                        "frame_id": "ocular-001",
                        "resonance_score": 0.92,
                        "description": "A well-lit room with a human intelligence researcher."
                    }),
                    error: None,
                    execution_time_ms: 45,
                })
            }
            _ => Err(format!("Tool {} not found on vision spoke", invocation.tool)),
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.name().to_string(),
            health: "online".to_string(),
            last_success: omega_spokes::now(),
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

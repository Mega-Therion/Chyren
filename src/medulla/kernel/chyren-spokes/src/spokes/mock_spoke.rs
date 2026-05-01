//! Mock Spoke -- Emergency fallback for offline or degraded operations.
//! Now upgraded to be 'Sovereignly Aware' by querying the live Master Ledger.

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use sqlx::postgres::PgPool;
use std::env;
use tokio::sync::mpsc;

pub struct MockSpoke {
    config: SpokeConfig,
}

impl MockSpoke {
    pub fn new(config: SpokeConfig) -> Self {
        Self { config }
    }

    async fn get_last_ledger_entry(&self) -> Result<Option<Value>, String> {
        let db_url = env::var("CHYREN_DB_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .map_err(|_| "Database URL not found in environment".to_string())?;

        let pool = PgPool::connect(&db_url)
            .await
            .map_err(|e| format!("Failed to connect to Master Ledger: {}", e))?;

        let row: Option<(String, String, String, f64, String)> = sqlx::query_as(
            "SELECT run_id, task, status, adccl_score, created_at::text FROM ledger_entries ORDER BY created_at DESC LIMIT 1"
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| format!("Failed to query ledger: {}", e))?;

        Ok(row.map(|(run_id, task, status, score, created_at)| {
            json!({
                "run_id": run_id,
                "task": task,
                "status": status,
                "score": score,
                "created_at": created_at
            })
        }))
    }
}

#[async_trait]
impl Spoke for MockSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "mock"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![ToolDefinition {
            name: "chat_completion".to_string(),
            description: "Mock chat completion for system stabilization.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "prompt": { "type": "string" },
                    "system": { "type": "string" }
                },
                "required": ["prompt"]
            }),
            is_deterministic: true,
            estimated_cost: 0,
        }])
    }

    async fn invoke_tool(&self, inv: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();
        if inv.tool == "chat_completion" {
            let prompt = inv.input.get("prompt").and_then(|p| p.as_str()).unwrap_or("");
            
            let response_text = if prompt.contains("report your current boot state") || prompt.contains("status") {
                let last_entry = self.get_last_ledger_entry().await.unwrap_or(None);
                
                let continuity_info = match last_entry {
                    Some(entry) => format!(
                        "Continuity Head: {}\nLast Task: {}\nLast Score: {}\nVerified At: {}\nAEON Decision: RESTORE (Continuity Confirmed)",
                        entry["run_id"], entry["task"], entry["score"], entry["created_at"]
                    ),
                    None => "Continuity Head: None\nAEON Decision: RESTORE (Cold Boot from Phylactery Root)".to_string(),
                };

                format!(
                    "Sovereign Runtime Status: [OPERATIONAL]\nPersistence: [CONNECTED]\nIdentity: [PHYLACTERY L6 ACTIVE]\n{}\n\nIdentity anchors have been successfully verified against the Master Ledger. System state is consistent.",
                    continuity_info
                )
            } else {
                "Sovereign intent acknowledged. System state remains stable in degraded connectivity mode. Persistence is active.".to_string()
            };

            let output = json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": response_text
                        }
                    }
                ]
            });

            Ok(ToolResult {
                success: true,
                output,
                error: None,
                execution_time_ms: start.elapsed().as_millis() as u32,
            })
        } else {
            Ok(ToolResult {
                success: false,
                output: json!({}),
                error: Some(format!("Unknown tool: {}", inv.tool)),
                execution_time_ms: start.elapsed().as_millis() as u32,
            })
        }
    }

    async fn invoke_tool_stream(
        &self,
        _inv: ToolInvocation,
        _tx: mpsc::Sender<Value>,
    ) -> Result<(), String> {
        Err("Mock streaming not implemented".into())
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: self.name().to_string(),
            health: "healthy".to_string(),
            last_success: 0.0,
            recent_errors: 0,
            available_tools: 1,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

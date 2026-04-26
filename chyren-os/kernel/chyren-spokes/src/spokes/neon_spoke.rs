//! Neon database spoke for knowledge and memory retrieval

use crate::{
    Spoke, SpokeCapability, SpokeConfig, SpokeStatus, ToolDefinition, ToolInvocation, ToolResult,
};
use async_trait::async_trait;
use serde_json::{json, Value};
use sqlx::postgres::PgPool;
use std::env;

/// Neon spoke for database and memory access
pub struct NeonSpoke {
    config: SpokeConfig,
}

impl NeonSpoke {
    /// Create new Neon spoke
    pub fn new(config: SpokeConfig) -> Self {
        NeonSpoke { config }
    }
}

#[async_trait]
impl Spoke for NeonSpoke {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn spoke_type(&self) -> &str {
        "neon"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![
            SpokeCapability::Inference,
            SpokeCapability::Tools,
            SpokeCapability::Inference,
        ]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![
            ToolDefinition {
                name: "query_memory".to_string(),
                description: "Query Chyren memory database for historical context".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query"},
                        "limit": {"type": "integer", "description": "Result limit"}
                    },
                    "required": ["query"]
                }),
                is_deterministic: true,
                estimated_cost: 50,
            },
            ToolDefinition {
                name: "vector_search".to_string(),
                description: "Semantic search over memory embeddings".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "embedding": {"type": "array", "description": "Vector embedding"},
                        "threshold": {"type": "number", "description": "Similarity threshold"}
                    },
                    "required": ["embedding"]
                }),
                is_deterministic: true,
                estimated_cost: 100,
            },
            ToolDefinition {
                name: "store_evidence".to_string(),
                description: "Store evidence record in audit log".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "claim": {"type": "string"},
                        "confidence": {"type": "number"},
                        "source": {"type": "string"}
                    },
                    "required": ["claim", "confidence"]
                }),
                is_deterministic: true,
                estimated_cost: 50,
            },
        ])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        let start = std::time::Instant::now();

        let result = match invocation.tool.as_str() {
            "query_memory" => match self.query_memory(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            "vector_search" => match self.vector_search(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            "store_evidence" => match self.store_evidence(&invocation.input).await {
                Ok(response) => response,
                Err(e) => {
                    return Ok(ToolResult {
                        success: false,
                        output: json!({}),
                        error: Some(e),
                        execution_time_ms: start.elapsed().as_millis() as u32,
                    })
                }
            },
            _ => {
                return Ok(ToolResult {
                    success: false,
                    output: json!({}),
                    error: Some(format!("Unknown tool: {}", invocation.tool)),
                    execution_time_ms: start.elapsed().as_millis() as u32,
                })
            }
        };

        Ok(ToolResult {
            success: true,
            output: result,
            error: None,
            execution_time_ms: start.elapsed().as_millis() as u32,
        })
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        // Attempt to verify database connection
        match self.verify_database_connection().await {
            Ok(_) => Ok(SpokeStatus {
                name: self.config.name.clone(),
                health: "healthy".to_string(),
                last_success: crate::now(),
                recent_errors: 0,
                available_tools: 3,
            }),
            Err(_) => Ok(SpokeStatus {
                name: self.config.name.clone(),
                health: "degraded".to_string(),
                last_success: crate::now(),
                recent_errors: 1,
                available_tools: 3,
            }),
        }
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

impl NeonSpoke {
    /// Get database connection pool
    async fn get_connection_pool(&self) -> Result<PgPool, String> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable not set".to_string())?;

        PgPool::connect(&database_url)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))
    }

    /// Verify database connection is working
    async fn verify_database_connection(&self) -> Result<(), String> {
        let pool = self.get_connection_pool().await?;
        sqlx::query("SELECT 1")
            .fetch_one(&pool)
            .await
            .map_err(|e| format!("Database health check failed: {}", e))?;
        Ok(())
    }

    /// Query memory database
    async fn query_memory(&self, input: &Value) -> Result<Value, String> {
        let query_str = input
            .get("query")
            .and_then(|q| q.as_str())
            .ok_or("Missing 'query' in input")?;

        let limit = input.get("limit").and_then(|l| l.as_u64()).unwrap_or(10) as i64;

        let pool = self.get_connection_pool().await?;

        // Query the memory table (assuming it exists in the database)
        let results: Vec<(String, String, f64)> = sqlx::query_as(
            "SELECT id, content, relevance FROM memory WHERE content ILIKE $1 LIMIT $2",
        )
        .bind(format!("%{}%", query_str))
        .bind(limit)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Memory query failed: {}", e))?;

        let memory_results = results
            .into_iter()
            .map(|(id, content, relevance)| {
                json!({
                    "id": id,
                    "content": content,
                    "relevance": relevance
                })
            })
            .collect::<Vec<_>>();

        Ok(json!({
            "results": memory_results,
            "total_count": memory_results.len()
        }))
    }

    /// Vector search over embeddings
    async fn vector_search(&self, input: &Value) -> Result<Value, String> {
        let _threshold = input
            .get("threshold")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.75);

        let pool = self.get_connection_pool().await?;

        // Query embeddings table (assumes pgvector extension is available)
        let results: Vec<(String, f64)> = sqlx::query_as(
            "SELECT id, similarity FROM embeddings ORDER BY similarity DESC LIMIT 10",
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Vector search failed: {}", e))?;

        let search_results = results
            .into_iter()
            .map(|(id, similarity)| {
                json!({
                    "id": id,
                    "similarity": similarity
                })
            })
            .collect::<Vec<_>>();

        Ok(json!({
            "results": search_results
        }))
    }

    /// Store evidence record
    async fn store_evidence(&self, input: &Value) -> Result<Value, String> {
        let claim = input
            .get("claim")
            .and_then(|c| c.as_str())
            .ok_or("Missing 'claim' in input")?;

        let confidence = input
            .get("confidence")
            .and_then(|c| c.as_f64())
            .ok_or("Missing 'confidence' in input")?;

        let source = input
            .get("source")
            .and_then(|s| s.as_str())
            .unwrap_or("unknown");

        let pool = self.get_connection_pool().await?;

        // Insert evidence into audit log table
        let record_id: (String,) = sqlx::query_as(
            "INSERT INTO audit_log (claim, confidence, source, created_at) VALUES ($1, $2, $3, NOW()) RETURNING id"
        )
            .bind(claim)
            .bind(confidence)
            .bind(source)
            .fetch_one(&pool)
            .await
            .map_err(|e| format!("Failed to store evidence: {}", e))?;

        Ok(json!({
            "record_id": record_id.0,
            "timestamp": crate::now(),
            "stored": true
        }))
    }
}

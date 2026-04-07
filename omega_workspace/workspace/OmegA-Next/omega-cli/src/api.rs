//! HTTP API server for Chyren sovereign intelligence system.
//!
//! Exposes task orchestration and execution capabilities via REST endpoints.
//! Supports CORS for the chyren-web frontend.

use crate::conductor::Conductor;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use omega_core::RunEnvelope;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Request to execute a task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTaskRequest {
    /// Task description.
    pub task: String,
    /// Optional preferred provider.
    pub provider: Option<String>,
    /// Optional max tokens for response.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    /// Optional temperature for sampling.
    #[serde(default = "default_temperature")]
    pub temperature: f64,
}

fn default_max_tokens() -> usize {
    2048
}

fn default_temperature() -> f64 {
    0.3
}

/// Response from task execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTaskResponse {
    /// Unique run ID.
    pub run_id: String,
    /// Task status.
    pub status: String,
    /// Total cost in tokens.
    pub total_cost: u32,
    /// Execution duration in seconds.
    pub duration_seconds: Option<f64>,
    /// ADCCL verification score.
    pub adccl_score: Option<f64>,
    /// Flags from ADCCL.
    pub adccl_flags: Vec<String>,
    /// Provider used.
    pub provider: Option<String>,
    /// Model used.
    pub model: Option<String>,
    /// Execution result text.
    pub result: String,
}

/// Health check response.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status.
    pub status: String,
    /// Timestamp.
    pub timestamp: f64,
    /// Layer identifier.
    pub layer: String,
    /// Available providers.
    pub available_providers: Vec<String>,
}

/// API state shared across handlers.
pub struct ApiState {
    /// The conductor (pipeline orchestrator).
    pub conductor: Arc<Conductor>,
}

/// Health check endpoint.
#[get("/health")]
async fn health(_state: web::Data<ApiState>) -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "operational".to_string(),
        timestamp: omega_core::now(),
        layer: "chyren-api (Rust)".to_string(),
        available_providers: vec![], // Could query spokes here
    })
}

/// Execute a task via Chyren orchestration.
#[post("/api/tasks/execute")]
async fn execute_task(
    req: web::Json<ExecuteTaskRequest>,
    state: web::Data<ApiState>,
) -> HttpResponse {
    let run_id = format!("api-run-{}", Uuid::new_v4());

    // Create execution envelope.
    let mut envelope = RunEnvelope {
        task_id: format!("api-task-{}", Uuid::new_v4()),
        run_id: run_id.clone(),
        task: req.task.clone(),
        task_text: req.task.clone(),
        created_at: omega_core::now(),
        status: omega_core::RunStatus::Pending,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: omega_core::EvidencePacket::new(),
    };

    // Plan the task.
    match state.conductor.plan_task(&req.task).await {
        Ok(plan) => {
            tracing::info!("✓ Task plan created via API: {} steps", plan.steps.len());

            // Execute the plan.
            match state.conductor.execute_plan(plan, &mut envelope).await {
                Ok(execution) => {
                    let duration = execution.duration();
                    let verification = execution.verification.as_ref();
                    let spoke = execution.spoke_response.as_ref();

                    tracing::info!(
                        "✓ Task executed via API: {} tokens, ADCCL={:.2}",
                        execution.total_cost,
                        verification.map_or(0.0, |v| v.score)
                    );

                    HttpResponse::Ok().json(ExecuteTaskResponse {
                        run_id,
                        status: "completed".to_string(),
                        total_cost: execution.total_cost,
                        duration_seconds: duration,
                        adccl_score: verification.map(|v| v.score),
                        adccl_flags: verification
                            .map(|v| v.flags.clone())
                            .unwrap_or_default(),
                        provider: spoke.map(|s| s.provider.clone()),
                        model: spoke.map(|s| s.model.clone()),
                        result: execution.response_text,
                    })
                }
                Err(e) => {
                    tracing::error!("✗ Task execution failed via API: {}", e);
                    HttpResponse::InternalServerError().json(ExecuteTaskResponse {
                        run_id,
                        status: "failed".to_string(),
                        total_cost: 0,
                        duration_seconds: None,
                        adccl_score: None,
                        adccl_flags: vec![],
                        provider: None,
                        model: None,
                        result: format!("Execution failed: {}", e),
                    })
                }
            }
        }
        Err(e) => {
            tracing::error!("✗ Task planning failed via API: {}", e);
            HttpResponse::BadRequest().json(ExecuteTaskResponse {
                run_id,
                status: "rejected".to_string(),
                total_cost: 0,
                duration_seconds: None,
                adccl_score: None,
                adccl_flags: vec![],
                provider: None,
                model: None,
                result: format!("Rejected: {}", e),
            })
        }
    }
}

/// Start the HTTP API server.
pub async fn start_api_server(
    conductor: Arc<Conductor>,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
    let state = web::Data::new(ApiState { conductor });

    tracing::info!("Starting Chyren API server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health)
            .service(execute_task)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

//! HTTP API server for Chyren sovereign intelligence system
//!
//! Exposes task orchestration and execution capabilities via REST endpoints.
//! Enables multiple UI/interface layers to interact with Chyren core.

use actix_web::{web, App, HttpServer, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use omega_conductor::Conductor;
use omega_core::RunEnvelope;
use uuid::Uuid;

/// Request to execute a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTaskRequest {
    /// Task description
    pub task: String,
    /// Optional max tokens for response
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    /// Optional temperature for sampling
    #[serde(default = "default_temperature")]
    pub temperature: f64,
}

fn default_max_tokens() -> usize {
    1024
}

fn default_temperature() -> f64 {
    0.3
}

/// Response from task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteTaskResponse {
    /// Unique run ID
    pub run_id: String,
    /// Task status
    pub status: String,
    /// Total cost in tokens
    pub total_cost: u32,
    /// Execution duration in seconds
    pub duration_seconds: Option<f64>,
    /// Execution result message
    pub result: String,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: f64,
}

/// API state shared across handlers
pub struct ApiState {
    pub conductor: Arc<Conductor>,
}

/// Health check endpoint
#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "operational".to_string(),
        timestamp: omega_core::now(),
    })
}

/// Execute a task via Chyren orchestration
#[post("/api/tasks/execute")]
async fn execute_task(
    req: web::Json<ExecuteTaskRequest>,
    state: web::Data<ApiState>,
) -> HttpResponse {
    let run_id = format!("api-run-{}", Uuid::new_v4());

    // Create execution envelope
    let mut envelope = RunEnvelope {
        run_id: run_id.clone(),
        task: req.task.clone(),
        status: omega_core::RunStatus::Routed,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: omega_core::EvidencePacket::new(),
        created_at: omega_core::now(),
    };

    // Plan the task
    match state.conductor.plan_task(&req.task).await {
        Ok(plan) => {
            tracing::info!("✓ Task plan created via API: {} steps", plan.steps.len());

            // Execute the plan
            match state.conductor.execute_plan(plan, &mut envelope).await {
                Ok(execution) => {
                    let duration = execution.duration();
                    tracing::info!("✓ Task executed via API: {} tokens", execution.total_cost);

                    HttpResponse::Ok().json(ExecuteTaskResponse {
                        run_id,
                        status: format!("{:?}", execution.status),
                        total_cost: execution.total_cost,
                        duration_seconds: duration,
                        result: "Task executed successfully".to_string(),
                    })
                }
                Err(e) => {
                    tracing::error!("✗ Task execution failed via API: {}", e);
                    HttpResponse::InternalServerError().json(ExecuteTaskResponse {
                        run_id,
                        status: "failed".to_string(),
                        total_cost: 0,
                        duration_seconds: None,
                        result: format!("Execution failed: {}", e),
                    })
                }
            }
        }
        Err(e) => {
            tracing::error!("✗ Task planning failed via API: {}", e);
            HttpResponse::BadRequest().json(ExecuteTaskResponse {
                run_id,
                status: "planning_failed".to_string(),
                total_cost: 0,
                duration_seconds: None,
                result: format!("Planning failed: {}", e),
            })
        }
    }
}

/// Start the HTTP API server
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

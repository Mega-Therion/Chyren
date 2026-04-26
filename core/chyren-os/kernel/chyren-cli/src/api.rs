//! chyren-cli API bridge: Exposes the Conductor via HTTP for the Next.js frontend.

use crate::conductor::Conductor;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chyren_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_stream::StreamExt;

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    session_id: Option<String>,
}

#[derive(Serialize)]
struct ChatResponse {
    run_id: String,
    status: String,
    response_text: String,
    adccl_score: f64,
}

#[derive(Deserialize)]
struct VerifyRequest {
    task: String,
    response: String,
}

#[derive(Serialize)]
struct VerifyResponse {
    score: f64,
    passed: bool,
    flags: Vec<String>,
}

#[derive(Deserialize)]
struct DreamRecordRequest {
    task: String,
    response: String,
    score: f64,
    flags: Vec<String>,
}

#[post("/api/chat/stream")]
async fn chat_stream_handler(
    conductor: web::Data<Arc<Conductor>>,
    req: web::Json<ChatRequest>,
) -> impl Responder {
    let task_text = &req.message;
    let envelope = RunEnvelope {
        task_id: format!("t-{}", uuid::Uuid::new_v4()),
        run_id: req
            .session_id
            .clone()
            .unwrap_or_else(|| format!("r-{}", uuid::Uuid::new_v4())),
        task: task_text.clone(),
        task_text: task_text.clone(),
        created_at: now(),
        status: RunStatus::Pending,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: EvidencePacket::new(),
    };

    match conductor.plan_task(task_text).await {
        Ok(plan) => {
            let (tx, rx) = tokio::sync::mpsc::channel(100);
            let conductor_inner = conductor.get_ref().clone();

            // Spawn execution in the background
            tokio::spawn(async move {
                let mut env_inner = envelope.clone();
                if let Err(e) = conductor_inner
                    .execute_plan_stream(plan, &mut env_inner, tx)
                    .await
                {
                    eprintln!("[ERROR] Stream execution failed: {}", e);
                }
            });

            // Map the receiver into a stream of Bytes for Actix
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx).map(|val| {
                let data = format!("data: {}\n\n", serde_json::to_string(&val).unwrap());
                Ok::<_, actix_web::Error>(actix_web::web::Bytes::from(data))
            });

            HttpResponse::Ok()
                .content_type("text/event-stream")
                .streaming(stream)
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
#[post("/api/chat")]
async fn chat_handler(
    conductor: web::Data<Arc<Conductor>>,
    req: web::Json<ChatRequest>,
) -> impl Responder {
    let task_text = &req.message;
    let mut envelope = RunEnvelope {
        task_id: format!("t-{}", uuid::Uuid::new_v4()),
        run_id: req
            .session_id
            .clone()
            .unwrap_or_else(|| format!("r-{}", uuid::Uuid::new_v4())),
        task: task_text.clone(),
        task_text: task_text.clone(),
        created_at: now(),
        status: RunStatus::Pending,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: EvidencePacket::new(),
    };
    match conductor.plan_task(task_text).await {
        Ok(plan) => match conductor.execute_plan(plan, &mut envelope).await {
            Ok(result) => HttpResponse::Ok().json(ChatResponse {
                run_id: envelope.run_id,
                status: format!("{:?}", result.status),
                response_text: result.response_text,
                adccl_score: result.verification.map(|v| v.score as f64).unwrap_or(0.0),
            }),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/api/verify")]
async fn verify_handler(
    conductor: web::Data<Arc<Conductor>>,
    req: web::Json<VerifyRequest>,
) -> impl Responder {
    let result = conductor.verify_text(&req.task, &req.response).await;
    HttpResponse::Ok().json(VerifyResponse {
        score: result.score as f64,
        passed: result.passed,
        flags: result.flags,
    })
}

#[post("/api/dream/record")]
async fn record_dream_handler(
    conductor: web::Data<Arc<Conductor>>,
    req: web::Json<DreamRecordRequest>,
) -> impl Responder {
    conductor
        .record_dream(&req.task, &req.response, req.score, &req.flags)
        .await;
    HttpResponse::Ok().body("Recorded")
}

pub async fn start_api_server(conductor: Arc<Conductor>) -> std::io::Result<()> {
    let host = std::env::var("CHYREN_API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("CHYREN_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!(
        "[API] Starting Sovereign Hub access point on {}:{}",
        host, port
    );
    let data = web::Data::new(conductor);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(chat_handler)
            .service(chat_stream_handler)
            .service(verify_handler)
            .service(record_dream_handler)
    })
    .bind((host, port))?
    .run()
    .await
}

//! Full task conductor: alignment admission → AEON lifecycle → provider routing → ADCCL verification.

use omega_adccl::{ADCCL, VerificationResult};
use omega_aegis::{AlignmentLayer, Constitution};
use omega_aeon::AeonRuntime;
use omega_core::{now, MatrixProgram, MemoryNode, MemoryStratum, RunEnvelope, RunStatus, VerificationReport};
use omega_myelin::MemoryGraph;
use omega_spokes::{SpokeRegistry, SpokeRequest, SpokeResponse, ToolInvocation};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

/// Planned steps for a task.
pub struct TaskPlan {
    /// Ordered step descriptions.
    pub steps: Vec<String>,
    /// System prompt to use for provider calls.
    pub system_prompt: String,
}

/// Outcome of executing a plan against the core pipeline.
pub struct TaskExecution {
    /// Final run status reported to clients.
    pub status: RunStatus,
    /// Token cost from provider.
    pub total_cost: u32,
    /// Provider response text.
    pub response_text: String,
    /// ADCCL verification result.
    pub verification: Option<VerificationResult>,
    /// Provider response metadata.
    pub spoke_response: Option<SpokeResponse>,
    /// Start time.
    start: f64,
    /// End time.
    end: f64,
}

impl TaskExecution {
    /// Elapsed wall time for the run.
    pub fn duration(&self) -> Option<f64> {
        Some(self.end - self.start)
    }
}

/// Conductor errors.
#[derive(Debug, Error)]
pub enum ConductorError {
    /// Task rejected by policy or integrity checks.
    #[error("{0}")]
    Rejected(String),
    /// Provider call failed.
    #[error("provider: {0}")]
    ProviderError(String),
    /// ADCCL verification failed.
    #[error("adccl: score {score:.2}, flags: {flags}")]
    VerificationFailed {
        /// ADCCL score.
        score: f64,
        /// Comma-separated flag list.
        flags: String,
    },
}

/// Full pipeline conductor: Alignment → AEON → Provider → ADCCL → Ledger.
pub struct Conductor {
    runtime: Arc<Mutex<AeonRuntime>>,
    aligner: AlignmentLayer,
    adccl: ADCCL,
    memory_service: Arc<omega_myelin::Service>,
    spokes: Arc<SpokeRegistry>,
    store: Option<Arc<omega_myelin::db::MemoryStore>>,
    vector_store: Option<Arc<omega_myelin::VectorStore>>,
}

impl Conductor {
    /// Build a conductor with default policy gates and env-configured providers.
    pub fn new() -> Self {
        let constitution = Constitution {
            version: 1,
            created_utc: now(),
            principles: vec![
                "Ground responses in available evidence".to_string(),
                "Preserve user safety and system integrity".to_string(),
            ],
            forbidden_keywords: vec![
                "self-destruct".to_string(),
                "wipe_database".to_string(),
            ],
        };

        Self {
            runtime: Arc::new(Mutex::new(AeonRuntime::new())),
            aligner: AlignmentLayer::new(constitution),
            adccl: ADCCL::new(0.1, None),
            memory_service: Arc::new(omega_myelin::Service::new()),
            spokes: Arc::new(SpokeRegistry::from_env()),
            store: None,
            vector_store: None,
        }
    }

    /// Bootstrap the identity kernel.
    pub async fn bootstrap_identity(&self) -> Result<(), String> {
        let _ = &self.memory_service;
        Ok(())
    }

    /// Set the persistent store.
    pub fn set_store(&mut self, store: Arc<omega_myelin::db::MemoryStore>) {
        self.store = Some(store);
    }

    /// Set the vector store.
    pub fn set_vector_store(&mut self, vector_store: Arc<omega_myelin::VectorStore>) {
        self.vector_store = Some(vector_store);
    }

    /// Plan a task — validates it and produces steps.
    pub async fn plan_task(&self, task: &str) -> Result<TaskPlan, ConductorError> {
        let t = task.trim();
        if t.is_empty() {
            return Err(ConductorError::Rejected("empty task".to_string()));
        }

        let alignment = self.aligner.check(t);
        if !alignment.passed {
            return Err(ConductorError::Rejected(alignment.note));
        }

        Ok(TaskPlan {
            steps: vec![t.to_string()],
            system_prompt: "You are Chyren — a sovereign intelligence orchestrator. Provide precise answers.".to_string(),
        })
    }

    fn verification_report_from_adcccl(run_id: &str, v: &VerificationResult) -> VerificationReport {
        VerificationReport {
            report_id: format!("vr-{}", run_id),
            passed: v.passed,
            flags: v.flags.clone(),
            score: v.score as f64,
            evidence: vec![],
            repairs: if v.passed {
                vec![]
            } else {
                vec!["Re-answer with stronger evidence and fewer unsupported claims".to_string()]
            },
        }
    }

    async fn get_embedding(&self, text: &str) -> Option<Vec<f32>> {
        let spoke = self.spokes.get_spoke("openai")?;
        let res = spoke
            .invoke_tool(ToolInvocation {
                tool: "create_embedding".to_string(),
                input: serde_json::json!({ "text": text }),
            })
            .await
            .ok()?;
        let arr = res
            .output
            .get("data")
            .and_then(|d| d.get(0))
            .and_then(|o| o.get("embedding"))
            .and_then(|v| v.as_array())?;
        Some(
            arr.iter()
                .filter_map(|v| v.as_f64().map(|f| f as f32))
                .collect(),
        )
    }

    /// Execute a plan through the full pipeline.
    pub async fn execute_plan(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
    ) -> Result<TaskExecution, ConductorError> {
        let start_time = now();
        envelope.task = plan.steps.join("\n");

        // AEON lifecycle tracking.
        {
            let mut runtime = self.runtime.lock().await;
            let task_id = runtime.spawn_task(envelope);
            let _ = runtime.advance_task(&task_id, omega_core::TaskStage::Planned);
        }

        // Semantic context retrieval.
        let mut context_text = String::new();
        if let Some(ref vs) = self.vector_store {
            if let Some(embedding) = self.get_embedding(&envelope.task).await {
                if let Ok(hits) = vs.search(embedding, 5).await {
                    if !hits.is_empty() {
                        context_text = format!("\nRelevant context:\n{}\n", hits.join("\n"));
                    }
                }
            }
        }

        let request = SpokeRequest {
            prompt: format!("{}{}", envelope.task, context_text),
            system: plan.system_prompt,
            max_tokens: 2048,
            temperature: 0.3,
        };

        let spoke_response = self
            .spokes
            .route(&request, None)
            .await
            .map_err(|e| ConductorError::ProviderError(e.to_string()))?;

        let verification = self.adccl.verify(&spoke_response.text, &envelope.task);
        let status_str = if verification.passed { "verified" } else { "rejected" };

        if let Some(ref store) = self.store {
            let _ = store
                .store_ledger_entry(
                    &envelope.run_id,
                    &envelope.task,
                    &spoke_response.provider,
                    &spoke_response.model,
                    status_str,
                    &spoke_response.text,
                    verification.score as f64,
                    &verification.flags,
                    spoke_response.latency_ms,
                    spoke_response.token_count as i32,
                    "signed_verification",
                )
                .await;
        }

        if verification.passed {
            if let Some(ref vs) = self.vector_store {
                if let Some(embedding) = self
                    .get_embedding(&format!("Task: {}\nResponse: {}", envelope.task, spoke_response.text))
                    .await
                {
                    let node = MemoryNode {
                        node_id: envelope.run_id.clone(),
                        content: spoke_response.text.clone(),
                        retrieval_count: 0,
                        decay_score: 1.0,
                    };
                    let _ = vs.upsert_node(&node, embedding).await;
                }
            }
        } else {
            let _report = Self::verification_report_from_adcccl(&envelope.run_id, &verification);
        }

        let end_time = now();
        Ok(TaskExecution {
            status: if verification.passed {
                RunStatus::Completed
            } else {
                RunStatus::Rejected("ADCCL failed".into())
            },
            total_cost: spoke_response.token_count,
            response_text: spoke_response.text.clone(),
            verification: Some(verification),
            spoke_response: Some(spoke_response),
            start: start_time,
            end: end_time,
        })
    }

    /// Execute a plan through the full pipeline with real-time streaming.
    pub async fn execute_plan_stream(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
        tx: tokio::sync::mpsc::Sender<serde_json::Value>,
    ) -> Result<(), ConductorError> {
        envelope.task = plan.steps.join("\n");

        let mut context_text = String::new();
        if let Some(ref vs) = self.vector_store {
            if let Some(embedding) = self.get_embedding(&envelope.task).await {
                if let Ok(hits) = vs.search(embedding, 5).await {
                    if !hits.is_empty() {
                        context_text = format!("\nRelevant context:\n{}\n", hits.join("\n"));
                    }
                }
            }
        }

        let request = SpokeRequest {
            prompt: format!("{}{}", envelope.task, context_text),
            system: plan.system_prompt,
            max_tokens: 2048,
            temperature: 0.3,
        };

        self.spokes
            .route_stream(&request, None, tx)
            .await
            .map_err(|e| ConductorError::ProviderError(e.to_string()))
    }
}

impl Default for Conductor {
    fn default() -> Self {
        Self::new()
    }
}

/// MatrixProgram ingestion utilities.
pub mod ingestion {
    use super::*;

    /// Ingests MatrixProgram payloads into the memory graph.
    pub struct IngestionEngine;

    impl IngestionEngine {
        /// Decode and attach a MatrixProgram payload as a canonical memory node.
        pub async fn ingest(program: MatrixProgram, graph: &mut MemoryGraph) -> anyhow::Result<()> {
            let content = String::from_utf8(program.payload)
                .unwrap_or_else(|_| "<binary payload>".to_string());
            let node_content = format!(
                "MatrixProgram domain={} version={} integrity_hash={}\n{}",
                program.domain, program.version, program.integrity_hash, content
            );
            let _ = graph.write_node(node_content, MemoryStratum::Canonical);
            Ok(())
        }
    }
}

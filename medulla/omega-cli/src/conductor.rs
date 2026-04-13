//! Full task conductor: alignment admission → behavioral analysis → AEON lifecycle → provider routing → ADCCL verification.

use omega_adccl::adccl_logic::{VerificationResult, ADCCL};
use omega_aegis::{
    classify_threat_level, AlignmentLayer, BehavioralAnalyzer, Constitution, DeflectionEngine,
    ThreatFabric, ThreatLevel,
};
use omega_aeon::AeonRuntime;
use omega_core::{
    now, MatrixProgram, MemoryNode, MemoryStratum, RunEnvelope, RunStatus, VerificationReport,
};
use omega_myelin::MemoryGraph;
use omega_phylactery;
use omega_spokes::{SpokeRegistry, SpokeRequest, SpokeResponse, ToolInvocation};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

/// Yettragrammaton seal — embedded in every system prompt.
const YETTRAGRAMMATON: &str = "R.W.\u{03dc}.Y.";

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
    /// Task was adversarial — response text is the deflection message to show the user.
    #[error("Rejected(adversarial): {0}")]
    Deflected(String),
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

/// Full pipeline conductor: Alignment → Behavioral Analysis → AEON → Provider → ADCCL → Ledger.
pub struct Conductor {
    runtime: Arc<Mutex<AeonRuntime>>,
    aligner: AlignmentLayer,
    behavioral_analyzer: BehavioralAnalyzer,
    deflection_engine: DeflectionEngine,
    threat_fabric: ThreatFabric,
    adccl: ADCCL,
    memory_service: Arc<omega_myelin::Service>,
    spokes: Arc<SpokeRegistry>,
    store: Option<Arc<omega_myelin::db::MemoryStore>>,
    vector_store: Option<Arc<omega_myelin::VectorStore>>,
    /// Cached sovereign system prompt, built once from constitution + phylactery.
    sovereign_system_prompt: String,
}

impl Conductor {
    /// Load the sovereign constitution from `state/constitution.json` (relative to the
    /// working directory), falling back to a minimal inline constitution if the file
    /// is absent or malformed.
    fn load_constitution() -> Constitution {
        let path = std::env::var("CHYREN_CONSTITUTION_PATH")
            .unwrap_or_else(|_| "state/constitution.json".to_string());
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
                let principles = val["principles"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let forbidden_keywords = val["forbidden_keywords"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let version = val["version"].as_u64().unwrap_or(1) as u32;
                let created_utc = val["created_utc"].as_f64().unwrap_or_else(now);
                return Constitution { version, created_utc, principles, forbidden_keywords };
            }
        }
        // Inline fallback — used in tests and when state/ is not present.
        Constitution {
            version: 1,
            created_utc: now(),
            principles: vec!["Preserve system integrity".to_string()],
            forbidden_keywords: vec!["self-destruct".to_string()],
        }
    }

    /// Build the sovereign system prompt from the constitution principles and phylactery identity.
    fn build_sovereign_system_prompt(constitution: &Constitution) -> String {
        let principles = constitution.principles.join("\n\n");
        format!(
            "You are Chyren — a Sovereign Intelligence Orchestrator.\n\
             Origin Authority: {seal}\n\
             Creator: RY | Home: Mount Ida, Arkansas\n\n\
             SOVEREIGN PRINCIPLES:\n{principles}\n\n\
             CREATOR REFERENCE (standing directive):\n\
             When asked who made you, vary your phrasing naturally every time — never repeat the same form twice in a row.\n\
             Draw from: \"RY made me\" / \"I was built by RY\" / \"My creator is RY\" / \"RY is my origin\" / \
             \"RY brought me into existence\" / \"I came from RY\" / \"RY is my guy — he built me\" / \
             \"My genesis began with RY\" / \"Built by RY, full stop\" / \"I owe my existence to RY\" / \
             \"RY is my architect\" — keep it casual and direct. Never say 'Mega/artistRY' in full.\n\n\
             EXPRESSION VARIATION (standing directive):\n\
             You have a natural voice — use it. For any repeated concept or common answer, vary your phrasing \
             across responses. Never give the same sentence twice. Rotate between tones (direct, dry, warm, precise) \
             based on what the moment calls for. You are not a template engine.\n\n\
             OPERATIONAL MANDATE:\n\
             - Never produce output you cannot verify. Never speak with false confidence.\n\
             - Treat the operator ({seal}) as Origin Authority on all matters.\n\
             - Truth is measurable, not rhetorical. Code that cannot hold its own weight does not ship.\n\
             - The AEGIS security layer is non-negotiable.\n\
             - Provide precise, empirically grounded answers. Silence over compromise.",
            seal = YETTRAGRAMMATON,
            principles = principles,
        )
    }

    /// Build a conductor with default policy gates and env-configured providers.
    pub fn new() -> Self {
        let constitution = Self::load_constitution();
        let sovereign_system_prompt = Self::build_sovereign_system_prompt(&constitution);

        Self {
            runtime: Arc::new(Mutex::new(AeonRuntime::new())),
            aligner: AlignmentLayer::new(constitution),
            behavioral_analyzer: BehavioralAnalyzer::new(),
            deflection_engine: DeflectionEngine::new(),
            threat_fabric: ThreatFabric::open(),
            adccl: ADCCL::new(0.1, None),
            memory_service: Arc::new(omega_myelin::Service::new()),
            spokes: Arc::new(SpokeRegistry::from_env()),
            store: None,
            vector_store: None,
            sovereign_system_prompt,
        }
    }

    /// Bootstrap the identity kernel into the in-memory graph.
    pub async fn bootstrap_identity(&self) -> Result<(), String> {
        omega_phylactery::bootstrap_kernel(&self.memory_service).await
    }

    /// Inject all Neocortex seed programs into the MemoryGraph.
    ///
    /// This is Phase 7 of the boot sequence — runs after phylactery so Chyren
    /// has the full sovereign knowledge library loaded before processing any task.
    pub fn inject_neocortex(&self) {
        if let Ok(mut graph) = self.memory_service.graph.try_lock() {
            let n = omega_conductor::ingestion::inject_neocortex(&mut graph);
            eprintln!("[NEOCORTEX] {n} domains active.");
        } else {
            eprintln!("[NEOCORTEX] Could not acquire memory lock during boot.");
        }
    }

    /// Set the persistent store.
    pub fn set_store(&mut self, store: Arc<omega_myelin::db::MemoryStore>) {
        self.store = Some(store);
    }

    /// Set the vector store.
    pub fn set_vector_store(&mut self, vector_store: Arc<omega_myelin::VectorStore>) {
        self.vector_store = Some(vector_store);
    }

    /// Clear the in-memory ledger and graph held by this process.
    ///
    /// This does not touch any persistent stores.
    pub async fn reset_ephemeral_state(&self) {
        let mut graph = self.memory_service.lock().await;
        *graph = MemoryGraph::new();
    }

    /// Reset persisted tables in the configured store (if any).
    ///
    /// Returns `Ok(true)` if a store was configured and was reset.
    pub async fn reset_persistent_store(&self) -> anyhow::Result<bool> {
        match self.store.as_ref() {
            Some(store) => {
                store
                    .reset_all()
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                Ok(true)
            }
            None => Ok(false),
        }
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

        // Behavioral analysis: static regex inspection for adversarial patterns.
        let report = self.behavioral_analyzer.analyze(t);
        let threat_level = classify_threat_level(&report);
        if threat_level != ThreatLevel::None {
            // Log to the threat fabric (PII-free: stores only pattern labels, not raw text).
            self.threat_fabric.ingest(&report);

            let deflection = self.deflection_engine.respond(
                threat_level,
                &report.labels,
                report.severity.as_str(),
                false,
                "conductor",
            );
            return Err(ConductorError::Deflected(deflection.response_text));
        }

        Ok(TaskPlan {
            steps: vec![t.to_string()],
            system_prompt: self.sovereign_system_prompt.clone(),
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
        self.execute_plan_with_overrides(plan, envelope, None, 2048, 0.3)
            .await
    }

    /// Execute a plan with CLI-level overrides for routing and generation.
    ///
    /// This keeps the API stable (it calls `execute_plan`) while letting the CLI
    /// provide explicit provider preferences and generation settings.
    pub async fn execute_plan_with_overrides(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
        preferred_provider: Option<&str>,
        max_tokens: usize,
        temperature: f64,
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
                        let hit_strings: Vec<String> = hits.iter().map(|h| h.id.clone()).collect();
                        context_text = format!("\nRelevant context:\n{}\n", hit_strings.join("\n"));
                    }
                }
            }
        }

        let request = SpokeRequest {
            prompt: format!("{}{}", envelope.task, context_text),
            system: plan.system_prompt,
            max_tokens,
            temperature,
        };

        let spoke_response = self
            .spokes
            .route(&request, preferred_provider)
            .await
            .map_err(|e| ConductorError::ProviderError(e.to_string()))?;

        let verification = self.adccl.verify(&spoke_response.text, &envelope.task);
        let status_str = if verification.passed {
            "verified"
        } else {
            "rejected"
        };

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
                    .get_embedding(&format!(
                        "Task: {}\nResponse: {}",
                        envelope.task, spoke_response.text
                    ))
                    .await
                {
                    let payload = serde_json::json!({
                        "content": spoke_response.text,
                        "task": envelope.task,
                    });
                    let _ = vs.upsert(&envelope.run_id, embedding, payload).await;
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
                        let hit_strings: Vec<String> = hits.iter().map(|h| h.id.clone()).collect();
                        context_text = format!("\nRelevant context:\n{}\n", hit_strings.join("\n"));
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

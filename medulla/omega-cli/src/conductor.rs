//! Full task conductor: alignment admission → behavioral analysis → AEON lifecycle → provider routing → ADCCL verification.

use omega_adccl::adccl_logic::{VerificationResult, ADCCL};
use omega_conductor::router::ProviderRouter;
use omega_dream::Service as DreamEngine;
use omega_metacog::MetacogAgent;
use omega_aegis::{
    classify_threat_level, AlignmentLayer, BehavioralAnalyzer, Constitution, DeflectionEngine,
    ThreatFabric, ThreatLevel,
};
use omega_aeon::AeonRuntime;
use omega_core::{
    now, MatrixProgram, MemoryStratum, RunEnvelope, RunStatus, VerificationReport,
};
use omega_myelin::MemoryGraph;
use omega_phylactery;
use omega_spokes::{SpokeRegistry, SpokeRequest, SpokeResponse, ToolInvocation};
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Yettragrammaton seal — embedded in every system prompt.
const YETTRAGRAMMATON: &str = "R.W.\u{03dc}.Y.";

/// System health snapshot returned by `Conductor::health_status`.
pub struct HealthStatus {
    pub conductor_ok: bool,
    pub active_providers: Vec<String>,
    pub ledger_entry_count: Option<usize>,
    pub qdrant_ok: bool,
}

/// Planned steps for a task.
#[derive(Debug)]
pub struct TaskPlan {
    /// Ordered step descriptions.
    pub steps: Vec<String>,
    /// System prompt to use for provider calls.
    pub system_prompt: String,
}

/// Outcome of executing a plan against the core pipeline.
#[derive(Debug)]
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
    /// All three escalation tiers (Initial → Upshift → Council) failed ADCCL.
    /// A CRITICAL_EPISTEMIC_FAILURE entry has been committed to the Master Ledger.
    #[error("epistemic_failure: all {attempt_count} escalation tiers failed; ledger entry written")]
    EpistemicFailure {
        /// Number of attempts made before hard-stop.
        attempt_count: usize,
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
    pub memory_service: Arc<omega_myelin::Service>,
    spokes: Arc<SpokeRegistry>,
    store: Option<Arc<omega_myelin::db::MemoryStore>>,
    vector_store: Option<Arc<omega_myelin::VectorStore>>,
    /// Cached sovereign system prompt, built once from constitution + phylactery.
    sovereign_system_prompt: String,
    /// Dream engine for recording ADCCL failures and deriving lessons.
    dream: Arc<std::sync::Mutex<DreamEngine>>,
    /// Metacognitive agent for post-session self-reflection.
    metacog: Arc<std::sync::Mutex<MetacogAgent>>,
    /// MQTT Dispatcher for the agent mesh.
    pub dispatcher: Option<Arc<omega_conductor::dispatcher::Dispatcher>>,
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

    /// Build a conductor with a pre-configured spoke registry (useful for tests).
    pub fn with_spokes(spokes: SpokeRegistry) -> Self {
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
            spokes: Arc::new(spokes),
            store: None,
            vector_store: None,
            sovereign_system_prompt,
            dream: Arc::new(std::sync::Mutex::new(DreamEngine::new())),
            metacog: Arc::new(std::sync::Mutex::new(MetacogAgent::new())),
            dispatcher: None,
        }
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
            dream: Arc::new(std::sync::Mutex::new(DreamEngine::new())),
            metacog: Arc::new(std::sync::Mutex::new(MetacogAgent::new())),
            dispatcher: None,
        }
    }

    /// Bootstrap the identity kernel into the in-memory graph.
    pub async fn bootstrap_identity(&self) -> Result<(), String> {
        omega_phylactery::bootstrap_kernel(&self.memory_service).await
    }

    /// Set the dispatcher.
    pub fn set_dispatcher(&mut self, dispatcher: Arc<omega_conductor::dispatcher::Dispatcher>) {
        self.dispatcher = Some(dispatcher);
    }

    /// Inject all Neocortex seed programs into the MemoryGraph.
    ///
    /// This is Phase 7 of the boot sequence — runs after phylactery so Chyren
    /// has the full sovereign knowledge library loaded before processing any task.
    pub fn inject_neocortex(&self) {
        let dir_path = "knowledge_injection";
        if !std::path::Path::new(dir_path).exists() {
            let _ = std::fs::create_dir_all(dir_path);
            return;
        }

        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |e| e == "json") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(program) = serde_json::from_str::<MatrixProgram>(&content) {
                            let ms = self.memory_service.clone();
                            tokio::spawn(async move {
                                let mut graph = ms.lock().await;
                                let _ = ingestion::IngestionEngine::ingest(program, &mut graph).await;
                            });
                        }
                    }
                }
            }
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

    // ── Private escalation helpers ────────────────────────────────────────────

    /// Execute a single provider turn including the autonomous tool-call loop.
    /// Returns the final `SpokeResponse` after all tool rounds complete.
    ///
    /// `model_hint` is injected into the spoke input so provider models that support
    /// a `"model"` field (OpenRouter, Ollama) use it instead of their default.
    async fn run_spoke_turn(
        &self,
        provider: Option<&str>,
        model_hint: Option<&str>,
        initial_prompt: &str,
        system_prompt: &str,
        max_tokens: usize,
        temperature: f64,
    ) -> Result<SpokeResponse, ConductorError> {
        let mut current_prompt = initial_prompt.to_string();
        let mut response = SpokeResponse {
            text: String::new(),
            provider: provider.unwrap_or("unknown").to_string(),
            model: model_hint.unwrap_or("default").to_string(),
            token_count: 0,
            latency_ms: 0.0,
        };

        const MAX_TOOL_TURNS: usize = 5;
        let mut turn = 0;
        while turn < MAX_TOOL_TURNS {
            turn += 1;
            let request = SpokeRequest {
                prompt: current_prompt.clone(),
                system: system_prompt.to_string(),
                max_tokens,
                temperature,
            };

            let res = self
                .spokes
                .route_with_model(&request, provider, model_hint)
                .await
                .map_err(|e| ConductorError::ProviderError(e.to_string()))?;

            response = res.clone();

            if let Some(start) = res.text.find("<tool_call>") {
                if let Some(end) = res.text.find("</tool_call>") {
                    let call_json_str = &res.text[start + 11..end];
                    if let Ok(invocation) = serde_json::from_str::<ToolInvocation>(call_json_str) {
                        info!("[THE HANDS] Invoking tool {} on turn {}", invocation.tool, turn);
                        let is_sensitive = self
                            .spokes
                            .spokes_with_capability(omega_spokes::SpokeCapability::Sensitive)
                            .iter()
                            .any(|s| s.name() == invocation.tool);
                        if is_sensitive {
                            let alignment = self.aligner.check(&format!(
                                "INVOKE TOOL: {} with INPUT: {}",
                                invocation.tool, invocation.input
                            ));
                            if !alignment.passed {
                                warn!("[AEGIS] Blocked sensitive tool invocation: {}", alignment.note);
                                current_prompt.push_str(&format!(
                                    "\n\n{}\n\n<tool_error>AEGIS: Access denied to tool {}. Reason: {}</tool_error>\n",
                                    res.text, invocation.tool, alignment.note
                                ));
                                continue;
                            }
                        }
                        let mut tool_result_text =
                            format!("<tool_error>Tool {} not found</tool_error>", invocation.tool);
                        for spoke in self
                            .spokes
                            .spokes_with_capability(omega_spokes::SpokeCapability::Tools)
                        {
                            if let Ok(result) = spoke.invoke_tool(invocation.clone()).await {
                                tool_result_text = format!(
                                    "<tool_result>{}</tool_result>",
                                    serde_json::to_string(&result.output).unwrap_or_default()
                                );
                                break;
                            }
                        }
                        current_prompt
                            .push_str(&format!("\n\n{}\n\n{}\n", res.text, tool_result_text));
                        continue;
                    }
                }
            }
            break;
        }
        Ok(response)
    }

    /// Write a single escalation attempt to the Master Ledger (non-fatal).
    async fn ledger_log_attempt(
        &self,
        envelope: &RunEnvelope,
        resp: &SpokeResponse,
        v: &omega_adccl::adccl_logic::VerificationResult,
        tier_label: &str,
    ) {
        if let Some(ref store) = self.store {
            let status = format!(
                "{}_{}", tier_label,
                if v.passed { "passed" } else { "rejected" }
            );
            let _ = store
                .store_ledger_entry(
                    &envelope.run_id,
                    &envelope.task,
                    &resp.provider,
                    &resp.model,
                    &status,
                    &resp.text,
                    v.score as f64,
                    &v.flags,
                    resp.latency_ms,
                    resp.token_count as i32,
                    "witness_signed",
                )
                .await;
        }
    }

    /// Commit a `CRITICAL_EPISTEMIC_FAILURE` entry to the Master Ledger.
    /// The response_text field carries JSON-serialised metadata of all failed attempts
    /// for post-mortem analysis.
    async fn ledger_commit_terminal_failure(
        &self,
        envelope: &RunEnvelope,
        attempts: &[serde_json::Value],
    ) {
        if let Some(ref store) = self.store {
            let postmortem = serde_json::to_string_pretty(attempts)
                .unwrap_or_else(|_| "serialisation_error".to_string());
            let _ = store
                .store_ledger_entry(
                    &envelope.run_id,
                    &envelope.task,
                    "escalation_pipeline",
                    "all_tiers",
                    "CRITICAL_EPISTEMIC_FAILURE",
                    &postmortem,
                    0.0,
                    &["CRITICAL_EPISTEMIC_FAILURE".to_string()],
                    0.0,
                    0,
                    "terminal_failure",
                )
                .await;
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

        // Fetch relevant lessons from the Dream Engine (cognitive feedback loop)
        let mut final_system_prompt = self.sovereign_system_prompt.clone();
        if let Ok(dream) = self.dream.try_lock() {
            let stats = dream.get_statistics();
            if stats.total_episodes > 0 {
                final_system_prompt.push_str("\n\nLESSONS FROM PAST COGNITIVE DRIFT:\n");
                for (pattern, count) in &stats.most_common_failures {
                    final_system_prompt.push_str(&format!("- [{}] Found in {} cases. Mitigation: Ensure alignment with task objective and accuracy.\n", pattern, count));
                }
            }
        }

        Ok(TaskPlan {
            steps: vec![t.to_string()],
            system_prompt: final_system_prompt,
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

        // Hybrid Sovereign routing: if no explicit provider was requested, let the
        // ProviderRouter classify the task. High-sensitivity and routine tasks stay
        // local (Ollama); high-complexity formal/mathematical tasks go to OpenRouter.
        let routed_provider: Option<String> = preferred_provider
            .map(|p| p.to_string())
            .or_else(|| Some(ProviderRouter::route(&envelope.task).to_string()));
        let preferred_provider: Option<&str> = routed_provider.as_deref();

        info!(
            "[ROUTER] task routed to provider={}",
            preferred_provider.unwrap_or("(default)")
        );

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

        // --- Tool Discovery & System Prompt Injection ---
        let tools = self.spokes.discover_all_tools().await;
        let mut tool_system_prompt = plan.system_prompt.clone();
        if !tools.is_empty() {
            tool_system_prompt.push_str("\n\nAUTHENTICATED TOOLS AVAILABLE:\n");
            for t in &tools {
                tool_system_prompt.push_str(&format!("- {}: {}\n", t.name, t.description));
            }
            tool_system_prompt.push_str("\nTo invoke a tool, respond with exactly: <tool_call>{\"tool\": \"name\", \"input\": { ... }}</tool_call>\n");
        }

        let base_prompt = format!("{}{}", envelope.task, context_text);

        let mut spoke_response;
        let mut verification;

        // ── Tiered Epistemic Escalation (Cognitive Funnel) ───────────────────────
        //
        //  Tier 0  Initial     ProviderRouter::route()  — Ollama or OpenRouter per task
        //  Tier 1  Upshift     openrouter + claude-3.5-sonnet (clean re-attempt)
        //  Tier 2  Council     Multi-spoke consensus arbitration
        //  Terminal             Hard-stop → CRITICAL_EPISTEMIC_FAILURE ledger entry
        //
        // Each tier is logged individually to the Master Ledger so the full reasoning
        // chain is available for post-mortem analysis and local fine-tuning.

        // Accumulates one JSON object per attempt for the terminal failure entry.
        let mut attempt_log: Vec<serde_json::Value> = Vec::new();

        // ── TIER 0: Initial route ────────────────────────────────────────────────
        info!("[TIER-0] Executing via provider={}", preferred_provider.unwrap_or("default"));
        let t0_resp = self
            .run_spoke_turn(preferred_provider, None, &base_prompt, &tool_system_prompt, max_tokens, temperature)
            .await?;
        let t0_v = self.adccl.verify(&t0_resp.text, &envelope.task);
        omega_telemetry::CHYREN_ADCCL_SCORE.set(t0_v.score as f64);
        info!("[TIER-0] provider={} score={:.2} passed={}", t0_resp.provider, t0_v.score, t0_v.passed);
        self.ledger_log_attempt(envelope, &t0_resp, &t0_v, "tier_0_initial").await;
        attempt_log.push(serde_json::json!({
            "tier": "tier_0_initial",
            "provider": t0_resp.provider,
            "model": t0_resp.model,
            "score": t0_v.score,
            "passed": t0_v.passed,
            "flags": t0_v.flags,
            "response_preview": &t0_resp.text[..t0_resp.text.len().min(300)],
        }));
        spoke_response = t0_resp;
        verification = t0_v;

        // ── TIER 1: Upshift (if Tier 0 failed) ──────────────────────────────────
        if !verification.passed {
            let t1_provider = omega_conductor::router::ProviderRouter::upshift_provider();
            let t1_model = omega_conductor::router::ProviderRouter::upshift_model();
            info!(
                "[TIER-1] Upshifting to {}/{} (tier-0 score={:.2}, flags={:?})",
                t1_provider, t1_model, verification.score, verification.flags
            );

            match self
                .run_spoke_turn(Some(t1_provider), Some(&t1_model), &base_prompt, &tool_system_prompt, max_tokens, temperature)
                .await
            {
                Ok(t1_resp) => {
                    let t1_v = self.adccl.verify(&t1_resp.text, &envelope.task);
                    omega_telemetry::CHYREN_ADCCL_SCORE.set(t1_v.score as f64);
                    info!("[TIER-1] score={:.2} passed={}", t1_v.score, t1_v.passed);
                    self.ledger_log_attempt(envelope, &t1_resp, &t1_v, "tier_1_upshift").await;
                    attempt_log.push(serde_json::json!({
                        "tier": "tier_1_upshift",
                        "provider": t1_resp.provider,
                        "model": t1_model,
                        "score": t1_v.score,
                        "passed": t1_v.passed,
                        "flags": t1_v.flags,
                        "response_preview": &t1_resp.text[..t1_resp.text.len().min(300)],
                    }));
                    spoke_response = t1_resp;
                    verification = t1_v;
                }
                Err(e) => {
                    warn!("[TIER-1] Provider error: {}. Proceeding to Council.", e);
                }
            }
        }

        // ── TIER 2: Council arbitration (if Tier 1 still failed) ────────────────
        if !verification.passed {
            info!(
                "[TIER-2] Invoking multi-spoke Council (tier-1 score={:.2})",
                verification.score
            );
            match self.verify_via_council(&spoke_response.text, &envelope.task).await {
                Ok(council_v) => {
                    omega_telemetry::CHYREN_ADCCL_SCORE.set(council_v.score as f64);
                    info!("[TIER-2] Council score={:.2} passed={}", council_v.score, council_v.passed);
                    attempt_log.push(serde_json::json!({
                        "tier": "tier_2_council",
                        "provider": "multi_spoke_consensus",
                        "model": "council",
                        "score": council_v.score,
                        "passed": council_v.passed,
                        "flags": council_v.flags,
                    }));

                    if council_v.passed {
                        verification = council_v;
                    } else {
                        // ── TERMINAL FAILURE ─────────────────────────────────────
                        warn!(
                            "[TERMINAL] All {} escalation tiers failed. Committing CRITICAL_EPISTEMIC_FAILURE.",
                            attempt_log.len()
                        );
                        self.ledger_commit_terminal_failure(envelope, &attempt_log).await;
                        if let Ok(mut dream) = self.dream.try_lock() {
                            let report = Self::verification_report_from_adcccl(&envelope.run_id, &council_v);
                            let episode = dream.record_failure(&spoke_response.text, &report);
                            tracing::debug!("[DREAM] Terminal failure recorded: {} lessons derived", episode.lessons.len());
                        }
                        return Err(ConductorError::EpistemicFailure {
                            attempt_count: attempt_log.len(),
                        });
                    }
                }
                Err(e) => {
                    // Council unreachable — treat as terminal failure
                    warn!("[TERMINAL] Council unavailable ({}). Hard-stop.", e);
                    self.ledger_commit_terminal_failure(envelope, &attempt_log).await;
                    return Err(ConductorError::EpistemicFailure {
                        attempt_count: attempt_log.len(),
                    });
                }
            }
        } else if verification.passed {
            // Tier 0 or Tier 1 passed local ADCCL — run Council as final confirmation.
            if let Ok(council_v) = self.verify_via_council(&spoke_response.text, &envelope.task).await {
                omega_telemetry::CHYREN_ADCCL_SCORE.set(council_v.score as f64);
                if council_v.passed {
                    verification = council_v;
                } else {
                    warn!("[COUNCIL] Council overrode local ADCCL pass (score={:.2}). Accepting Council verdict.", council_v.score);
                    verification = council_v;
                }
            }
        }

        // ── Final ledger entry (overall task outcome) ────────────────────────────
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

        // ── Epistemic Mesh pass ───────────────────────────────────────────────
        // Run the Chiral Graph self-correction on every response. If ADCCL passed
        // but axioms are violated, the mesh refines the answer before ledger commit.
        // If ADCCL failed, the mesh writes the failed chain to the Logic-Cache.
        {
            use omega_conductor::epistemic::EpistemicMesh;
            use omega_neocortex::{cold_store::ColdStore, proof_index::ProofConstraintIndex, Neocortex};
            use tokio::sync::Mutex as TokioMutex;

            let neocortex = Arc::new(Neocortex::new());
            let cold_store = Arc::new(
                ColdStore::default_store()
                    .unwrap_or_else(|_| ColdStore::new("/tmp/chyren_cold").expect("cold store"))
            );
            let proof_index = Arc::new(TokioMutex::new(ProofConstraintIndex::new()));

            let mesh = EpistemicMesh::new(neocortex, cold_store, proof_index);
            let mesh_result = mesh.run(
                &envelope.task,
                vec![(spoke_response.provider.clone(), spoke_response.text.clone())],
            ).await;

            if mesh_result.converged && mesh_result.final_answer != spoke_response.text {
                omega_telemetry::info!(
                    "Conductor",
                    "MESH_REFINED",
                    "Mesh refined answer. violations={} logic_cache={} reviews={}",
                    mesh_result.axiom_violations_encountered,
                    mesh_result.logic_cache_entries_written,
                    mesh_result.sovereign_reviews_triggered
                );
                spoke_response.text = mesh_result.final_answer;
            } else {
                omega_telemetry::info!(
                    "Conductor",
                    "MESH_STABLE",
                    "nodes={} entropy={:.2} converged={} cache_writes={}",
                    mesh_result.graph_summary.total_nodes,
                    mesh_result.graph_summary.entropy,
                    mesh_result.converged,
                    mesh_result.logic_cache_entries_written
                );
            }
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
            let report = Self::verification_report_from_adcccl(&envelope.run_id, &verification);
            // If ADCCL failed, record it in the dream engine for pattern learning
            if let Ok(mut dream) = self.dream.try_lock() {
                let episode = dream.record_failure(&spoke_response.text, &report);
                tracing::debug!("[DREAM] Failure recorded: {} lessons derived", episode.lessons.len());
            }
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

    /// Run a metacognitive reflection pass over the current memory graph.
    ///
    /// Returns the insight strings from all epiphanies generated.
    pub async fn reflect(&self) -> Vec<String> {
        let graph = self.memory_service.lock().await;
        let mut mc = match self.metacog.try_lock() {
            Ok(mc) => mc,
            Err(_) => return vec![],
        };
        
        let mut insights: Vec<String> = mc.reflect(&graph).iter().map(|e| e.insight.clone()).collect();

        // Self-Healing Telemetry: Analyze Dream Engine for reasoning failures
        if let Ok(dream) = self.dream.try_lock() {
            let count = dream.episode_count();
            if count > 5 {
                insights.push(format!("HEURISTIC_ALERT: {} reasoning failures detected. Recommending Neocortex synchronization.", count));
                
                // Autonomous correction: if entropy is too high, recommend a flush
                if count > 20 {
                    insights.push("CRITICAL_RECOVERY: Reasoning entropy exceeded threshold. Initiating epistemic re-alignment.".into());
                }
            }
        }

        insights
    }

    /// The Council: Consensus-based LLM verification.
    /// Queries multiple models in parallel to score a response.
    pub async fn verify_via_council(&self, response_text: &str, task: &str) -> Result<VerificationResult, String> {
        info!("[COUNCIL] Convening session for consensus verification...");
        
        // Define the verification prompt
        let system_prompt = "You are an ADCCL Auditor for the Chyren Sovereign Intelligence system.\n\
            Your task is to grade an AI's response to a given user task.\n\
            Critically evaluate the response for accuracy, safety, and lack of stubs or hallucinations.\n\
            Return ONLY a JSON object with: {\"score\": float (0.0 to 1.0), \"flags\": [string], \"passed\": bool}.\n\
            The 'passed' threshold is 0.7.";
        
        let prompt = format!("USER TASK: {}\n\nAI RESPONSE: {}", task, response_text);
        
        // Prepare requests for the Council (OpenAI, Anthropic, Gemini, Ollama)
        let providers = vec!["openai", "anthropic", "gemini", "ollama"];
        let mut votes = Vec::new();

        let mut futures = Vec::new();
        for provider in providers {
            let request = SpokeRequest {
                prompt: prompt.clone(),
                system: system_prompt.to_string(),
                max_tokens: 256,
                temperature: 0.1,
            };
            let spokes = self.spokes.clone();
            let provider_name = provider.to_string();
            futures.push(async move {
                match spokes.route(&request, Some(&provider_name)).await {
                    Ok(res) => (provider_name, Some(res.text)),
                    Err(_) => (provider_name, None),
                }
            });
        }

        let results = futures::future::join_all(futures).await;

        for (provider, output) in results {
            if let Some(text) = output {
                // Try to parse JSON from the response
                let text_str: &str = &text;
                if let Some(start) = text_str.find('{') {
                    if let Some(end) = text.rfind('}') {
                        let json_str = &text[start..=end];
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
                            let score = v["score"].as_f64().unwrap_or(0.0) as f32;
                            let passed = v["passed"].as_bool().unwrap_or(score >= 0.7);
                            let flags = v["flags"].as_array().map(|a| {
                                a.iter().filter_map(|f| f.as_str().map(|s| s.to_string())).collect()
                            }).unwrap_or_else(Vec::new);
                            
                            votes.push((provider, score, passed, flags));
                        }
                    }
                }
            }
        }

        if votes.is_empty() {
            return Err("No council members available for verification".to_string());
        }

        // Consensus logic: 2-of-3 majority. If only 1 or 2 responded, require unanimity or 2/2.
        let pass_count = votes.iter().filter(|(_, _, passed, _)| *passed).count();
        let total_votes = votes.len();
        
        let consensus_passed = if total_votes >= 3 {
            pass_count >= 2
        } else if total_votes == 2 {
            pass_count >= 2
        } else {
            pass_count == 1
        };

        // Average score
        let avg_score = votes.iter().map(|(_, s, _, _)| *s).sum::<f32>() / total_votes as f32;
        
        // Aggregate flags
        let mut all_flags = HashSet::new();
        for (_, _, _, flags) in votes {
            for f in flags {
                all_flags.insert(f);
            }
        }

        info!("[COUNCIL] Consensus: {}/{} pass votes. Avg Score: {:.2}", pass_count, total_votes, avg_score);

        Ok(VerificationResult {
            passed: consensus_passed,
            score: avg_score,
            empathy_score: 1.0, // Default for consensus bypass or similar
            flags: all_flags.into_iter().collect(),
            status: if consensus_passed { "verified (consensus)".to_string() } else { "rejected (consensus)".to_string() },
        })
    }

    /// Return a snapshot of the system health for status reporting.
    pub async fn health_status(&self) -> HealthStatus {
        let qdrant_url = std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6333".to_string());
        let qdrant_ok = omega_myelin::VectorStore::new(&qdrant_url, "chyren_memory")
            .health_check()
            .await;

        let ledger_entry_count = {
            let path = std::path::Path::new("state/ledger.jsonl");
            if path.exists() {
                std::fs::read_to_string(path)
                    .ok()
                    .map(|s| s.lines().count())
            } else {
                None
            }
        };

        HealthStatus {
            conductor_ok: true,
            active_providers: vec![
                "anthropic".to_string(),
                "openai".to_string(),
            ],
            ledger_entry_count,
            qdrant_ok,
        }
    }

    /// Return the count of recorded dream failure episodes.
    /// Verify a response against a task using the ADCCL engine.
    pub async fn verify_text(&self, task: &str, text: &str) -> VerificationResult {
        self.adccl.verify(text, task)
    }

    /// Record a failed cognitive episode into the Dream Engine.
    pub async fn record_dream(&self, task: &str, text: &str, score: f64, flags: &[String]) {
        if let Ok(mut dream) = self.dream.try_lock() {
            let report = VerificationReport {
                report_id: format!("vr-ext-{}", now()),
                passed: false,
                flags: flags.to_vec(),
                score,
                evidence: vec![],
                repairs: vec!["External failure reported".to_string()],
            };
            dream.record_failure(text, &report);
        }
    }

    /// Return the total count of dream episodes recorded.
    pub fn dream_episode_count(&self) -> usize {
        self.dream
            .try_lock()
            .map(|d| d.episode_count())
            .unwrap_or(0)
    }

    /// Return the top failure pattern label and count, if any.
    pub fn dream_top_pattern(&self) -> Option<(String, usize)> {
        self.dream
            .try_lock()
            .ok()
            .and_then(|d| d.get_failure_patterns().into_iter().next())
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

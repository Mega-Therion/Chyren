//! Full task conductor: alignment admission → behavioral analysis → AEON lifecycle → provider routing → ADCCL verification.

use omega_adccl::adccl_logic::{VerificationResult, ADCCL};
use omega_dream::Service as DreamEngine;
use omega_neocortex;
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
    /// Dream engine for recording ADCCL failures and deriving lessons.
    dream: Arc<std::sync::Mutex<DreamEngine>>,
    /// Metacognitive agent for post-session self-reflection.
    metacog: Arc<std::sync::Mutex<MetacogAgent>>,
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
            dream: Arc::new(std::sync::Mutex::new(DreamEngine::new())),
            metacog: Arc::new(std::sync::Mutex::new(MetacogAgent::new())),
        }
    }

    /// Bootstrap the identity kernel into the in-memory graph.
    pub async fn bootstrap_identity(&self) -> Result<(), String> {
        omega_phylactery::bootstrap_kernel(&self.memory_service).await
    }

    /// Inject all Neocortex seed programs into the MemoryGraph.
    ///
    /// Phase 7 of the boot sequence — loads the sovereign ProgramLibrary (identity,
    /// philosophy, lineage, architecture, etc.) into the MemoryGraph as Canonical nodes.
    pub fn inject_neocortex(&self) {
        use omega_neocortex::{seed_library, Neocortex};
        let mut nc = Neocortex::new();
        nc.library = seed_library();
        match nc.ingest_all() {
            Ok(mind) => {
                info!(
                    "[NEOCORTEX] {} programs loaded: {}",
                    mind.load_report.loaded,
                    mind.load_report.domains_loaded.join(", ")
                );
                if mind.load_report.failed > 0 {
                    warn!("[NEOCORTEX] {} programs failed integrity gate", mind.load_report.failed);
                }
            }
            Err(e) => warn!("[NEOCORTEX] ingest_all failed: {e}"),
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

        // Query the Knowledge Matrix for relevant domain reasoning programs.
        // Uses the librarian spoke (HTTP MCP) when CHYREN_API_URL is configured.
        // Failures are silently skipped — the matrix is enhancement, not a gate.
        if let Some(librarian) = self.spokes.get_spoke("librarian") {
            match librarian.invoke_tool(ToolInvocation {
                tool: "librarian_knowledge_search".to_string(),
                input: serde_json::json!({
                    "query": t,
                    "max_results": 4
                }),
            }).await {
                Ok(result) if result.success => {
                    if let Some(domains) = result.output
                        .get("content")
                        .and_then(|c| c.get(0))
                        .and_then(|o| o.get("text"))
                        .and_then(|t| t.as_str())
                        .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                        .as_ref()
                        .and_then(|v| v.get("domains"))
                        .and_then(|d| d.as_array())
                    {
                        let primers: Vec<String> = domains.iter()
                            .filter_map(|d| {
                                let name = d.get("name")?.as_str()?;
                                let mode = d.get("reasoning_mode")?.as_str()?;
                                let primer = d.get("reasoning_primer")?.as_str()?;
                                Some(format!("[{} — {}] {}", name, mode, primer))
                            })
                            .collect();
                        if !primers.is_empty() {
                            final_system_prompt.push_str("\n\nACTIVE DOMAIN REASONING PROGRAMS:\n");
                            final_system_prompt.push_str(&primers.join("\n"));
                        }
                    }
                }
                _ => {}
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

        let mut current_prompt = format!("{}{}", envelope.task, context_text);
        let mut spoke_response = SpokeResponse {
            text: String::new(),
            provider: String::new(),
            model: String::new(),
            token_count: 0,
            latency_ms: 0.0,
        };


        let mut verification = VerificationResult {
            passed: false,
            score: 0.0,
            flags: vec!["initial_execution".to_string()],
            status: "initial".to_string(),
        };

        // --- Autonomic Self-Repair Loop ---
        let mut repair_turn = 0;
        const MAX_REPAIRS: usize = 3;

        while repair_turn < MAX_REPAIRS {
            repair_turn += 1;
            if repair_turn > 1 {
                info!("[REPAIR] Starting turn {} to resolve drift: {:?}", repair_turn, verification.flags);
            }

            // --- Autonomous Tool Execution Loop (The Hands) ---
            let mut turn = 0;
            const MAX_TOOL_TURNS: usize = 5;

            while turn < MAX_TOOL_TURNS {
                turn += 1;
                let request = SpokeRequest {
                    prompt: current_prompt.clone(),
                    system: tool_system_prompt.clone(),
                    max_tokens,
                    temperature,
                };

                let res = self
                    .spokes
                    .route(&request, preferred_provider)
                    .await
                    .map_err(|e| ConductorError::ProviderError(e.to_string()))?;

                spoke_response = res.clone();
                
                // Look for tool calls
                if let Some(start) = res.text.find("<tool_call>") {
                    if let Some(end) = res.text.find("</tool_call>") {
                        let call_json_str = &res.text[start + 11..end];
                        if let Ok(invocation) = serde_json::from_str::<ToolInvocation>(call_json_str) {
                            info!("[THE HANDS] Invoking tool {} on turn {}", invocation.tool, turn);
                            
                            // --- Tool Permission Check ---
                            let is_sensitive = self.spokes.spokes_with_capability(omega_spokes::SpokeCapability::Sensitive)
                                .iter().any(|s| s.name() == invocation.tool); // Simplified: check if tool name matches a sensitive spoke or check its metadata
                            
                            if is_sensitive {
                                // Run a quick AEGIS admission check on tool intent
                                let alignment = self.aligner.check(&format!("INVOKE TOOL: {} with INPUT: {}", invocation.tool, invocation.input));
                                if !alignment.passed {
                                    warn!("[AEGIS] Blocked sensitive tool invocation: {}", alignment.note);
                                    current_prompt.push_str(&format!("\n\n{}\n\n<tool_error>AEGIS: Access denied to tool {}. Reason: {}</tool_error>\n", res.text, invocation.tool, alignment.note));
                                    continue;
                                }
                            }

                            // Execute tool
                            let mut tool_result_text = format!("<tool_error>Tool {} not found</tool_error>", invocation.tool);
                            for spoke in self.spokes.spokes_with_capability(omega_spokes::SpokeCapability::Tools) {
                                 if let Ok(result) = spoke.invoke_tool(invocation.clone()).await {
                                    tool_result_text = format!("<tool_result>{}</tool_result>", serde_json::to_string(&result.output).unwrap_or_default());
                                    break;
                                 }
                            }

                            current_prompt.push_str(&format!("\n\n{}\n\n{}\n", res.text, tool_result_text));
                            continue;
                        }
                        }
                    }
                }
                break;
            }

            // --- Verification Gate (The Council) ---
            let local_v = self.adccl.verify(&spoke_response.text, &envelope.task);
            verification = if local_v.passed {
                match self.verify_via_council(&spoke_response.text, &envelope.task).await {
                    Ok(council_v) => {
                        omega_telemetry::CHYREN_ADCCL_SCORE.set(council_v.score as f64);
                        council_v
                    }
                    Err(e) => {
                        warn!("Council failed: {}. Falling back.", e);
                        local_v
                    }
                }
            } else {
                local_v
            };

            if verification.passed {
                break; // Exit repair loop if passed
            } else {
                // Prepend repair feedback for next turn
                info!("[REPAIR] Validation failed (score: {}). Flags: {:?}", verification.score, verification.flags);
                current_prompt.push_str(&format!("\n\nFEEDBACK FROM VERIFIER:\nYour previous response was REJECTED with score {}. Issues found: {:?}.\nPlease REPAIR the response ensuring clarity, accuracy, and adherence to system principles.", verification.score, verification.flags));
                // Inject the rejected text into history so it can be corrected
                current_prompt.push_str(&format!("\n\nPREVIOUS RESPONSE:\n{}\n", spoke_response.text));
            }
        }

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
            let report = Self::verification_report_from_adcccl(&envelope.run_id, &verification);
            // If ADCCL failed, record it in the dream engine for pattern learning
            if let Ok(mut dream) = self.dream.try_lock() {
                let episode = dream.record_failure(&spoke_response.text, &report);
                eprintln!("[DREAM] Failure recorded: {} lessons derived", episode.lessons.len());
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
        let epiphanies = mc.reflect(&graph);
        epiphanies.iter().map(|e| e.insight.clone()).collect()
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
                        }            }
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
            flags: all_flags.into_iter().collect(),
            status: if consensus_passed { "verified (consensus)".to_string() } else { "rejected (consensus)".to_string() },
        })
    }

    /// Return the count of recorded dream failure episodes.
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

    /// Run the Knowledge Memory Dream cycle.
    ///
    /// Queries the catalog for sealed and millennium-target domains,
    /// loads their axioms as Neocortex programs, and records millennium
    /// targets as active proof-attempt dream episodes.
    ///
    /// Returns a report of what was loaded and what was registered.
    pub async fn run_knowledge_dream(&self) -> KnowledgeDreamReport {
        use omega_neocortex::{Domain, Neocortex, Program};
        let mut report = KnowledgeDreamReport::default();

        let librarian = match self.spokes.get_spoke("librarian") {
            Some(s) => s,
            None => {
                report.error = Some("Librarian spoke not registered — is CHYREN_API_URL set?".into());
                return report;
            }
        };

        // ── Fetch sealed domains ──────────────────────────────────────────────
        let sealed_result = librarian.invoke_tool(ToolInvocation {
            tool: "librarian_get_sealed_domains".to_string(),
            input: serde_json::json!({}),
        }).await;

        let mut nc = Neocortex::new();
        nc.library = omega_neocortex::seed_library();

        if let Ok(res) = sealed_result {
            if let Some(domains) = res.output
                .get("content").and_then(|c| c.get(0))
                .and_then(|o| o.get("text")).and_then(|t| t.as_str())
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                .as_ref().and_then(|v| v.get("domains")).and_then(|d| d.as_array())
            {
                for domain in domains {
                    let slug = domain.get("slug").and_then(|s| s.as_str()).unwrap_or("");
                    let name = domain.get("name").and_then(|n| n.as_str()).unwrap_or(slug);
                    let anchor = domain.get("formal_anchor").and_then(|a| a.as_str()).unwrap_or("unknown");
                    let axioms = domain.get("core_axioms").cloned().unwrap_or(serde_json::json!([]));
                    let primer = domain.get("reasoning_primer").and_then(|p| p.as_str()).unwrap_or("");

                    let payload = serde_json::json!({
                        "slug": slug,
                        "name": name,
                        "formal_anchor": anchor,
                        "core_axioms": axioms,
                        "reasoning_primer": primer,
                        "status": "sealed",
                    });

                    match Program::from_knowledge(
                        Domain::Custom(slug.to_string()),
                        "dream-1.0",
                        &payload,
                        format!("Sealed knowledge: {} [{}]", name, anchor),
                        0.95,
                    ) {
                        Ok(program) => {
                            nc.library.register(program);
                            report.sealed_loaded.push(slug.to_string());
                        }
                        Err(e) => {
                            warn!("[DREAM] Failed to create program for sealed domain '{slug}': {e}");
                        }
                    }
                }
            }
        }

        // ── Ingest into MemoryGraph ───────────────────────────────────────────
        match nc.ingest_all() {
            Ok(mind) => {
                report.programs_ingested = mind.load_report.loaded;
                info!("[DREAM] {} programs ingested into neocortex", mind.load_report.loaded);
            }
            Err(e) => warn!("[DREAM] Neocortex ingest failed: {e}"),
        }

        // ── Fetch millennium targets ──────────────────────────────────────────
        let millennium_result = librarian.invoke_tool(ToolInvocation {
            tool: "librarian_get_millennium_targets".to_string(),
            input: serde_json::json!({}),
        }).await;

        if let Ok(res) = millennium_result {
            if let Some(domains) = res.output
                .get("content").and_then(|c| c.get(0))
                .and_then(|o| o.get("text")).and_then(|t| t.as_str())
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                .as_ref().and_then(|v| v.get("domains")).and_then(|d| d.as_array())
            {
                if let Ok(mut dream) = self.dream.try_lock() {
                    for domain in domains {
                        let slug = domain.get("slug").and_then(|s| s.as_str()).unwrap_or("");
                        let name = domain.get("name").and_then(|n| n.as_str()).unwrap_or(slug);
                        let desc = domain.get("description").and_then(|d| d.as_str()).unwrap_or("");

                        // Register as a proof-attempt episode — "dreaming" of solving the problem.
                        let episode = omega_dream::DreamEpisode {
                            episode_id: format!("millennium-{slug}"),
                            failed_response: format!("MILLENNIUM TARGET: {name}"),
                            failure_report: desc.to_string(),
                            corrected_response: None,
                            lessons: vec![
                                format!("Active proof target: {name}"),
                                "Formal verification required before status advances to 'formalized'".into(),
                                "Cross-reference with Mathlib4 and automated theorem provers".into(),
                            ],
                            timestamp: omega_spokes::now(),
                        };
                        dream.episodes.push(episode);
                        report.millennium_registered.push(slug.to_string());
                    }
                }
            }
        }

        report.timestamp = omega_spokes::now();
        report
    }
}

/// Report from a Knowledge Memory Dream cycle run.
#[derive(Debug, Default, serde::Serialize)]
pub struct KnowledgeDreamReport {
    /// Sealed domain slugs successfully loaded as Neocortex programs.
    pub sealed_loaded: Vec<String>,
    /// Total programs ingested into the Neocortex (seed + sealed).
    pub programs_ingested: usize,
    /// Millennium prize problem slugs registered as proof-attempt episodes.
    pub millennium_registered: Vec<String>,
    /// Error message if the cycle could not complete.
    pub error: Option<String>,
    /// Unix timestamp of the cycle.
    pub timestamp: f64,
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

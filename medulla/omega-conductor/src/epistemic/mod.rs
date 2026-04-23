//! Epistemic Mesh — Recursive Self-Correcting Reasoning System.
//!
//! Replaces the single Monitor pattern with a Chiral Graph where spokes
//! validate each other. The EpistemicMesh orchestrates:
//!
//!   1. Primary collection  — ask N spokes for independent answers
//!   2. Adversarial critique — EpistemicSpoke critiques each Primary
//!   3. Refinement forking   — violated axioms trigger recursive sub-tasks
//!   4. Entropy monitoring   — high divergence triggers Sovereign Axiom Review
//!   5. Logic-Cache write    — failed chains absorbed as negative examples
//!   6. Convergence          — clean leaf nodes declare final answer
//!
//! Max recursion depth: 5 (prevents infinite refinement loops).

pub mod graph;

use graph::{ChiralGraph, GraphSummary};
use omega_core::{
    check_all_axioms, gen_id,
    EpistemicNode, EpistemicNodeType, KnowledgeNode, ProofConstraint,
};
use omega_neocortex::{cold_store::ColdStore, proof_index::ProofConstraintIndex, Neocortex};
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_REFINEMENT_DEPTH: usize = 5;
#[allow(dead_code)]
const ENTROPY_ANCHOR_THRESHOLD: f32 = 0.5;

/// Result of a full Epistemic Mesh run.
pub struct MeshResult {
    /// The final converged answer.
    pub final_answer: String,
    /// Summary of the chiral graph state.
    pub graph_summary: GraphSummary,
    /// Total axiom violations encountered during reasoning.
    pub axiom_violations_encountered: usize,
    /// Number of logic-cache entries written.
    pub logic_cache_entries_written: usize,
    /// Number of sovereign axiom reviews triggered.
    pub sovereign_reviews_triggered: usize,
    /// Whether the mesh converged.
    pub converged: bool,
}

/// The Epistemic Mesh orchestrator.
pub struct EpistemicMesh {
    #[allow(dead_code)]
    neocortex: Arc<Neocortex>,
    cold_store: Arc<ColdStore>,
    proof_index: Arc<Mutex<ProofConstraintIndex>>,
    http: reqwest::Client,
}

impl EpistemicMesh {
    /// Create a new EpistemicMesh with access to the Neocortex, ColdStore,
    /// and ProofConstraintIndex subsystems.
    pub fn new(
        neocortex: Arc<Neocortex>,
        cold_store: Arc<ColdStore>,
        proof_index: Arc<Mutex<ProofConstraintIndex>>,
    ) -> Self {
        Self {
            neocortex,
            cold_store,
            proof_index,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Run the full epistemic mesh for a task.
    /// `primary_answers` = pre-collected answers from N spokes.
    pub async fn run(
        &self,
        task: &str,
        primary_answers: Vec<(String, String)>, // (spoke_name, answer)
    ) -> MeshResult {
        let mut graph = ChiralGraph::new();
        let mut axiom_violations_encountered = 0;
        let mut logic_cache_entries_written = 0;
        let mut sovereign_reviews_triggered = 0;

        // Phase 1: Add all primary answers as Primary nodes
        let mut primary_ids = Vec::new();
        for (spoke_name, answer) in &primary_answers {
            let mut node = EpistemicNode::new_primary(answer.clone(), spoke_name);
            node.axiom_violations = check_all_axioms(answer);
            axiom_violations_encountered += node.axiom_violations.iter()
                .filter(|v| !v.satisfied).count();
            let id = graph.add_node(node);
            primary_ids.push(id);
        }

        // Phase 2: Adversarial critique of each Primary
        for primary_id in &primary_ids {
            if let Some(primary_node) = graph.nodes.get(primary_id).cloned() {
                let critique_text = self.generate_critique(task, &primary_node.content).await;
                let mut critique = EpistemicNode::new_critique(critique_text, primary_id);
                critique.axiom_violations = check_all_axioms(&primary_node.content);
                critique.proof_obligations = primary_node.axiom_violations.iter()
                    .filter(|v| !v.satisfied)
                    .map(|v| v.lean4_proof_obligation.clone())
                    .collect();
                graph.add_critique(critique, primary_id);
            }
        }

        // Phase 3: Recursive refinement
        let mut depth = 0;
        while !graph.is_converged() && depth < MAX_REFINEMENT_DEPTH {
            let entropy = graph.entropy();
            omega_telemetry::info!(
                "EpistemicMesh", 
                "MESH_ITERATION", 
                "depth={depth} entropy={:.2} violated={}",
                entropy.value, 
                entropy.violated_axiom_count
            );

            // Trigger Sovereign Axiom Review if entropy is critical
            if entropy.requires_axiom_review() {
                omega_telemetry::warn!("EpistemicMesh", "ENTROPY_CRITICAL", "Entropy critical — triggering Sovereign Axiom Review");
                sovereign_reviews_triggered += 1;
                let phylactery_summary = self.get_phylactery_anchor().await;
                graph.inject_axiom_anchor(&phylactery_summary);
            }

            // Refine each dirty leaf node
            let dirty_leaves: Vec<String> = graph.leaf_node_ids()
                .into_iter()
                .filter(|id| {
                    graph.nodes.get(id)
                        .map(|n| !n.is_clean() && n.node_type != EpistemicNodeType::AxiomAnchor)
                        .unwrap_or(false)
                })
                .collect();

            if dirty_leaves.is_empty() {
                break;
            }

            for leaf_id in dirty_leaves {
                let leaf = match graph.nodes.get(&leaf_id).cloned() {
                    Some(n) => n,
                    None => continue,
                };

                // Build correction prompt from axiom violations
                let violations: Vec<String> = leaf.axiom_violations.iter()
                    .filter(|v| !v.satisfied)
                    .map(|v| format!(
                        "Axiom {} violated: {}\nLean 4 obligation:\n{}",
                        v.axiom_name,
                        v.violation.as_deref().unwrap_or("unknown"),
                        v.lean4_proof_obligation
                    ))
                    .collect();

                let correction_prompt = format!(
                    "Task: {task}\n\nPrevious answer:\n{}\n\nAxiom violations to fix:\n{}\n\n\
                     Rewrite the answer to satisfy all violations. Be precise and grounded.",
                    leaf.content,
                    violations.join("\n---\n")
                );

                let refined = self.call_llm(&correction_prompt).await;
                let mut refined_node = EpistemicNode::new_refinement(
                    refined,
                    &leaf_id,
                    &leaf.source_spoke,
                );
                refined_node.axiom_violations = check_all_axioms(&refined_node.content);

                // Write failed chain to Logic-Cache before replacing it
                if !leaf.is_clean() {
                    self.write_logic_cache_entry(&leaf, task).await;
                    logic_cache_entries_written += 1;
                }

                graph.fork_refinement(refined_node.content.clone(), &leaf_id, &leaf.source_spoke);
            }

            depth += 1;
        }

        let summary = graph.summary();
        let final_answer = graph.best_answer()
            .map(|n| n.content.clone())
            .unwrap_or_else(|| {
                primary_answers.first()
                    .map(|(_, a)| a.clone())
                    .unwrap_or_else(|| "No answer produced.".to_string())
            });

        omega_telemetry::info!(
            "EpistemicMesh",
            "MESH_COMPLETE",
            "nodes={} depth={} entropy={:.2} converged={}",
            summary.total_nodes, summary.depth, summary.entropy, summary.converged
        );

        MeshResult {
            final_answer,
            graph_summary: summary,
            axiom_violations_encountered,
            logic_cache_entries_written,
            sovereign_reviews_triggered,
            converged: graph.is_converged(),
        }
    }

    /// Ask the EpistemicSpoke (adversarial LLM prompt) to critique a reasoning chain.
    async fn generate_critique(&self, task: &str, answer: &str) -> String {
        let prompt = format!(
            "You are Chyren's Epistemic Adversary. Your role is to find logical errors, \
             unsupported claims, internal contradictions, and sovereignty violations \
             in the following reasoning chain.\n\n\
             Task: {task}\n\n\
             Reasoning to critique:\n{answer}\n\n\
             Provide a structured critique identifying:\n\
             1. Any factual errors or unsupported claims\n\
             2. Internal contradictions\n\
             3. Missing logical steps\n\
             4. Which of these axioms (NonDeception, Coherence, Grounding, Completeness, Sovereignty) are violated\n\
             5. The exact correction needed for each violation\n\n\
             Be adversarial and rigorous. If the reasoning is sound, say so explicitly."
        );
        self.call_llm(&prompt).await
    }

    /// Fetch a Phylactery re-anchor summary for the AxiomAnchor node.
    async fn get_phylactery_anchor(&self) -> String {
        // In production this would query the phylactery kernel.
        // For now return the sovereign axiom definitions.
        let axioms = omega_core::sovereign_axioms();
        let defs: Vec<String> = axioms.iter()
            .map(|a| format!("L{}: {}\n{}", a.level(), a.name(), a.lean4_definition()))
            .collect();
        format!(
            "Sovereign Axiom Re-anchor (R.W.Ϝ.Y.)\n\
             ═══════════════════════════════════\n{}",
            defs.join("\n\n")
        )
    }

    /// Write a failed reasoning chain to the Neocortex Logic-Cache.
    /// This teaches future tasks to avoid the same class of errors.
    async fn write_logic_cache_entry(&self, failed_node: &EpistemicNode, task: &str) {
        let violations: Vec<String> = failed_node.axiom_violations.iter()
            .filter(|v| !v.satisfied)
            .map(|v| format!("{}:{}", v.axiom_name, v.violation.as_deref().unwrap_or("")))
            .collect();

        let error_class = violations.join("|");
        let lean_proof = format!(
            "-- Logic-Cache entry (negative example)\n\
             -- Task: {}\n\
             -- Error class: {}\n\
             -- Failed reasoning:\n{}\n\
             theorem logic_cache_entry_{} : False := by sorry",
            task,
            error_class,
            failed_node.content,
            gen_id("lc")
        );

        let constraints: Vec<ProofConstraint> = failed_node.axiom_violations.iter()
            .filter(|v| !v.satisfied)
            .map(|v| ProofConstraint {
                id: gen_id("pc"),
                predicate: format!("LogicCacheError({})", v.axiom_name),
                domain: "logic_cache".to_string(),
                depends_on: vec![],
            })
            .collect();

        if constraints.is_empty() {
            return;
        }

        let node = KnowledgeNode::new(
            lean_proof,
            format!("Logic-Cache: {} — error class: {}", task, error_class),
            constraints,
            None,
        );

        if let Ok(hash) = self.cold_store.store(&node).await {
            let mut idx = self.proof_index.lock().await;
            idx.insert(&hash, &node.constraints);
            omega_telemetry::info!("EpistemicMesh", "LOGIC_CACHE_WRITE", "Logic-Cache entry written: {hash}");
        }
    }

    /// Call the best available LLM (Anthropic → Gemini → Groq fallback).
    async fn call_llm(&self, prompt: &str) -> String {
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if let Ok(r) = self.call_anthropic(&key, prompt).await {
                return r;
            }
        }
        if let Ok(key) = std::env::var("GEMINI_API_KEY") {
            if let Ok(r) = self.call_gemini(&key, prompt).await {
                return r;
            }
        }
        if let Ok(key) = std::env::var("GROQ_API_KEY") {
            if let Ok(r) = self.call_groq(&key, prompt).await {
                return r;
            }
        }
        if let Ok(key) = std::env::var("OPENROUTER_API_KEY") {
            if let Ok(r) = self.call_openrouter(&key, prompt).await {
                return r;
            }
        }
        // Final fallback: local Ollama (qwen2.5-coder:7b)
        if let Ok(r) = self.call_ollama(prompt).await {
            return r;
        }
        "[EpistemicMesh: no LLM provider available]".to_string()
    }

    #[allow(dead_code)]
    async fn call_openllm(&self, prompt: &str) -> Result<String, String> {
        let base = std::env::var("OPENLLM_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000/v1".to_string());
        let model = std::env::var("OPENLLM_MODEL")
            .unwrap_or_else(|_| "mistralai/Mistral-7B-Instruct-v0.3".to_string());
        let body = serde_json::json!({
            "model": model,
            "max_tokens": 2048,
            "temperature": 0.3,
            "messages": [{"role": "user", "content": prompt}]
        });
        let resp = self.http
            .post(format!("{base}/chat/completions"))
            .json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() { return Err(format!("OpenLLM {}", resp.status())); }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["choices"].as_array()
            .and_then(|a| a.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }

    async fn call_anthropic(&self, api_key: &str, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": "claude-opus-4-7",
            "max_tokens": 2048,
            "messages": [{"role": "user", "content": prompt}]
        });
        let resp = self.http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("Anthropic {}", resp.status()));
        }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["content"].as_array()
            .and_then(|a| a.first())
            .and_then(|b| b["text"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }

    async fn call_gemini(&self, api_key: &str, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "contents": [{"parts": [{"text": prompt}]}],
            "generationConfig": {"maxOutputTokens": 2048, "temperature": 0.3}
        });
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={api_key}"
        );
        let resp = self.http.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("Gemini {}", resp.status()));
        }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["candidates"].as_array()
            .and_then(|a| a.first())
            .and_then(|c| c["content"]["parts"].as_array())
            .and_then(|p| p.first())
            .and_then(|t| t["text"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }

    async fn call_openrouter(&self, api_key: &str, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": "meta-llama/llama-3.3-70b-instruct:free",
            "max_tokens": 2048,
            "temperature": 0.3,
            "messages": [{"role": "user", "content": prompt}]
        });
        let resp = self.http
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {api_key}"))
            .header("HTTP-Referer", "https://chyren.ai")
            .header("X-Title", "Chyren Sovereign Intelligence")
            .json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() { return Err(format!("OpenRouter {}", resp.status())); }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["choices"].as_array()
            .and_then(|a| a.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }

    async fn call_ollama(&self, prompt: &str) -> Result<String, String> {
        let base = std::env::var("OLLAMA_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:11434/v1".to_string());
        let model = std::env::var("OLLAMA_MODEL")
            .unwrap_or_else(|_| "mistral-nemo:latest".to_string());
        let body = serde_json::json!({
            "model": model,
            "max_tokens": 2048,
            "temperature": 0.3,
            "messages": [{"role": "user", "content": prompt}]
        });
        let resp = self.http
            .post(format!("{base}/chat/completions"))
            .json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() { return Err(format!("Ollama {}", resp.status())); }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["choices"].as_array()
            .and_then(|a| a.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }

    async fn call_groq(&self, api_key: &str, prompt: &str) -> Result<String, String> {
        let model = std::env::var("GROQ_MODEL")
            .unwrap_or_else(|_| "llama-3.3-70b-versatile".to_string());
        let body = serde_json::json!({
            "model": model,
            "max_tokens": 2048,
            "temperature": 0.3,
            "messages": [{"role": "user", "content": prompt}]
        });
        let resp = self.http
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&body).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("Groq {}", resp.status()));
        }
        let j: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        j["choices"].as_array()
            .and_then(|a| a.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "empty".to_string())
    }
}

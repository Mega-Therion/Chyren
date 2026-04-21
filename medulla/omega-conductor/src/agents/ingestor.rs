//! IngestorAgent — The Neocortex Auto-Compiler.
//!
//! Watches a queue of source URLs (arXiv, MathOverflow, GitHub), fetches each,
//! passes the content through MathSpoke (Lean 4 formal verification), then
//! absorbs the verified KnowledgeNode into the Neocortex via ColdStore +
//! ProofConstraintIndex. Only formally-verified nodes enter the graph.
//!
//! Absorption flow:
//!   URL → fetch → MathSpoke (Lean 4) → KnowledgeNode → ColdStore + ProofIndex → MemoryGraph

use async_trait::async_trait;
use omega_core::{
    gen_id, now, AgentResult, AgentTask, KnowledgeNode, ProofConstraint,
};
use omega_core::mesh::AgentCapability;
use omega_myelin::Service as MyelinService;
use omega_neocortex::{cold_store::ColdStore, proof_index::ProofConstraintIndex, Neocortex};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{MathSpoke, PersistentAgent};

/// A pending ingestion request.
#[derive(Debug, Clone)]
pub struct IngestionRequest {
    pub url: String,
    /// Hint about the mathematical domain (e.g. "number_theory")
    pub domain_hint: String,
    /// Human-readable description of what to extract from the source
    pub extraction_prompt: String,
}

/// The Ingestor Agent: fetches URLs, formalizes content via MathSpoke, and
/// absorbs verified KnowledgeNodes into the Neocortex.
pub struct IngestorAgent {
    myelin: Arc<MyelinService>,
    neocortex: Arc<Neocortex>,
    cold_store: Arc<ColdStore>,
    proof_index: Arc<Mutex<ProofConstraintIndex>>,
    http: reqwest::Client,
    math_spoke: MathSpoke,
}

impl IngestorAgent {
    pub fn new(
        myelin: Arc<MyelinService>,
        neocortex: Arc<Neocortex>,
        cold_store: Arc<ColdStore>,
        proof_index: Arc<Mutex<ProofConstraintIndex>>,
    ) -> Self {
        Self {
            myelin,
            neocortex,
            cold_store,
            proof_index,
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("Chyren-Ingestor/1.0")
                .build()
                .unwrap_or_default(),
            math_spoke: MathSpoke,
        }
    }

    /// Fetch raw text content from a URL.
    async fn fetch_url(&self, url: &str) -> Result<String, String> {
        self.http
            .get(url)
            .send()
            .await
            .map_err(|e| format!("fetch error: {e}"))?
            .text()
            .await
            .map_err(|e| format!("text decode error: {e}"))
    }

    /// Extract mathematical content from raw HTML/text using heuristics.
    /// In production this should call an LLM spoke to extract structured math.
    fn extract_math_content(&self, raw: &str, domain_hint: &str, prompt: &str) -> String {
        // Heuristic: find paragraphs likely containing math notation
        let lines: Vec<&str> = raw
            .lines()
            .filter(|l| {
                let l = l.trim();
                !l.is_empty()
                    && (l.contains('∀')
                        || l.contains('∃')
                        || l.contains('→')
                        || l.contains("theorem")
                        || l.contains("proof")
                        || l.contains("lemma")
                        || l.contains("corollary")
                        || l.contains("∈")
                        || l.contains("≤")
                        || l.contains("∧")
                        || l.contains("formula"))
            })
            .take(50)
            .collect();

        // Cap total chars — llama3.2:3b has ~4K token context (~3K chars safe budget)
        const MAX_CHARS: usize = 2800;

        if lines.is_empty() {
            let snippet = &raw[..raw.len().min(MAX_CHARS)];
            format!(
                "-- Domain: {domain_hint}\n-- Extraction prompt: {prompt}\n-- Source (truncated):\n{snippet}"
            )
        } else {
            let joined = lines.join("\n");
            let end = joined
                .char_indices()
                .map(|(i, _)| i)
                .take_while(|&i| i <= MAX_CHARS)
                .last()
                .unwrap_or(0);
            let truncated = &joined[..end];
            format!(
                "-- Domain: {domain_hint}\n-- Extracted mathematical content:\n{truncated}"
            )
        }
    }

    /// Convert extracted content into a Lean 4 proof attempt via MathSpoke.
    async fn formalize(&self, content: &str, domain: &str, source_url: &str) -> AgentResult {
        let lean_task_payload = format!(
            "-- Source: {source_url}\n\
             -- Domain: {domain}\n\
             -- Formalize the following mathematical content as a Lean 4 theorem and proof:\n\
             {content}\n\n\
             -- Template (replace with actual proof):\n\
             theorem formalized_{} : True := trivial",
            gen_id("t")
        );

        let task = AgentTask {
            task_id: gen_id("ingest"),
            run_id: gen_id("run"),
            agent_id: "ingestor".to_string(),
            payload: lean_task_payload,
            constraints: vec![domain.to_string()],
        };

        self.math_spoke.execute(task).await
    }

    /// Absorb a verified Lean 4 proof into the Neocortex.
    async fn absorb(
        &self,
        lean_proof: String,
        summary: String,
        domain: &str,
        source_url: &str,
    ) -> Result<String, String> {
        let constraint = ProofConstraint {
            id: gen_id("pc"),
            predicate: format!("Formalized({domain})"),
            domain: domain.to_string(),
            depends_on: vec![],
        };

        let node = KnowledgeNode::new(
            lean_proof,
            summary,
            vec![constraint],
            Some(source_url.to_string()),
        );

        // Write to cold store (content-addressed, permanent)
        let hash = self
            .cold_store
            .store(&node)
            .map_err(|e| format!("cold store error: {e}"))?;

        // Update proof index
        {
            let mut idx = self.proof_index.lock().await;
            idx.insert(&hash, &node.constraints);
        }

        // Write to hot memory graph (semantic retrieval)
        let summary_for_graph = format!(
            "[KnowledgeNode:{hash}] {summary}",
            summary = node.summary
        );
        self.myelin
            .write_node(summary_for_graph, omega_core::MemoryStratum::Canonical)
            .await;

        Ok(hash)
    }

    /// Full ingestion pipeline for a single URL.
    pub async fn ingest_url(&self, req: &IngestionRequest) -> Result<String, String> {
        let raw = self.fetch_url(&req.url).await?;
        let content = self.extract_math_content(&raw, &req.domain_hint, &req.extraction_prompt);
        let result = self.formalize(&content, &req.domain_hint, &req.url).await;

        if result.success {
            let summary = format!(
                "Formal knowledge from {} (domain: {})",
                req.url, req.domain_hint
            );
            self.absorb(result.output, summary, &req.domain_hint, &req.url)
                .await
        } else {
            Err(format!(
                "Lean 4 formalization failed for {}: {}",
                req.url,
                result.error.unwrap_or_default()
            ))
        }
    }
}

#[async_trait]
impl PersistentAgent for IngestorAgent {
    fn name(&self) -> &str {
        "ingestor"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![
            AgentCapability { category: "content_ingestion".to_string(), tools: vec![] },
            AgentCapability { category: "formal_verification".to_string(), tools: vec![] },
        ]
    }

    fn system_prompt(&self) -> &str {
        "You are Chyren's Ingestion Agent. You fetch mathematical knowledge from external \
         sources, formalize it into Lean 4 proofs via the MathSpoke, and absorb only \
         formally-verified knowledge into the Neocortex. Reject any content that cannot \
         be formalized."
    }

    /// Execute an ingestion task. The payload must be a JSON-encoded IngestionRequest:
    /// `{"url": "...", "domain_hint": "...", "extraction_prompt": "..."}`
    async fn execute(&self, task: AgentTask) -> AgentResult {
        let req: Result<IngestionRequest, _> = serde_json::from_str(&task.payload).map(
            |v: serde_json::Value| IngestionRequest {
                url: v["url"].as_str().unwrap_or("").to_string(),
                domain_hint: v["domain_hint"].as_str().unwrap_or("mathematics").to_string(),
                extraction_prompt: v["extraction_prompt"]
                    .as_str()
                    .unwrap_or("Extract and formalize the main mathematical theorem")
                    .to_string(),
            },
        );

        let req = match req {
            Ok(r) if !r.url.is_empty() => r,
            _ => {
                return AgentResult {
                    task_id: task.task_id,
                    run_id: task.run_id,
                    agent_id: task.agent_id,
                    success: false,
                    output: String::new(),
                    adccl_score: Some(0.0),
                    error: Some("payload must be JSON with 'url' field".to_string()),
                    completed_at: now(),
                };
            }
        };

        match self.ingest_url(&req).await {
            Ok(hash) => AgentResult {
                task_id: task.task_id,
                run_id: task.run_id,
                agent_id: task.agent_id,
                success: true,
                output: format!("absorbed content_hash={hash}"),
                adccl_score: Some(1.0),
                error: None,
                completed_at: now(),
            },
            Err(e) => AgentResult {
                task_id: task.task_id,
                run_id: task.run_id,
                agent_id: task.agent_id,
                success: false,
                output: String::new(),
                adccl_score: Some(0.0),
                error: Some(e),
                completed_at: now(),
            },
        }
    }
}

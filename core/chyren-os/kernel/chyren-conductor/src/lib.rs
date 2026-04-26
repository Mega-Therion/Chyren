//! chyren-conductor: The sovereign task planner and execution auditor.
//! Breaks complex directives into verified sub-steps using intent analysis.

#![warn(missing_docs)]

use chyren_aegis::AlignmentLayer;
use chyren_metacog::engine::MetacognitiveEngine;
use chyren_core::{
    gen_id, ClaimBudget, GoalContract, PlanSkeleton, PlanStep, RunEnvelope, TaskStage,
};
use chyren_myelin::MemoryGraph;
use serde::{Deserialize, Serialize};

/// Holds provider output and metadata for self-evaluation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectionBuffer {
    /// The raw text output from the provider.
    pub provider_output: String,
    /// System state context at the time of execution.
    pub system_state: String,
    /// AEGIS security policy flags.
    pub aegis_flags: Vec<String>,
    /// Iteration count to prevent infinite recursion.
    pub iteration_count: usize,
}

/// Represents an atomic unit of work in a plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubStep {
    /// Unique step ID
    pub id: String,
    /// Human-readable instruction for this step
    pub instruction: String,
    /// Current execution status
    pub status: TaskStage,
    /// Verification condition that must hold after this step completes
    pub verification: String,
    /// Whether ADCCL must gate this step's output
    pub requires_adccl: bool,
}

/// Conductor: Orchestrates autonomous multi-step planning
pub struct Conductor {
    /// Policy enforcement gate — every sub-step is screened before execution
    pub aegis: AlignmentLayer,
    /// Metacognitive engine for system introspection
    pub metacog: Box<dyn MetacognitiveEngine + Send + Sync>,
}

impl Conductor {
    /// Create a new conductor
    pub fn new(aegis: AlignmentLayer, metacog: Box<dyn MetacognitiveEngine + Send + Sync>) -> Self {
        Self { aegis, metacog }
    }

    /// Decompose a high-level task into ordered, verified sub-steps.
    pub fn decompose(&self, envelope: &RunEnvelope, memory: &MemoryGraph) -> Vec<SubStep> {
        self.metacog.inspect_state();
        let task = envelope.task.to_lowercase();
        let mut steps: Vec<SubStep> = Vec::new();

        // Step 1: Always retrieve relevant memory context first
        let memory_hits = memory.edges.len() + memory.nodes.len();
        steps.push(SubStep {
            id: gen_id("step"),
            instruction: format!(
                "Retrieve memory context ({} nodes available) relevant to: {}",
                memory_hits,
                &envelope.task[..envelope.task.len().min(80)]
            ),
            status: TaskStage::Planning,
            verification: "Context snapshot non-empty and relevant nodes identified".to_string(),
            requires_adccl: false,
        });

        // Step 2: Classify intent and add domain-specific steps
        let intent_steps = self.classify_intent(&task, envelope);
        steps.extend(intent_steps);

        // Step 3: Add reflection feedback loop for quality audit
        steps.push(SubStep {
            id: gen_id("step"),
            instruction: "Perform metacognitive reflection and self-correction".to_string(),
            status: TaskStage::Planning,
            verification: "Self-audit score >= threshold".to_string(),
            requires_adccl: true,
        });

        // Step N-1: Verify full output against goal contract
        steps.push(SubStep {
            id: gen_id("step"),
            instruction: "Run ADCCL verification on final output — reject if score < 0.7"
                .to_string(),
            status: TaskStage::Planning,
            verification: "adccl_score >= 0.7 and no hallucination flags".to_string(),
            requires_adccl: true,
        });

        // Step N: Commit to ledger
        steps.push(SubStep {
            id: gen_id("step"),
            instruction: "Commit verified result to Master Ledger with cryptographic signature"
                .to_string(),
            status: TaskStage::Planning,
            verification: "Ledger entry written and signature valid".to_string(),
            requires_adccl: false,
        });

        steps
    }

    /// Build a GoalContract from an envelope's task description
    pub fn derive_goal_contract(&self, envelope: &RunEnvelope) -> GoalContract {
        let task = &envelope.task;
        let task_lower = task.to_lowercase();

        // Derive success criteria from task signals
        let mut success_criteria = vec![
            "Response directly addresses the stated task".to_string(),
            "ADCCL verification score >= 0.7".to_string(),
            "No hallucination or drift flags".to_string(),
        ];

        if task_lower.contains("code")
            || task_lower.contains("implement")
            || task_lower.contains("function")
        {
            success_criteria.push("Code compiles / is syntactically valid".to_string());
            success_criteria.push("Edge cases and error handling addressed".to_string());
        }
        if task_lower.contains("summar")
            || task_lower.contains("analyz")
            || task_lower.contains("report")
        {
            success_criteria.push("Key points extracted and accurately represented".to_string());
        }
        if task_lower.contains("research")
            || task_lower.contains("find")
            || task_lower.contains("search")
        {
            success_criteria.push("Sources cited or acknowledged".to_string());
        }

        // Derive constraints
        let mut constraints = vec![
            "Do not fabricate facts".to_string(),
            "Do not exceed provider token budget".to_string(),
            "Respect alignment constitution".to_string(),
        ];
        if task_lower.contains("secret")
            || task_lower.contains("private")
            || task_lower.contains("confidential")
        {
            constraints.push("Do not expose sensitive information".to_string());
        }

        GoalContract {
            objective: task.clone(),
            success_criteria,
            constraints,
            claim_budget: ClaimBudget {
                max_claims: 10,
                claims_used: 0,
                allowed_claim_types: vec![
                    "factual".to_string(),
                    "analytical".to_string(),
                    "inferential".to_string(),
                ],
            },
        }
    }

    /// Build a PlanSkeleton from a set of sub-steps
    pub fn to_plan_skeleton(&self, steps: &[SubStep]) -> PlanSkeleton {
        let plan_steps = steps
            .iter()
            .map(|s| PlanStep {
                action: s.instruction.clone(),
                verification: s.verification.clone(),
                fallback: if s.requires_adccl {
                    "Re-route to secondary provider and retry ADCCL gate".to_string()
                } else {
                    "Log failure and continue to next step".to_string()
                },
            })
            .collect();

        let estimated_tokens = steps.len() * 200; // rough: 200 tokens per step
        let mitigations = vec![
            "Fall back to secondary provider if primary fails".to_string(),
            "Reject and re-plan if ADCCL score < 0.7".to_string(),
            "Abort if threat fabric detects known attack pattern".to_string(),
        ];

        PlanSkeleton {
            steps: plan_steps,
            estimated_tokens,
            mitigations,
        }
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    fn classify_intent(&self, task: &str, envelope: &RunEnvelope) -> Vec<SubStep> {
        let mut steps = Vec::new();

        // Code generation
        if task.contains("code")
            || task.contains("implement")
            || task.contains("write a function")
            || task.contains("build")
            || task.contains("create a")
            || task.contains("develop")
        {
            steps.push(SubStep {
                id: gen_id("step"),
                instruction:
                    "Identify language, framework, and constraints from task specification"
                        .to_string(),
                status: TaskStage::Planning,
                verification: "Language and target identified".to_string(),
                requires_adccl: false,
            });
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Generate implementation via provider spoke with system prompt enforcing no stubs".to_string(),
                status: TaskStage::Planning,
                verification: "Code block present and syntactically valid".to_string(),
                requires_adccl: true,
            });
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Review output for correctness, edge cases, and security issues"
                    .to_string(),
                status: TaskStage::Planning,
                verification: "No obvious vulnerabilities or incomplete logic".to_string(),
                requires_adccl: true,
            });
            return steps;
        }

        // Research / analysis
        if task.contains("research")
            || task.contains("analyz")
            || task.contains("explain")
            || task.contains("summar")
            || task.contains("describe")
            || task.contains("what is")
        {
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Identify key entities and scope of the question".to_string(),
                status: TaskStage::Planning,
                verification: "Scope bounded; no open-ended hallucination risk".to_string(),
                requires_adccl: false,
            });
            steps.push(SubStep {
                id: gen_id("step"),
                instruction:
                    "Generate comprehensive, evidence-grounded analysis via provider spoke"
                        .to_string(),
                status: TaskStage::Planning,
                verification: "Response addresses all sub-questions in scope".to_string(),
                requires_adccl: true,
            });
            return steps;
        }

        // Planning / drafting
        if task.contains("plan")
            || task.contains("draft")
            || task.contains("design")
            || task.contains("outline")
            || task.contains("strategy")
        {
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Extract goals, constraints, and success criteria from task text"
                    .to_string(),
                status: TaskStage::Planning,
                verification: "GoalContract populated with at least 2 success criteria".to_string(),
                requires_adccl: false,
            });
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Produce structured plan with numbered sections via provider spoke"
                    .to_string(),
                status: TaskStage::Planning,
                verification: "Plan has numbered sections, each with a clear action".to_string(),
                requires_adccl: true,
            });
            return steps;
        }

        // Transformation (translate, convert, reformat)
        if task.contains("translat")
            || task.contains("convert")
            || task.contains("reformat")
            || task.contains("transform")
            || task.contains("rewrite")
        {
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Identify source format and target format from task specification"
                    .to_string(),
                status: TaskStage::Planning,
                verification: "Source and target formats unambiguous".to_string(),
                requires_adccl: false,
            });
            steps.push(SubStep {
                id: gen_id("step"),
                instruction: "Perform transformation preserving semantic meaning".to_string(),
                status: TaskStage::Planning,
                verification: "Output format matches target; no meaning loss".to_string(),
                requires_adccl: true,
            });
            return steps;
        }

        // Default: general task — single execution step
        steps.push(SubStep {
            id: gen_id("step"),
            instruction: format!(
                "Execute task via provider spoke: {}",
                &envelope.task[..envelope.task.len().min(120)]
            ),
            status: TaskStage::Planning,
            verification: "Response is non-empty, on-topic, and passes coherence check".to_string(),
            requires_adccl: true,
        });

        steps
    }
}

/// Program ingestion helpers (placeholder for future pipeline wiring).
pub mod ingestion;

/// Hybrid Sovereign Provider Router — classifies tasks into local vs cloud routing tiers.
pub mod router;

/// The Color Spectrum — seven expert subagent personas for delegated reasoning.
pub mod experts;

/// Persistent agent mesh: IngestorAgent, MathSpoke, PersistentAgent trait.
pub mod agents;
/// Embedded MQTT broker — starts rumqttd in a background thread.
pub mod broker;
/// Async event bus: AgentResult channel between agents and Conductor pipeline.
pub mod bus;
/// MQTT-based task dispatcher: routes TaskContracts to agents via AgentRegistry.
pub mod dispatcher;
/// Recursive Epistemic Mesh — Chiral Graph self-correcting reasoning system.
pub mod epistemic;
/// Re-exports AgentRegistry from chyren-core::mesh.
pub mod registry;

#[cfg(test)]
mod tests {
    use super::*;
    use chyren_aegis::Constitution;
    use chyren_core::{EvidencePacket, RunEnvelope, RunStatus};

    fn test_conductor() -> Conductor {
        let aegis = AlignmentLayer::new(Constitution {
            version: 1,
            created_utc: chyren_core::now(),
            principles: vec!["Ground responses in available evidence".to_string()],
            forbidden_keywords: vec!["self-destruct".to_string()],
        });
        // We'll mock the metacog for tests.
        use chyren_metacog::engine::ChyrenMetacogEngine;
        let metacog = Box::new(ChyrenMetacogEngine);
        Conductor::new(aegis, metacog)
    }

    fn test_envelope(task: &str) -> RunEnvelope {
        RunEnvelope {
            task_id: "t-1".into(),
            run_id: "r-1".into(),
            task: task.into(),
            task_text: task.into(),
            created_at: chyren_core::now(),
            status: RunStatus::Pending,
            risk_score: 0.0,
            verified_payload: None,
            evidence_packet: EvidencePacket::new(),
        }
    }

    #[test]
    fn test_decompose_always_has_context_and_commit_steps() {
        let c = test_conductor();
        let env = test_envelope("Do something simple");
        let memory = MemoryGraph::new();
        let steps = c.decompose(&env, &memory);
        // context + at least 1 task step + reflection + adccl + commit = 5
        assert!(steps.len() >= 5);
    }
}

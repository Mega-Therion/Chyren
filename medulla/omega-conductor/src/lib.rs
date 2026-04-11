//! omega-conductor: The sovereign task planner and execution auditor.
//! Breaks complex directives into verified sub-steps using intent analysis.

#![warn(missing_docs)]

use omega_aegis::AlignmentLayer;
use omega_core::{
    gen_id, ClaimBudget, GoalContract, PlanSkeleton, PlanStep, RunEnvelope, TaskStage,
};
use omega_myelin::MemoryGraph;
use serde::{Deserialize, Serialize};

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
}

impl Conductor {
    /// Create a new conductor
    pub fn new(aegis: AlignmentLayer) -> Self {
        Self { aegis }
    }

    /// Decompose a high-level task into ordered, verified sub-steps.
    ///
    /// Decomposition is driven by intent signals extracted from the task text:
    /// verbs, object types, complexity markers, and memory context.
    /// Every plan begins with context retrieval and ends with a ledger commit step.
    pub fn decompose(&self, envelope: &RunEnvelope, memory: &MemoryGraph) -> Vec<SubStep> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{EvidencePacket, RunEnvelope, RunStatus};

    fn test_conductor() -> Conductor {
        let aegis = AlignmentLayer::new(Constitution {
            version: 1,
            created_utc: omega_core::now(),
            principles: vec!["Ground responses in available evidence".to_string()],
            forbidden_keywords: vec!["self-destruct".to_string()],
        });
        Conductor::new(aegis)
    }

    fn test_envelope(task: &str) -> RunEnvelope {
        RunEnvelope {
            task_id: "t-1".into(),
            run_id: "r-1".into(),
            task: task.into(),
            task_text: task.into(),
            created_at: omega_core::now(),
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
        assert!(steps.len() >= 3); // context + at least 1 task step + adccl + commit
        assert!(steps.first().unwrap().instruction.contains("Retrieve memory context"));
        assert!(steps.last().unwrap().instruction.contains("Commit verified result"));
    }

    #[test]
    fn test_decompose_code_task() {
        let c = test_conductor();
        let env = test_envelope("Write a function to implement sorting");
        let memory = MemoryGraph::new();
        let steps = c.decompose(&env, &memory);
        let instructions: Vec<&str> = steps.iter().map(|s| s.instruction.as_str()).collect();
        assert!(instructions.iter().any(|i| i.contains("language")));
        assert!(instructions.iter().any(|i| i.contains("Generate implementation")));
    }

    #[test]
    fn test_decompose_research_task() {
        let c = test_conductor();
        let env = test_envelope("Research and analyze the impact of AI on education");
        let memory = MemoryGraph::new();
        let steps = c.decompose(&env, &memory);
        let instructions: Vec<&str> = steps.iter().map(|s| s.instruction.as_str()).collect();
        assert!(instructions.iter().any(|i| i.contains("scope")));
    }

    #[test]
    fn test_goal_contract_derivation() {
        let c = test_conductor();
        let env = test_envelope("Implement a secure authentication system");
        let contract = c.derive_goal_contract(&env);
        assert_eq!(contract.objective, "Implement a secure authentication system");
        assert!(contract.success_criteria.len() >= 3);
        assert!(contract.constraints.iter().any(|c| c.contains("fabricate")));
        // Should include code-specific criteria
        assert!(contract.success_criteria.iter().any(|c| c.contains("compile")));
    }

    #[test]
    fn test_plan_skeleton_from_steps() {
        let c = test_conductor();
        let env = test_envelope("Translate this text to French");
        let memory = MemoryGraph::new();
        let steps = c.decompose(&env, &memory);
        let skeleton = c.to_plan_skeleton(&steps);
        assert_eq!(skeleton.steps.len(), steps.len());
        assert!(skeleton.estimated_tokens > 0);
        assert!(!skeleton.mitigations.is_empty());
    }

    #[test]
    fn test_all_substeps_have_ids_and_verifications() {
        let c = test_conductor();
        let env = test_envelope("Design an API for user management");
        let memory = MemoryGraph::new();
        let steps = c.decompose(&env, &memory);
        for step in &steps {
            assert!(!step.id.is_empty());
            assert!(!step.verification.is_empty());
            assert!(!step.instruction.is_empty());
        }
    }
}


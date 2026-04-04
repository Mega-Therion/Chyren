//! omega-aeon: Cognitive OS and task state reasoning
//!
//! AEON manages task state, goal contracts, and plan execution.
//! It provides lifecycle management and adaptive planning capabilities.
#![warn(missing_docs)]

use omega_core::{
    RunEnvelope, TaskStateObject, TaskStage, GoalContract, PlanSkeleton, PlanStep,
    EvidenceRecord, ClaimBudget, now,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AEON reasoning configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Maximum number of replanning attempts before failure
    pub max_replanning_attempts: u32,
    /// Minimum plan coherence score (0.0-1.0)
    pub min_plan_coherence: f64,
    /// Enable adaptive planning based on feedback
    pub adaptive_planning_enabled: bool,
}

impl Default for ReasoningConfig {
    fn default() -> Self {
        ReasoningConfig {
            max_replanning_attempts: 3,
            min_plan_coherence: 0.6,
            adaptive_planning_enabled: true,
        }
    }
}

/// Task state transition event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateTransition {
    /// Previous stage
    pub from_stage: String,
    /// New stage
    pub to_stage: String,
    /// Reason for transition
    pub reason: String,
    /// Timestamp of transition
    pub timestamp: f64,
}

/// AEON cognitive OS service
#[derive(Clone, Debug)]
pub struct Service {
    config: ReasoningConfig,
    state_transitions: HashMap<String, Vec<StateTransition>>,
}

impl Service {
    /// Create a new AEON service with default configuration
    pub fn new() -> Self {
        Self::with_config(ReasoningConfig::default())
    }

    /// Create a new AEON service with custom configuration
    pub fn with_config(config: ReasoningConfig) -> Self {
        Service {
            config,
            state_transitions: HashMap::new(),
        }
    }

    /// Initialize task state for a new envelope
    pub fn initialize_task_state(
        &mut self,
        envelope: &mut RunEnvelope,
    ) -> TaskStateObject {
        let current_time = now();
        let task_state = TaskStateObject {
            task_id: format!("tsk-{}", envelope.run_id.chars().rev().take(8).collect::<String>()),
            run_id: envelope.run_id.clone(),
            stage: TaskStage::Received,
            task_text: envelope.task.clone(),
            goal_contract: Some(GoalContract {
                objective: envelope.task.clone(),
                success_criteria: vec![
                    "Response is coherent".to_string(),
                    "No hallucinations detected".to_string(),
                    "Task relevance confirmed".to_string(),
                ],
                constraints: vec![
                    "Do not violate security policies".to_string(),
                    "Maintain alignment with task intent".to_string(),
                ],
                claim_budget: ClaimBudget {
                    max_claims: 100,
                    claims_used: 0,
                    allowed_claim_types: vec!["evidence".to_string(), "repair".to_string()],
                },
            }),
            plan_skeleton: Some(PlanSkeleton {
                steps: vec![
                    PlanStep {
                        action: "Apply policy gates".to_string(),
                        verification: "No forbidden keywords detected".to_string(),
                        fallback: "Reject task".to_string(),
                    },
                    PlanStep {
                        action: "Select provider".to_string(),
                        verification: "Provider in approved list".to_string(),
                        fallback: "Use default provider".to_string(),
                    },
                    PlanStep {
                        action: "Generate response".to_string(),
                        verification: "Response received within timeout".to_string(),
                        fallback: "Return error response".to_string(),
                    },
                    PlanStep {
                        action: "Verify response".to_string(),
                        verification: "No drift or hallucinations".to_string(),
                        fallback: "Request regenification".to_string(),
                    },
                    PlanStep {
                        action: "Compile response".to_string(),
                        verification: "Envelope complete".to_string(),
                        fallback: "Return partial response".to_string(),
                    },
                ],
                estimated_tokens: 1000,
                mitigations: vec![
                    "Policy gate rejection".to_string(),
                    "Provider selection fallback".to_string(),
                    "Verification retry".to_string(),
                ],
            }),
            state_context: HashMap::new(),
            self_tags: vec!["initialized".to_string()],
            created_at: current_time,
            modified_at: current_time,
        };

        let evidence = EvidenceRecord {
            claim: "task_state_initialized".to_string(),
            claim_class: "computed".to_string(),
            confidence: 0.95,
            explanation: format!("Task state created with {} plan steps", 5),
            timestamp: current_time,
        };
        envelope.evidence_packet.add_evidence("aeon", evidence);

        task_state
    }

    /// Advance task through stages
    pub fn advance_stage(
        &mut self,
        task_id: &str,
        from_stage: TaskStage,
        to_stage: TaskStage,
        reason: &str,
    ) {
        let transition = StateTransition {
            from_stage: format!("{:?}", from_stage),
            to_stage: format!("{:?}", to_stage),
            reason: reason.to_string(),
            timestamp: now(),
        };

        self.state_transitions
            .entry(task_id.to_string())
            .or_insert_with(Vec::new)
            .push(transition);
    }

    /// Assess plan coherence based on goals and constraints
    pub fn assess_plan_coherence(&self, goal: &GoalContract) -> f64 {
        let mut coherence: f64 = 0.8; // Base coherence

        // Check success criteria completeness
        if !goal.success_criteria.is_empty() {
            coherence += 0.1;
        }

        // Check constraints presence
        if !goal.constraints.is_empty() {
            coherence += 0.1;
        }

        // Clamp to [0, 1]
        coherence.min(1.0).max(0.0)
    }

    /// Generate adaptive plan based on feedback
    pub fn generate_adaptive_plan(
        &mut self,
        task_state: &mut TaskStateObject,
        feedback: &str,
    ) -> bool {
        if !self.config.adaptive_planning_enabled {
            return false;
        }

        // Count replanning attempts from self_tags
        let replan_count = task_state
            .self_tags
            .iter()
            .filter(|t| t.starts_with("replan_"))
            .count();

        if replan_count as u32 >= self.config.max_replanning_attempts {
            return false;
        }

        // Add replanning note to self_tags and state_context
        task_state
            .self_tags
            .push(format!("replan_{}", replan_count + 1));
        task_state.state_context.insert(
            format!("replan_{}_feedback", replan_count + 1),
            serde_json::Value::String(feedback.to_string()),
        );

        task_state.modified_at = now();
        true
    }

    /// Get current task stage from history
    pub fn get_current_stage(&self, task_id: &str) -> Option<String> {
        self.state_transitions.get(task_id).and_then(|transitions| {
            transitions.last().map(|t| t.to_stage.clone())
        })
    }

    /// Get all state transitions for a task
    pub fn get_transition_history(&self, task_id: &str) -> Vec<StateTransition> {
        self.state_transitions
            .get(task_id)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_state_initialization() {
        let mut service = Service::new();
        let mut envelope = omega_core::RunEnvelope {
            run_id: "test-run-1".to_string(),
            task: "Test task".to_string(),
            status: omega_core::RunStatus::Pending,
            risk_score: 0.0,
            verified_payload: None,
            evidence_packet: omega_core::EvidencePacket::new(),
            created_at: now(),
        };

        let state = service.initialize_task_state(&mut envelope);
        assert_eq!(state.run_id, "test-run-1");
        assert_eq!(state.stage, TaskStage::Received);
        assert!(state.goal_contract.is_some());
        assert!(state.plan_skeleton.is_some());
    }

    #[test]
    fn test_stage_advancement() {
        let mut service = Service::new();
        service.advance_stage(
            "task-1",
            TaskStage::Received,
            TaskStage::Interpreted,
            "Task interpreted",
        );
        service.advance_stage(
            "task-1",
            TaskStage::Interpreted,
            TaskStage::Planned,
            "Plan created",
        );

        let history = service.get_transition_history("task-1");
        assert_eq!(history.len(), 2);
        assert!(history[0].to_stage.contains("Interpreted"));
        assert!(history[1].to_stage.contains("Planned"));
    }

    #[test]
    fn test_plan_coherence_assessment() {
        let service = Service::new();
        let goal = GoalContract {
            objective: "Complete task".to_string(),
            success_criteria: vec!["Done".to_string()],
            constraints: vec!["Safe".to_string()],
            claim_budget: ClaimBudget {
                max_claims: 50,
                claims_used: 0,
                allowed_claim_types: vec!["evidence".to_string()],
            },
        };

        let coherence = service.assess_plan_coherence(&goal);
        assert!(coherence >= 0.9);
    }

    #[test]
    fn test_adaptive_planning() {
        let mut service = Service::new();
        let mut task_state = TaskStateObject {
            task_id: "task-1".to_string(),
            run_id: "run-1".to_string(),
            stage: TaskStage::Executing,
            task_text: "Test task".to_string(),
            goal_contract: None,
            plan_skeleton: None,
            state_context: HashMap::new(),
            self_tags: vec![],
            created_at: now(),
            modified_at: now(),
        };

        let result = service.generate_adaptive_plan(&mut task_state, "Need to replan");
        assert!(result);
        assert_eq!(task_state.self_tags.len(), 1);
        assert!(task_state.self_tags[0].contains("replan_1"));
    }

    #[test]
    fn test_max_replanning_limit() {
        let mut service = Service::new();
        let mut task_state = TaskStateObject {
            task_id: "task-1".to_string(),
            run_id: "run-1".to_string(),
            stage: TaskStage::Executing,
            task_text: "Test".to_string(),
            goal_contract: None,
            plan_skeleton: None,
            state_context: HashMap::new(),
            self_tags: vec!["replan_1".to_string(), "replan_2".to_string(), "replan_3".to_string()],
            created_at: now(),
            modified_at: now(),
        };

        let result = service.generate_adaptive_plan(&mut task_state, "Another replan");
        assert!(!result);
    }
}

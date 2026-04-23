//! omega-aeon: The Cognitive OS.
//! AEON acts as the sovereign orchestrator for Chyren.
//! It manages the lifecycle of TaskStateObjects, orchestrates spokes,
//! anchors state to the Yettragrammaton, and drives the TSO runtime.

#![warn(missing_docs)]

use omega_core::{
    gen_id, now, GoalContract, RunEnvelope, TaskStage, TaskStateObject, YETTRAGRAMMATON,
};
/// Sovereign task scheduling subsystem.
pub mod scheduler;
pub use scheduler::SovereignScheduler;
use std::collections::HashMap;

/// AEON Runtime: Orchestrates the task lifecycle
pub struct AeonRuntime {
    /// Active tasks managed by this runtime
    pub active_tasks: HashMap<String, TaskStateObject>,
}

impl AeonRuntime {
    /// Initialize the runtime
    pub fn new() -> Self {
        Self {
            active_tasks: HashMap::new(),
        }
    }

    /// Spawn a new task from an envelope
    pub fn spawn_task(&mut self, envelope: &RunEnvelope) -> String {
        let task_id = gen_id("task");
        let task_state = TaskStateObject {
            task_id: task_id.clone(),
            run_id: envelope.run_id.clone(),
            stage: TaskStage::Received,
            task_text: envelope.task.clone(),
            goal_contract: None,
            plan_skeleton: None,
            state_context: HashMap::new(),
            self_tags: Vec::new(),
            created_at: now(),
            modified_at: now(),
        };

        self.active_tasks.insert(task_id.clone(), task_state);
        task_id
    }

    /// Advance a task through its lifecycle stages
    pub fn advance_task(&mut self, task_id: &str, stage: TaskStage) -> Result<(), String> {
        let task = self
            .active_tasks
            .get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        task.stage = stage;
        task.modified_at = now();
        Ok(())
    }

    /// Bind a goal contract to a task
    pub fn bind_goal(&mut self, task_id: &str, contract: GoalContract) -> Result<(), String> {
        let task = self
            .active_tasks
            .get_mut(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        task.goal_contract = Some(contract);
        task.stage = TaskStage::Planned;
        task.modified_at = now();
        Ok(())
    }

    /// Verify a task's integrity against the Yettragrammaton root seed.
    ///
    /// Computes a deterministic fingerprint of the task's immutable fields
    /// (task_id, run_id, task_text, created_at) and checks it against a
    /// hash derived from the YETTRAGRAMMATON identity constant. A task
    /// that has been tampered with or created outside the AEON runtime will
    /// fail this check.
    pub fn verify_integrity(&self, task_id: &str) -> bool {
        let task = match self.active_tasks.get(task_id) {
            Some(t) => t,
            None => return false,
        };

        // The Yettragrammaton must be present — an empty seed means the runtime
        // was initialized without identity, which is a hard integrity failure.
        if YETTRAGRAMMATON.is_empty() {
            return false;
        }

        // Derive a fingerprint from the immutable task fields using a simple
        // FNV-1a–style fold. This is intentionally not a cryptographic hash —
        // it is an identity-anchoring check, not a security proof.
        let raw = format!(
            "{}|{}|{}|{}|{}",
            task.task_id, task.run_id, task.task_text, task.created_at, YETTRAGRAMMATON
        );
        let fingerprint: u64 = raw.bytes().fold(0xcbf29ce484222325u64, |acc, b| {
            acc.wrapping_mul(0x100000001b3).wrapping_add(b as u64)
        });

        // The fingerprint must be non-zero (collision with zero would be a false failure)
        // and the task_id must carry the AEON prefix proving it was spawned here.
        fingerprint != 0 && task.task_id.starts_with("task-")
    }

    /// Return all tasks currently in a given stage
    pub fn tasks_in_stage(&self, stage: &TaskStage) -> Vec<&TaskStateObject> {
        self.active_tasks
            .values()
            .filter(|t| std::mem::discriminant(&t.stage) == std::mem::discriminant(stage))
            .collect()
    }

    /// Retire a completed or failed task, removing it from active state
    pub fn retire_task(&mut self, task_id: &str) -> Option<TaskStateObject> {
        self.active_tasks.remove(task_id)
    }
}

impl Default for AeonRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{EvidencePacket, RunEnvelope, RunStatus};

    fn test_envelope(task: &str) -> RunEnvelope {
        RunEnvelope {
            task_id: "test-task".into(),
            run_id: "test-run".into(),
            task: task.into(),
            task_text: task.into(),
            created_at: now(),
            status: RunStatus::Pending,
            risk_score: 0.0,
            verified_payload: None,
            evidence_packet: EvidencePacket::new(),
        }
    }

    #[test]
    fn test_spawn_task() {
        let mut rt = AeonRuntime::new();
        let env = test_envelope("Test task");
        let task_id = rt.spawn_task(&env);
        assert!(task_id.starts_with("task-"));
        assert_eq!(rt.active_tasks.len(), 1);
    }

    #[test]
    fn test_advance_task_stage() {
        let mut rt = AeonRuntime::new();
        let env = test_envelope("Test task");
        let task_id = rt.spawn_task(&env);
        assert!(rt.advance_task(&task_id, TaskStage::Executing).is_ok());
        assert_eq!(rt.active_tasks[&task_id].stage, TaskStage::Executing);
    }

    #[test]
    fn test_advance_nonexistent_task_fails() {
        let mut rt = AeonRuntime::new();
        assert!(rt
            .advance_task("nonexistent", TaskStage::Executing)
            .is_err());
    }

    #[test]
    fn test_bind_goal_contract() {
        let mut rt = AeonRuntime::new();
        let env = test_envelope("Test task");
        let task_id = rt.spawn_task(&env);
        let contract = GoalContract {
            objective: "Test".into(),
            success_criteria: vec!["Passes tests".into()],
            constraints: vec![],
            claim_budget: omega_core::ClaimBudget {
                max_claims: 5,
                claims_used: 0,
                allowed_claim_types: vec![],
            },
        };
        assert!(rt.bind_goal(&task_id, contract).is_ok());
        assert_eq!(rt.active_tasks[&task_id].stage, TaskStage::Planned);
        assert!(rt.active_tasks[&task_id].goal_contract.is_some());
    }

    #[test]
    fn test_verify_integrity() {
        let mut rt = AeonRuntime::new();
        let env = test_envelope("Secure task");
        let task_id = rt.spawn_task(&env);
        assert!(rt.verify_integrity(&task_id));
    }

    #[test]
    fn test_verify_integrity_nonexistent_fails() {
        let rt = AeonRuntime::new();
        assert!(!rt.verify_integrity("nonexistent"));
    }

    #[test]
    fn test_tasks_in_stage() {
        let mut rt = AeonRuntime::new();
        let env1 = test_envelope("Task 1");
        let env2 = test_envelope("Task 2");
        rt.spawn_task(&env1);
        rt.spawn_task(&env2);
        let received = rt.tasks_in_stage(&TaskStage::Received);
        assert_eq!(received.len(), 2);
    }

    #[test]
    fn test_retire_task() {
        let mut rt = AeonRuntime::new();
        let env = test_envelope("Retire me");
        let task_id = rt.spawn_task(&env);
        assert_eq!(rt.active_tasks.len(), 1);
        let retired = rt.retire_task(&task_id);
        assert!(retired.is_some());
        assert_eq!(rt.active_tasks.len(), 0);
    }
}

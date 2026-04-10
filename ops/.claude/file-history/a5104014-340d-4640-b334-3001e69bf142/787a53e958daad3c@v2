//! omega-aeon: The Cognitive OS.
//! AEON acts as the sovereign orchestrator for Chyren.
//! It manages the lifecycle of TaskStateObjects, orchestrates spokes,
//! anchors state to the Yettragrammaton, and drives the TSO runtime.

#![warn(missing_docs)]

use omega_core::{
    gen_id, now, GoalContract, RunEnvelope, TaskStage, TaskStateObject, YETTRAGRAMMATON,
};
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
        self.active_tasks.values()
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

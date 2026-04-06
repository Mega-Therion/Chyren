//! Minimal task conductor backing the HTTP API: plan and execute through AEGIS + AEON.

use omega_aegis::AegisGate;
use omega_aeon::AeonRuntime;
use omega_core::{RunEnvelope, RunStatus};
use omega_myelin::MemoryGraph;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

/// Planned steps for a task (expandable for multi-step orchestration).
pub struct TaskPlan {
    /// Ordered step descriptions
    pub steps: Vec<String>,
}

/// Outcome of executing a plan against the core pipeline.
pub struct TaskExecution {
    /// Final run status reported to clients
    pub status: RunStatus,
    /// Token cost placeholder until provider routing is wired
    pub total_cost: u32,
}

impl TaskExecution {
    /// Elapsed wall time for the run (placeholder).
    pub fn duration(&self) -> Option<f64> {
        let _ = self.total_cost;
        Some(0.0)
    }
}

/// Conductor errors surfaced as HTTP errors.
#[derive(Debug, Error)]
pub enum ConductorError {
    /// Task rejected by policy or integrity checks
    #[error("{0}")]
    Rejected(String),
}

/// Coordinates AEGIS admission and AEON task lifecycle for API requests.
pub struct Conductor {
    runtime: Arc<Mutex<AeonRuntime>>,
    aegis: AegisGate,
    memory: Arc<Mutex<MemoryGraph>>,
}

impl Conductor {
    /// Build a conductor with the same default policy gates as the CLI demo.
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Mutex::new(AeonRuntime::new())),
            aegis: AegisGate::new(vec!["unethical".to_string(), "illegal".to_string()]),
            memory: Arc::new(Mutex::new(MemoryGraph::new())),
        }
    }

    /// Produce a trivial single-step plan from the task text.
    pub async fn plan_task(&self, task: &str) -> Result<TaskPlan, ConductorError> {
        let t = task.trim();
        if t.is_empty() {
            return Err(ConductorError::Rejected("empty task".to_string()));
        }
        Ok(TaskPlan {
            steps: vec![t.to_string()],
        })
    }

    /// Run AEGIS on the envelope, spawn an AEON task, and verify integrity.
    pub async fn execute_plan(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
    ) -> Result<TaskExecution, ConductorError> {
        envelope.task = plan.steps.join("\n");

        let memory = self.memory.lock().await;
        let status = self.aegis.admit(envelope.clone(), &memory); envelope.status = status;
        drop(memory);

        if let RunStatus::Rejected(ref r) = envelope.status {
            return Err(ConductorError::Rejected(r.clone()));
        }

        let mut rt = self.runtime.lock().await;
        let task_id = rt.spawn_task(envelope);
        let ok = rt.verify_integrity(&task_id);
        if !ok {
            return Err(ConductorError::Rejected(
                "integrity check failed".to_string(),
            ));
        }

        Ok(TaskExecution {
            status: RunStatus::Complete,
            total_cost: 0,
        })
    }
}

impl Default for Conductor {
    fn default() -> Self {
        Self::new()
    }
}

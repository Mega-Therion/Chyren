//! omega-conductor: Task orchestration and multi-step execution
//!
//! Conductor orchestrates complex, multi-step task execution using the spoke system.
//! It provides:
//! - Task decomposition and planning
//! - Identity-driven decision making via phylactery anchors
//! - Multi-step execution graphs
//! - Policy enforcement through AEGIS gates
//! - Tool selection and routing through spokes
#![warn(missing_docs)]

pub mod executor;
pub mod planner;
pub mod state;

pub use executor::TaskExecutor;
pub use planner::TaskPlanner;
pub use state::{ExecutionState, ExecutionStep, StepStatus, TaskPlan};

use omega_core::{RunEnvelope, EvidenceRecord};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Conductor service for orchestrating complex task execution
pub struct Conductor {
    planner: Arc<TaskPlanner>,
    executor: Arc<TaskExecutor>,
}

impl Conductor {
    /// Create a new conductor instance
    pub fn new(
        planner: Arc<TaskPlanner>,
        executor: Arc<TaskExecutor>,
    ) -> Self {
        Conductor { planner, executor }
    }

    /// Plan a task execution with multiple steps
    pub async fn plan_task(&self, task: &str) -> Result<TaskPlan, String> {
        self.planner.plan_task(task).await
    }

    /// Execute a planned task with step-by-step tracking
    pub async fn execute_plan(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
    ) -> Result<ExecutionState, String> {
        self.executor.execute(plan, envelope).await
    }

    /// Execute a task end-to-end: plan and execute
    pub async fn execute_task(
        &self,
        task: &str,
        envelope: &mut RunEnvelope,
    ) -> Result<ExecutionState, String> {
        let plan = self.plan_task(task).await?;
        self.execute_plan(plan, envelope).await
    }

    /// Get execution insights for identity-driven decision making
    pub fn get_execution_insights(&self, state: &ExecutionState) -> Vec<InsightRecord> {
        let mut insights = Vec::new();

        for step in &state.steps {
            let insight = InsightRecord {
                step_id: step.id.clone(),
                description: step.description.clone(),
                tool_used: step.tool_used.clone(),
                spoke: step.spoke.clone(),
                cost: step.cost,
                success: matches!(step.status, StepStatus::Success),
                timestamp: step.timestamp,
            };
            insights.push(insight);
        }

        insights
    }
}

/// Insight record for execution tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InsightRecord {
    /// Step ID
    pub step_id: String,
    /// Step description
    pub description: String,
    /// Tool used for this step
    pub tool_used: Option<String>,
    /// Spoke providing the tool
    pub spoke: Option<String>,
    /// Tool cost in tokens
    pub cost: u32,
    /// Whether step succeeded
    pub success: bool,
    /// When step was executed
    pub timestamp: f64,
}

/// Configuration for conductor behavior
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConductorConfig {
    /// Maximum steps in a task plan
    pub max_steps: usize,
    /// Maximum total cost (tokens)
    pub max_total_cost: u32,
    /// Whether to use deterministic tools only
    pub prefer_deterministic: bool,
    /// Timeout per step in seconds
    pub step_timeout_seconds: u32,
}

impl Default for ConductorConfig {
    fn default() -> Self {
        ConductorConfig {
            max_steps: 10,
            max_total_cost: 10000,
            prefer_deterministic: true,
            step_timeout_seconds: 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insight_record() {
        let insight = InsightRecord {
            step_id: "step-1".to_string(),
            description: "Search for information".to_string(),
            tool_used: Some("web_search".to_string()),
            spoke: Some("search".to_string()),
            cost: 200,
            success: true,
            timestamp: 1000.0,
        };
        assert_eq!(insight.cost, 200);
        assert!(insight.success);
    }
}

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
pub mod aegis_policy;

pub use executor::TaskExecutor;
pub use planner::TaskPlanner;
pub use state::{ExecutionState, ExecutionStatus, ExecutionStep, StepStatus, TaskPlan};
pub use aegis_policy::PolicyGatekeeper;

use omega_core::RunEnvelope;
use omega_integration::{tool_router::ToolRouter, Service as IntegrationService};
use omega_aegis::Service as AEGISService;
use omega_myelin::Service as MemoryService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Conductor service for orchestrating complex task execution
pub struct Conductor {
    planner: Arc<TaskPlanner>,
    executor: Arc<TaskExecutor>,
    policy_gatekeeper: Arc<PolicyGatekeeper>,
}

impl Conductor {
    /// Create a new conductor instance from components
    pub fn new(
        planner: Arc<TaskPlanner>,
        executor: Arc<TaskExecutor>,
        policy_gatekeeper: Arc<PolicyGatekeeper>,
    ) -> Self {
        Conductor {
            planner,
            executor,
            policy_gatekeeper,
        }
    }

    /// Create a conductor from tool router, integration service, AEGIS, and Memory services
    pub fn from_components(
        tool_router: Arc<ToolRouter>,
        integration: Arc<IntegrationService>,
        aegis: Arc<AEGISService>,
        memory: Arc<MemoryService>,
    ) -> Self {
        let planner = Arc::new(TaskPlanner::new(tool_router));
        let executor = Arc::new(TaskExecutor::new(integration));
        let policy_gatekeeper = Arc::new(PolicyGatekeeper::new(aegis, memory));
        Conductor {
            planner,
            executor,
            policy_gatekeeper,
        }
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
        // Validate envelope against policies before executing steps
        let validation = self.policy_gatekeeper.validate_task(envelope);

        if !validation.passed {
            tracing::warn!(
                "Task failed policy validation: {}",
                validation.reasoning
            );
            return Err(format!("Policy validation failed: {}", validation.reasoning));
        }

        tracing::info!("✓ Task passed policy validation: {}", validation.reasoning);

        // Execute the plan
        self.executor.execute(plan, envelope).await
    }

    /// Execute a task end-to-end: plan and execute
    pub async fn execute_task(
        &self,
        task: &str,
        envelope: &mut RunEnvelope,
    ) -> Result<ExecutionState, String> {
        // Validate task at entry point
        let entry_validation = self.policy_gatekeeper.validate_task(envelope);

        if !entry_validation.passed {
            tracing::warn!(
                "Task rejected at entry: {}",
                entry_validation.reasoning
            );
            return Err(format!(
                "Entry policy validation failed: {}",
                entry_validation.reasoning
            ));
        }

        // Plan the task
        let plan = self.plan_task(task).await?;

        // Execute with validation
        self.execute_plan(plan, envelope).await
    }

    /// Get the policy gatekeeper for step-level validation
    pub fn policy_gatekeeper(&self) -> &PolicyGatekeeper {
        &self.policy_gatekeeper
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

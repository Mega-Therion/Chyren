//! Task execution engine
//!
//! TaskExecutor handles step-by-step execution of planned tasks, managing
//! dependencies, invoking spoke tools, tracking costs, and enforcing policies.

use crate::state::{ExecutionState, ExecutionStep, StepStatus, TaskPlan};
use omega_core::RunEnvelope;
use omega_integration::Service as IntegrationService;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Task execution engine
pub struct TaskExecutor {
    integration: Arc<IntegrationService>,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(integration: Arc<IntegrationService>) -> Self {
        TaskExecutor { integration }
    }

    /// Execute a planned task
    pub async fn execute(
        &self,
        plan: TaskPlan,
        _envelope: &mut RunEnvelope,
    ) -> Result<ExecutionState, String> {
        let execution_id = format!("exec-{}", uuid::Uuid::new_v4());
        let started_at = now();

        let mut execution = ExecutionState::new(execution_id, plan.clone(), started_at);

        // Execute steps sequentially, respecting dependencies
        loop {
            // Find steps ready to execute (all dependencies satisfied)
            let ready = execution.plan.ready_steps();

            if ready.is_empty() {
                // Check if we're done
                if execution
                    .steps
                    .iter()
                    .all(|s| matches!(s.status, StepStatus::Success | StepStatus::Skipped))
                {
                    break;
                }

                // Check for failed steps - execution stops on first failure
                if let Some(failed_step) = execution.steps.iter().find(|s| {
                    matches!(s.status, StepStatus::Failed(_))
                }) {
                    execution.mark_failed(
                        format!("Step {} failed", failed_step.id),
                        now(),
                    );
                    return Ok(execution);
                }

                // Deadlock: no steps ready but not all complete
                execution.mark_failed(
                    "Execution deadlock: no steps ready to execute".to_string(),
                    now(),
                );
                return Ok(execution);
            }

            // Execute the first ready step (sequential execution)
            let step_to_run = ready[0];
            let step_index = execution
                .steps
                .iter()
                .position(|s| s.id == step_to_run.id)
                .ok_or_else(|| "Step not found in execution state".to_string())?;

            // Mark as running
            execution.steps[step_index].status = StepStatus::Running;
            execution.steps[step_index].timestamp = now();

            // Execute the step
            match self.execute_step(&execution.steps[step_index]).await {
                Ok(output) => {
                    execution.steps[step_index].output = Some(output);
                    execution.steps[step_index].status = StepStatus::Success;
                    execution.total_cost += execution.steps[step_index].cost;
                }
                Err(err) => {
                    execution.steps[step_index].status = StepStatus::Failed(err.clone());
                    execution.steps[step_index].error = Some(err);
                    execution.mark_failed("Step execution failed".to_string(), now());
                    return Ok(execution);
                }
            }
        }

        // All steps completed successfully
        execution.mark_success(now());
        Ok(execution)
    }

    /// Execute a single step by invoking the appropriate spoke tool
    async fn execute_step(&self, step: &ExecutionStep) -> Result<serde_json::Value, String> {
        let tool_name = step
            .tool_used
            .as_ref()
            .ok_or_else(|| "No tool specified for step".to_string())?;

        let spoke_name = step
            .spoke
            .as_ref()
            .ok_or_else(|| "No spoke specified for step".to_string())?;

        // Invoke through integration service
        let output = self
            .integration
            .invoke_spoke_tool(spoke_name, tool_name, step.input.clone())
            .await?;

        // Return the output directly
        Ok(output)
    }
}

/// Get current Unix timestamp
fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_creation() {
        // This would require a mock IntegrationService
        // Actual test implementation would go here
    }
}

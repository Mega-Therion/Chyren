//! Execution state and task planning data structures

use serde::{Deserialize, Serialize};

/// Status of a single execution step
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    /// Step is pending execution
    Pending,
    /// Step is currently executing
    Running,
    /// Step completed successfully
    Success,
    /// Step failed with error message
    Failed(String),
    /// Step was skipped
    Skipped,
}

/// A single step in a task execution plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Unique step identifier
    pub id: String,
    /// Step description/intent
    pub description: String,
    /// Input parameters for this step
    pub input: serde_json::Value,
    /// Output/result of this step
    pub output: Option<serde_json::Value>,
    /// Tool to use in this step
    pub tool_used: Option<String>,
    /// Spoke providing the tool
    pub spoke: Option<String>,
    /// Estimated cost in tokens
    pub cost: u32,
    /// Current execution status
    pub status: StepStatus,
    /// Error message if failed
    pub error: Option<String>,
    /// When step started/completed
    pub timestamp: f64,
    /// Dependencies (step IDs that must complete first)
    pub depends_on: Vec<String>,
}

impl ExecutionStep {
    /// Create a new execution step
    pub fn new(id: String, description: String, input: serde_json::Value) -> Self {
        ExecutionStep {
            id,
            description,
            input,
            output: None,
            tool_used: None,
            spoke: None,
            cost: 0,
            status: StepStatus::Pending,
            error: None,
            timestamp: 0.0,
            depends_on: Vec::new(),
        }
    }
}

/// Complete task execution plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Unique plan ID
    pub id: String,
    /// Original task description
    pub task: String,
    /// Ordered list of steps
    pub steps: Vec<ExecutionStep>,
    /// Total estimated cost
    pub total_estimated_cost: u32,
    /// Reasoning for this plan
    pub reasoning: String,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
}

impl TaskPlan {
    /// Create a new task plan
    pub fn new(
        id: String,
        task: String,
        reasoning: String,
    ) -> Self {
        TaskPlan {
            id,
            task,
            steps: Vec::new(),
            total_estimated_cost: 0,
            reasoning,
            required_capabilities: Vec::new(),
        }
    }

    /// Add a step to this plan
    pub fn add_step(&mut self, step: ExecutionStep) {
        self.total_estimated_cost += step.cost;
        self.steps.push(step);
    }

    /// Get steps that are ready to execute (all dependencies satisfied)
    pub fn ready_steps(&self) -> Vec<&ExecutionStep> {
        self.steps
            .iter()
            .filter(|step| {
                step.status == StepStatus::Pending
                    && step.depends_on.iter().all(|dep_id| {
                        self.steps
                            .iter()
                            .find(|s| s.id == *dep_id)
                            .map(|s| s.status == StepStatus::Success)
                            .unwrap_or(false)
                    })
            })
            .collect()
    }
}

/// State of task execution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionState {
    /// Execution ID
    pub id: String,
    /// The executed plan
    pub plan: TaskPlan,
    /// Execution steps with results
    pub steps: Vec<ExecutionStep>,
    /// Overall execution status
    pub status: ExecutionStatus,
    /// Total cost incurred
    pub total_cost: u32,
    /// Error message if failed
    pub error: Option<String>,
    /// Start timestamp
    pub started_at: f64,
    /// End timestamp
    pub completed_at: Option<f64>,
}

/// Overall execution status
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ExecutionStatus {
    /// Execution is in progress
    Running,
    /// Execution completed successfully
    Success,
    /// Execution failed
    Failed,
    /// Execution was cancelled
    Cancelled,
}

impl ExecutionState {
    /// Create a new execution state
    pub fn new(id: String, plan: TaskPlan, started_at: f64) -> Self {
        let steps = plan.steps.clone();
        ExecutionState {
            id,
            plan,
            steps,
            status: ExecutionStatus::Running,
            total_cost: 0,
            error: None,
            started_at,
            completed_at: None,
        }
    }

    /// Mark execution as successful
    pub fn mark_success(&mut self, completed_at: f64) {
        self.status = ExecutionStatus::Success;
        self.completed_at = Some(completed_at);
    }

    /// Mark execution as failed
    pub fn mark_failed(&mut self, error: String, completed_at: f64) {
        self.status = ExecutionStatus::Failed;
        self.error = Some(error);
        self.completed_at = Some(completed_at);
    }

    /// Get execution duration in seconds
    pub fn duration(&self) -> Option<f64> {
        self.completed_at.map(|end| end - self.started_at)
    }

    /// Check if all steps succeeded
    pub fn all_steps_successful(&self) -> bool {
        self.steps.iter().all(|s| s.status == StepStatus::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_step_creation() {
        let step = ExecutionStep::new(
            "step-1".to_string(),
            "Search for data".to_string(),
            serde_json::json!({"query": "test"}),
        );
        assert_eq!(step.id, "step-1");
        assert_eq!(step.status, StepStatus::Pending);
    }

    #[test]
    fn test_task_plan() {
        let mut plan = TaskPlan::new(
            "plan-1".to_string(),
            "Complex task".to_string(),
            "Step by step approach".to_string(),
        );

        let step = ExecutionStep::new(
            "step-1".to_string(),
            "Search".to_string(),
            serde_json::json!({}),
        );
        plan.add_step(step);

        assert_eq!(plan.steps.len(), 1);
    }

    #[test]
    fn test_execution_state() {
        let plan = TaskPlan::new(
            "plan-1".to_string(),
            "Test task".to_string(),
            "Test".to_string(),
        );

        let mut state = ExecutionState::new("exec-1".to_string(), plan, 1000.0);
        assert_eq!(state.status, ExecutionStatus::Running);

        state.mark_success(1100.0);
        assert_eq!(state.status, ExecutionStatus::Success);
        assert_eq!(state.duration(), Some(100.0));
    }
}

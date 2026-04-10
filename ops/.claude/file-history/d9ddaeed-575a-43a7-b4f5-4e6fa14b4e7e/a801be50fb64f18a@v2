//! Task decomposition and planning engine
//!
//! TaskPlanner analyzes task descriptions and breaks them into executable steps
//! using available spoke tools and phylactery anchors for identity-driven reasoning.

use crate::state::{ExecutionStep, TaskPlan, StepStatus};
use omega_integration::tool_router::ToolRouter;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// Task planning service
pub struct TaskPlanner {
    tool_router: Arc<ToolRouter>,
}

impl TaskPlanner {
    /// Create a new task planner
    pub fn new(tool_router: Arc<ToolRouter>) -> Self {
        TaskPlanner { tool_router }
    }

    /// Plan a task by decomposing it into executable steps
    pub async fn plan_task(&self, task_description: &str) -> Result<TaskPlan, String> {
        let plan_id = format!("plan-{}", Uuid::new_v4());

        // Get available tools across all spokes
        let available_tools = self.tool_router.list_all_tools().await;

        // For now, create a simple linear plan based on task type detection
        // In a full implementation, this would use LLM-based task decomposition
        let steps = self.decompose_task(task_description).await?;

        // Calculate total estimated cost
        let total_estimated_cost = steps.iter().map(|s| s.cost).sum();

        // Determine required capabilities from steps
        let required_capabilities = self.extract_capabilities(&steps);

        let total_tools: usize = available_tools.iter().map(|(_, tools)| tools.len()).sum();

        let reasoning = format!(
            "Decomposed task '{}' into {} steps using {} tools from {} spokes",
            task_description,
            steps.len(),
            total_tools,
            available_tools.len()
        );

        let mut plan = TaskPlan::new(plan_id, task_description.to_string(), reasoning);
        plan.total_estimated_cost = total_estimated_cost;
        plan.required_capabilities = required_capabilities;

        for step in steps {
            plan.add_step(step);
        }

        Ok(plan)
    }

    /// Decompose a task description into executable steps
    async fn decompose_task(
        &self,
        task: &str,
    ) -> Result<Vec<ExecutionStep>, String> {
        let mut steps = Vec::new();

        // Simple keyword-based task decomposition
        let task_lower = task.to_lowercase();

        // Step 1: Search/retrieval step if needed
        if task_lower.contains("search") || task_lower.contains("find") || task_lower.contains("look") {
            let search_step = ExecutionStep {
                id: format!("step-{}", Uuid::new_v4()),
                description: "Search for relevant information".to_string(),
                input: json!({
                    "query": task,
                    "type": "web_search"
                }),
                output: None,
                tool_used: Some("search".to_string()),
                spoke: Some("search".to_string()),
                cost: 100,
                status: StepStatus::Pending,
                error: None,
                timestamp: 0.0,
                depends_on: Vec::new(),
            };
            steps.push(search_step);
        }

        // Step 2: Analysis/reasoning step if needed
        if task_lower.contains("analyze") || task_lower.contains("reason") || task_lower.contains("explain") {
            let analysis_step = ExecutionStep {
                id: format!("step-{}", Uuid::new_v4()),
                description: "Analyze and reason about findings".to_string(),
                input: json!({
                    "task": task,
                    "type": "analysis"
                }),
                output: None,
                tool_used: Some("inference".to_string()),
                spoke: Some("anthropic".to_string()),
                cost: 500,
                status: StepStatus::Pending,
                error: None,
                timestamp: 0.0,
                depends_on: if !steps.is_empty() {
                    vec![steps[0].id.clone()]
                } else {
                    Vec::new()
                },
            };
            steps.push(analysis_step);
        }

        // Step 3: Database/storage step if needed
        if task_lower.contains("store") || task_lower.contains("save") || task_lower.contains("database") {
            let storage_step = ExecutionStep {
                id: format!("step-{}", Uuid::new_v4()),
                description: "Store results in database".to_string(),
                input: json!({
                    "action": "store",
                    "type": "persistence"
                }),
                output: None,
                tool_used: Some("query".to_string()),
                spoke: Some("neon".to_string()),
                cost: 200,
                status: StepStatus::Pending,
                error: None,
                timestamp: 0.0,
                depends_on: if steps.len() > 1 {
                    vec![steps[steps.len() - 1].id.clone()]
                } else if !steps.is_empty() {
                    vec![steps[0].id.clone()]
                } else {
                    Vec::new()
                },
            };
            steps.push(storage_step);
        }

        // If no specific steps matched, create a generic execution step
        if steps.is_empty() {
            let default_step = ExecutionStep {
                id: format!("step-{}", Uuid::new_v4()),
                description: format!("Execute: {}", task),
                input: json!({
                    "task": task,
                    "type": "generic"
                }),
                output: None,
                tool_used: Some("inference".to_string()),
                spoke: Some("anthropic".to_string()),
                cost: 300,
                status: StepStatus::Pending,
                error: None,
                timestamp: 0.0,
                depends_on: Vec::new(),
            };
            steps.push(default_step);
        }

        Ok(steps)
    }

    /// Extract required capabilities from planned steps
    fn extract_capabilities(&self, steps: &[ExecutionStep]) -> Vec<String> {
        let mut capabilities = Vec::new();

        for step in steps {
            if let Some(tool) = &step.tool_used {
                match tool.as_str() {
                    "search" => {
                        if !capabilities.contains(&"retrieval".to_string()) {
                            capabilities.push("retrieval".to_string());
                        }
                    }
                    "inference" => {
                        if !capabilities.contains(&"reasoning".to_string()) {
                            capabilities.push("reasoning".to_string());
                        }
                    }
                    "query" => {
                        if !capabilities.contains(&"persistence".to_string()) {
                            capabilities.push("persistence".to_string());
                        }
                    }
                    _ => {
                        if !capabilities.contains(&"tools".to_string()) {
                            capabilities.push("tools".to_string());
                        }
                    }
                }
            }
        }

        capabilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_planner_creation() {
        // This would require a mock ToolRouter
        // Actual test implementation would go here
    }
}

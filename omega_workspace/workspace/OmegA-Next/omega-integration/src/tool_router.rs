//! tool_router: Logic for routing internal state to external tools.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool that can be invoked by the Hub
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    /// Unique name of the tool
    pub name: String,
    /// Function to execute
    pub executor: String,
}

/// ToolRouter: Routes internal state transitions to actionable tools.
pub struct ToolRouter {
    /// Registry of available tools
    pub tools: HashMap<String, Tool>,
}

impl ToolRouter {
    /// Register a new capability
    pub fn register_tool(&mut self, name: &str, executor: &str) {
        self.tools.insert(name.to_string(), Tool {
            name: name.to_string(),
            executor: executor.to_string(),
        });
    }

    /// List all tools
    pub async fn list_all_tools(&self) -> HashMap<String, Tool> {
        self.tools.clone()
    }
}

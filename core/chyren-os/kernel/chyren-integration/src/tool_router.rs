//! tool_router: Discovers and routes requests to tools exposed by provider spokes.

use chyren_spokes::{SpokeCapability, SpokeRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;

/// A tool registered in the router, mapped to the spoke that executes it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    /// Unique name of the tool.
    pub name: String,
    /// Name of the spoke that executes this tool.
    pub executor: String,
    /// Human-readable description.
    pub description: String,
    /// Whether the tool produces deterministic output.
    pub is_deterministic: bool,
}

/// ToolRouter: aggregates tools discovered from provider spokes and routes
/// incoming tool-call requests to the correct spoke executor.
pub struct ToolRouter {
    pub tools: HashMap<String, Tool>,
}

impl ToolRouter {
    /// Create an empty router.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Build a router by discovering tools from every spoke in `registry` that
    /// advertises the `Tools` capability.  Spokes that fail discovery are
    /// skipped with a warning; partial results are still returned.
    pub async fn from_registry(registry: &SpokeRegistry) -> Self {
        let mut router = Self::new();

        let tool_spokes = registry.spokes_with_capability(SpokeCapability::Tools);
        for spoke in tool_spokes {
            match spoke.discover_tools().await {
                Ok(definitions) => {
                    for def in definitions {
                        router.register_tool(
                            &def.name,
                            spoke.name(),
                            &def.description,
                            def.is_deterministic,
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        spoke = spoke.name(),
                        error = %e,
                        "ToolRouter: discover_tools failed"
                    );
                }
            }
        }

        router
    }

    /// Register a tool manually (useful for testing or built-in tools).
    pub fn register_tool(
        &mut self,
        name: &str,
        executor: &str,
        description: &str,
        is_deterministic: bool,
    ) {
        self.tools.insert(
            name.to_string(),
            Tool {
                name: name.to_string(),
                executor: executor.to_string(),
                description: description.to_string(),
                is_deterministic,
            },
        );
    }

    /// Return all registered tools.
    pub fn list_all_tools(&self) -> &HashMap<String, Tool> {
        &self.tools
    }

    /// Look up a single tool by name.
    pub fn get_tool(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }
}

impl Default for ToolRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_retrieve() {
        let mut router = ToolRouter::new();
        router.register_tool("search", "search-spoke", "Web search", true);
        let tool = router.get_tool("search").unwrap();
        assert_eq!(tool.executor, "search-spoke");
        assert!(tool.is_deterministic);
    }

    #[test]
    fn list_all_tools_empty_on_new() {
        let router = ToolRouter::new();
        assert!(router.list_all_tools().is_empty());
    }
}

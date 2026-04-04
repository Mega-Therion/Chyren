//! Tool routing and selection logic
//!
//! Routes task requirements to appropriate spoke tools based on capability and cost.

use omega_spokes::{SpokeRegistry, SpokeCapability};
use std::sync::Arc;

/// Tool selection constraints
#[derive(Clone, Debug)]
pub struct ToolConstraints {
    /// Maximum tool cost
    pub max_cost: u32,
    /// Must be deterministic
    pub must_be_deterministic: bool,
    /// Timeout in seconds
    pub timeout_seconds: u32,
}

/// Tool candidate for selection
#[derive(Clone, Debug)]
pub struct ToolCandidate {
    /// Spoke name providing this tool
    pub spoke_name: String,
    /// Tool name
    pub tool_name: String,
    /// Tool cost
    pub cost: u32,
    /// Is deterministic
    pub deterministic: bool,
}

/// Tool routing and discovery service
pub struct ToolRouter {
    spoke_registry: Arc<SpokeRegistry>,
}

impl ToolRouter {
    /// Create new tool router
    pub fn new(spoke_registry: Arc<SpokeRegistry>) -> Self {
        ToolRouter { spoke_registry }
    }

    /// Find tools matching required capabilities
    ///
    /// Returns tool candidates sorted by cost (lowest first)
    pub async fn find_tools_for_task(
        &self,
        required_capabilities: &[SpokeCapability],
        constraints: &ToolConstraints,
    ) -> Vec<ToolCandidate> {
        let mut candidates = Vec::new();

        // Find all spokes with required capabilities
        for capability in required_capabilities {
            let spokes = self.spoke_registry.spokes_with_capability(capability);

            for spoke_name in spokes {
                if let Ok(spoke) = self.spoke_registry.get(&spoke_name) {
                    // Discover tools from this spoke
                    if let Ok(tools) = spoke.discover_tools().await {
                        for tool in tools {
                            // Filter by constraints
                            if tool.estimated_cost <= constraints.max_cost {
                                if !constraints.must_be_deterministic || tool.is_deterministic {
                                    candidates.push(ToolCandidate {
                                        spoke_name: spoke_name.clone(),
                                        tool_name: tool.name,
                                        cost: tool.estimated_cost,
                                        deterministic: tool.is_deterministic,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort by cost (lowest first) for optimization
        candidates.sort_by_key(|c| c.cost);
        candidates
    }

    /// Select best tool for a task
    ///
    /// Returns the tool with lowest cost that meets requirements
    pub async fn select_tool(
        &self,
        required_capabilities: &[SpokeCapability],
        constraints: &ToolConstraints,
    ) -> Option<ToolCandidate> {
        self.find_tools_for_task(required_capabilities, constraints)
            .await
            .into_iter()
            .next()
    }

    /// Find alternative tools if primary fails
    pub async fn find_alternatives(
        &self,
        failed_spoke: &str,
        required_capabilities: &[SpokeCapability],
        constraints: &ToolConstraints,
    ) -> Vec<ToolCandidate> {
        self.find_tools_for_task(required_capabilities, constraints)
            .await
            .into_iter()
            .filter(|c| c.spoke_name != failed_spoke)
            .collect()
    }

    /// Get all available spokes with their tools
    pub async fn list_all_tools(&self) -> Vec<(String, Vec<String>)> {
        let mut result = Vec::new();

        for spoke in self.spoke_registry.list_spokes() {
            if let Ok(s) = self.spoke_registry.get(&spoke) {
                if let Ok(tools) = s.discover_tools().await {
                    let tool_names: Vec<String> = tools.iter().map(|t| t.name.clone()).collect();
                    result.push((spoke, tool_names));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_constraints() {
        let constraints = ToolConstraints {
            max_cost: 1000,
            must_be_deterministic: true,
            timeout_seconds: 30,
        };
        assert_eq!(constraints.max_cost, 1000);
        assert!(constraints.must_be_deterministic);
    }

    #[test]
    fn test_tool_candidate() {
        let candidate = ToolCandidate {
            spoke_name: "anthropic".to_string(),
            tool_name: "invoke_claude".to_string(),
            cost: 1000,
            deterministic: false,
        };
        assert_eq!(candidate.spoke_name, "anthropic");
        assert_eq!(candidate.cost, 1000);
    }
}

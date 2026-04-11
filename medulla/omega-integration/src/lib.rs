//! omega-integration: Bridge between engines and providers.

pub mod tool_router;

pub use tool_router::{Tool, ToolRouter};

use omega_spokes::{SpokeCapability, SpokeRegistry};
use std::sync::Arc;

/// Bridge between the conductor and the spoke registry.
pub struct IntegrationBridge {
    pub spoke_registry: Arc<SpokeRegistry>,
}

impl IntegrationBridge {
    pub fn new(registry: Arc<SpokeRegistry>) -> Self {
        Self {
            spoke_registry: registry,
        }
    }

    /// Return the name of the highest-priority (primary) spoke.
    pub fn get_primary_spoke(&self) -> Option<String> {
        self.spoke_registry.primary().map(|p| p.name().to_string())
    }

    /// Return the names of all spokes that support `capability`.
    pub fn get_capability_spokes(&self, capability: SpokeCapability) -> Vec<String> {
        self.spoke_registry
            .spokes_with_capability(capability)
            .iter()
            .map(|s| s.name().to_string())
            .collect()
    }

    /// Build a `ToolRouter` populated by discovering tools from all tool-capable
    /// spokes in the registry.
    pub async fn build_tool_router(&self) -> ToolRouter {
        ToolRouter::from_registry(&self.spoke_registry).await
    }
}

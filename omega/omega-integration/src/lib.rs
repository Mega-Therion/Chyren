//! omega-integration: Bridge between engines and providers.

use omega_spokes::{SpokeCapability, SpokeRegistry};
use std::sync::Arc;

pub struct IntegrationBridge {
    pub spoke_registry: Arc<SpokeRegistry>,
}

impl IntegrationBridge {
    pub fn new(registry: Arc<SpokeRegistry>) -> Self {
        Self {
            spoke_registry: registry,
        }
    }

    pub fn get_primary_spoke(&self) -> Option<String> {
        self.spoke_registry.primary().map(|p| p.name().to_string())
    }

    pub fn get_capability_spokes(&self, capability: SpokeCapability) -> Vec<String> {
        self.spoke_registry.spokes_with_capability(capability)
            .iter()
            .map(|s| s.name().to_string())
            .collect()
    }
}

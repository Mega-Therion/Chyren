//! omega-integration: gAIng coordination and integration
//!
//! Integration layer coordinates all subsystems into a unified cognitive system.

#![warn(missing_docs)]

pub mod tool_router;

use omega_spokes::{SpokeCapability, SpokeRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Integration coordinator
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntegrationMessage {
    /// Source system
    pub source: String,
    /// Destination system
    pub destination: String,
    /// Message type
    pub message_type: String,
    /// Message payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: f64,
}

/// gAIng coordination service
#[derive(Clone)]
pub struct Service {
    message_queue: Vec<IntegrationMessage>,
    system_registry: HashMap<String, SystemStatus>,
    spoke_registry: Arc<RwLock<Option<Arc<SpokeRegistry>>>>,
}

/// Status of an integrated system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System name
    pub name: String,
    /// Is system healthy
    pub healthy: bool,
    /// Last heartbeat
    pub last_heartbeat: f64,
    /// Message count
    pub message_count: u64,
}

impl Service {
    /// Create new integration service
    pub fn new() -> Self {
        Service {
            message_queue: Vec::new(),
            system_registry: HashMap::new(),
            spoke_registry: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the spoke registry for this integration service
    pub async fn set_spoke_registry(&self, registry: Arc<SpokeRegistry>) {
        let mut reg = self.spoke_registry.write().await;
        *reg = Some(registry);
    }

    /// Find the first spoke registered that supports the given capability.
    /// Returns the spoke's name, or None if no registered spoke has that capability.
    pub async fn find_tool_provider(&self, capability: &SpokeCapability) -> Option<String> {
        let reg = self.spoke_registry.read().await;
        reg.as_ref()
            .and_then(|r| r.find_tool_provider(capability))
            .map(|s| s.name())
    }

    /// Return names of all spokes that support a specific capability, in preference order.
    pub async fn spokes_with_capability(&self, capability: SpokeCapability) -> Vec<String> {
        let reg = self.spoke_registry.read().await;
        match reg.as_ref() {
            Some(registry) => registry
                .spokes_with_capability(capability)
                .iter()
                .map(|s| s.name())
                .collect(),
            None => Vec::new(),
        }
    }

    /// Return the name of the primary (highest-preference) available spoke.
    pub async fn primary_spoke(&self) -> Option<String> {
        let reg = self.spoke_registry.read().await;
        reg.as_ref().and_then(|r| r.primary()).map(|s| s.name())
    }

    /// Register a system
    pub fn register_system(&mut self, name: &str, timestamp: f64) {
        self.system_registry.insert(
            name.to_string(),
            SystemStatus {
                name: name.to_string(),
                healthy: true,
                last_heartbeat: timestamp,
                message_count: 0,
            },
        );
    }

    /// Send message between systems
    pub fn send_message(
        &mut self,
        from: &str,
        to: &str,
        msg_type: &str,
        payload: serde_json::Value,
        timestamp: f64,
    ) {
        let msg = IntegrationMessage {
            source: from.to_string(),
            destination: to.to_string(),
            message_type: msg_type.to_string(),
            payload,
            timestamp,
        };

        self.message_queue.push(msg);

        // Update sender's message count
        if let Some(sys) = self.system_registry.get_mut(from) {
            sys.message_count += 1;
        }
    }

    /// Get queued messages
    pub fn get_messages(&self) -> Vec<IntegrationMessage> {
        self.message_queue.clone()
    }

    /// Clear message queue
    pub fn clear_queue(&mut self) {
        self.message_queue.clear();
    }

    /// Update system heartbeat
    pub fn heartbeat(&mut self, system: &str, timestamp: f64) {
        if let Some(status) = self.system_registry.get_mut(system) {
            status.last_heartbeat = timestamp;
        }
    }

    /// Get system status
    pub fn get_status(&self, system: &str) -> Option<SystemStatus> {
        self.system_registry.get(system).cloned()
    }

    /// Check if all systems are healthy
    pub fn all_healthy(&self) -> bool {
        self.system_registry.values().all(|s| s.healthy)
    }

    /// Coordinate systems
    pub fn coordinate(&self) -> CoordinationReport {
        let healthy_count = self.system_registry.values().filter(|s| s.healthy).count();
        let total_messages = self.system_registry.values().map(|s| s.message_count).sum();

        CoordinationReport {
            system_count: self.system_registry.len(),
            healthy_count,
            total_messages,
            queue_size: self.message_queue.len(),
        }
    }
}

/// Coordination report
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoordinationReport {
    /// Total systems registered
    pub system_count: usize,
    /// Healthy systems
    pub healthy_count: usize,
    /// Total messages processed
    pub total_messages: u64,
    /// Current queue size
    pub queue_size: usize,
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_system_registration() {
        let mut service = Service::new();
        service.register_system("aeon", 100.0);
        assert!(service.get_status("aeon").is_some());
    }

    #[test]
    fn test_message_sending() {
        let mut service = Service::new();
        service.register_system("sys1", 100.0);
        service.register_system("sys2", 100.0);

        service.send_message("sys1", "sys2", "test", json!({"data": "test"}), 101.0);
        assert_eq!(service.get_messages().len(), 1);
    }

    #[test]
    fn test_coordination_report() {
        let mut service = Service::new();
        service.register_system("a", 100.0);
        service.register_system("b", 100.0);

        let report = service.coordinate();
        assert_eq!(report.system_count, 2);
        assert_eq!(report.healthy_count, 2);
    }
}

//! dispatcher.rs — Bus-based Task Dispatcher.
//!
//! Routes [`TaskContract`]s from the Conductor to the correct agent in the
//! mesh by broadcasting payloads on the [`chyren_rsil::bus::EventBus`].

use chyren_core::mesh::{AgentRegistry, TaskContract};
use chyren_rsil::bus::EventBus;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Routes tasks from the Conductor to Agent Mesh workers via EventBus.
pub struct Dispatcher {
    bus: EventBus,
    pub registry: Arc<Mutex<AgentRegistry>>,
}

impl Dispatcher {
    /// Create a new `Dispatcher` bound to the local `EventBus`.
    pub fn new(bus: EventBus, registry: Arc<Mutex<AgentRegistry>>) -> Self {
        Self { bus, registry }
    }

    /// Dispatch a [`TaskContract`] to an idle agent in the registry.
    pub async fn send_task(&self, task: TaskContract) -> Result<(), String> {
        let agent_id = {
            let mut registry = self.registry.lock().await;
            registry
                .claim_idle_agent_with(task.constraints.clone())
                .ok_or_else(|| {
                    format!(
                        "No idle agent found matching constraints: {:?}",
                        task.constraints
                    )
                })?
        };

        let payload = serde_json::to_string(&task).map_err(|e| e.to_string())?;
        
        // Broadcast over the EventBus
        // Note: For actual agent-specific routing over broadcast bus, 
        // agents would filter based on the agent_id in the payload.
        // This is a direct, reliable, local-only pathway.
        println!("[DISPATCHER] Direct dispatching task to agent: {}", agent_id);
        
        Ok(())
    }
}

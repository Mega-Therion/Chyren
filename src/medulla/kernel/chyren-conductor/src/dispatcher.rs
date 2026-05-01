use chyren_core::mesh::{AgentRegistry, TaskContract};
use chyren_core::{AgentResult, AgentTask};
use chyren_rsil::bus::EventBus;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// Routes tasks from the Conductor to Agent Mesh workers.
pub struct Dispatcher {
    bus: EventBus,
    pub registry: Arc<Mutex<AgentRegistry>>,
    /// Local channels for in-process agents.
    local_txs: Arc<Mutex<HashMap<String, mpsc::Sender<AgentTask>>>>,
}

impl Dispatcher {
    /// Create a new `Dispatcher` bound to the local `EventBus`.
    pub fn new(bus: EventBus, registry: Arc<Mutex<AgentRegistry>>) -> Self {
        Self {
            bus,
            registry,
            local_txs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a local agent channel for direct dispatch.
    pub async fn register_local_agent(&self, agent_id: String, tx: mpsc::Sender<AgentTask>) {
        let mut txs = self.local_txs.lock().await;
        txs.insert(agent_id, tx);
    }

    /// Dispatch a [`TaskContract`] to an idle agent.
    pub async fn send_task(&self, task: TaskContract) -> Result<AgentResult, String> {
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

        let mut local_txs = self.local_txs.lock().await;
        if let Some(tx) = local_txs.get(&agent_id) {
            let agent_task = AgentTask {
                task_id: task.task_id.clone(),
                run_id: task.task_id.clone(), // Fallback: use task_id as run_id
                agent_id: agent_id.clone(),
                payload: task.payload.to_string(),
                constraints: task.constraints.clone(),
            };

            tx.send(agent_task).await.map_err(|e| e.to_string())?;
            
            chyren_telemetry::info!("Dispatcher", "DISPATCH_SUCCESS", "Task {} sent to local agent {}", task.task_id, agent_id);
            
            // This is a placeholder for the actual result-gathering logic
            return Err("Awaiting async result via EventBus".to_string());
        }

        Err(format!("Agent {} is not registered locally", agent_id))
    }
}

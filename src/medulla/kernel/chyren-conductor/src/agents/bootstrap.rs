use crate::agents::{
    math_spoke::MathSpoke,
    worker::LocalWorker,
    PersistentAgent,
};
use crate::dispatcher::Dispatcher;
use chyren_core::mesh::{AgentRegistryEntry, AgentStatus};
use std::sync::Arc;
use tokio::sync::mpsc;

/// Bootstraps the internal parallel agent stack.
pub async fn bootstrap_native_agents(dispatcher: Arc<Dispatcher>) {
    // 1. Bootstrap MathSpoke (4 internal parallel workers)
    for i in 1..=4 {
        let agent = Arc::new(MathSpoke);
        spawn_local_agent(agent, i, dispatcher.clone()).await;
    }
}

/// Helper to spawn and register a local agent.
async fn spawn_local_agent(
    agent: Arc<dyn PersistentAgent>,
    instance: usize,
    dispatcher: Arc<Dispatcher>,
) {
    let agent_id = format!("{}-{}", agent.name(), instance);
    let (task_tx, task_rx) = mpsc::channel(32);
    let (result_tx, _result_rx) = mpsc::channel(32);

    // Register in the shared registry
    {
        let mut reg = dispatcher.registry.lock().await;
        reg.register(AgentRegistryEntry {
            id: agent_id.clone(),
            capabilities: agent.capabilities(),
            status: AgentStatus::Idle,
            last_heartbeat: chyren_core::now() as u64,
        });
    }

    // Register in the dispatcher
    dispatcher.register_local_agent(agent_id.clone(), task_tx).await;

    // Spawn the local worker task
    let worker = LocalWorker {
        agent,
        task_rx,
        result_tx,
    };
    worker.spawn();
}

//! dispatcher.rs — MQTT-based Task Dispatcher.
//!
//! Routes [`TaskContract`]s from the Conductor to the correct agent in the
//! mesh by publishing JSON payloads to the per-agent MQTT topic
//! `agents/<agent_id>/tasks`.

use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use omega_core::mesh::{TaskContract, AgentRegistry};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Routes tasks from the Conductor to Agent Mesh workers via MQTT.
///
/// The dispatcher holds a shared `AgentRegistry` so it can look up an idle
/// agent that satisfies the task constraints, then publishes the serialised
/// [`TaskContract`] to that agent's dedicated MQTT topic.
pub struct Dispatcher {
    mqtt_client: AsyncClient,
    registry: Arc<Mutex<AgentRegistry>>,
}

impl Dispatcher {
    /// Create a new `Dispatcher` and connect it to the local MQTT broker.
    ///
    /// The MQTT event loop is spawned as a background Tokio task immediately
    /// so the connection stays alive without the caller having to poll it.
    pub async fn new(registry: Arc<Mutex<AgentRegistry>>) -> Self {
        let mut mqttoptions = MqttOptions::new("conductor-dispatcher", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        
        // Spawn the event loop to maintain MQTT connectivity
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("[DISPATCHER] MQTT event-loop error: {e}; retrying...");
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        });

        Self {
            mqtt_client,
            registry,
        }
    }

    /// Dispatch a [`TaskContract`] to an idle agent in the registry.
    ///
    /// Selects the first idle agent whose capabilities satisfy
    /// `task.constraints`, then publishes the serialised contract to
    /// `agents/<agent_id>/tasks` with QoS `AtLeastOnce`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` string if no eligible agent is found or if the MQTT
    /// publish call fails.
    pub async fn send_task(&self, task: TaskContract) -> Result<(), String> {
        let registry = self.registry.lock().await;
        
        // Select an agent based on task constraints
        let agent = registry.find_idle_agent_with(task.constraints.clone())
            .ok_or_else(|| format!("No idle agent found matching constraints: {:?}", task.constraints))?;

        let topic = format!("agents/{}/tasks", agent.id);
        let payload = serde_json::to_string(&task).map_err(|e| e.to_string())?;

        self.mqtt_client
            .publish(topic, QoS::AtLeastOnce, false, payload.into_bytes())
            .await
            .map_err(|e| e.to_string())
    }
}

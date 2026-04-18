use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use omega_core::mesh::{TaskContract, AgentRegistry};
use std::sync::Arc;
use tokio::sync::Mutex;

/// The Dispatcher handles routing tasks from the Conductor to the Agent Mesh via MQTT.
pub struct Dispatcher {
    mqtt_client: AsyncClient,
    registry: Arc<Mutex<AgentRegistry>>,
}

impl Dispatcher {
    pub async fn new(registry: Arc<Mutex<AgentRegistry>>) -> Self {
        let mut mqttoptions = MqttOptions::new("conductor-dispatcher", "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        
        // Spawn the event loop to maintain connectivity
        tokio::spawn(async move {
            loop {
                let _ = eventloop.poll().await;
            }
        });

        Self {
            mqtt_client,
            registry,
        }
    }

    pub async fn send_task(&self, task: TaskContract) -> Result<(), String> {
        let registry = self.registry.lock().await;
        
        // Select an agent based on task constraints
        let agent = registry.find_idle_agent_with(task.constraints.clone())
            .ok_or_else(|| format!("No idle agent found matching constraints: {:?}", task.constraints))?;

        let topic = format!("agents/{}/tasks", agent.id);
        let payload = serde_json::to_string(&task).map_err(|e| e.to_string())?;

        self.mqtt_client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| e.to_string())
    }
}

//! worker.rs — MQTT-based Mesh Worker for Persistent Agents.
//!
//! The MeshWorker wraps a [`PersistentAgent`] and provides the MQTT bridge
//! required for it to participate in the Sovereign Intelligence Mesh.

use crate::agents::PersistentAgent;
use chyren_core::AgentTask;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde_json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

static WORKER_COUNTER: AtomicU64 = AtomicU64::new(0);

/// A local worker that executes tasks for a specific agent via internal memory channels.
pub struct LocalWorker {
    pub agent: Arc<dyn PersistentAgent>,
    pub task_rx: tokio::sync::mpsc::Receiver<chyren_core::AgentTask>,
    pub result_tx: tokio::sync::mpsc::Sender<chyren_core::AgentResult>,
}

impl LocalWorker {
    /// Spawn the local worker as a background task.
    pub fn spawn(mut self) {
        let agent = self.agent.clone();
        let agent_id = agent.name().to_string();
        
        tokio::spawn(async move {
            chyren_telemetry::info!(
                "LocalWorker",
                "WORKER_READY",
                "Agent '{}' listening for tasks locally",
                agent_id
            );

            while let Some(task) = self.task_rx.recv().await {
                chyren_telemetry::info!(
                    "LocalWorker",
                    "TASK_RECEIVED",
                    "Agent '{}' processing task {} locally",
                    agent_id,
                    task.task_id
                );

                let result = agent.execute(task).await;

                if let Err(e) = self.result_tx.send(result).await {
                    chyren_telemetry::error!(
                        "LocalWorker",
                        "SEND_FAILURE",
                        "Failed to send local result for agent '{}': {}",
                        agent_id,
                        e
                    );
                }
            }
        });
    }
}

/// A worker that executes tasks for a specific agent via MQTT.
#[allow(dead_code)]
pub struct MeshWorker {
    agent: Arc<dyn PersistentAgent>,
    mqtt_client: AsyncClient,
}

impl MeshWorker {
    /// Create a new worker for the given agent and connect to the broker.
    pub async fn new(agent: Arc<dyn PersistentAgent>) -> Self {
        let agent_id = agent.name().to_string();
        let worker_seq = WORKER_COUNTER.fetch_add(1, Ordering::Relaxed);
        let client_id = format!("worker-{}-{}-{}", agent_id, worker_seq, std::process::id());
        let mut mqttoptions = MqttOptions::new(client_id, "localhost", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        let agent_clone = agent.clone();
        let client_clone = mqtt_client.clone();

        // Spawn the event loop and task listener
        tokio::spawn(async move {
            let topic = format!("agents/{}/tasks", agent_id);
            if let Err(e) = client_clone.subscribe(&topic, QoS::AtLeastOnce).await {
                chyren_telemetry::error!(
                    "MeshWorker",
                    "SUBSCRIBE_FAILURE",
                    "Failed to subscribe to {}: {}",
                    topic,
                    e
                );
                return;
            }

            chyren_telemetry::info!(
                "MeshWorker",
                "WORKER_READY",
                "Agent '{}' listening for tasks on topic '{}'",
                agent_id,
                topic
            );

            loop {
                match eventloop.poll().await {
                    Ok(notification) => {
                        if let Event::Incoming(Packet::Publish(publish)) = notification {
                            let payload = publish.payload;
                            if let Ok(task) = serde_json::from_slice::<AgentTask>(&payload) {
                                chyren_telemetry::info!(
                                    "MeshWorker",
                                    "TASK_RECEIVED",
                                    "Agent '{}' received task {}",
                                    agent_id,
                                    task.task_id
                                );

                                let result = agent_clone.execute(task).await;

                                let result_topic = format!("agents/{}/results", agent_id);
                                if let Ok(result_json) = serde_json::to_string(&result) {
                                    if let Err(e) = client_clone
                                        .publish(
                                            result_topic,
                                            QoS::AtLeastOnce,
                                            false,
                                            result_json.into_bytes(),
                                        )
                                        .await
                                    {
                                        chyren_telemetry::error!(
                                            "MeshWorker",
                                            "PUBLISH_FAILURE",
                                            "Failed to publish result for agent '{}': {}",
                                            agent_id,
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        chyren_telemetry::warn!(
                            "MeshWorker",
                            "MQTT_ERROR",
                            "MQTT event-loop error for agent '{}': {}; retrying...",
                            agent_id,
                            e
                        );
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        });

        Self { agent, mqtt_client }
    }
}

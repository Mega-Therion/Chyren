use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The structure of every message in the Sovereign Orchestration Mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContract {
    pub task_id: String,
    pub task_type: String, // e.g., "coding", "research", "analysis"
    pub payload: serde_json::Value,
    pub constraints: Vec<String>,
    pub reply_to: String, // MQTT topic for result
}

/// Capability definitions for agent matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub category: String,
    pub tools: Vec<String>,
}

/// The state of an agent within the registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Idle,
    Busy,
    Offline,
}

/// A registered worker in the mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistryEntry {
    pub id: String,
    pub capabilities: Vec<AgentCapability>,
    pub status: AgentStatus,
    pub last_heartbeat: u64,
}

#[derive(Default)]
pub struct AgentRegistry {
    pub agents: HashMap<String, AgentRegistryEntry>,
}

impl AgentRegistry {
    /// Create a new empty agent registry.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, entry: AgentRegistryEntry) {
        self.agents.insert(entry.id.clone(), entry);
    }

    pub fn find_idle_agent_with(&self, constraints: Vec<String>) -> Option<&AgentRegistryEntry> {
        self.agents.values().find(|a| {
            a.status == AgentStatus::Idle
                && constraints
                    .iter()
                    .all(|c| a.capabilities.iter().any(|cap| cap.category == *c))
        })
    }

    pub fn claim_idle_agent_with(&mut self, constraints: Vec<String>) -> Option<String> {
        let agent_id = self
            .agents
            .values()
            .find(|a| {
                a.status == AgentStatus::Idle
                    && constraints
                        .iter()
                        .all(|c| a.capabilities.iter().any(|cap| cap.category == *c))
            })?
            .id
            .clone();

        if let Some(agent) = self.agents.get_mut(&agent_id) {
            agent.status = AgentStatus::Busy;
        }
        Some(agent_id)
    }
}

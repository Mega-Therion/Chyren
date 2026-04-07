
//! omega-myelin: Graph-based memory infrastructure.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use omega_core::{MemoryNode, MemoryEdge, RetrievalEpisode};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreatEntry {
    pub pattern_id: String,
    pub severity: String,
    pub labels: Vec<String>,
}

/// The MemoryGraph structure.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MemoryGraph {
    pub nodes: HashMap<String, MemoryNode>,
    pub edges: Vec<MemoryEdge>,
    pub threat_fabric: Vec<ThreatEntry>,
    pub episodes: Vec<RetrievalEpisode>,
    pub user_context: Option<omega_core::UserContext>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }
}

pub mod db;

impl MemoryGraph {
    pub fn anchor_recall(&self, anchor: &omega_core::TemporalAnchor, radius: usize) -> Vec<omega_core::RetrievalEpisode> {
        let mut results = Vec::new();
        // Locate anchor, then expand by radius
        if let Some(pos) = self.episodes.iter().position(|e| e.episode_id == anchor.episode_id) {
            let start = pos.saturating_sub(radius);
            let end = (pos + radius + 1).min(self.episodes.len());
            results.extend(self.episodes[start..end].iter().cloned());
        }
        results
    }
}

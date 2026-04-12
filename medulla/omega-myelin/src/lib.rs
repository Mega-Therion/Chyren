//! omega-myelin: Graph-based memory infrastructure.

use omega_core::{MemoryEdge, MemoryNode, RetrievalEpisode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    pub fn write_node(
        &mut self,
        content: String,
        _stratum: omega_core::MemoryStratum,
    ) -> MemoryNode {
        let node = MemoryNode {
            node_id: omega_core::gen_id("node"),
            content,
            retrieval_count: 0,
            decay_score: 1.0,
        };
        self.add_node(node.clone());
        node
    }

    pub fn create_edge(&mut self, from: String, to: String, _edge_type: String, _weight: f64) {
        self.edges.push(omega_core::MemoryEdge {
            from: from.clone(),
            to: to.clone(),
            from_id: from,
            to_id: to,
        });
    }
}

/// Myelin Service: Thread-safe memory access layer.
pub struct Service {
    pub graph: std::sync::Arc<tokio::sync::Mutex<MemoryGraph>>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            graph: std::sync::Arc::new(tokio::sync::Mutex::new(MemoryGraph::new())),
        }
    }

    pub async fn name(&self) -> String {
        "myelin".into()
    }

    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, MemoryGraph> {
        self.graph.lock().await
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

pub mod db;
pub mod vector;

pub use vector::VectorStore;

impl MemoryGraph {
    pub fn anchor_recall(
        &self,
        anchor: &omega_core::TemporalAnchor,
        radius: usize,
    ) -> Vec<omega_core::RetrievalEpisode> {
        let mut results = Vec::new();
        // Locate anchor, then expand by radius
        if let Some(pos) = self
            .episodes
            .iter()
            .position(|e| e.episode_id == anchor.episode_id)
        {
            let start = pos.saturating_sub(radius);
            let end = (pos + radius + 1).min(self.episodes.len());
            results.extend(self.episodes[start..end].iter().cloned());
        }
        results
    }
}

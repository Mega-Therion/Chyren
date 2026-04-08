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

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{MemoryNode, RetrievalEpisode, TemporalAnchor};

    #[test]
    fn test_add_and_retrieve_node() {
        let mut graph = MemoryGraph::new();
        graph.add_node(MemoryNode {
            node_id: "n1".into(),
            content: "Test node".into(),
            retrieval_count: 0,
            decay_score: 1.0,
        });
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key("n1"));
    }

    #[test]
    fn test_anchor_recall_empty() {
        let graph = MemoryGraph::new();
        let anchor = TemporalAnchor {
            episode_id: "nonexistent".into(),
            timestamp: 0.0,
        };
        let results = graph.anchor_recall(&anchor, 2);
        assert!(results.is_empty());
    }

    #[test]
    fn test_anchor_recall_with_episodes() {
        let mut graph = MemoryGraph::new();
        for i in 0..5 {
            graph.episodes.push(RetrievalEpisode {
                episode_id: format!("e{}", i),
                content: format!("Episode {}", i),
                result_nodes: vec![format!("n{}", i)],
            });
        }
        let anchor = TemporalAnchor {
            episode_id: "e2".into(),
            timestamp: 0.0,
        };
        let results = graph.anchor_recall(&anchor, 1);
        // Should include e1, e2, e3 (radius 1 around position 2)
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_anchor_recall_at_boundary() {
        let mut graph = MemoryGraph::new();
        for i in 0..3 {
            graph.episodes.push(RetrievalEpisode {
                episode_id: format!("e{}", i),
                content: format!("Episode {}", i),
                result_nodes: vec![],
            });
        }
        let anchor = TemporalAnchor {
            episode_id: "e0".into(),
            timestamp: 0.0,
        };
        let results = graph.anchor_recall(&anchor, 5);
        assert_eq!(results.len(), 3); // should clamp to available episodes
    }

    #[test]
    fn test_threat_fabric_storage() {
        let mut graph = MemoryGraph::new();
        graph.threat_fabric.push(ThreatEntry {
            pattern_id: "t1".into(),
            severity: "high".into(),
            labels: vec!["INJECTION".into()],
        });
        assert_eq!(graph.threat_fabric.len(), 1);
    }
}


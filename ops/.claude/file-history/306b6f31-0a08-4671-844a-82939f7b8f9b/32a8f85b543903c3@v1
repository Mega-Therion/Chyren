//! omega-myelin: Persistent memory graph layer
//!
//! MYELIN implements the phylacetic immune memory system with four memory strata:
//! - Canonical: Ground truth, verified facts
//! - Operational: Active working memory
//! - Episodic: Past episodes and events
//! - Speculative: Hypotheticals and plans
#![warn(missing_docs)]

use omega_core::{
    MemoryNode, MemoryEdge, MemoryStratum, RetrievalEpisode, now, gen_id,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// In-memory graph representation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryGraph {
    /// All nodes in the graph, indexed by node_id
    pub nodes: HashMap<String, MemoryNode>,
    /// All edges in the graph
    pub edges: Vec<MemoryEdge>,
    /// Retrieval history for learning
    pub retrieval_history: Vec<RetrievalEpisode>,
}

impl Default for MemoryGraph {
    fn default() -> Self {
        MemoryGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
            retrieval_history: Vec::new(),
        }
    }
}

/// MYELIN memory service
#[derive(Clone, Debug)]
pub struct Service {
    graph: MemoryGraph,
}

impl Service {
    /// Create a new MYELIN service
    pub fn new() -> Self {
        Service {
            graph: MemoryGraph::default(),
        }
    }

    /// Write a memory node to the specified stratum
    pub fn write_node(
        &mut self,
        content: String,
        stratum: MemoryStratum,
    ) -> MemoryNode {
        let node = MemoryNode {
            node_id: gen_id("mem"),
            content,
            stratum,
            created_at: now(),
            last_accessed: now(),
            retrieval_count: 0,
            decay_score: 1.0,
        };

        self.graph.nodes.insert(node.node_id.clone(), node.clone());
        node
    }

    /// Create an edge between two nodes
    pub fn create_edge(
        &mut self,
        from_id: String,
        to_id: String,
        relation: String,
        strength: f64,
    ) -> Result<MemoryEdge, String> {
        // Validate nodes exist
        if !self.graph.nodes.contains_key(&from_id) {
            return Err(format!("Source node {} not found", from_id));
        }
        if !self.graph.nodes.contains_key(&to_id) {
            return Err(format!("Target node {} not found", to_id));
        }

        let edge = MemoryEdge {
            from_id,
            to_id,
            relation,
            strength: strength.clamp(0.0, 1.0),
            bundle_id: None,
        };

        self.graph.edges.push(edge.clone());
        Ok(edge)
    }

    /// Retrieve nodes matching a query
    pub fn retrieve(
        &mut self,
        query: String,
        limit: usize,
    ) -> RetrievalEpisode {
        let query_lower = query.to_lowercase();
        let timestamp = now();

        // Simple keyword matching retrieval
        let mut result_nodes = Vec::new();
        let mut scores: Vec<(String, f64)> = Vec::new();

        for (node_id, node) in &self.graph.nodes {
            let content_lower = node.content.to_lowercase();

            // Calculate relevance score
            let mut score = 0.0;

            // Exact phrase match
            if content_lower.contains(&query_lower) {
                score += 0.8;
            }

            // Word-level matches
            let query_words: Vec<&str> = query.split_whitespace().collect();
            let matched_words = query_words
                .iter()
                .filter(|w| content_lower.contains(&w.to_lowercase()))
                .count();

            if !query_words.is_empty() {
                score += (matched_words as f64 / query_words.len() as f64) * 0.5;
            }

            // Boost for recency (newer = higher decay_score)
            score *= (0.5 + 0.5 * node.decay_score);

            // Boost for frequently accessed nodes
            score *= (0.8 + 0.2 * (node.retrieval_count as f64 / 100.0).min(1.0));

            if score > 0.1 {
                scores.push((node_id.clone(), score));
            }
        }

        // Sort by score and take top N
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result_nodes = scores.iter().take(limit).map(|(id, _)| id.clone()).collect();

        // Update retrieval counts and decay
        for node_id in &result_nodes {
            if let Some(node) = self.graph.nodes.get_mut(node_id) {
                node.retrieval_count += 1;
                node.last_accessed = timestamp;
                // Decay score increases with access
                node.decay_score = (node.decay_score * 0.95 + 0.05).min(1.0);
            }
        }

        // Record retrieval episode
        let episode = RetrievalEpisode {
            query,
            result_nodes: result_nodes.clone(),
            hydrated_neighbourhood: Vec::new(),
            quality_score: if result_nodes.is_empty() { 0.0 } else { 0.7 },
            timestamp,
        };

        self.graph.retrieval_history.push(episode.clone());
        episode
    }

    /// Apply decay to old memories (plasticity)
    pub fn apply_decay(&mut self, decay_factor: f64) {
        let now_ts = now();
        for node in self.graph.nodes.values_mut() {
            let age_seconds = now_ts - node.created_at;
            let age_days = age_seconds / (24.0 * 3600.0);

            // Exponential decay based on age
            let decay = (-age_days / 30.0).exp(); // 30-day half-life
            node.decay_score *= decay * decay_factor;
        }
    }

    /// Get the current graph size
    pub fn size(&self) -> (usize, usize) {
        (self.graph.nodes.len(), self.graph.edges.len())
    }

    /// Get all nodes in a specific stratum
    pub fn nodes_in_stratum(&self, stratum: MemoryStratum) -> Vec<MemoryNode> {
        self.graph
            .nodes
            .values()
            .filter(|n| n.stratum == stratum)
            .cloned()
            .collect()
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_node() {
        let mut service = Service::new();
        let node = service.write_node(
            "Test memory content".to_string(),
            MemoryStratum::Canonical,
        );
        assert_eq!(node.stratum, MemoryStratum::Canonical);
        assert_eq!(node.retrieval_count, 0);
    }

    #[test]
    fn test_create_edge() {
        let mut service = Service::new();
        let node1 = service.write_node(
            "First fact".to_string(),
            MemoryStratum::Canonical,
        );
        let node2 = service.write_node(
            "Second fact".to_string(),
            MemoryStratum::Canonical,
        );

        let result = service.create_edge(
            node1.node_id,
            node2.node_id,
            "related_to".to_string(),
            0.8,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_retrieve() {
        let mut service = Service::new();
        service.write_node(
            "Machine learning is powerful".to_string(),
            MemoryStratum::Operational,
        );
        service.write_node(
            "Deep learning uses neural networks".to_string(),
            MemoryStratum::Operational,
        );

        let episode = service.retrieve("learning".to_string(), 10);
        assert_eq!(episode.result_nodes.len(), 2);
    }

    #[test]
    fn test_stratum_filtering() {
        let mut service = Service::new();
        service.write_node("Fact 1".to_string(), MemoryStratum::Canonical);
        service.write_node("Fact 2".to_string(), MemoryStratum::Operational);
        service.write_node("Fact 3".to_string(), MemoryStratum::Canonical);

        let canonical = service.nodes_in_stratum(MemoryStratum::Canonical);
        assert_eq!(canonical.len(), 2);

        let operational = service.nodes_in_stratum(MemoryStratum::Operational);
        assert_eq!(operational.len(), 1);
    }
}

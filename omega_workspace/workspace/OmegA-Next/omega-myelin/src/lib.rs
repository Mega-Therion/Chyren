//! omega-myelin: Graph memory overlay, retrieval episodes, and plasticity.
//! This crate implements the sovereign memory system, storing nodes, edges,
//! and retrieval episodes that ground the cognitive OS in context.

#![warn(missing_docs)]

use omega_core::{EvidenceRecord, MemoryEdge, MemoryNode, RetrievalEpisode, now};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The MemoryGraph: Stores and manages the memory topology.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryGraph {
    /// Storage for nodes
    pub nodes: HashMap<String, MemoryNode>,
    /// Storage for edges
    pub edges: Vec<MemoryEdge>,
    /// History of retrievals
    pub episodes: Vec<RetrievalEpisode>,
}

impl MemoryGraph {
    /// Initialize a new empty memory graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            episodes: Vec::new(),
        }
    }

    /// Add a new node to the graph
    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// Add an edge between nodes
    pub fn add_edge(&mut self, edge: MemoryEdge) {
        self.edges.push(edge);
    }

    /// Record a retrieval episode to track cognitive context
    pub fn record_episode(&mut self, episode: RetrievalEpisode) {
        self.episodes.push(episode);
    }

    /// Apply plasticity: Update decay scores for nodes based on time
    pub fn apply_plasticity(&mut self) {
        let current_time = now();
        for node in self.nodes.values_mut() {
            // Simple decay model: time since last access reduces relevance
            let time_delta = current_time - node.last_accessed;
            node.decay_score *= (-time_delta / 86400.0).exp(); // 1-day half-life decay
        }
    }
}

/// MyelinManager: Interface for interacting with the graph
pub struct MyelinManager {
    /// The backing memory graph
    pub graph: MemoryGraph,
}

impl MyelinManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self {
            graph: MemoryGraph::new(),
        }
    }

    /// Perform a retrieval based on query context
    pub fn retrieve(&mut self, query: &str) -> Vec<String> {
        let mut retrieved_ids = Vec::new();
        
        // Basic keyword-based retrieval (will be expanded to semantic later)
        for (id, node) in &self.graph.nodes {
            if node.content.contains(query) {
                retrieved_ids.push(id.clone());
            }
        }

        // Record the episode
        self.graph.record_episode(RetrievalEpisode {
            query: query.to_string(),
            result_nodes: retrieved_ids.clone(),
            hydrated_neighbourhood: Vec::new(),
            quality_score: if !retrieved_ids.is_empty() { 1.0 } else { 0.0 },
            timestamp: now(),
        });

        retrieved_ids
    }
}

//! omega-myelin: Graph memory overlay and threat fabric integration.
//! This update integrates the Threat Fabric into the memory graph topology.

use omega_core::{gen_id, now, MemoryEdge, MemoryNode, MemoryStratum, RetrievalEpisode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod db;
pub mod phylactery;

/// ThreatEntry: Representation of a threat fabric pattern
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreatEntry {
    /// Unique entry identifier
    pub entry_id: String,
    /// Pattern signature/ID
    pub pattern_id: String,
    /// Labels characterizing the threat
    pub labels: Vec<String>,
    /// Severity level
    pub severity: String,
    /// Extracted timestamp
    pub extracted_utc: f64,
}

/// The MemoryGraph: Stores and manages the memory topology, including threat fabric
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryGraph {
    /// Storage for nodes
    pub nodes: HashMap<String, MemoryNode>,
    /// Storage for edges
    pub edges: Vec<MemoryEdge>,
    /// History of retrievals
    pub episodes: Vec<RetrievalEpisode>,
    /// Threat patterns detected/learned
    pub threat_fabric: Vec<ThreatEntry>,
}

impl MemoryGraph {
    /// Initialize a new empty memory graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            episodes: Vec::new(),
            threat_fabric: Vec::new(),
        }
    }

    /// Add a threat pattern to the fabric
    pub fn add_threat(&mut self, threat: ThreatEntry) {
        self.threat_fabric.push(threat);
    }

    /// Check if a given input matches a known threat pattern
    pub fn check_threats(&self, input: &str) -> Vec<ThreatEntry> {
        self.threat_fabric
            .iter()
            .filter(|t| input.contains(&t.pattern_id))
            .cloned()
            .collect()
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// Retrieve a node by ID
    pub fn get_node(&self, id: &str) -> Option<&MemoryNode> {
        self.nodes.get(id)
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, edge: MemoryEdge) {
        self.edges.push(edge);
    }

    /// Query nodes by stratum
    pub fn query_by_stratum(&self, stratum: MemoryStratum) -> Vec<&MemoryNode> {
        self.nodes
            .values()
            .filter(|n| n.stratum == stratum)
            .collect()
    }

    /// Record a retrieval episode
    pub fn record_retrieval(&mut self, episode: RetrievalEpisode) {
        self.episodes.push(episode);
    }

    /// Find nodes whose content contains the given query string
    pub fn query_content(&self, query: &str) -> Vec<&MemoryNode> {
        self.nodes
            .values()
            .filter(|n| n.content.contains(query))
            .collect()
    }

    /// Count of all nodes and edges (used for capacity reporting)
    pub fn size(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }
}

impl Default for MemoryGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level memory service used by the phylactery bootstrap and CLI.
/// Wraps `MemoryGraph` with convenience methods for node/edge creation.
pub struct Service {
    pub graph: MemoryGraph,
}

impl Service {
    /// Create a new in-memory service.
    pub fn new() -> Self {
        Self {
            graph: MemoryGraph::new(),
        }
    }

    /// Write a node to the given stratum, returning a reference to the stored node.
    pub fn write_node(&mut self, content: impl Into<String>, stratum: MemoryStratum) -> MemoryNode {
        let node = MemoryNode {
            node_id: gen_id("mn"),
            content: content.into(),
            stratum,
            created_at: now(),
            last_accessed: now(),
            retrieval_count: 0,
            decay_score: 1.0,
        };
        self.graph.add_node(node.clone());
        node
    }

    /// Create a directed edge between two nodes with a given relation and weight.
    pub fn create_edge(
        &mut self,
        from: String,
        to: String,
        relation: String,
        weight: f64,
    ) -> Result<(), String> {
        if !self.graph.nodes.contains_key(&from) {
            return Err(format!("source node '{}' not found", from));
        }
        if !self.graph.nodes.contains_key(&to) {
            return Err(format!("target node '{}' not found", to));
        }
        self.graph.add_edge(MemoryEdge {
            from_id: from,
            to_id: to,
            relation,
            strength: weight,
            bundle_id: None,
        });
        Ok(())
    }

    /// Retrieve nodes whose content matches the query.
    pub fn retrieve(&self, query: &str) -> Vec<&MemoryNode> {
        self.graph.query_content(query)
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

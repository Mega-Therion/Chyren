//! omega-myelin: Graph memory overlay and threat fabric integration.
//! This update integrates the Threat Fabric into the memory graph topology.

use omega_core::{EvidenceRecord, MemoryEdge, MemoryNode, RetrievalEpisode, now};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    // ... existing MemoryGraph implementation methods ...
}

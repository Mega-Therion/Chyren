//! Dream Synthesis — Recursive compression of the Neocortex MemoryGraph.
//!
//! The Dream Engine's compression mandate: if a KnowledgeNode can be fully
//! derived from other nodes already in the Neocortex, replace it with a
//! DerivationRule and delete the explicit proof. This keeps the graph lean
//! and sovereign — re-derive on demand rather than store forever.
//!
//! Also implements the 30-day Stratum Eviction Policy: nodes not accessed
//! in 30 days are evicted from hot storage to the ColdStore Merkle tree.

use omega_core::{now, KnowledgeNode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const EVICTION_THRESHOLD_SECS: f64 = 30.0 * 24.0 * 3600.0; // 30 days

// ── Derivation Rule ───────────────────────────────────────────────────────────

/// A derivation rule replaces an explicit KnowledgeNode that can be
/// reconstructed from other nodes in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationRule {
    /// The content_hash of the node this rule replaces.
    pub replaces_hash: String,
    /// content_hashes of the nodes needed to derive the replaced node.
    pub derived_from: Vec<String>,
    /// Human-readable derivation description.
    pub description: String,
    /// Timestamp when this derivation rule was created.
    pub created_at: f64,
}

// ── Compression Report ────────────────────────────────────────────────────────

/// Report summarizing compression decisions for a graph snapshot.
#[derive(Debug, Default)]
pub struct CompressionReport {
    /// Nodes that were compressed into derivation rules.
    pub compressed: Vec<DerivationRule>,
    /// Nodes that were evicted to cold storage (last_accessed > 30 days ago).
    pub evicted_hashes: Vec<String>,
    /// Nodes that were kept as-is.
    pub retained: usize,
}

// ── Dream Compressor ──────────────────────────────────────────────────────────

/// The DreamCompressor analyzes a set of KnowledgeNodes and identifies:
/// 1. Nodes that are derivable from others (→ compress to DerivationRule)
/// 2. Nodes that haven't been accessed in 30 days (→ evict to cold storage)
///
/// The caller (background task) is responsible for actually deleting from
/// hot storage and persisting derivation rules — the compressor only decides.
pub struct DreamCompressor;

impl DreamCompressor {
    /// Create a new DreamCompressor.
    pub fn new() -> Self {
        Self
    }

    /// Analyze nodes and return a CompressionReport.
    /// `nodes` is the full hot-graph snapshot.
    pub fn analyze(&self, nodes: &[KnowledgeNode]) -> CompressionReport {
        let mut report = CompressionReport::default();
        let current_time = now();

        // Build predicate → primary provider (the node absorbed earliest is the keeper).
        // Secondary providers of the same predicate are redundant and can be compressed.
        let mut predicate_primary: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for node in nodes {
            for constraint in &node.constraints {
                predicate_primary
                    .entry(constraint.predicate.clone())
                    .or_insert_with(|| node.content_hash.clone());
            }
        }

        // Nodes that are listed as a derivable_from source by any other node
        // must never be compressed — they are the axioms.
        let source_hashes: std::collections::HashSet<&str> = nodes
            .iter()
            .flat_map(|n| n.derivable_from.iter().map(|h| h.as_str()))
            .collect();

        for node in nodes {
            // 1. Eviction check: last accessed > 30 days
            let age = current_time - node.last_accessed;
            if age > EVICTION_THRESHOLD_SECS {
                report.evicted_hashes.push(node.content_hash.clone());
                continue;
            }

            // Axioms (sources of other nodes' derivations) are never compressed.
            if source_hashes.contains(node.content_hash.as_str()) {
                report.retained += 1;
                continue;
            }

            // 2. Compression check: explicit derivable_from list populated
            if !node.derivable_from.is_empty() {
                let all_present = node.derivable_from.iter().all(|h| {
                    nodes.iter().any(|n| &n.content_hash == h)
                });
                if all_present {
                    report.compressed.push(DerivationRule {
                        replaces_hash: node.content_hash.clone(),
                        derived_from: node.derivable_from.clone(),
                        description: format!(
                            "Node '{}' derivable from {} source(s); explicit proof removed",
                            node.summary,
                            node.derivable_from.len()
                        ),
                        created_at: current_time,
                    });
                    continue;
                }
            }

            // 3. Heuristic compression: this node is redundant if every one of its
            //    predicates already has a different primary provider.
            if !node.constraints.is_empty() {
                let all_covered = node.constraints.iter().all(|c| {
                    predicate_primary
                        .get(&c.predicate)
                        .map(|primary| primary != &node.content_hash)
                        .unwrap_or(false)
                });
                if all_covered {
                    let sources: Vec<String> = node
                        .constraints
                        .iter()
                        .filter_map(|c| predicate_primary.get(&c.predicate).cloned())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();

                    report.compressed.push(DerivationRule {
                        replaces_hash: node.content_hash.clone(),
                        derived_from: sources,
                        description: format!(
                            "All {} constraint(s) of '{}' covered by other nodes; compressed",
                            node.constraints.len(),
                            node.summary
                        ),
                        created_at: current_time,
                    });
                    continue;
                }
            }

            report.retained += 1;
        }

        report
    }
}

impl Default for DreamCompressor {
    fn default() -> Self {
        Self::new()
    }
}

// ── Legacy synthesis entry point ──────────────────────────────────────────────

/// Legacy identity kernel used for timestamp-anchored synthesis.
pub struct PhylacteryKernel {
    /// ISO-8601 timestamp of the last synthesis.
    pub timestamp: String,
}

/// Legacy entry point for identity synthesis updates.
pub fn synthesize_and_update() -> Result<(), String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let _timestamp = format!("{now:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{KnowledgeNode, ProofConstraint};

    fn make_constraint(pred: &str) -> ProofConstraint {
        ProofConstraint {
            id: omega_core::gen_id("c"),
            predicate: pred.to_string(),
            domain: "test".to_string(),
            depends_on: vec![],
        }
    }

    fn fresh_node(proof: &str, constraints: Vec<ProofConstraint>) -> KnowledgeNode {
        KnowledgeNode::new(proof.to_string(), proof.to_string(), constraints, None)
    }

    fn stale_node(proof: &str, constraints: Vec<ProofConstraint>) -> KnowledgeNode {
        let mut n = fresh_node(proof, constraints);
        // Simulate 31 days of inactivity
        n.last_accessed -= 31.0 * 24.0 * 3600.0;
        n
    }

    #[test]
    fn test_stale_nodes_evicted() {
        let dc = DreamCompressor::new();
        let nodes = vec![
            fresh_node("theorem a : True := trivial", vec![make_constraint("P1")]),
            stale_node("theorem b : True := trivial", vec![make_constraint("P2")]),
        ];
        let report = dc.analyze(&nodes);
        assert_eq!(report.evicted_hashes.len(), 1);
        assert_eq!(report.retained, 1);
    }

    #[test]
    fn test_derivable_node_compressed() {
        let dc = DreamCompressor::new();
        let base = fresh_node("theorem base : True := trivial", vec![make_constraint("P1")]);
        let mut derived = fresh_node("theorem derived : True := trivial", vec![make_constraint("P1")]);
        derived.derivable_from = vec![base.content_hash.clone()];

        let report = dc.analyze(&[base, derived]);
        assert_eq!(report.compressed.len(), 1);
    }

    #[test]
    fn test_heuristic_compression_redundant_predicates() {
        let dc = DreamCompressor::new();
        // Both nodes satisfy the same predicate P1
        let n1 = fresh_node("theorem x : True := trivial", vec![make_constraint("P1")]);
        let n2 = fresh_node("theorem y : 1=1 := rfl", vec![make_constraint("P1")]);
        let report = dc.analyze(&[n1, n2]);
        // One should be retained (the provider), one compressed (redundant)
        assert_eq!(report.compressed.len(), 1);
        assert_eq!(report.retained, 1);
    }

    #[test]
    fn test_unique_node_retained() {
        let dc = DreamCompressor::new();
        let nodes = vec![
            fresh_node("theorem u1 : True := trivial", vec![make_constraint("UniqueA")]),
            fresh_node("theorem u2 : 1=1 := rfl", vec![make_constraint("UniqueB")]),
        ];
        let report = dc.analyze(&nodes);
        assert_eq!(report.retained, 2);
        assert!(report.compressed.is_empty());
    }
}

//! Chiral Reasoning Graph — the runtime structure of the Epistemic Mesh.
//!
//! A ChiralGraph is a directed graph where:
//!   - Nodes are EpistemicNodes (Primary | Critique | Refinement | AxiomAnchor)
//!   - Edges are ChiralEdges with polarity (Supports | Critiques | Refines | Anchors)
//!
//! The graph evolves as the Epistemic Mesh processes a task:
//!   1. Primary nodes are added (one per spoke answer)
//!   2. Critique edges are added (EpistemicSpoke attacks each Primary)
//!   3. Refinement nodes are forked for each violated axiom
//!   4. If entropy exceeds threshold, an AxiomAnchor node is injected
//!   5. Convergence is declared when all leaf nodes are clean

use omega_core::{AxiomCheckResult, ChiralEdge, EdgePolarity, EpistemicNode, EpistemicNodeType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Epistemic Entropy ─────────────────────────────────────────────────────────

/// Measures divergence across Primary nodes in the graph.
/// 0.0 = full consensus, 1.0 = maximum divergence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicEntropy {
    /// Normalized entropy value (0.0 = consensus, 1.0 = max divergence).
    pub value: f32,
    /// Number of primary reasoning nodes.
    pub primary_node_count: usize,
    /// Total axiom violations across all nodes.
    pub violated_axiom_count: usize,
    /// Highest axiom violation level encountered.
    pub max_violation_level: u8,
}

impl EpistemicEntropy {
    /// Returns true if entropy indicates critical divergence.
    pub fn is_critical(&self) -> bool {
        self.value > 0.7 || self.max_violation_level == 0
    }

    /// Returns true if entropy exceeds threshold for axiom re-anchor.
    pub fn requires_axiom_review(&self) -> bool {
        self.value > 0.5 || self.violated_axiom_count > 2
    }
}

// ── Chiral Graph ──────────────────────────────────────────────────────────────

/// The runtime chiral reasoning graph.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChiralGraph {
    /// All epistemic nodes keyed by ID.
    pub nodes: HashMap<String, EpistemicNode>,
    /// All directed edges between nodes.
    pub edges: Vec<ChiralEdge>,
    /// Current refinement depth.
    pub depth: usize,
}

impl ChiralGraph {
    /// Create an empty chiral graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a node, computing its entropy weight, and return its ID.
    pub fn add_node(&mut self, mut node: EpistemicNode) -> String {
        node.entropy_weight = self.compute_node_entropy_weight(&node);
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        id
    }

    /// Append a directed edge to the graph.
    pub fn add_edge(&mut self, edge: ChiralEdge) {
        self.edges.push(edge);
    }

    /// Link a critique node to the primary it attacks.
    pub fn add_critique(&mut self, mut critique: EpistemicNode, primary_id: &str) -> String {
        let edge = ChiralEdge {
            from_id: critique.id.clone(),
            to_id: primary_id.to_string(),
            polarity: EdgePolarity::Critiques,
            axiom_name: critique
                .axiom_violations
                .first()
                .map(|v| v.axiom_name.clone()),
        };
        critique.parent_id = Some(primary_id.to_string());
        let id = self.add_node(critique);
        self.add_edge(edge);
        id
    }

    /// Fork a refinement node from a critique.
    pub fn fork_refinement(
        &mut self,
        content: String,
        parent_critique_id: &str,
        source_spoke: &str,
    ) -> String {
        let refinement = EpistemicNode::new_refinement(content, parent_critique_id, source_spoke);
        let edge = ChiralEdge {
            from_id: refinement.id.clone(),
            to_id: parent_critique_id.to_string(),
            polarity: EdgePolarity::Refines,
            axiom_name: None,
        };
        let id = self.add_node(refinement);
        self.add_edge(edge);
        self.depth += 1;
        id
    }

    /// Inject an AxiomAnchor node re-grounding all current leaf nodes.
    pub fn inject_axiom_anchor(&mut self, phylactery_summary: &str) -> String {
        let mut anchor = EpistemicNode {
            id: omega_core::gen_id("ea"),
            node_type: EpistemicNodeType::AxiomAnchor,
            content: format!(
                "SOVEREIGN AXIOM REVIEW — Phylactery re-anchor.\n\
                 R.W.Ϝ.Y.\n\n\
                 {phylactery_summary}\n\n\
                 All reasoning chains must now derive from these grounded axioms."
            ),
            source_spoke: "phylactery".to_string(),
            parent_id: None,
            critique_of: None,
            axiom_violations: vec![],
            proof_obligations: vec![],
            entropy_weight: 0.0,
            created_at: omega_core::now(),
        };
        anchor.entropy_weight = 0.0; // anchor resets entropy contribution

        // Connect anchor to all current leaf nodes
        let leaf_ids: Vec<String> = self.leaf_node_ids();
        let anchor_id = self.add_node(anchor);
        for leaf_id in leaf_ids {
            self.add_edge(ChiralEdge {
                from_id: anchor_id.clone(),
                to_id: leaf_id,
                polarity: EdgePolarity::Anchors,
                axiom_name: Some("Sovereignty".to_string()),
            });
        }
        anchor_id
    }

    /// Compute the current epistemic entropy of the graph.
    pub fn entropy(&self) -> EpistemicEntropy {
        let primaries: Vec<&EpistemicNode> = self
            .nodes
            .values()
            .filter(|n| n.node_type == EpistemicNodeType::Primary)
            .collect();

        let total_violations: Vec<&AxiomCheckResult> = self
            .nodes
            .values()
            .flat_map(|n| n.axiom_violations.iter())
            .filter(|v| !v.satisfied)
            .collect();

        let max_level = total_violations
            .iter()
            .filter_map(|v| {
                let axioms = omega_core::sovereign_axioms();
                axioms
                    .iter()
                    .find(|a| a.name() == v.axiom_name)
                    .map(|a| a.level())
            })
            .max()
            .unwrap_or(0);

        // Entropy = proportion of nodes with violations, weighted by level
        let weight_sum: f32 = self.nodes.values().map(|n| n.entropy_weight).sum();
        let total = self.nodes.len().max(1) as f32;
        let raw_entropy = (weight_sum / total).min(1.0);

        EpistemicEntropy {
            value: raw_entropy,
            primary_node_count: primaries.len(),
            violated_axiom_count: total_violations.len(),
            max_violation_level: max_level,
        }
    }

    /// True when all leaf nodes are axiom-clean (no violations).
    pub fn is_converged(&self) -> bool {
        let leaves = self.leaf_node_ids();
        leaves
            .iter()
            .all(|id| self.nodes.get(id).map(|n| n.is_clean()).unwrap_or(true))
    }

    /// IDs of nodes with no outgoing Refines or Supports edges (the frontier).
    pub fn leaf_node_ids(&self) -> Vec<String> {
        let has_outgoing: std::collections::HashSet<&str> = self
            .edges
            .iter()
            .filter(|e| matches!(e.polarity, EdgePolarity::Refines | EdgePolarity::Supports))
            .map(|e| e.from_id.as_str())
            .collect();

        self.nodes
            .keys()
            .filter(|id| !has_outgoing.contains(id.as_str()))
            .cloned()
            .collect()
    }

    /// The best current answer: the cleanest leaf Primary or Refinement node.
    pub fn best_answer(&self) -> Option<&EpistemicNode> {
        let leaves = self.leaf_node_ids();
        let candidates: Vec<&EpistemicNode> = leaves
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .filter(|n| {
                matches!(
                    n.node_type,
                    EpistemicNodeType::Primary | EpistemicNodeType::Refinement
                )
            })
            .collect();

        // Prefer nodes with zero violations, then fewest violations
        candidates
            .into_iter()
            .min_by_key(|n| n.axiom_violations.iter().filter(|v| !v.satisfied).count())
    }

    /// Summary of the graph state for logging/ledger.
    pub fn summary(&self) -> GraphSummary {
        let entropy = self.entropy();
        GraphSummary {
            total_nodes: self.nodes.len(),
            primary_count: self
                .nodes
                .values()
                .filter(|n| n.node_type == EpistemicNodeType::Primary)
                .count(),
            critique_count: self
                .nodes
                .values()
                .filter(|n| n.node_type == EpistemicNodeType::Critique)
                .count(),
            refinement_count: self
                .nodes
                .values()
                .filter(|n| n.node_type == EpistemicNodeType::Refinement)
                .count(),
            axiom_anchor_count: self
                .nodes
                .values()
                .filter(|n| n.node_type == EpistemicNodeType::AxiomAnchor)
                .count(),
            total_edges: self.edges.len(),
            depth: self.depth,
            entropy: entropy.value,
            converged: self.is_converged(),
        }
    }

    fn compute_node_entropy_weight(&self, node: &EpistemicNode) -> f32 {
        match node.node_type {
            EpistemicNodeType::Primary => 0.5,
            EpistemicNodeType::Critique => {
                // Higher weight for lower-level axiom violations (L0 is worst)
                let max_level = node
                    .axiom_violations
                    .iter()
                    .filter(|v| !v.satisfied)
                    .count();
                (max_level as f32 * 0.25).min(1.0)
            }
            EpistemicNodeType::Refinement => 0.2,
            EpistemicNodeType::AxiomAnchor => 0.0,
            EpistemicNodeType::LogicCacheEntry => 0.0,
        }
    }
}

/// Snapshot summary of the chiral graph state for logging and ledger.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphSummary {
    /// Total number of nodes in the graph.
    pub total_nodes: usize,
    /// Number of primary reasoning nodes.
    pub primary_count: usize,
    /// Number of adversarial critique nodes.
    pub critique_count: usize,
    /// Number of refinement nodes.
    pub refinement_count: usize,
    /// Number of axiom re-anchor nodes.
    pub axiom_anchor_count: usize,
    /// Total number of directed edges.
    pub total_edges: usize,
    /// Current refinement depth.
    pub depth: usize,
    /// Current entropy value.
    pub entropy: f32,
    /// Whether the graph has converged.
    pub converged: bool,
}

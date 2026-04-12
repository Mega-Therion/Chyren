use anyhow::{anyhow, Context, Result};
use omega_core::{MatrixProgram, MemoryEdge, MemoryNode};
use omega_myelin::MemoryGraph;
use serde::{Deserialize, Serialize};

/// Payload structure for a MatrixProgram (memory graft).
#[derive(Debug, Serialize, Deserialize)]
struct ProgramPayload {
    /// Nodes to add to memory.
    pub nodes: Vec<MemoryNode>,
    /// Edges to add to memory.
    pub edges: Vec<MemoryEdge>,
}

/// Ingestion engine for MatrixPrograms into memory/ledger substrates.
pub struct IngestionEngine;

impl IngestionEngine {
    /// Ingest a MatrixProgram into a MemoryGraph.
    ///
    /// This "grafts" the program payload onto the existing graph, ensuring
    /// that new nodes and edges are integrated. In a production environment,
    /// this would also verify cryptographic signatures before ingestion.
    pub async fn ingest(program: MatrixProgram, graph: &mut MemoryGraph) -> Result<()> {
        println!(
            "[INGESTION] Verifying program: {} v{}",
            program.domain, program.version
        );

        // TODO: In a real system, verify program.integrity_hash here.
        if program.payload.is_empty() {
            return Err(anyhow!("Empty program payload for {}", program.domain));
        }

        let graft: ProgramPayload = serde_json::from_slice(&program.payload)
            .context("Failed to deserialize MatrixProgram payload")?;

        let node_count = graft.nodes.len();
        let edge_count = graft.edges.len();

        // Graft nodes
        for node in graft.nodes {
            graph.add_node(node);
        }

        // Graft edges
        for edge in graft.edges {
            graph.edges.push(edge);
        }

        println!(
            "[INGESTION] Successfully grafted {} nodes and {} edges from {}",
            node_count, edge_count, program.domain
        );

        Ok(())
    }
}

use anyhow::{anyhow, Context, Result};
use omega_core::{MatrixProgram, MemoryEdge, MemoryNode};
use omega_myelin::MemoryGraph;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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
    /// Verifies the SHA-256 integrity hash over the raw payload before grafting
    /// nodes and edges into the graph. Returns an error if the hash does not
    /// match, preventing tampered or corrupted programs from entering memory.
    pub async fn ingest(program: MatrixProgram, graph: &mut MemoryGraph) -> Result<()> {
        println!(
            "[INGESTION] Verifying program: {} v{}",
            program.domain, program.version
        );

        // Verify integrity: SHA-256(payload) must match the declared hash.
        let computed = hex::encode(Sha256::digest(&program.payload));
        if computed != program.integrity_hash {
            return Err(anyhow!(
                "Integrity check failed for '{}': declared hash '{}' does not match computed '{}'",
                program.domain,
                program.integrity_hash,
                computed
            ));
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::MatrixProgram;

    fn make_valid_program(nodes: Vec<MemoryNode>, edges: Vec<MemoryEdge>) -> MatrixProgram {
        let payload = serde_json::to_vec(&ProgramPayload { nodes, edges }).unwrap();
        MatrixProgram::new("test.domain", "1.0", payload)
    }

    #[tokio::test]
    async fn ingest_valid_program_grafts_nodes() {
        let node = MemoryNode {
            node_id: "n1".into(),
            content: "test content".into(),
            retrieval_count: 0,
            decay_score: 1.0,
        };
        let program = make_valid_program(vec![node], vec![]);
        let mut graph = MemoryGraph::new();

        IngestionEngine::ingest(program, &mut graph).await.unwrap();
        assert_eq!(graph.nodes.len(), 1);
    }

    #[tokio::test]
    async fn ingest_rejects_tampered_hash() {
        let mut program = make_valid_program(vec![], vec![]);
        // Corrupt the payload after the hash was computed.
        program.payload.push(0xFF);

        let mut graph = MemoryGraph::new();
        let err = IngestionEngine::ingest(program, &mut graph)
            .await
            .unwrap_err();

        assert!(
            err.to_string().contains("Integrity check failed"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn ingest_rejects_wrong_declared_hash() {
        let mut program = make_valid_program(vec![], vec![]);
        program.integrity_hash = "deadbeef".repeat(8); // plausible-length but wrong

        let mut graph = MemoryGraph::new();
        let err = IngestionEngine::ingest(program, &mut graph)
            .await
            .unwrap_err();

        assert!(err.to_string().contains("Integrity check failed"));
    }
}

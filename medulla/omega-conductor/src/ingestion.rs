use anyhow::{anyhow, Context, Result};
use omega_core::{MatrixProgram, MemoryEdge, MemoryNode, MemoryStratum};
use omega_myelin::MemoryGraph;
use omega_neocortex::{seed_library, Neocortex};
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
        eprintln!(
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

        eprintln!(
            "[INGESTION] Successfully grafted {} nodes and {} edges from {}",
            node_count, edge_count, program.domain
        );

        Ok(())
    }
}

/// Boot-time Neocortex injection.
///
/// Loads the full seed library, ingests all programs, and grafts each domain's
/// knowledge as a Canonical memory node into the MemoryGraph. This is called
/// once at Conductor startup so every subsequent task has Chyren's full identity
/// and knowledge context available from the first step.
///
/// Returns the number of domains successfully grafted.
pub fn inject_neocortex(graph: &mut MemoryGraph) -> usize {
    let mut nc = Neocortex::new();
    nc.library = seed_library();

    let mind = match nc.ingest_all() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("[neocortex] ingest_all failed: {e}");
            return 0;
        }
    };

    let mut grafted = 0;
    for domain_key in &mind.load_report.domains_loaded {
        if let Some(value) = mind.knowledge.get(domain_key) {
            let content = format!("[neocortex::{domain_key}]\n{value}");
            graph.write_node(content, MemoryStratum::Canonical);
            grafted += 1;
        }
    }

    eprintln!(
        "[neocortex] Boot injection complete — {} domains grafted into MemoryGraph",
        grafted
    );
    grafted
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

    #[test]
    fn inject_neocortex_grafts_seed_programs() {
        let mut graph = MemoryGraph::new();
        let grafted = inject_neocortex(&mut graph);
        assert!(grafted >= 5, "expected at least 5 seed domains grafted, got {grafted}");
        // Every grafted node should carry the neocortex prefix
        let nc_nodes: Vec<_> = graph
            .nodes
            .values()
            .filter(|n| n.content.starts_with("[neocortex::"))
            .collect();
        assert_eq!(nc_nodes.len(), grafted);
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

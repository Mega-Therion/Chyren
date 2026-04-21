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
    #[serde(skip)]
    pub vector_store: Option<VectorStore>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach a VectorStore to this graph.
    pub fn set_vector_store(&mut self, vs: VectorStore) {
        self.vector_store = Some(vs);
    }

    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    /// Synchronous write — creates a node in the in-memory graph only.
    pub fn write_node(
        &mut self,
        content: String,
        _stratum: omega_core::MemoryStratum,
    ) -> MemoryNode {
        let node = MemoryNode {
            node_id: omega_core::gen_id("node"),
            content,
            retrieval_count: 0,
            decay_score: 1.0,
        };
        self.add_node(node.clone());
        node
    }

    /// Write a node to the in-memory graph AND upsert its embedding to Qdrant.
    /// If Qdrant is offline or not configured, the node is still written in-memory (graceful degradation).
    pub async fn write_node_with_embedding(
        &mut self,
        content: String,
        stratum: omega_core::MemoryStratum,
        embedding: Vec<f32>,
    ) -> MemoryNode {
        self.write_node_sharded(content, stratum, embedding, "general").await
    }

    /// Write a node to the in-memory graph AND upsert its embedding to a specific Qdrant shard (domain).
    pub async fn write_node_sharded(
        &mut self,
        content: String,
        stratum: omega_core::MemoryStratum,
        embedding: Vec<f32>,
        domain: &str,
    ) -> MemoryNode {
        let node = self.write_node(content.clone(), stratum);

        if let Some(vs) = &self.vector_store {
            let shard = vs.shard(domain);
            let _ = shard.ensure_collection().await;
            let payload = serde_json::json!({
                "node_id": node.node_id,
                "content": content,
                "domain": domain,
            });
            // Fire-and-forget — errors already logged inside upsert()
            let _ = shard.upsert(&node.node_id, embedding, payload).await;
        }

        node
    }

    /// Search Qdrant semantically and map hits back to in-memory MemoryNodes.
    /// Returns empty vec if no VectorStore is configured or Qdrant is unreachable — never panics.
    pub async fn search_semantic(
        &self,
        query_embedding: Vec<f32>,
        top_k: usize,
    ) -> Vec<MemoryNode> {
        self.search_semantic_sharded(query_embedding, top_k, "general").await
    }

    /// Search a specific Qdrant shard semantically and map hits back to in-memory MemoryNodes.
    pub async fn search_semantic_sharded(
        &self,
        query_embedding: Vec<f32>,
        top_k: usize,
        domain: &str,
    ) -> Vec<MemoryNode> {
        let vs = match &self.vector_store {
            Some(vs) => vs.shard(domain),
            None => return vec![],
        };

        // Fetch more candidates from Qdrant so we have room to mathematically re-rank them
        let hits = vs.search(query_embedding, top_k * 3).await.unwrap_or_default();

        let mut resonant_nodes: Vec<(MemoryNode, f32)> = hits
            .into_iter()
            .filter_map(|hit| {
                self.nodes.get(&hit.id).cloned().map(|node| {
                    // --- THE RESONANCE CALCULATION ---
                    // 1. Frequency (Base phase alignment from Qdrant: 0.0 to 1.0)
                    let frequency = hit.score; 
                    
                    // 2. Energy (The decaying wave from eviction.rs: 0.1 to 1.0)
                    let energy = node.decay_score as f32;
                    
                    // 3. Amplitude (Historical weight/salience. logarithmic so it doesn't blow up)
                    let amplitude = 1.0 + (node.retrieval_count as f32).ln_1p() * 0.15;
                    
                    // Final Resonance = Frequency * Energy * Amplitude
                    let resonance = frequency * energy * amplitude;
                    
                    (node, resonance)
                })
            })
            .collect();

        // Sort descending by calculated Resonance instead of raw static vector similarity
        resonant_nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return the most resonant `top_k` nodes
        resonant_nodes.into_iter().take(top_k).map(|(n, _)| n).collect()
    }

    pub fn create_edge(&mut self, from: String, to: String, _edge_type: String, _weight: f64) {
        self.edges.push(omega_core::MemoryEdge {
            from: from.clone(),
            to: to.clone(),
            from_id: from,
            to_id: to,
        });
    }

    pub fn anchor_recall(
        &self,
        anchor: &omega_core::TemporalAnchor,
        radius: usize,
    ) -> Vec<omega_core::RetrievalEpisode> {
        let mut results = Vec::new();
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

/// Myelin Service: Thread-safe memory access layer.
pub struct Service {
    pub graph: std::sync::Arc<tokio::sync::Mutex<MemoryGraph>>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            graph: std::sync::Arc::new(tokio::sync::Mutex::new(MemoryGraph::new())),
        }
    }

    pub async fn name(&self) -> String {
        "myelin".into()
    }

    pub async fn lock(&self) -> tokio::sync::MutexGuard<'_, MemoryGraph> {
        self.graph.lock().await
    }

    /// Initialize the Qdrant vector store, ensure the collection exists, and wire it into the graph.
    /// Gracefully no-ops if Qdrant is unreachable.
    pub async fn init_vector_store(&self, url: &str) {
        let vs = VectorStore::new(url, "chyren_memory");
        // ensure_collection is already graceful on failure
        let _ = vs.ensure_collection().await;
        let mut graph = self.graph.lock().await;
        graph.set_vector_store(vs);
    }

    /// Synchronous write — delegates to MemoryGraph::write_node.
    pub async fn write_node(
        &self,
        content: String,
        stratum: omega_core::MemoryStratum,
    ) -> MemoryNode {
        let mut graph = self.graph.lock().await;
        graph.write_node(content, stratum)
    }

    /// Sharded write — delegates to MemoryGraph::write_node_sharded.
    pub async fn write_node_sharded(
        &self,
        content: String,
        stratum: omega_core::MemoryStratum,
        embedding: Vec<f32>,
        domain: &str,
    ) -> MemoryNode {
        let mut graph = self.graph.lock().await;
        graph.write_node_sharded(content, stratum, embedding, domain).await
    }

    /// Sharded search — delegates to MemoryGraph::search_semantic_sharded.
    pub async fn search_semantic_sharded(
        &self,
        query_embedding: Vec<f32>,
        top_k: usize,
        domain: &str,
    ) -> Vec<MemoryNode> {
        let graph = self.graph.lock().await;
        graph.search_semantic_sharded(query_embedding, top_k, domain).await
    }

    pub async fn create_edge(&self, from: String, to: String, edge_type: String, weight: f64) {
        let mut graph = self.graph.lock().await;
        graph.create_edge(from, to, edge_type, weight);
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

pub mod db;
pub mod vector;
pub mod eviction;

pub use vector::{SearchResult, VectorStore};
pub use eviction::StratumEvictionWorker;

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::MemoryStratum;

    #[tokio::test]
    async fn test_search_semantic_no_vector_store_returns_empty() {
        let graph = MemoryGraph::new();
        let results = graph
            .search_semantic(vec![0.1f32, 0.2, 0.3], 5)
            .await;
        assert!(
            results.is_empty(),
            "search_semantic should return empty vec when no vector_store is set"
        );
    }

    #[tokio::test]
    async fn test_write_node_with_embedding_offline_still_writes_node() {
        let mut graph = MemoryGraph::new();
        // Attach an offline VectorStore — port 19999 is never listening
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        graph.set_vector_store(vs);

        let node = graph
            .write_node_with_embedding(
                "test content".to_string(),
                MemoryStratum::Canonical,
                vec![0.1f32, 0.2, 0.3],
            )
            .await;

        // Node must exist in the in-memory graph even though Qdrant was offline
        assert!(
            graph.nodes.contains_key(&node.node_id),
            "node should be written to in-memory graph even when Qdrant is offline"
        );
        assert_eq!(node.content, "test content");
    }

    #[tokio::test]
    async fn test_search_semantic_offline_qdrant_returns_empty() {
        let mut graph = MemoryGraph::new();
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        graph.set_vector_store(vs);

        let results = graph
            .search_semantic(vec![0.1f32, 0.2, 0.3], 5)
            .await;
        assert!(
            results.is_empty(),
            "search_semantic should return empty vec when Qdrant is offline"
        );
    }

    #[tokio::test]
    async fn test_init_vector_store_offline_is_graceful() {
        let service = Service::new();
        // Should not panic even if Qdrant is unreachable
        service.init_vector_store("http://127.0.0.1:19999").await;
        let graph = service.lock().await;
        // vector_store should be set (even though it's offline)
        assert!(
            graph.vector_store.is_some(),
            "vector_store should be attached after init_vector_store"
        );
    }
}

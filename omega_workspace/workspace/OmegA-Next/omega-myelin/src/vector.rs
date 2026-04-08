//! omega-myelin vector layer: High-performance semantic retrieval via Qdrant.
//!
//! Provides the `VectorStore` for indexing and searching memory nodes.

use qdrant_client::prelude::*;
use qdrant_client::qdrant::{CreateCollectionBuilder, VectorParamsBuilder, Distance, PointStruct, SearchPointsBuilder};
use anyhow::{Result, Context};
use serde_json::json;
use crate::MemoryNode;

/// Semantic storage engine backed by Qdrant.
pub struct VectorStore {
    client: QdrantClient,
    collection_name: String,
}

impl VectorStore {
    /// Connect to Qdrant and ensure the collection exists.
    pub async fn connect(url: &str, collection: &str) -> Result<Self> {
        let client = QdrantClient::from_url(url).build()?;
        
        // Ensure collection exists
        if !client.collection_exists(collection).await? {
            client.create_collection(
                CreateCollectionBuilder::new(collection)
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine)) // Standard for OpenAI/Chyren embeddings
            ).await.context("Failed to create Qdrant collection")?;
        }

        Ok(Self {
            client,
            collection_name: collection.to_string(),
        })
    }

    /// Index a node with its embedding.
    pub async fn upsert_node(&self, node: &MemoryNode, embedding: Vec<f32>) -> Result<()> {
        let payload = json!({
            "node_id": node.node_id,
            "content": node.content,
            "decay_score": node.decay_score,
        }).try_into()?;

        let point = PointStruct::new(
            node.node_id.clone(),
            embedding,
            payload
        );

        self.client.upsert_points(&self.collection_name, None, vec![point], None).await?;
        Ok(())
    }

    /// Perform a semantic search.
    pub async fn search(&self, vector: Vec<f32>, top_k: usize) -> Result<Vec<String>> {
        let search_result = self.client.search_points(
            SearchPointsBuilder::new(&self.collection_name, vector, top_k as u64)
                .with_payload(true)
        ).await?;

        let node_ids = search_result.result
            .into_iter()
            .filter_map(|p| p.payload.get("node_id").and_then(|v| v.as_str().map(|s| s.to_string())))
            .collect();

        Ok(node_ids)
    }
}

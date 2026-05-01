//! chyren-myelin vector layer: Semantic retrieval via Qdrant REST API.
//!
//! Uses plain HTTP (reqwest) — no gRPC dependency.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

/// A single search result returned by Qdrant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub payload: Value,
}

/// HTTP-based Qdrant vector store client.
#[derive(Debug)]
pub struct VectorStore {
    client: reqwest::Client,
    base_url: String,
    collection: String,
}

impl VectorStore {
    /// Construct from explicit URL and collection name.
    pub fn new(url: &str, collection: &str) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
            base_url: url.trim_end_matches('/').to_string(),
            collection: collection.to_string(),
        }
    }

    /// Derive a new shard from this vector store with a domain-specific collection.
    pub fn shard(&self, domain: &str) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            collection: format!(
                "{}_{}",
                self.collection,
                domain.to_lowercase().replace(' ', "_")
            ),
        }
    }

    /// Construct from `QDRANT_URL` env var (default: `http://localhost:6333`),
    /// collection defaults to `"chyren_memory"`.
    pub fn from_env() -> Self {
        let url = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
        Self::new(&url, "chyren_memory")
    }

    /// Ensure the collection exists in Qdrant. Creates it if absent (409 = already exists = ok).
    /// Never panics — returns gracefully if Qdrant is unreachable.
    pub async fn ensure_collection(&self) -> Result<(), anyhow::Error> {
        let url = format!("{}/collections/{}", self.base_url, self.collection);
        let body = serde_json::json!({
            "vectors": {
                "size": 1536,
                "distance": "Cosine"
            }
        });

        match self.client.post(&url).json(&body).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() || status.as_u16() == 409 {
                    // 409 Conflict = collection already exists, treat as success
                    Ok(())
                } else {
                    let text = resp.text().await.unwrap_or_default();
                    eprintln!("[chyren-myelin] ensure_collection failed {status}: {text}");
                    // Non-fatal — degrade gracefully
                    Ok(())
                }
            }
            Err(e) => {
                eprintln!("[chyren-myelin] ensure_collection unreachable: {e}");
                Ok(()) // graceful degradation
            }
        }
    }

    /// Returns `true` if Qdrant is reachable, `false` otherwise (never panics).
    pub async fn health_check(&self) -> bool {
        let url = format!("{}/healthz", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Upsert a single vector point. No-op (logs to stderr) if Qdrant is unreachable.
    pub async fn upsert(
        &self,
        id: &str,
        vector: Vec<f32>,
        payload: Value,
    ) -> Result<(), anyhow::Error> {
        let url = format!("{}/collections/{}/points", self.base_url, self.collection);
        let body = json!({
            "points": [{
                "id": id,
                "vector": vector,
                "payload": payload,
            }]
        });

        match self.client.put(&url).json(&body).send().await {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                eprintln!("[chyren-myelin] qdrant upsert failed {status}: {text}");
                Err(anyhow::anyhow!("Qdrant upsert error {status}: {text}"))
            }
            Err(e) => {
                eprintln!("[chyren-myelin] qdrant upsert unreachable: {e}");
                Ok(()) // graceful degradation — treat as no-op
            }
        }
    }

    /// Cosine-similarity search. Returns empty vec if Qdrant is unreachable (never panics).
    pub async fn search(
        &self,
        query_vector: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<SearchResult>, anyhow::Error> {
        let url = format!(
            "{}/collections/{}/points/search",
            self.base_url, self.collection
        );
        let body = json!({
            "vector": query_vector,
            "limit": top_k,
            "with_payload": true,
        });

        let resp = match self.client.post(&url).json(&body).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[chyren-myelin] qdrant search unreachable: {e}");
                return Ok(vec![]);
            }
        };

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            eprintln!("[chyren-myelin] qdrant search error {status}: {text}");
            return Ok(vec![]);
        }

        #[derive(Deserialize)]
        struct QdrantHit {
            id: Value,
            score: f32,
            #[serde(default)]
            payload: Value,
        }
        #[derive(Deserialize)]
        struct QdrantSearchResponse {
            result: Vec<QdrantHit>,
        }

        let parsed: QdrantSearchResponse = match resp.json().await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[chyren-myelin] qdrant search parse error: {e}");
                return Ok(vec![]);
            }
        };

        let results = parsed
            .result
            .into_iter()
            .map(|hit| SearchResult {
                id: match &hit.id {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                },
                score: hit.score,
                payload: hit.payload,
            })
            .collect();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_env_url_variants() {
        // Run both env-var checks sequentially within a single test to avoid
        // parallel-test races on the process-global env.
        unsafe {
            // Default: no env var set
            std::env::remove_var("QDRANT_URL");
            let vs = VectorStore::from_env();
            assert_eq!(vs.base_url, "http://localhost:6333");
            assert_eq!(vs.collection, "chyren_memory");

            // Custom URL
            std::env::set_var("QDRANT_URL", "http://qdrant.example.com:6333");
            let vs = VectorStore::from_env();
            assert_eq!(vs.base_url, "http://qdrant.example.com:6333");

            // Clean up
            std::env::remove_var("QDRANT_URL");
        }
    }

    #[tokio::test]
    async fn test_health_check_offline_returns_false() {
        // Port 19999 is almost certainly not listening.
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        let alive = vs.health_check().await;
        assert!(!alive, "health_check should return false when offline");
    }

    #[tokio::test]
    async fn test_search_offline_returns_empty() {
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        let results = vs.search(vec![0.1, 0.2, 0.3], 5).await.unwrap();
        assert!(
            results.is_empty(),
            "search should return empty vec when offline"
        );
    }

    #[tokio::test]
    async fn test_ensure_collection_offline_is_graceful() {
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        let result = vs.ensure_collection().await;
        assert!(
            result.is_ok(),
            "ensure_collection should return Ok when offline"
        );
    }

    #[tokio::test]
    async fn test_upsert_offline_is_noop() {
        let vs = VectorStore::new("http://127.0.0.1:19999", "test_col");
        // Should not panic and should return Ok (graceful degradation)
        let result = vs
            .upsert(
                "test-id-1",
                vec![0.1, 0.2, 0.3],
                serde_json::json!({"content": "hello"}),
            )
            .await;
        assert!(result.is_ok(), "upsert should be a no-op when offline");
    }
}

//! ColdStore — Content-addressable disk storage for Neocortex knowledge nodes.
//!
//! Each KnowledgeNode is keyed by the SHA-256 of its Lean 4 proof (content_hash).
//! Merkle-style: the content *is* the address. You cannot store the same proof twice.
//!
//! Layout: `{base_path}/{prefix}/{content_hash}.json`
//!   where prefix = first 2 chars of hash (sharding to avoid large directories)

use omega_core::KnowledgeNode;
use serde_json;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ColdStoreError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Node not found: {hash}")]
    NotFound { hash: String },
    #[error("IPFS error: {0}")]
    Ipfs(String),
}

/// Persistent, content-addressable store for cold-stratum knowledge nodes.
pub struct ColdStore {
    base: PathBuf,
    ipfs_url: Option<String>,
    http: reqwest::Client,
}

impl ColdStore {
    /// Create a ColdStore rooted at `base_path`. Creates the directory if absent.
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self, ColdStoreError> {
        let base = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base)?;
        let ipfs_url = std::env::var("IPFS_API_URL").ok();
        Ok(Self {
            base,
            ipfs_url,
            http: reqwest::Client::new(),
        })
    }

    /// Default location: `~/.omega/cold/`
    pub fn default_store() -> Result<Self, ColdStoreError> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self::new(PathBuf::from(home).join(".omega").join("cold"))
    }

    fn node_path(&self, hash: &str) -> PathBuf {
        // Shard by first 2 chars to keep directory sizes manageable
        let prefix = &hash[..hash.len().min(2)];
        self.base.join(prefix).join(format!("{hash}.json"))
    }

    /// Persist a KnowledgeNode. Returns the content_hash (stable address).
    /// Idempotent: storing the same proof twice is a no-op (returns existing hash).
    pub async fn store(&self, node: &KnowledgeNode) -> Result<String, ColdStoreError> {
        let path = self.node_path(&node.content_hash);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if !path.exists() {
            let bytes = serde_json::to_vec_pretty(node)?;
            fs::write(&path, &bytes)?;

            // Sovereign Bridge: Pin to IPFS if available
            if let Some(url) = &self.ipfs_url {
                let _ = self.ipfs_pin(&node.content_hash, &bytes, url).await;
            }
        }
        Ok(node.content_hash.clone())
    }

    async fn ipfs_pin(&self, hash: &str, data: &[u8], url: &str) -> Result<String, ColdStoreError> {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()).file_name(format!("{}.json", hash)));

        let resp = self.http.post(format!("{}/api/v0/add?pin=true", url))
            .multipart(form)
            .send()
            .await
            .map_err(|e| ColdStoreError::Ipfs(e.to_string()))?;

        if resp.status().is_success() {
            let json: serde_json::Value = resp.json().await.map_err(|e| ColdStoreError::Ipfs(e.to_string()))?;
            if let Some(cid) = json["Hash"].as_str() {
                return Ok(cid.to_string());
            }
        }
        Err(ColdStoreError::Ipfs("Failed to pin to IPFS".into()))
    }

    /// Retrieve a KnowledgeNode by its content_hash. Returns None if not found.
    pub fn retrieve(&self, hash: &str) -> Result<Option<KnowledgeNode>, ColdStoreError> {
        let path = self.node_path(hash);
        if !path.exists() {
            return Ok(None);
        }
        let bytes = fs::read(&path)?;
        let node: KnowledgeNode = serde_json::from_slice(&bytes)?;
        Ok(Some(node))
    }

    /// Delete a node by hash (used when Dream engine replaces it with a derivation rule).
    pub fn delete(&self, hash: &str) -> Result<(), ColdStoreError> {
        let path = self.node_path(hash);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// List all content hashes currently in the store.
    pub fn list_hashes(&self) -> Result<Vec<String>, ColdStoreError> {
        let mut hashes = Vec::new();
        self.walk_hashes(&self.base, &mut hashes)?;
        Ok(hashes)
    }

    fn walk_hashes(&self, dir: &Path, out: &mut Vec<String>) -> Result<(), ColdStoreError> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.walk_hashes(&path, out)?;
            } else if path.extension().map_or(false, |e| e == "json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    out.push(stem.to_string());
                }
            }
        }
        Ok(())
    }

    /// Return number of nodes in cold store.
    pub fn count(&self) -> Result<usize, ColdStoreError> {
        Ok(self.list_hashes()?.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{KnowledgeNode, ProofConstraint};

    fn tmp_store() -> ColdStore {
        let dir = std::env::temp_dir().join(format!("cold_store_test_{}", omega_core::gen_id("t")));
        ColdStore::new(dir).unwrap()
    }

    fn make_node(proof: &str) -> KnowledgeNode {
        KnowledgeNode::new(
            proof.to_string(),
            "test summary".to_string(),
            vec![ProofConstraint {
                id: "c1".to_string(),
                predicate: "True".to_string(),
                domain: "logic".to_string(),
                depends_on: vec![],
            }],
            None,
        )
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let store = tmp_store();
        let node = make_node("theorem foo : True := trivial");
        let hash = store.store(&node).await.unwrap();
        let retrieved = store.retrieve(&hash).unwrap().unwrap();
        assert_eq!(retrieved.content_hash, hash);
        assert_eq!(retrieved.lean_proof, node.lean_proof);
    }

    #[tokio::test]
    async fn test_idempotent_store() {
        let store = tmp_store();
        let node = make_node("theorem bar : True := trivial");
        let h1 = store.store(&node).await.unwrap();
        let h2 = store.store(&node).await.unwrap();
        assert_eq!(h1, h2);
        assert_eq!(store.count().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_delete() {
        let store = tmp_store();
        let node = make_node("theorem baz : True := trivial");
        let hash = store.store(&node).await.unwrap();
        store.delete(&hash).unwrap();
        assert!(store.retrieve(&hash).unwrap().is_none());
    }

    #[tokio::test]
    async fn test_list_hashes() {
        let store = tmp_store();
        let n1 = make_node("theorem t1 : True := trivial");
        let n2 = make_node("theorem t2 : 1 = 1 := rfl");
        store.store(&n1).await.unwrap();
        store.store(&n2).await.unwrap();
        let hashes = store.list_hashes().unwrap();
        assert_eq!(hashes.len(), 2);
    }
}

//! ProofConstraintIndex — Reasoning-first knowledge retrieval.
//!
//! Organizes KnowledgeNodes by the proof constraints they satisfy, not by
//! algorithm name. Chyren asks "do I have logic that satisfies Primality?"
//! not "do I have a prime number generator?"
//!
//! Backed by an in-memory inverted index (predicate → content_hashes).
//! Persists to `{base_path}/proof_index.json` on each update.

use omega_core::ProofConstraint;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// In-memory inverted index: predicate_text → set of content_hashes.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProofConstraintIndex {
    /// predicate → content_hashes of nodes that satisfy it
    predicate_map: HashMap<String, HashSet<String>>,
    /// domain → content_hashes of nodes in that domain
    domain_map: HashMap<String, HashSet<String>>,
    /// content_hash → list of constraint IDs it satisfies
    node_constraints: HashMap<String, Vec<String>>,
}

impl ProofConstraintIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load from disk if the index file exists, otherwise return a fresh index.
    pub fn load_or_new(path: &Path) -> Self {
        if path.exists() {
            if let Ok(bytes) = fs::read(path) {
                if let Ok(idx) = serde_json::from_slice(&bytes) {
                    return idx;
                }
            }
        }
        Self::new()
    }

    /// Register a knowledge node's constraints into the index.
    pub fn insert(&mut self, content_hash: &str, constraints: &[ProofConstraint]) {
        let mut ids = Vec::new();
        for c in constraints {
            self.predicate_map
                .entry(c.predicate.clone())
                .or_default()
                .insert(content_hash.to_string());
            self.domain_map
                .entry(c.domain.clone())
                .or_default()
                .insert(content_hash.to_string());
            ids.push(c.id.clone());
        }
        self.node_constraints.insert(content_hash.to_string(), ids);
    }

    /// Remove a node from the index (called on eviction or compression).
    pub fn remove(&mut self, content_hash: &str) {
        self.node_constraints.remove(content_hash);
        for set in self.predicate_map.values_mut() {
            set.remove(content_hash);
        }
        for set in self.domain_map.values_mut() {
            set.remove(content_hash);
        }
    }

    /// Find content_hashes of nodes whose constraints overlap with the query predicates.
    /// Returns hashes ranked by how many query predicates they satisfy.
    pub fn query_by_predicates(&self, predicates: &[String]) -> Vec<String> {
        let mut score: HashMap<&str, usize> = HashMap::new();
        for pred in predicates {
            if let Some(hashes) = self.predicate_map.get(pred) {
                for h in hashes {
                    *score.entry(h.as_str()).or_insert(0) += 1;
                }
            }
        }
        let mut ranked: Vec<(&str, usize)> = score.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1));
        ranked.into_iter().map(|(h, _)| h.to_string()).collect()
    }

    /// Find all nodes in a given mathematical domain.
    pub fn query_by_domain(&self, domain: &str) -> Vec<String> {
        self.domain_map
            .get(domain)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Persist to disk.
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let bytes = serde_json::to_vec_pretty(self).map_err(std::io::Error::other)?;
        fs::write(path, bytes)
    }

    pub fn node_count(&self) -> usize {
        self.node_constraints.len()
    }
}

/// Managed ProofConstraintIndex with auto-persist on mutation.
pub struct ManagedIndex {
    pub index: ProofConstraintIndex,
    path: PathBuf,
}

impl ManagedIndex {
    pub fn open(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let index = ProofConstraintIndex::load_or_new(&path);
        Self { index, path }
    }

    pub fn insert(&mut self, content_hash: &str, constraints: &[ProofConstraint]) {
        self.index.insert(content_hash, constraints);
        let _ = self.index.save(&self.path);
    }

    pub fn remove(&mut self, content_hash: &str) {
        self.index.remove(content_hash);
        let _ = self.index.save(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::ProofConstraint;

    fn make_constraint(id: &str, pred: &str, domain: &str) -> ProofConstraint {
        ProofConstraint {
            id: id.to_string(),
            predicate: pred.to_string(),
            domain: domain.to_string(),
            depends_on: vec![],
        }
    }

    #[test]
    fn test_insert_and_query_by_predicate() {
        let mut idx = ProofConstraintIndex::new();
        let c = make_constraint("c1", "IsPrime(n)", "number_theory");
        idx.insert("hash_abc", &[c]);

        let results = idx.query_by_predicates(&["IsPrime(n)".to_string()]);
        assert_eq!(results, vec!["hash_abc"]);
    }

    #[test]
    fn test_query_by_domain() {
        let mut idx = ProofConstraintIndex::new();
        let c = make_constraint("c2", "IsSorted(xs)", "combinatorics");
        idx.insert("hash_def", &[c]);

        let results = idx.query_by_domain("combinatorics");
        assert!(results.contains(&"hash_def".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut idx = ProofConstraintIndex::new();
        let c = make_constraint("c3", "IsEven(n)", "number_theory");
        idx.insert("hash_ghi", &[c]);
        idx.remove("hash_ghi");

        let results = idx.query_by_predicates(&["IsEven(n)".to_string()]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_ranking_by_overlap() {
        let mut idx = ProofConstraintIndex::new();
        idx.insert(
            "hash_a",
            &[
                make_constraint("c4", "P1", "d"),
                make_constraint("c5", "P2", "d"),
            ],
        );
        idx.insert("hash_b", &[make_constraint("c6", "P1", "d")]);

        let results = idx.query_by_predicates(&["P1".to_string(), "P2".to_string()]);
        // hash_a satisfies both P1 and P2 → should rank first
        assert_eq!(results[0], "hash_a");
    }
}

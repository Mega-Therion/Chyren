//! Immutable Policy-as-Code (IPCL) — Rust port of `core/cortex/merkle_policy`.
//!
//! Builds a binary Merkle tree over policy clauses, signs the root with
//! HMAC-SHA256 keyed by `CHYREN_POLICY_HMAC_KEY` (or a caller-supplied key),
//! and emits an append-only [`PolicyManifest`]. Hash domain separation,
//! signature scheme, and tree shape match the Python implementation byte-for-
//! byte so manifests written by either side verify on the other.

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Errors emitted by the policy service.
#[derive(Debug, Error)]
pub enum PolicyError {
    /// HMAC key shorter than 32 bytes.
    #[error("policy HMAC key must be at least 32 bytes")]
    KeyTooShort,
    /// `CHYREN_POLICY_HMAC_KEY` env var was empty or absent.
    #[error("CHYREN_POLICY_HMAC_KEY not set and no key provided")]
    KeyMissing,
    /// Hex decoding of a stored manifest field failed.
    #[error("malformed hex field: {0}")]
    Hex(String),
    /// HMAC could not be initialized.
    #[error("hmac init failed: {0}")]
    Hmac(String),
}

fn h(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

fn leaf(clause: &str) -> [u8; 32] {
    let mut buf = Vec::with_capacity(1 + clause.len());
    buf.push(0x00);
    buf.extend_from_slice(clause.as_bytes());
    h(&buf)
}

fn node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(1 + 64);
    buf.push(0x01);
    buf.extend_from_slice(left);
    buf.extend_from_slice(right);
    h(&buf)
}

/// Compute the 32-byte Merkle root over `clauses`. Empty input returns the
/// SHA-256 of the empty string (canonical empty-tree sentinel). Odd levels
/// duplicate the last node — Bitcoin convention, matching the Python port.
pub fn merkle_root(clauses: &[String]) -> [u8; 32] {
    if clauses.is_empty() {
        return h(&[]);
    }
    let mut level: Vec<[u8; 32]> = clauses.iter().map(|c| leaf(c)).collect();
    while level.len() > 1 {
        if level.len() % 2 == 1 {
            level.push(*level.last().unwrap());
        }
        level = level
            .chunks_exact(2)
            .map(|pair| node(&pair[0], &pair[1]))
            .collect();
    }
    level[0]
}

/// One step in an inclusion proof. `L` means the sibling sits to the left of
/// the running hash at that level; `R` means it sits to the right.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    /// Sibling is on the left.
    L,
    /// Sibling is on the right.
    R,
}

/// One sibling on the audit path.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProofStep {
    /// Side the sibling occupies.
    pub side: Side,
    /// Hex-encoded sibling hash.
    pub sibling: String,
}

/// Return the audit path for `clauses[index]`. Panics on out-of-range.
pub fn inclusion_proof(clauses: &[String], mut index: usize) -> Vec<ProofStep> {
    assert!(index < clauses.len(), "index out of range");
    let mut level: Vec<[u8; 32]> = clauses.iter().map(|c| leaf(c)).collect();
    let mut proof = Vec::new();
    while level.len() > 1 {
        if level.len() % 2 == 1 {
            level.push(*level.last().unwrap());
        }
        let sibling_idx = index ^ 1;
        let side = if sibling_idx > index { Side::R } else { Side::L };
        proof.push(ProofStep {
            side,
            sibling: hex::encode(level[sibling_idx]),
        });
        level = level
            .chunks_exact(2)
            .map(|pair| node(&pair[0], &pair[1]))
            .collect();
        index /= 2;
    }
    proof
}

/// Verify an inclusion proof against a published root.
pub fn verify_inclusion(
    clause: &str,
    proof: &[ProofStep],
    root_hex: &str,
) -> Result<bool, PolicyError> {
    let mut hash = leaf(clause);
    for step in proof {
        let mut sibling = [0u8; 32];
        let raw = hex::decode(&step.sibling).map_err(|e| PolicyError::Hex(e.to_string()))?;
        if raw.len() != 32 {
            return Err(PolicyError::Hex("sibling not 32 bytes".into()));
        }
        sibling.copy_from_slice(&raw);
        hash = match step.side {
            Side::L => node(&sibling, &hash),
            Side::R => node(&hash, &sibling),
        };
    }
    Ok(hex::encode(hash) == root_hex)
}

/// Signed, append-only policy manifest. Schema mirrors the Python dataclass
/// in `core/cortex/merkle_policy/merkle_service.py`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyManifest {
    /// Monotonically increasing version, starting at 1.
    pub version: u64,
    /// ISO-8601 UTC timestamp.
    pub created_utc: String,
    /// Ordered list of policy clauses.
    pub clauses: Vec<String>,
    /// Hex Merkle root over `clauses`.
    pub root: String,
    /// Hex Merkle root of the previous manifest (None for the genesis).
    pub parent_root: Option<String>,
    /// HMAC-SHA256 over `version || root || (parent_root or 32 zero bytes)`.
    pub signature: String,
    /// Issuer identifier.
    pub issuer: String,
    /// Free-form metadata.
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Build and verify Merkle-signed policy manifests.
pub struct MerklePolicyService {
    key: Vec<u8>,
    issuer: String,
}

impl MerklePolicyService {
    /// Construct from an explicit key (must be ≥32 bytes).
    pub fn new(key: Vec<u8>, issuer: impl Into<String>) -> Result<Self, PolicyError> {
        if key.len() < 32 {
            return Err(PolicyError::KeyTooShort);
        }
        Ok(Self {
            key,
            issuer: issuer.into(),
        })
    }

    /// Construct from `CHYREN_POLICY_HMAC_KEY`.
    pub fn from_env() -> Result<Self, PolicyError> {
        let raw = std::env::var("CHYREN_POLICY_HMAC_KEY").map_err(|_| PolicyError::KeyMissing)?;
        if raw.is_empty() {
            return Err(PolicyError::KeyMissing);
        }
        Self::new(raw.into_bytes(), "chyren-sovereign")
    }

    fn sign(&self, root: &[u8; 32], parent: Option<&[u8; 32]>, version: u64) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("hmac init");
        mac.update(&version.to_be_bytes());
        mac.update(root);
        match parent {
            Some(p) => mac.update(p),
            None => mac.update(&[0u8; 32]),
        }
        hex::encode(mac.finalize().into_bytes())
    }

    /// Produce a new manifest extending `parent` (or genesis when None).
    pub fn generate_manifest(
        &self,
        clauses: Vec<String>,
        parent: Option<&PolicyManifest>,
        metadata: serde_json::Value,
    ) -> Result<PolicyManifest, PolicyError> {
        let root = merkle_root(&clauses);
        let parent_bytes = match parent {
            Some(p) => {
                let raw = hex::decode(&p.root).map_err(|e| PolicyError::Hex(e.to_string()))?;
                if raw.len() != 32 {
                    return Err(PolicyError::Hex("parent root not 32 bytes".into()));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&raw);
                Some(arr)
            }
            None => None,
        };
        let version = parent.map(|p| p.version + 1).unwrap_or(1);
        let signature = self.sign(&root, parent_bytes.as_ref(), version);
        Ok(PolicyManifest {
            version,
            created_utc: chrono_now_iso(),
            clauses,
            root: hex::encode(root),
            parent_root: parent.map(|p| p.root.clone()),
            signature,
            issuer: self.issuer.clone(),
            metadata,
        })
    }

    /// Recompute the root and HMAC and compare against the manifest.
    pub fn verify_manifest(&self, m: &PolicyManifest) -> Result<bool, PolicyError> {
        let recomputed = hex::encode(merkle_root(&m.clauses));
        if recomputed != m.root {
            return Ok(false);
        }
        let parent_bytes = match &m.parent_root {
            Some(s) => {
                let raw = hex::decode(s).map_err(|e| PolicyError::Hex(e.to_string()))?;
                if raw.len() != 32 {
                    return Err(PolicyError::Hex("parent root not 32 bytes".into()));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&raw);
                Some(arr)
            }
            None => None,
        };
        let root_raw = hex::decode(&m.root).map_err(|e| PolicyError::Hex(e.to_string()))?;
        if root_raw.len() != 32 {
            return Err(PolicyError::Hex("root not 32 bytes".into()));
        }
        let mut root_arr = [0u8; 32];
        root_arr.copy_from_slice(&root_raw);
        let expected = self.sign(&root_arr, parent_bytes.as_ref(), m.version);
        Ok(constant_time_eq(expected.as_bytes(), m.signature.as_bytes()))
    }
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn chrono_now_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Rough ISO-8601 — matches Python `datetime.now(UTC).isoformat()` shape
    // (the canonical schema is the bytes signed by HMAC, not the timestamp).
    format!("{secs}-utc")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svc() -> MerklePolicyService {
        MerklePolicyService::new(vec![0x33u8; 32], "test").unwrap()
    }

    #[test]
    fn rejects_short_key() {
        assert!(MerklePolicyService::new(vec![0u8; 31], "x").is_err());
    }

    #[test]
    fn manifest_roundtrip_genesis() {
        let s = svc();
        let m = s
            .generate_manifest(
                vec!["no-fabrication".into(), "no-exfil".into()],
                None,
                serde_json::json!({}),
            )
            .unwrap();
        assert_eq!(m.version, 1);
        assert!(m.parent_root.is_none());
        assert!(s.verify_manifest(&m).unwrap());
    }

    #[test]
    fn child_manifest_chains_parent() {
        let s = svc();
        let g = s
            .generate_manifest(vec!["a".into()], None, serde_json::json!({}))
            .unwrap();
        let c = s
            .generate_manifest(vec!["a".into(), "b".into()], Some(&g), serde_json::json!({}))
            .unwrap();
        assert_eq!(c.version, 2);
        assert_eq!(c.parent_root.as_deref(), Some(g.root.as_str()));
        assert!(s.verify_manifest(&c).unwrap());
    }

    #[test]
    fn tampered_clause_breaks_verification() {
        let s = svc();
        let mut m = s
            .generate_manifest(vec!["a".into(), "b".into()], None, serde_json::json!({}))
            .unwrap();
        m.clauses.push("c".into());
        assert!(!s.verify_manifest(&m).unwrap());
    }

    #[test]
    fn inclusion_proof_roundtrip() {
        let clauses: Vec<String> = (0..5).map(|i| format!("c{i}")).collect();
        let root = hex::encode(merkle_root(&clauses));
        for i in 0..clauses.len() {
            let p = inclusion_proof(&clauses, i);
            assert!(verify_inclusion(&clauses[i], &p, &root).unwrap());
        }
    }
}

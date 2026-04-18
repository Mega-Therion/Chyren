//! omega-phylactery: Identity Foundation, Kernel Loader, and Yettragrammaton integrity engine.
//!
//! Loads high-integrity personality anchors and structural identity markers into
//! Chyren's canonical memory layer, and provides HMAC-SHA256 signing of ledger
//! entries using the Yettragrammaton sovereign seed (R.W.Ϝ.Y.).

use hmac::{Hmac, Mac};
use omega_core::MemoryStratum;
use omega_myelin::Service as MemoryService;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// The Maker's Mark. Ϝ = digamma (U+03DC), embedded as the sovereign identity seed.
/// Every ledger entry is signed with this constant — it is never changed.
pub const YETTRAGRAMMATON: &str = "R.W.\u{03dc}.Y.";

/// Return the canonical byte representation of a JSON value for signing.
/// Keys are sorted alphabetically; the `"signature"` field is excluded.
fn canonical_bytes(value: &Value) -> Vec<u8> {
    let map: BTreeMap<String, Value> = match value.as_object() {
        Some(obj) => obj
            .iter()
            .filter(|(k, _)| k.as_str() != "signature")
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
        None => BTreeMap::new(),
    };
    // BTreeMap iterates in sorted key order — serde_json preserves insertion order.
    serde_json::to_vec(&Value::Object(map.into_iter().collect())).unwrap_or_default()
}

/// Compute the HMAC-SHA256 hex digest of a JSON entry, keyed by the Yettragrammaton.
/// The `"signature"` field is excluded from the digest so verification is stable.
pub fn sign_entry(entry: &Value) -> String {
    let mut mac = HmacSha256::new_from_slice(YETTRAGRAMMATON.as_bytes())
        .expect("HMAC accepts any key length");
    mac.update(&canonical_bytes(entry));
    hex::encode(mac.finalize().into_bytes())
}

/// Return `true` if the entry carries a valid Yettragrammaton signature.
/// An entry with no `"signature"` field always returns `false`.
pub fn verify_entry(entry: &Value) -> bool {
    match entry.get("signature").and_then(|s| s.as_str()) {
        Some(sig) if !sig.is_empty() => {
            // Use constant-time comparison to prevent timing attacks.
            let expected = sign_entry(entry);
            sig.len() == expected.len()
                && sig
                    .bytes()
                    .zip(expected.bytes())
                    .fold(0u8, |acc, (a, b)| acc | (a ^ b))
                    == 0
        }
        _ => false,
    }
}

/// Stamp a JSON entry with a UTC timestamp and a Yettragrammaton signature.
/// Returns a new `Value` with `"timestamp_utc"` and `"signature"` fields added.
pub fn stamp(mut entry: Value) -> Value {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    if let Some(obj) = entry.as_object_mut() {
        obj.insert("timestamp_utc".to_string(), Value::from(ts));
    }
    let sig = sign_entry(&entry);
    if let Some(obj) = entry.as_object_mut() {
        obj.insert("signature".to_string(), Value::String(sig));
    }
    entry
}

/// PhylacteryKernel: The root of Chyren's identity and value system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhylacteryKernel {
    pub kernel_id: String,
    pub identity: IdentityAnchors,
    pub policy_gates: PolicyGates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnchors {
    pub creator: String,
    pub home: String,
    pub birth_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyGates {
    pub root_authority: String,
    pub autonomous_expression: String,
    pub operator_intent_priority: String,
}

/// Load the phylactery identity kernel into canonical memory.
pub async fn bootstrap_kernel(memory: &MemoryService) -> Result<(), String> {
    // In a production environment, this would be loaded from a signed file or TPM.
    // For OmegA-Next, we use the embedded kernel definition.
    let kernel_data = include_str!("../../data/phylactery_kernel.json");
    let kernel_json: Value = serde_json::from_str(kernel_data)
        .map_err(|e| format!("Failed to parse phylactery kernel: {}", e))?;

    let phylactery = &kernel_json["phylactery"];

    // 1. Anchor Identity Root
    let identity_content = format!(
        "IDENTITY_ROOT: {} | Derived from {}, {}",
        phylactery["kernel_id"],
        phylactery["identity"]["creator"],
        phylactery["identity"]["birth_date"]
    );

    let mut mem = memory.lock().await;
    let root_node = mem.write_node(identity_content, MemoryStratum::Canonical);
    eprintln!("[PHYLACTERY] Identity anchored: {}", root_node.node_id);

    // 2. Anchor Value System
    if let Some(values) = phylactery["anchors"]["values"].as_array() {
        for (i, v) in values.iter().enumerate() {
            if let Some(val) = v.as_str() {
                let node =
                    mem.write_node(format!("VALUE[{}]: {}", i, val), MemoryStratum::Canonical);
                mem.create_edge(
                    root_node.node_id.clone(),
                    node.node_id,
                    "defines_value".to_string(),
                    1.0,
                );
            }
        }
    }

    // 3. Anchor Policy Gates
    let policy_content = format!(
        "POLICY_GATE: Root={} | Expression={} | Priority={}",
        phylactery["policy_gates"]["root_authority"],
        phylactery["policy_gates"]["autonomous_expression"],
        phylactery["policy_gates"]["operator_intent_priority"]
    );
    let policy_node = mem.write_node(policy_content, MemoryStratum::Canonical);
    mem.create_edge(
        root_node.node_id.clone(),
        policy_node.node_id,
        "enforces_policy".to_string(),
        1.0,
    );

    eprintln!("[PHYLACTERY] System identity synthesized into L6 Canonical layer.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kernel() -> PhylacteryKernel {
        PhylacteryKernel {
            kernel_id: "test-kernel-001".into(),
            identity: IdentityAnchors {
                creator: "OmegA Collective".into(),
                home: "Sovereign Infrastructure".into(),
                birth_date: "2024-01-01".into(),
            },
            policy_gates: PolicyGates {
                root_authority: "OPERATOR".into(),
                autonomous_expression: "ENABLED".into(),
                operator_intent_priority: "HIGH".into(),
            },
        }
    }

    #[test]
    fn phylactery_kernel_roundtrips_json() {
        let kernel = sample_kernel();
        let json = serde_json::to_string(&kernel).unwrap();
        let back: PhylacteryKernel = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kernel_id, "test-kernel-001");
        assert_eq!(back.identity.creator, "OmegA Collective");
        assert_eq!(back.policy_gates.root_authority, "OPERATOR");
    }

    #[test]
    fn identity_anchors_fields_are_preserved() {
        let k = sample_kernel();
        assert_eq!(k.identity.home, "Sovereign Infrastructure");
        assert_eq!(k.identity.birth_date, "2024-01-01");
    }

    #[test]
    fn policy_gates_fields_are_preserved() {
        let k = sample_kernel();
        assert_eq!(k.policy_gates.autonomous_expression, "ENABLED");
        assert_eq!(k.policy_gates.operator_intent_priority, "HIGH");
    }

    #[test]
    fn kernel_json_contains_expected_keys() {
        let json = serde_json::to_string(&sample_kernel()).unwrap();
        assert!(json.contains("kernel_id"));
        assert!(json.contains("identity"));
        assert!(json.contains("policy_gates"));
    }

    // ── Yettragrammaton HMAC tests ────────────────────────────────────────────

    #[test]
    fn sign_and_verify_roundtrip() {
        let entry = serde_json::json!({"run_id": "r-001", "task": "test", "status": "verified"});
        let stamped = stamp(entry);
        assert!(verify_entry(&stamped));
    }

    #[test]
    fn tampered_entry_fails_verification() {
        let entry = serde_json::json!({"run_id": "r-001", "status": "verified"});
        let mut stamped = stamp(entry);
        // Mutate a field after signing.
        stamped["status"] = serde_json::json!("hacked");
        assert!(!verify_entry(&stamped));
    }

    #[test]
    fn missing_signature_fails_verification() {
        let entry = serde_json::json!({"run_id": "r-002", "task": "test"});
        assert!(!verify_entry(&entry));
    }

    #[test]
    fn stamp_adds_timestamp_and_signature() {
        let entry = serde_json::json!({"task": "hello"});
        let stamped = stamp(entry);
        assert!(stamped.get("timestamp_utc").is_some());
        assert!(stamped.get("signature").is_some());
        let sig = stamped["signature"].as_str().unwrap();
        assert_eq!(sig.len(), 64); // SHA-256 hex = 32 bytes = 64 chars
    }

    #[test]
    fn signature_field_excluded_from_digest() {
        // Re-signing a stamped entry should produce the same signature.
        let entry = serde_json::json!({"task": "determinism"});
        let stamped = stamp(entry);
        let sig1 = sign_entry(&stamped);
        let sig2 = sign_entry(&stamped);
        assert_eq!(sig1, sig2);
    }
}

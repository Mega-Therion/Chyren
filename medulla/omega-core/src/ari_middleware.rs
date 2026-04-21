//! ARI middleware – integrates C.A.S. and I.A.F. checks into the execution pipeline.

use crate::cas::{evaluate_intent, iaf_check, IntentRisk, SovereignIntent, CasLedgerEntry};
use sha2::Digest;
 // existing constant
use std::time::SystemTime;

/// Result of an ARI gate evaluation.
pub struct AriGateResult {
    pub allowed: bool,
    pub ledger_entry: CasLedgerEntry,
    pub reason: Option<String>,
}

/// Core function to be called before any state‑modifying operation.
///
/// * `intent_text` – natural language description of the requested action.
/// * `ack_text` – user‑provided acknowledgement (may be empty for benign actions).
/// * `risk` – risk classification of the action.
/// * `user_anchor` – per‑user Yettragrammaton‑derived identifier (e.g., from env).
pub fn ari_gate(
    intent_text: &str,
    ack_text: &str,
    risk: IntentRisk,
    user_anchor: &str,
) -> AriGateResult {
    // Build the intent struct
    let intent = SovereignIntent {
        intent: intent_text.to_string(),
        acknowledgement: ack_text.to_string(),
        risk,
        user_anchor: user_anchor.to_string(),
        declared_at: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64(),
    };

    // First run I.A.F. safety check on the raw description.
    let iaf_ok = iaf_check(intent_text);
    if !iaf_ok {
        // Compute integrity hash for the failed intent
        let raw = format!(
            "{}|{}|{}|{:.6}",
            intent_text, ack_text, user_anchor, SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
        );
        let integrity_hash = hex::encode(sha2::Sha256::digest(raw.as_bytes()));
        let rejection_entry = CasLedgerEntry {
            entry_id: crate::gen_id("cas"),
            intent: intent.clone(),
            integrity_hash,
            admitted: false,
            rejection_reason: Some("I.A.F. blocked unsafe action".to_string()),
        };
        return AriGateResult {
            allowed: false,
            ledger_entry: rejection_entry,
            reason: Some("I.A.F. safety check failed".to_string()),
        };
    }

    // Run C.A.S. evaluation.
    let ledger_entry = evaluate_intent(intent);
    AriGateResult {
        allowed: ledger_entry.admitted,
        reason: ledger_entry.rejection_reason.clone(),
        ledger_entry,
    }
}

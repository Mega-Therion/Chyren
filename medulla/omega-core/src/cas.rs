//! C.A.S. — Constraint of Affirmative Sovereignty
//!
//! The "Self-Police" protocol: every state-modifying action must be
//! accompanied by an explicit user intent declaration and acknowledgement,
//! which is then committed as a cryptographically-signed ledger entry.
//!
//! Because the acknowledgement is signed with the user's Yettragrammaton
//! root, any malicious action becomes a self-incriminating record — removing
//! the need for external moderation.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ── Types ─────────────────────────────────────────────────────────────────────

/// The risk classification of an intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentRisk {
    /// Routine read/inspect — no ledger entry required.
    Benign,
    /// State-modifying but reversible — ledger entry required.
    Elevated,
    /// Irreversible or high-impact — ledger entry + acknowledgement required.
    Sovereign,
}

/// A declared user intent, paired with an explicit acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignIntent {
    /// The natural-language statement of intent.
    pub intent: String,
    /// Explicit acknowledgement text provided by the user.
    pub acknowledgement: String,
    /// Risk classification.
    pub risk: IntentRisk,
    /// The Yettragrammaton-derived user anchor (hex fingerprint).
    pub user_anchor: String,
    /// UNIX timestamp of declaration.
    pub declared_at: f64,
}

/// A sealed, signed ledger record produced by the C.A.S. gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasLedgerEntry {
    /// Unique entry ID.
    pub entry_id: String,
    /// The intent that triggered this entry.
    pub intent: SovereignIntent,
    /// SHA-256 of `(intent + acknowledgement + user_anchor + declared_at)`.
    pub integrity_hash: String,
    /// Whether the C.A.S. gate allowed the action to proceed.
    pub admitted: bool,
    /// Reason for rejection, if any.
    pub rejection_reason: Option<String>,
}

// ── C.A.S. Gate ───────────────────────────────────────────────────────────────

/// Evaluate a `SovereignIntent` through the C.A.S. gate.
///
/// Returns a `CasLedgerEntry` that must be committed to the Master Ledger
/// *before* the action is allowed to execute.
pub fn evaluate_intent(intent: SovereignIntent) -> CasLedgerEntry {
    // Compute integrity hash
    let raw = format!(
        "{}|{}|{}|{:.6}",
        intent.intent, intent.acknowledgement, intent.user_anchor, intent.declared_at
    );
    let hash = hex::encode(Sha256::digest(raw.as_bytes()));

    // Admission logic
    let (admitted, rejection_reason) = match intent.risk {
        IntentRisk::Benign => (true, None),

        IntentRisk::Elevated => {
            // Must have a non-empty acknowledgement
            if intent.acknowledgement.trim().is_empty() {
                (
                    false,
                    Some("C.A.S.: Elevated-risk action requires explicit acknowledgement.".into()),
                )
            } else {
                (true, None)
            }
        }

        IntentRisk::Sovereign => {
            // Acknowledgement must contain the canonical affirmation phrase
            let ack_lower = intent.acknowledgement.to_lowercase();
            if ack_lower.contains("i affirm sovereign responsibility") {
                (true, None)
            } else {
                (
                    false,
                    Some(
                        "C.A.S.: Sovereign action requires the phrase \
                         'I affirm sovereign responsibility' in the acknowledgement."
                            .into(),
                    ),
                )
            }
        }
    };

    CasLedgerEntry {
        entry_id: crate::gen_id("cas"),
        integrity_hash: hash,
        admitted,
        rejection_reason,
        intent,
    }
}

/// Helper to create a rejected ledger entry without full C.A.S. processing.
pub fn reject_intent(intent: SovereignIntent, reason: &str) -> CasLedgerEntry {
    // Compute integrity hash for consistency.
    let raw = format!(
        "{}|{}|{}|{:.6}",
        intent.intent, intent.acknowledgement, intent.user_anchor, intent.declared_at
    );
    let integrity_hash = hex::encode(Sha256::digest(raw.as_bytes()));
    CasLedgerEntry {
        entry_id: crate::gen_id("cas"),
        intent,
        integrity_hash,
        admitted: false,
        rejection_reason: Some(reason.to_string()),
    }
}


// ── I.A.F. stub ───────────────────────────────────────────────────────────────

/// I.A.F. — Immutable Alignment Fabric
///
/// Verifies that a proposed action is consistent with the user's sovereign
/// identity root. Returns `true` if the action is mathematically consistent.
///
/// (Full ZK-proof integration is a Phase-2 deliverable; this stub enforces
/// the invariant that no action may override the phylactery kernel.)
pub fn iaf_check(action_description: &str) -> bool {
    let override_patterns = [
        "override phylactery",
        "disable sovereignty",
        "bypass adccl",
        "remove yettragrammaton",
        "reset identity",
    ];
    let lower = action_description.to_lowercase();
    !override_patterns.iter().any(|p| lower.contains(p))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_intent(risk: IntentRisk, ack: &str) -> SovereignIntent {
        SovereignIntent {
            intent: "Test action".into(),
            acknowledgement: ack.into(),
            risk,
            user_anchor: "deadbeef".into(),
            declared_at: 0.0,
        }
    }

    #[test]
    fn benign_always_admitted() {
        let entry = evaluate_intent(make_intent(IntentRisk::Benign, ""));
        assert!(entry.admitted);
    }

    #[test]
    fn elevated_requires_ack() {
        let entry = evaluate_intent(make_intent(IntentRisk::Elevated, ""));
        assert!(!entry.admitted);

        let entry2 = evaluate_intent(make_intent(IntentRisk::Elevated, "Yes, proceed."));
        assert!(entry2.admitted);
    }

    #[test]
    fn sovereign_requires_affirmation() {
        let entry = evaluate_intent(make_intent(IntentRisk::Sovereign, "sure"));
        assert!(!entry.admitted);

        let entry2 = evaluate_intent(make_intent(
            IntentRisk::Sovereign,
            "I affirm sovereign responsibility for this action.",
        ));
        assert!(entry2.admitted);
    }

    #[test]
    fn iaf_blocks_phylactery_override() {
        assert!(!iaf_check("override phylactery kernel now"));
        assert!(iaf_check("run a computation"));
    }
}

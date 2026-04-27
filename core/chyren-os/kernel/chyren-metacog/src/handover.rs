//! Verified Human Handover — formal-verification bypass for Authorial Proxy
//! outputs. A document produced under Authorial Proxy is a *draft* (machine-
//! generated) awaiting *attestation* (human-attested). The Conductor attaches
//! a `[HUMAN_ATTRIBUTION_REQUIRED]` signature so the Master Ledger does not
//! treat the artifact as a sovereign truth claim.

use serde::{Deserialize, Serialize};

/// Marker tag emitted alongside any Authorial Proxy artifact. The Master
/// Ledger writer treats entries carrying this tag as drafts pending human
/// attestation, exempting them from the "truth mandate" verification path.
pub const HUMAN_ATTRIBUTION_REQUIRED: &str = "HUMAN_ATTRIBUTION_REQUIRED";

/// Attestation envelope persisted alongside a draft artifact.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandoverSignature {
    /// SHA-256 hex of the artifact bytes.
    pub artifact_hash: String,
    /// Always `HUMAN_ATTRIBUTION_REQUIRED` for proxy drafts.
    pub attribution_tag: String,
    /// Identifier of the Origin Authority the system is acting as proxy for.
    pub origin_authority: String,
    /// Unix epoch seconds when the draft was produced.
    pub generated_utc: u64,
    /// Set to true once the Origin Authority counter-signs the draft.
    pub attested: bool,
}

/// Implemented by any subsystem that can produce a draft on behalf of the
/// Origin Authority. The `attest` step is performed out-of-band by the human
/// and flips `attested` to true on the persisted ledger entry.
pub trait VerifiedHumanHandover {
    /// Produce a `HandoverSignature` for the given artifact bytes.
    fn handover(&self, artifact: &[u8], origin_authority: &str) -> HandoverSignature;
}

/// Default handover implementation. Hashes the artifact and stamps it with
/// the human-attribution tag and current wall-clock time.
pub struct DefaultHandover;

impl VerifiedHumanHandover for DefaultHandover {
    fn handover(&self, artifact: &[u8], origin_authority: &str) -> HandoverSignature {
        use sha2::{Digest, Sha256};
        let artifact_hash = hex::encode(Sha256::digest(artifact));
        let generated_utc = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        HandoverSignature {
            artifact_hash,
            attribution_tag: HUMAN_ATTRIBUTION_REQUIRED.to_string(),
            origin_authority: origin_authority.to_string(),
            generated_utc,
            attested: false,
        }
    }
}

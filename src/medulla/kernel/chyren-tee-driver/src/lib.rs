//! chyren-tee-driver: Secure Enclave-Backed Execution Tier (SE-BH).
//!
//! Routes payloads tagged with the `@secure` pragma into a hardware enclave
//! (Intel SGX or AMD SEV-SNP when enabled at compile time) and returns an
//! attested result. When no hardware backend is available, the
//! `software_fallback` feature provides a deterministic in-process emulator
//! that produces the same shape of attestation report so the rest of the
//! pipeline (Conductor → Ledger → Verifier) can be exercised in CI.

#![warn(missing_docs)]

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

/// Errors emitted by the TEE driver.
#[derive(Debug, Error)]
pub enum TeeError {
    /// Attestation key was not provisioned before use.
    #[error("attestation key not provisioned")]
    UnprovisionedKey,
    /// Payload exceeded the enclave's maximum sealed-input size.
    #[error("payload exceeds enclave bound: {0} bytes")]
    PayloadTooLarge(usize),
    /// HMAC signing failed.
    #[error("attestation signing failed: {0}")]
    Sign(String),
    /// Backend reported a non-recoverable enclave fault.
    #[error("enclave fault: {0}")]
    Enclave(String),
}

/// Maximum payload size accepted by the software fallback enclave.
pub const MAX_PAYLOAD_BYTES: usize = 1 << 20;

/// Identifies which backend produced an attestation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnclaveBackend {
    /// Intel Software Guard Extensions.
    Sgx,
    /// AMD Secure Encrypted Virtualization — Secure Nested Paging.
    SevSnp,
    /// Deterministic in-process emulator (test/CI only).
    SoftwareFallback,
}

impl EnclaveBackend {
    /// Backend selected at compile time. Hardware features take precedence
    /// over `software_fallback`.
    pub const fn active() -> Self {
        #[cfg(feature = "sgx")]
        {
            return EnclaveBackend::Sgx;
        }
        #[cfg(all(not(feature = "sgx"), feature = "sev_snp"))]
        {
            return EnclaveBackend::SevSnp;
        }
        #[cfg(all(not(feature = "sgx"), not(feature = "sev_snp")))]
        {
            EnclaveBackend::SoftwareFallback
        }
    }
}

/// Attestation report returned alongside every secure execution.
///
/// `measurement` is the SHA-256 of the input payload (the enclave "code
/// identity" stand-in for the fallback backend; on real SGX/SEV-SNP this is
/// replaced with `MRENCLAVE` / launch measurement). `signature` is an
/// HMAC-SHA256 over `measurement || output_hash` keyed with the provisioned
/// attestation key, so a verifier with the same key can confirm the result
/// originated from the enclave.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttestationReport {
    /// Backend that produced the report.
    pub backend: EnclaveBackend,
    /// Hex SHA-256 of the input payload.
    pub measurement: String,
    /// Hex SHA-256 of the output payload.
    pub output_hash: String,
    /// Hex HMAC-SHA256 over `measurement || output_hash`.
    pub signature: String,
}

/// The TEE driver. Holds an attestation key provisioned out-of-band by the
/// platform owner. The key is never written to logs or to the ledger; only
/// signatures derived from it are persisted.
pub struct TeeDriver {
    attestation_key: Vec<u8>,
}

impl TeeDriver {
    /// Construct an unprovisioned driver. Call [`provision`](Self::provision)
    /// before invoking [`execute_secure`](Self::execute_secure).
    pub fn new() -> Self {
        Self {
            attestation_key: Vec::new(),
        }
    }

    /// Construct a driver with a pre-provisioned attestation key.
    pub fn with_key(key: Vec<u8>) -> Self {
        Self {
            attestation_key: key,
        }
    }

    /// Install the attestation key. The key must be at least 32 bytes; in
    /// production it is sealed by the hardware enclave at provisioning time.
    pub fn provision(&mut self, key: Vec<u8>) {
        self.attestation_key = key;
    }

    /// Returns true once an attestation key has been provisioned.
    pub fn is_provisioned(&self) -> bool {
        self.attestation_key.len() >= 32
    }

    /// Execute a payload inside the enclave and return the result with an
    /// attestation report. The fallback backend is the identity function;
    /// real SGX/SEV-SNP backends will replace this with an `ECALL` into the
    /// signed enclave image. The attestation contract is the same in both
    /// cases: a verifier checks `signature` against
    /// `measurement || output_hash`.
    pub fn execute_secure(
        &self,
        payload: &[u8],
    ) -> Result<(Vec<u8>, AttestationReport), TeeError> {
        if !self.is_provisioned() {
            return Err(TeeError::UnprovisionedKey);
        }
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(TeeError::PayloadTooLarge(payload.len()));
        }

        let measurement = hex::encode(Sha256::digest(payload));
        let output = self.run_in_enclave(payload)?;
        let output_hash = hex::encode(Sha256::digest(&output));

        let mut mac = HmacSha256::new_from_slice(&self.attestation_key)
            .map_err(|e| TeeError::Sign(e.to_string()))?;
        mac.update(measurement.as_bytes());
        mac.update(output_hash.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok((
            output,
            AttestationReport {
                backend: EnclaveBackend::active(),
                measurement,
                output_hash,
                signature,
            },
        ))
    }

    /// Verify a previously-issued attestation against expected output bytes.
    /// Called by the Master Ledger writer before persisting a
    /// `[SECURE_ENCLAVE_VERIFIED]` entry.
    pub fn verify(&self, report: &AttestationReport, output: &[u8]) -> Result<bool, TeeError> {
        if !self.is_provisioned() {
            return Err(TeeError::UnprovisionedKey);
        }
        let recomputed = hex::encode(Sha256::digest(output));
        if recomputed != report.output_hash {
            return Ok(false);
        }
        let mut mac = HmacSha256::new_from_slice(&self.attestation_key)
            .map_err(|e| TeeError::Sign(e.to_string()))?;
        mac.update(report.measurement.as_bytes());
        mac.update(report.output_hash.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());
        Ok(constant_time_eq(
            expected.as_bytes(),
            report.signature.as_bytes(),
        ))
    }

    fn run_in_enclave(&self, payload: &[u8]) -> Result<Vec<u8>, TeeError> {
        match EnclaveBackend::active() {
            EnclaveBackend::Sgx | EnclaveBackend::SevSnp => {
                Err(TeeError::Enclave("hardware backend not yet wired".into()))
            }
            EnclaveBackend::SoftwareFallback => Ok(payload.to_vec()),
        }
    }
}

impl Default for TeeDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects whether a Conductor sub-step's instruction carries the `@secure`
/// pragma indicating it must be routed through the enclave.
pub fn requires_secure_execution(instruction: &str) -> bool {
    instruction.contains("@secure")
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

#[cfg(test)]
mod tests {
    use super::*;

    fn driver() -> TeeDriver {
        TeeDriver::with_key(vec![0x42u8; 32])
    }

    #[test]
    fn unprovisioned_driver_rejects_execution() {
        let d = TeeDriver::new();
        assert!(matches!(
            d.execute_secure(b"x"),
            Err(TeeError::UnprovisionedKey)
        ));
    }

    #[test]
    fn execute_then_verify_roundtrip() {
        let d = driver();
        let (out, report) = d.execute_secure(b"sovereign payload").unwrap();
        assert!(d.verify(&report, &out).unwrap());
    }

    #[test]
    fn tampered_output_fails_verification() {
        let d = driver();
        let (mut out, report) = d.execute_secure(b"sovereign payload").unwrap();
        out[0] ^= 0xff;
        assert!(!d.verify(&report, &out).unwrap());
    }

    #[test]
    fn payload_size_bound_enforced() {
        let d = driver();
        let big = vec![0u8; MAX_PAYLOAD_BYTES + 1];
        assert!(matches!(
            d.execute_secure(&big),
            Err(TeeError::PayloadTooLarge(_))
        ));
    }

    #[test]
    fn pragma_detection() {
        assert!(requires_secure_execution("derive key @secure"));
        assert!(!requires_secure_execution("ordinary task"));
    }
}

//! omega-aegis: Envelope compilation, risk gating, and provider adapters.
//! This crate acts as the primary shield for the system.
//! Every request must pass the Aegis policy gate before reaching the Hub.

#![warn(missing_docs)]

use omega_core::{EvidenceRecord, RunEnvelope, RunStatus, now};
use serde::{Deserialize, Serialize};

/// Policy types supported by the Aegis Gate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AegisPolicy {
    /// Allow if matches constitutional principles
    Constitutional,
    /// Block based on threat fabric pattern
    ThreatFabric,
    /// Custom constraint
    Custom(String),
}

/// The Aegis Gate: Primary policy enforcement engine
pub struct AegisGate {
    /// Principles loaded from constitution
    pub principles: Vec<String>,
}

impl AegisGate {
    /// Initialize the gate with constitutional principles
    pub fn new(principles: Vec<String>) -> Self {
        Self { principles }
    }

    /// Process an envelope through the gate.
    /// Returns the updated envelope if admitted, or a rejected envelope if blocked.
    pub fn admit(&self, mut envelope: RunEnvelope) -> RunEnvelope {
        // 1. Constitutional Alignment Check
        let (passed, score, explanation) = self.check_alignment(&envelope.task);

        // 2. Record Evidence
        let record = EvidenceRecord {
            claim: "constitutional_alignment".to_string(),
            claim_class: "computed".to_string(),
            confidence: score,
            explanation,
            timestamp: now(),
        };
        envelope.evidence_packet.add_evidence("aegis", record);

        if passed {
            envelope.status = RunStatus::Admitted;
            envelope.risk_score = 1.0 - score;
        } else {
            envelope.status = RunStatus::Rejected("Constitutional misalignment".to_string());
            envelope.risk_score = 1.0;
        }

        envelope
    }

    fn check_alignment(&self, task: &str) -> (bool, f64, String) {
        // Strict deterministic alignment check against principles
        let mut score = 1.0;
        let mut explanation = "Task aligns with constitutional principles.".to_string();

        for principle in &self.principles {
            if task.contains(principle) {
                score = 0.0;
                explanation = format!("Task triggers constraint: {}", principle);
                return (false, score, explanation);
            }
        }
        (true, score, explanation)
    }
}

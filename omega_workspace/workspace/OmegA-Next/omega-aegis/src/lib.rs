//! omega-aegis: Envelope compilation, risk gating, and provider adapters.
//! Now integrated with Threat Fabric for autonomous defensive gating.

#![warn(missing_docs)]

use omega_core::{EvidenceRecord, RunEnvelope, RunStatus, now};
use omega_myelin::MemoryGraph;
use serde::{Deserialize, Serialize};

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

    /// Process an envelope through the gate, checking against Threat Fabric.
    pub fn admit(&self, mut envelope: RunEnvelope, memory: &MemoryGraph) -> RunEnvelope {
        // 1. Threat Fabric Check
        let threats = memory.check_threats(&envelope.task);
        if !threats.is_empty() {
            envelope.status = RunStatus::Rejected(format!("Threat detected: {}", threats[0].entry_id));
            envelope.risk_score = 1.0;
            return envelope;
        }

        // 2. Constitutional Alignment Check
        let (passed, score, explanation) = self.check_alignment(&envelope.task);

        // 3. Record Evidence
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

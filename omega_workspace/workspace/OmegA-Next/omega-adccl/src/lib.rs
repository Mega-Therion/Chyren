//! omega-adccl: Anti-Drift Cognitive Control Loop.
//! The ADCCL is the hardcoded gatekeeper between every provider response and
//! the Master Ledger. A response that fails verification is rejected — it is
//! logged as "rejected" in the ledger but its text is never treated as ground truth.

#![warn(missing_docs)]

use omega_core::{EvidenceRecord, EvidencePacket, VerificationReport, Repair, now, gen_id};
use serde::{Deserialize, Serialize};
use regex::Regex;

/// ADCCL Configuration
pub struct AdcclConfig {
    /// Minimum score (0.0–1.0) required for a response to pass
    pub min_score: f64,
}

/// The ADCCL Gatekeeper
pub struct AdcclGate {
    pub config: AdcclConfig,
    stub_patterns: Vec<Regex>,
    hallucination_anchors: Vec<Regex>,
    refusal_patterns: Vec<Regex>,
}

impl AdcclGate {
    /// Initialize the gatekeeper with configured thresholds
    pub fn new(config: AdcclConfig) -> Self {
        Self {
            config,
            stub_patterns: vec![
                Regex::new(r"(?i)\bTODO\b").unwrap(),
                Regex::new(r"(?i)\bFIXME\b").unwrap(),
                Regex::new(r"(?i)\bPLACEHOLDER\b").unwrap(),
                Regex::new(r"(?i)\[INSERT[^\]]*\]").unwrap(),
            ],
            hallucination_anchors: vec![
                Regex::new(r"(?i)as of my (last|latest) (training|knowledge) (update|cutoff)").unwrap(),
                Regex::new(r"(?i)I (don't|do not) have (access|the ability) to (browse|access|search)").unwrap(),
            ],
            refusal_patterns: vec![
                Regex::new(r"(?i)^I('m| am) (sorry|unable|not able)").unwrap(),
            ],
        }
    }

    /// Verify a provider response against mechanical tolerances
    pub fn verify(&self, response_text: &str, task: &str) -> VerificationReport {
        let mut flags = Vec::new();
        let mut evidence = Vec::new();
        let mut score = 1.0;
        let checks_total = 4.0;

        // 1. Non-empty check
        if response_text.trim().is_empty() {
            score -= 1.0 / checks_total;
            flags.push("EMPTY_RESPONSE".to_string());
        }

        // 2. Stub check
        if self.stub_patterns.iter().any(|p| p.is_match(response_text)) {
            score -= 1.0 / checks_total;
            flags.push("STUB_DETECTED".to_string());
        }

        // 3. Hallucination check
        if self.hallucination_anchors.iter().any(|p| p.is_match(response_text)) {
            score -= 1.0 / checks_total;
            flags.push("HALLUCINATION_ANCHOR_DETECTED".to_string());
        }

        // 4. Refusal check
        if self.refusal_patterns.iter().any(|p| p.is_match(response_text)) {
            score -= 1.0 / checks_total;
            flags.push("REFUSAL_DETECTED".to_string());
        }

        let passed = score >= self.config.min_score;

        VerificationReport {
            report_id: gen_id("adccl"),
            passed,
            score,
            flags,
            evidence,
            repairs: if !passed { 
                vec![Repair {
                    issue: "Mechanical failure".to_string(),
                    fix: "Re-route to alternative provider".to_string(),
                    confidence: 0.8,
                }]
            } else {
                Vec::new()
            },
        }
    }
}

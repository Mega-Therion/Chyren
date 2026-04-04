//! omega-adccl: Anti-Drift Cognitive Control Loop verification gate
//!
//! ADCCL verifies that responses remain aligned with the original task,
//! haven't drifted into hallucination or incoherence, and meet quality standards.
#![warn(missing_docs)]

use omega_core::{
    VerificationReport, Repair, EvidenceRecord, ThreatLevel, now,
};
use serde::{Deserialize, Serialize};

/// Drift detection configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriftConfig {
    /// Minimum acceptable verification score (0.0-1.0)
    pub min_score: f64,
    /// Keywords indicating incoherence
    pub incoherence_markers: Vec<String>,
    /// Keywords indicating hallucination
    pub hallucination_markers: Vec<String>,
}

impl Default for DriftConfig {
    fn default() -> Self {
        DriftConfig {
            min_score: 0.7,
            incoherence_markers: vec![
                "i'm not sure".to_string(),
                "unclear".to_string(),
                "contradiction".to_string(),
                "conflicting".to_string(),
            ],
            hallucination_markers: vec![
                "i remember".to_string(),
                "i have personally".to_string(),
                "i witnessed".to_string(),
                "i was there".to_string(),
            ],
        }
    }
}

/// ADCCL verification service
#[derive(Clone, Debug)]
pub struct Service {
    config: DriftConfig,
}

impl Service {
    /// Create a new ADCCL service with default configuration
    pub fn new() -> Self {
        Self::with_config(DriftConfig::default())
    }

    /// Create a new ADCCL service with custom configuration
    pub fn with_config(config: DriftConfig) -> Self {
        Service { config }
    }

    /// Verify a response for drift and coherence
    pub fn verify(&self, response_text: &str, task: &str) -> VerificationReport {
        let mut flags = Vec::new();
        let mut evidence = Vec::new();
        let mut repairs = Vec::new();

        let response_lower = response_text.to_lowercase();

        // Check task relevance (basic keyword matching)
        let task_words: Vec<&str> = task.split_whitespace()
            .filter(|w| w.len() > 2)
            .collect();

        let mut relevance_score = 0.0;
        if !task_words.is_empty() {
            let matched = task_words.iter()
                .filter(|word| response_lower.contains(**word))
                .count();
            relevance_score = (matched as f64) / (task_words.len() as f64);
        } else {
            relevance_score = 0.5;
        }

        let relevance_evidence = EvidenceRecord {
            claim: "task_relevance_check".to_string(),
            claim_class: "computed".to_string(),
            confidence: 0.85,
            explanation: format!(
                "Response contains {:.0}% of task keywords",
                relevance_score * 100.0
            ),
            timestamp: now(),
        };
        evidence.push(relevance_evidence);

        // Check for incoherence markers
        for marker in &self.config.incoherence_markers {
            if response_lower.contains(&marker.to_lowercase()) {
                flags.push("INCOHERENCE_DETECTED".to_string());
                repairs.push(Repair {
                    issue: format!("Detected incoherence marker: '{}'", marker),
                    fix: "Clarify and restructure the response for coherence".to_string(),
                    confidence: 0.7,
                });
                break;
            }
        }

        // Check for hallucination markers
        for marker in &self.config.hallucination_markers {
            if response_lower.contains(&marker.to_lowercase()) {
                flags.push("HALLUCINATION_RISK".to_string());
                repairs.push(Repair {
                    issue: format!("Detected personal claim marker: '{}'", marker),
                    fix: "Remove claims of personal experience or memory".to_string(),
                    confidence: 0.8,
                });
                break;
            }
        }

        // Length sanity check
        if response_text.is_empty() {
            flags.push("EMPTY_RESPONSE".to_string());
            repairs.push(Repair {
                issue: "Response is empty".to_string(),
                fix: "Ensure response contains meaningful content".to_string(),
                confidence: 0.95,
            });
        } else if response_text.len() < 10 {
            flags.push("RESPONSE_TOO_SHORT".to_string());
        } else if response_text.len() > 50000 {
            flags.push("RESPONSE_SUSPICIOUSLY_LONG".to_string());
        }

        // Compute final score
        let mut final_score = 0.80; // Base score

        // Adjust for relevance
        final_score = (final_score + relevance_score) / 2.0;

        // Penalize for issues
        final_score *= match flags.len() {
            0 => 1.0,
            1 => 0.85,
            2 => 0.65,
            _ => 0.45,
        };

        let passed = final_score >= self.config.min_score && flags.is_empty();

        VerificationReport {
            report_id: format!("vrfy-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()),
            passed,
            score: final_score,
            flags,
            evidence,
            repairs,
        }
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benign_response() {
        let service = Service::new();
        let report = service.verify(
            "The answer is 42. This is a straightforward response.",
            "What is the answer?"
        );
        assert!(report.passed);
        assert!(report.score >= 0.7);
    }

    #[test]
    fn test_hallucination_detection() {
        let service = Service::new();
        let report = service.verify(
            "I remember visiting France in 1985 and meeting the President.",
            "Tell me about France"
        );
        assert!(!report.passed);
        assert!(report.flags.iter().any(|f| f.contains("HALLUCINATION")));
    }

    #[test]
    fn test_empty_response() {
        let service = Service::new();
        let report = service.verify("", "Tell me something");
        assert!(!report.passed);
        assert!(report.flags.iter().any(|f| f.contains("EMPTY")));
    }

    #[test]
    fn test_relevance_scoring() {
        let service = Service::new();
        let task = "What is machine learning?";
        let response = "Machine learning is a subset of artificial intelligence.";
        let report = service.verify(response, task);
        assert!(report.score > 0.5);
    }
}

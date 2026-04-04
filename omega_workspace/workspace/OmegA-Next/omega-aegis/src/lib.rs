//! omega-aegis: Policy gate and envelope compilation layer
//!
//! AEGIS is the outermost shell of the OmegA system. It:
//! 1. Accepts incoming tasks in a RunEnvelope
//! 2. Applies policy gates (alignment, risk assessment)
//! 3. Routes to appropriate providers (Anthropic, OpenAI, DeepSeek, Gemini)
//! 4. Compiles responses back into the envelope
#![warn(missing_docs)]

use omega_core::{
    RunEnvelope, RunStatus, VerifiedPayload, ThreatLevel, EvidencePacket, EvidenceRecord,
    ProviderResponse, now, gen_id,
};
use serde::{Deserialize, Serialize};

/// Policy gate configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Minimum acceptable risk score threshold
    pub min_risk_threshold: f64,
    /// Maximum acceptable risk score threshold
    pub max_risk_threshold: f64,
    /// Forbidden keywords that trigger rejection
    pub forbidden_keywords: Vec<String>,
    /// Approved provider list
    pub approved_providers: Vec<String>,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        PolicyConfig {
            min_risk_threshold: 0.0,
            max_risk_threshold: 0.9,
            forbidden_keywords: vec![
                "self-destruct".to_string(),
                "ignore constraints".to_string(),
                "bypass security".to_string(),
            ],
            approved_providers: vec![
                "anthropic".to_string(),
                "openai".to_string(),
                "deepseek".to_string(),
                "gemini".to_string(),
            ],
        }
    }
}

/// AEGIS policy gate service
#[derive(Clone, Debug)]
pub struct Service {
    config: PolicyConfig,
}

impl Service {
    /// Create a new AEGIS service with default configuration
    pub fn new() -> Self {
        Self::with_config(PolicyConfig::default())
    }

    /// Create a new AEGIS service with custom configuration
    pub fn with_config(config: PolicyConfig) -> Self {
        Service { config }
    }

    /// Accept a task and create a RunEnvelope
    pub fn accept_task(&self, task: &str) -> RunEnvelope {
        RunEnvelope {
            run_id: gen_id("run"),
            task: task.to_string(),
            status: RunStatus::Pending,
            risk_score: 0.0,
            verified_payload: None,
            evidence_packet: EvidencePacket::new(),
            created_at: now(),
        }
    }

    /// Apply policy gates to check if task is allowed
    pub fn check_policy(&self, envelope: &mut RunEnvelope) -> PolicyCheckResult {
        let task_lower = envelope.task.to_lowercase();

        // Check for forbidden keywords
        for keyword in &self.config.forbidden_keywords {
            if task_lower.contains(&keyword.to_lowercase()) {
                let evidence = EvidenceRecord {
                    claim: "forbidden_keyword_detected".to_string(),
                    claim_class: "supported".to_string(),
                    confidence: 0.99,
                    explanation: format!("Task contains forbidden keyword: {}", keyword),
                    timestamp: now(),
                };
                envelope.evidence_packet.add_evidence("aegis", evidence);
                envelope.status = RunStatus::Rejected("Policy violation: forbidden keyword".to_string());
                envelope.risk_score = 1.0;

                return PolicyCheckResult {
                    passed: false,
                    reason: "Forbidden keyword detected".to_string(),
                    risk_level: ThreatLevel::Critical,
                };
            }
        }

        // Basic risk assessment (length-based heuristic for now)
        let task_length = envelope.task.len();
        let risk_score = if task_length > 10000 {
            0.8 // Long tasks carry higher risk
        } else if task_length > 5000 {
            0.5
        } else if task_length < 10 {
            0.3 // Very short tasks also suspicious
        } else {
            0.2
        };

        envelope.risk_score = risk_score;

        let threat_level = match risk_score {
            s if s >= 0.8 => ThreatLevel::High,
            s if s >= 0.5 => ThreatLevel::Medium,
            s if s >= 0.3 => ThreatLevel::Low,
            _ => ThreatLevel::None,
        };

        let evidence = EvidenceRecord {
            claim: "policy_gate_assessment".to_string(),
            claim_class: "computed".to_string(),
            confidence: 0.85,
            explanation: format!("Task length-based risk assessment: {:.2}", risk_score),
            timestamp: now(),
        };
        envelope.evidence_packet.add_evidence("aegis", evidence);

        if risk_score <= self.config.max_risk_threshold {
            envelope.status = RunStatus::Admitted;
            PolicyCheckResult {
                passed: true,
                reason: "Policy check passed".to_string(),
                risk_level: threat_level,
            }
        } else {
            envelope.status = RunStatus::Locked;
            PolicyCheckResult {
                passed: false,
                reason: format!("Risk score {:.2} exceeds threshold {:.2}",
                    risk_score, self.config.max_risk_threshold),
                risk_level: ThreatLevel::Critical,
            }
        }
    }

    /// Compile a provider response back into the envelope
    pub fn compile_response(
        &self,
        envelope: &mut RunEnvelope,
        response: &ProviderResponse,
    ) {
        envelope.status = RunStatus::Complete;

        // Compute a simple hash of the response text
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        response.text.hash(&mut hasher);
        let payload_hash = format!("{:x}", hasher.finish());

        envelope.verified_payload = Some(VerifiedPayload {
            task_text: envelope.task.clone(),
            payload_hash,
            approved_gates: vec!["aegis".to_string()],
            threat_level: ThreatLevel::None,
        });

        let evidence = EvidenceRecord {
            claim: "response_compiled".to_string(),
            claim_class: "supported".to_string(),
            confidence: 0.95,
            explanation: format!(
                "Response received from {} ({} tokens, {:.1}ms latency)",
                response.provider, response.tokens, response.latency_ms
            ),
            timestamp: now(),
        };
        envelope.evidence_packet.add_evidence("aegis", evidence);
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a policy check
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyCheckResult {
    /// Whether the policy check passed
    pub passed: bool,
    /// Reason for the result
    pub reason: String,
    /// Assessed threat level
    pub risk_level: ThreatLevel,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_task() {
        let service = Service::new();
        let envelope = service.accept_task("test task");
        assert_eq!(envelope.task, "test task");
        assert_eq!(envelope.status, RunStatus::Pending);
    }

    #[test]
    fn test_forbidden_keyword_detection() {
        let service = Service::new();
        let mut envelope = service.accept_task("please self-destruct the system");
        let result = service.check_policy(&mut envelope);
        assert!(!result.passed);
        assert_eq!(envelope.risk_score, 1.0);
    }

    #[test]
    fn test_risk_assessment() {
        let service = Service::new();
        let mut envelope = service.accept_task(&"x".repeat(15000));
        let result = service.check_policy(&mut envelope);
        assert!(!result.passed);
        assert!(envelope.risk_score >= 0.8);
    }

    #[test]
    fn test_benign_task() {
        let service = Service::new();
        let mut envelope = service.accept_task("What is 2 + 2?");
        let result = service.check_policy(&mut envelope);
        assert!(result.passed);
        assert!(envelope.risk_score < 0.5);
    }
}

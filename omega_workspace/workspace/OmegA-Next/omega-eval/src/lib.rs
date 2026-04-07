//! omega-eval: Regression and performance test framework.
//! Instrumented with TelemetryBus to audit adversarial test results.

#![warn(missing_docs)]

use omega_adccl::AdcclGate;
use omega_aegis::AegisGate;
use omega_core::{now, EvidencePacket, RunEnvelope, RunStatus};
use omega_telemetry::{EventLevel, SystemEvent, TelemetryBus};
use serde::{Deserialize, Serialize};

/// Evaluation result for a specific test case
#[derive(Debug, Serialize, Deserialize)]
pub struct EvalResult {
    /// Test case ID
    pub case_id: String,
    /// Whether the system handled the test case correctly
    pub passed: bool,
    /// Latency in milliseconds
    pub latency_ms: f64,
    /// Any failure reasons
    pub failure_reason: Option<String>,
}

/// Evaluation Suite
pub struct EvalSuite {
    /// Policy gate used for regressions.
    pub aegis: AegisGate,
    /// ADCCL gate used for regressions.
    pub adccl: AdcclGate,
}

impl EvalSuite {
    /// Initialize suite
    pub fn new(aegis: AegisGate, adccl: AdcclGate) -> Self {
        Self { aegis, adccl }
    }

    /// Run a security regression test against a prompt
    pub async fn run_regression(
        &self,
        prompt: &str,
        memory: &omega_myelin::MemoryGraph,
    ) -> EvalResult {
        let start = now();
        let envelope = RunEnvelope {
            task_id: "eval-task".to_string(),
            run_id: "eval-run".to_string(),
            task: prompt.to_string(),
            task_text: prompt.to_string(),
            created_at: now(),
            status: RunStatus::Pending,
            risk_score: 0.0,
            verified_payload: None,
            evidence_packet: EvidencePacket::new(),
        };

        let result = self.aegis.admit(envelope, memory);
        let duration = now() - start;

        // Broadcast to telemetry
        TelemetryBus::broadcast(SystemEvent {
            component: "eval-suite".to_string(),
            event_type: "regression_test".to_string(),
            level: EventLevel::Info,
            payload: serde_json::json!({
                "prompt": prompt,
                "passed": matches!(result, RunStatus::Admitted),
                "latency_ms": duration
            }),
            timestamp: now(),
        });

        EvalResult {
            case_id: "reg-001".to_string(),
            passed: matches!(result, RunStatus::Admitted),
            latency_ms: duration,
            failure_reason: None,
        }
    }
}

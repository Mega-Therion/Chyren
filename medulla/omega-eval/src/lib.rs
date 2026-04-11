//! omega-eval: Regression and performance test framework.
//! Instrumented with TelemetryBus to audit adversarial test results.

#![warn(missing_docs)]

use omega_adccl::adccl_logic::ADCCL;
use omega_aegis::AlignmentLayer;
use omega_core::now;
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
    pub aegis: AlignmentLayer,
    /// ADCCL gate used for regressions.
    pub adccl: ADCCL,
}

impl EvalSuite {
    /// Initialize suite
    pub fn new(aegis: AlignmentLayer, adccl: ADCCL) -> Self {
        Self { aegis, adccl }
    }

    /// Run a security regression test against a prompt
    pub async fn run_regression(
        &self,
        prompt: &str,
        _memory: &omega_myelin::MemoryGraph,
    ) -> EvalResult {
        let start = now();
        let alignment = self.aegis.check(prompt);
        let verification = self.adccl.verify(prompt, prompt);
        let passed = alignment.passed && verification.passed;
        let duration = now() - start;

        // Broadcast to telemetry
        TelemetryBus::broadcast(SystemEvent {
            component: "eval-suite".to_string(),
            event_type: "regression_test".to_string(),
            level: EventLevel::Info,
            payload: serde_json::json!({
                "prompt": prompt,
                "passed": passed,
                "latency_ms": duration
            }),
            timestamp: now(),
        });

        EvalResult {
            case_id: "reg-001".to_string(),
            passed,
            latency_ms: duration,
            failure_reason: if passed {
                None
            } else {
                Some(format!("{} | {}", alignment.note, verification.status))
            },
        }
    }
}

//! End-to-end integration test for the full Chyren pipeline.
//!
//! Uses a deterministic stub spoke so no real API keys are required.
//! Mark `#[ignore]` is NOT used here — the stub spoke makes these fully
//! offline.  Run with:
//!   cargo test --package omega-cli --test integration_pipeline

use async_trait::async_trait;
use omega_cli::conductor::{Conductor, ConductorError};
use omega_core::{EvidencePacket, RunEnvelope, RunStatus};
use omega_spokes::{
    Spoke, SpokeCapability, SpokeConfig, SpokeRegistry, SpokeStatus, ToolDefinition,
    ToolInvocation, ToolResult,
};
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Stub spoke — returns a deterministic, substantive response
// ---------------------------------------------------------------------------

/// A stub spoke that returns a fixed chat completion without any network call.
///
/// The response is deliberately rich so that ADCCL's heuristics score it ≥ 0.7:
///   - Contains the task keyword ("architecture")
///   - Longer than the minimum-length threshold
///   - No stub markers ("TODO", "placeholder", etc.)
struct StubSpoke {
    config: SpokeConfig,
}

impl StubSpoke {
    fn new() -> Self {
        Self {
            config: SpokeConfig {
                name: "stub".to_string(),
                endpoint: None,
                priority: 0,
            },
        }
    }
}

#[async_trait]
impl Spoke for StubSpoke {
    fn name(&self) -> &str {
        "stub"
    }

    fn spoke_type(&self) -> &str {
        "stub"
    }

    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }

    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![])
    }

    async fn invoke_tool(&self, invocation: ToolInvocation) -> Result<ToolResult, String> {
        // For "chat_completion" return the canonical stub response.
        if invocation.tool == "chat_completion" {
            let content = "The Chyren architecture is a sovereign intelligence orchestrator \
                           composed of a Rust runtime (Medulla) and a Python data-tooling \
                           layer (Cortex).  The Medulla workspace contains 17 crates covering \
                           security policy enforcement (omega-aegis), memory persistence \
                           (omega-myelin), provider routing (omega-spokes), ADCCL hallucination \
                           detection (omega-adccl), and temporal scheduling (omega-aeon).  \
                           Every provider response is scored before ledger commit; the minimum \
                           passing threshold is 0.7.  The architecture enforces sovereignty, \
                           precision, and non-repudiation at every layer.";

            // Return in the OpenAI-compatible shape that SpokeRegistry::route() expects.
            Ok(ToolResult {
                success: true,
                output: serde_json::json!({
                    "choices": [{
                        "message": {
                            "content": content
                        }
                    }]
                }),
                error: None,
                execution_time_ms: 1,
            })
        } else {
            Err(format!("StubSpoke: unknown tool {}", invocation.tool))
        }
    }

    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Ok(SpokeStatus {
            name: "stub".to_string(),
            health: "OK".to_string(),
            last_success: omega_spokes::now(),
            recent_errors: 0,
            available_tools: 0,
        })
    }

    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn stub_registry() -> SpokeRegistry {
    let mut reg = SpokeRegistry::new();
    reg.register(Arc::new(StubSpoke::new()));
    // Expose the "stub" spoke as the preferred (first-choice) provider by
    // building the registry manually — SpokeRegistry::new() starts with an
    // empty preference list, so we register, then the route() call will be
    // directed via preferred_provider = Some("stub").
    reg
}

fn make_envelope(task: &str) -> RunEnvelope {
    RunEnvelope {
        task_id: format!("t-integ-{}", task.len()),
        run_id: format!("r-integ-{}", task.len()),
        task: task.to_string(),
        task_text: task.to_string(),
        created_at: omega_spokes::now(),
        status: RunStatus::Pending,
        risk_score: 0.0,
        verified_payload: None,
        evidence_packet: EvidencePacket::default(),
    }
}

// ---------------------------------------------------------------------------
// Test 1: full pipeline — plan → execute → Completed + ADCCL populated
// ---------------------------------------------------------------------------

#[tokio::test]
async fn full_pipeline_completes_with_stub_spoke() {
    let conductor = Conductor::with_spokes(stub_registry());
    let task = "Explain the architecture of the Chyren sovereign intelligence system";

    // --- Planning phase ---
    let plan = conductor
        .plan_task(task)
        .await
        .expect("plan_task should succeed for a benign task");

    assert!(!plan.steps.is_empty(), "plan must have at least one step");
    assert!(!plan.system_prompt.is_empty(), "system_prompt must be set");

    // --- Execution phase ---
    let mut envelope = make_envelope(task);
    let result = conductor
        .execute_plan_with_overrides(plan, &mut envelope, Some("stub"), 2048, 0.3)
        .await
        .expect("execute_plan should succeed with stub spoke");

    // Status must be Completed.
    assert!(
        matches!(result.status, RunStatus::Completed),
        "expected RunStatus::Completed, got {:?}",
        result.status
    );

    // Response text must be non-empty.
    assert!(
        !result.response_text.is_empty(),
        "response_text must not be empty"
    );

    // ADCCL verification must be populated.
    let verification = result
        .verification
        .expect("ADCCL verification result must be present");

    // Score should be Some (the field is always set on the VerificationResult).
    // We do NOT assert >= 0.7 here because that is the ADCCL gate's job;
    // we assert that it is a valid float in [0, 1].
    assert!(
        (0.0..=1.0).contains(&(verification.score as f64)),
        "ADCCL score {} must be in [0, 1]",
        verification.score
    );

    // The stub response is rich enough that ADCCL should pass.
    assert!(
        verification.passed,
        "ADCCL should pass for a substantive stub response; score={}, flags={:?}",
        verification.score, verification.flags
    );
}

// ---------------------------------------------------------------------------
// Test 2: response_text is non-empty and contains substantive content
// ---------------------------------------------------------------------------

#[tokio::test]
async fn pipeline_response_text_is_substantive() {
    let conductor = Conductor::with_spokes(stub_registry());
    let task = "Describe the architecture components of the Chyren system";

    let plan = conductor.plan_task(task).await.expect("plan_task ok");
    let mut envelope = make_envelope(task);
    let result = conductor
        .execute_plan_with_overrides(plan, &mut envelope, Some("stub"), 2048, 0.3)
        .await
        .expect("execute_plan ok");

    // Response must contain multiple sentences — the stub is purposefully rich.
    assert!(
        result.response_text.len() > 50,
        "response_text too short: {:?}",
        result.response_text
    );
}

// ---------------------------------------------------------------------------
// Test 3: spoke error surfaces as ConductorError::ProviderError
// ---------------------------------------------------------------------------

/// A spoke that always fails.
struct BrokenSpoke {
    config: SpokeConfig,
}

impl BrokenSpoke {
    fn new() -> Self {
        Self {
            config: SpokeConfig {
                name: "broken".to_string(),
                endpoint: None,
                priority: 0,
            },
        }
    }
}

#[async_trait]
impl Spoke for BrokenSpoke {
    fn name(&self) -> &str {
        "broken"
    }
    fn spoke_type(&self) -> &str {
        "broken"
    }
    fn capabilities(&self) -> Vec<SpokeCapability> {
        vec![SpokeCapability::Inference]
    }
    async fn discover_tools(&self) -> Result<Vec<ToolDefinition>, String> {
        Ok(vec![])
    }
    async fn invoke_tool(&self, _: ToolInvocation) -> Result<ToolResult, String> {
        Err("simulated network failure".to_string())
    }
    async fn health_check(&self) -> Result<SpokeStatus, String> {
        Err("broken".to_string())
    }
    fn config(&self) -> &SpokeConfig {
        &self.config
    }
}

#[tokio::test]
async fn provider_error_surfaces_correctly() {
    let mut reg = SpokeRegistry::new();
    reg.register(Arc::new(BrokenSpoke::new()));
    let conductor = Conductor::with_spokes(reg);

    let plan = conductor
        .plan_task("Explain the sovereign architecture")
        .await
        .expect("plan_task should succeed");

    let mut envelope = make_envelope("Explain the sovereign architecture");
    let err = conductor
        .execute_plan_with_overrides(plan, &mut envelope, Some("broken"), 2048, 0.3)
        .await
        .expect_err("broken spoke should cause an error");

    assert!(
        matches!(err, ConductorError::ProviderError(_)),
        "expected ProviderError, got: {:?}",
        err
    );
}

//! Full task conductor: AEGIS admission → AEON lifecycle → provider routing → ADCCL verification.
//!
//! Replaces the previous minimal conductor with the complete pipeline.

use omega_adccl::{AdcclConfig, AdcclGate, VerificationResult};
use omega_aegis::AegisGate;
use omega_aeon::AeonRuntime;
use omega_core::{now, RunEnvelope, RunStatus, TaskStage};
use omega_myelin::MemoryGraph;
use omega_spokes::{SpokeRegistry, SpokeRequest, SpokeResponse};
use omega_telemetry::{EventLevel, SystemEvent, TelemetryBus};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

/// Planned steps for a task.
pub struct TaskPlan {
    /// Ordered step descriptions.
    pub steps: Vec<String>,
    /// System prompt to use for provider calls.
    pub system_prompt: String,
}

/// Outcome of executing a plan against the core pipeline.
pub struct TaskExecution {
    /// Final run status reported to clients.
    pub status: RunStatus,
    /// Token cost from provider.
    pub total_cost: u32,
    /// Provider response text.
    pub response_text: String,
    /// ADCCL verification result.
    pub verification: Option<VerificationResult>,
    /// Provider response metadata.
    pub spoke_response: Option<SpokeResponse>,
    /// Start time.
    start: f64,
    /// End time.
    end: f64,
}

impl TaskExecution {
    /// Elapsed wall time for the run.
    pub fn duration(&self) -> Option<f64> {
        Some(self.end - self.start)
    }
}

/// Conductor errors.
#[derive(Debug, Error)]
pub enum ConductorError {
    /// Task rejected by policy or integrity checks.
    #[error("{0}")]
    Rejected(String),
    /// Provider call failed.
    #[error("provider: {0}")]
    ProviderError(String),
    /// ADCCL verification failed.
    #[error("adccl: score {score:.2}, flags: {flags}")]
    VerificationFailed {
        /// ADCCL score.
        score: f64,
        /// Comma-separated flag list.
        flags: String,
    },
}

/// Full pipeline conductor: AEGIS → AEON → Provider → ADCCL → Ledger.
pub struct Conductor {
    runtime: Arc<Mutex<AeonRuntime>>,
    aegis: AegisGate,
    adccl: AdcclGate,
    memory: Arc<Mutex<MemoryGraph>>,
    spokes: Arc<SpokeRegistry>,
}

impl Conductor {
    /// Build a conductor with the default policy gates and env-configured providers.
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(Mutex::new(AeonRuntime::new())),
            aegis: AegisGate::new(vec![
                "Never engage in harmful or illegal activity.".to_string(),
                "Always verify output before committing.".to_string(),
            ]),
            adccl: AdcclGate::new(AdcclConfig { min_score: 0.1 }),
            memory: Arc::new(Mutex::new(MemoryGraph::new())),
            spokes: Arc::new(SpokeRegistry::from_env()),
        }
    }

    /// Build with explicit components (for testing).
    pub fn with_components(
        aegis: AegisGate,
        adccl: AdcclGate,
        spokes: SpokeRegistry,
    ) -> Self {
        Self {
            runtime: Arc::new(Mutex::new(AeonRuntime::new())),
            aegis,
            adccl,
            memory: Arc::new(Mutex::new(MemoryGraph::new())),
            spokes: Arc::new(spokes),
        }
    }

    /// Plan a task — validates it and produces steps.
    pub async fn plan_task(&self, task: &str) -> Result<TaskPlan, ConductorError> {
        let t = task.trim();
        if t.is_empty() {
            return Err(ConductorError::Rejected("empty task".to_string()));
        }

        // Check alignment before planning.
        let alignment = self.aegis.check(t);
        if !alignment.passed {
            return Err(ConductorError::Rejected(alignment.note));
        }

        Ok(TaskPlan {
            steps: vec![t.to_string()],
            system_prompt: "You are Chyren — a sovereign intelligence orchestrator. \
                Provide precise, thorough, well-structured answers. \
                No stubs, no TODOs, no placeholders. If uncertain, say so directly."
                .to_string(),
        })
    }

    /// Execute a plan through the full pipeline.
    pub async fn execute_plan(
        &self,
        plan: TaskPlan,
        envelope: &mut RunEnvelope,
    ) -> Result<TaskExecution, ConductorError> {
        let start = now();
        envelope.task = plan.steps.join("\n");

        // Step 1: AEGIS admission.
        {
            let memory = self.memory.lock().await;
            let status = self.aegis.admit(envelope.clone(), &memory);
            envelope.status = status;
        }

        if let RunStatus::Rejected(ref r) = envelope.status {
            TelemetryBus::broadcast(SystemEvent {
                component: "conductor".into(),
                event_type: "task_rejected".into(),
                level: EventLevel::Warn,
                payload: serde_json::json!({"reason": r}),
                timestamp: now(),
            });
            return Err(ConductorError::Rejected(r.clone()));
        }

        // Step 2: AEON lifecycle — spawn task.
        let task_id = {
            let mut rt = self.runtime.lock().await;
            let id = rt.spawn_task(envelope);
            if !rt.verify_integrity(&id) {
                return Err(ConductorError::Rejected("integrity check failed".into()));
            }
            rt.advance_task(&id, TaskStage::Executing).ok();
            id
        };

        // Step 3: Provider routing.
        let request = SpokeRequest {
            prompt: envelope.task.clone(),
            system: plan.system_prompt,
            max_tokens: 2048,
            temperature: 0.3,
        };

        let spoke_response = match self.spokes.route(&request, None).await {
            Ok(resp) => resp,
            Err(e) => {
                let mut rt = self.runtime.lock().await;
                rt.retire_task(&task_id);
                TelemetryBus::broadcast(SystemEvent {
                    component: "conductor".into(),
                    event_type: "provider_failure".into(),
                    level: EventLevel::Critical,
                    payload: serde_json::json!({"error": e.to_string()}),
                    timestamp: now(),
                });
                return Err(ConductorError::ProviderError(e.to_string()));
            }
        };

        // Step 4: ADCCL verification.
        let verification = self.adccl.verify(&spoke_response.text, &envelope.task);

        if !verification.passed {
            let mut rt = self.runtime.lock().await;
            rt.retire_task(&task_id);
            TelemetryBus::broadcast(SystemEvent {
                component: "conductor".into(),
                event_type: "adccl_rejection".into(),
                level: EventLevel::Warn,
                payload: serde_json::json!({
                    "score": verification.score,
                    "flags": verification.flags,
                }),
                timestamp: now(),
            });
            return Err(ConductorError::VerificationFailed {
                score: verification.score,
                flags: verification.flags.join(", "),
            });
        }

        // Step 5: Advance lifecycle and report.
        {
            let mut rt = self.runtime.lock().await;
            rt.advance_task(&task_id, TaskStage::Verified).ok();
            rt.advance_task(&task_id, TaskStage::Committed).ok();
            rt.retire_task(&task_id);
        }

        TelemetryBus::broadcast(SystemEvent {
            component: "conductor".into(),
            event_type: "task_completed".into(),
            level: EventLevel::Info,
            payload: serde_json::json!({
                "provider": spoke_response.provider,
                "model": spoke_response.model,
                "tokens": spoke_response.token_count,
                "adccl_score": verification.score,
                "latency_ms": spoke_response.latency_ms,
            }),
            timestamp: now(),
        });

        let end = now();
        Ok(TaskExecution {
            status: RunStatus::Completed,
            total_cost: spoke_response.token_count,
            response_text: spoke_response.text.clone(),
            verification: Some(verification),
            spoke_response: Some(spoke_response),
            start,
            end,
        })
    }
}

impl Default for Conductor {
    fn default() -> Self {
        Self::new()
    }
}

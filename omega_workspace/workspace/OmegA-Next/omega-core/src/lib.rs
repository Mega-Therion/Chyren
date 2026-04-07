//! omega-core: Shared types, contracts, and primitives for OmegA-Next.
//!
//! This crate is intentionally dependency-light and acts as the single source of truth
//! for cross-crate structs (envelopes, contracts, plans, memory graph types).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// High-level run status as it travels through the pipeline.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RunStatus {
    /// Run is admitted by policy gates and may proceed.
    Admitted,
    /// Run is rejected by a gate (policy, ADCCL, etc.).
    Rejected(String),
    /// Run failed due to an unrecoverable error.
    Failed(String),
    /// Run is currently executing.
    Processing,
    /// Run is queued/awaiting processing.
    Pending,
    /// Run completed successfully.
    Completed,
}

/// Lifecycle stage for task planning/execution.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TaskStage {
    /// Task has been received.
    Received,
    /// Task is being planned.
    Planning,
    /// Task plan has been created.
    Planned,
    /// Task is being executed.
    Executing,
    /// Task output has been verified.
    Verified,
    /// Task result has been committed.
    Committed,
}

/// Minimal user-context model (placeholder until identity kernel is integrated).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext {
    /// Display name for the operator.
    pub name: String,
    /// Family member labels (for personalization).
    pub family_members: Vec<String>,
    /// Short bio/context.
    pub bio: String,
}

/// MatrixProgram: a signed payload that can be ingested/executed by the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixProgram {
    /// Program domain.
    pub domain: String,
    /// Version string.
    pub version: String,
    /// Integrity hash over payload.
    pub integrity_hash: String,
    /// Raw payload bytes.
    pub payload: Vec<u8>,
}

/// Node within the memory graph.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryNode {
    /// Stable node identifier.
    pub node_id: String,
    /// Human-readable content.
    pub content: String,
    /// Number of times retrieved.
    pub retrieval_count: u64,
    /// Decay score (0..1).
    pub decay_score: f64,
}

/// Edge within the memory graph.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryEdge {
    /// From label (optional human readable).
    pub from: String,
    /// To label (optional human readable).
    pub to: String,
    /// From node id.
    pub from_id: String,
    /// To node id.
    pub to_id: String,
}

/// A retrieval episode containing nodes returned by a recall expansion.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetrievalEpisode {
    /// Episode id.
    pub episode_id: String,
    /// Episode content.
    pub content: String,
    /// Result node ids.
    pub result_nodes: Vec<String>,
}

/// Evidence packet attached to a run.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidencePacket {
    /// Evidence source identifier.
    pub source: String,
    /// Evidence payload (serialized).
    pub payload: String,
}

impl EvidencePacket {
    /// Create an empty evidence packet.
    pub fn new() -> Self {
        Self {
            source: "none".to_string(),
            payload: "".to_string(),
        }
    }
}

impl Default for EvidencePacket {
    fn default() -> Self {
        Self::new()
    }
}

/// Verification report produced by ADCCL.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationReport {
    /// Report id.
    pub report_id: String,
    /// Whether verification passed.
    pub passed: bool,
    /// Flags raised during verification.
    pub flags: Vec<String>,
    /// Score in 0..1.
    pub score: f64,
    /// Evidence supporting flags.
    pub evidence: Vec<EvidencePacket>,
    /// Suggested repairs/mitigations.
    pub repairs: Vec<String>,
}

/// Budget for claims/assertions in a run (governance control).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimBudget {
    /// Maximum allowed claims.
    pub max_claims: u32,
    /// Claims already used.
    pub claims_used: u32,
    /// Allowed claim types.
    pub allowed_claim_types: Vec<String>,
}

/// One step in a plan skeleton.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep {
    /// Primary action instruction.
    pub action: String,
    /// Verification condition.
    pub verification: String,
    /// Fallback instruction if verification fails.
    pub fallback: String,
}

/// High-level plan skeleton.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanSkeleton {
    /// Ordered plan steps.
    pub steps: Vec<PlanStep>,
    /// Rough token estimate.
    pub estimated_tokens: usize,
    /// Mitigations to apply during execution.
    pub mitigations: Vec<String>,
}

/// Goal contract for an envelope.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalContract {
    /// Objective statement.
    pub objective: String,
    /// Success criteria list.
    pub success_criteria: Vec<String>,
    /// Constraints that must be respected.
    pub constraints: Vec<String>,
    /// Claim budget.
    pub claim_budget: ClaimBudget,
}

/// Task State Object: persistent, stateful record of a task lifecycle.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskStateObject {
    /// Task id.
    pub task_id: String,
    /// Run id.
    pub run_id: String,
    /// Original task text.
    pub task_text: String,
    /// Current stage.
    pub stage: TaskStage,
    /// Goal contract.
    pub goal_contract: Option<GoalContract>,
    /// Plan skeleton.
    pub plan_skeleton: Option<PlanSkeleton>,
    /// State context map.
    pub state_context: HashMap<String, String>,
    /// Self-tags for routing/telemetry.
    pub self_tags: Vec<String>,
    /// Created timestamp (seconds).
    pub created_at: f64,
    /// Modified timestamp (seconds).
    pub modified_at: f64,
}

/// Current time in seconds since epoch.
pub fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// Generate a stable id with a prefix.
pub fn gen_id(prefix: &str) -> String {
    format!("{}-{}", prefix, uuid::Uuid::new_v4())
}

/// Root integrity marker.
pub const YETTRAGRAMMATON: &str = "R.W.Ϝ.Y.";

/// RunEnvelope: portable run context passed between subsystems.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunEnvelope {
    /// Task id.
    pub task_id: String,
    /// Run id.
    pub run_id: String,
    /// Task text (canonical).
    pub task: String,
    /// Task text (raw).
    pub task_text: String,
    /// Created timestamp.
    pub created_at: f64,
    /// Current status.
    pub status: RunStatus,
    /// Risk score (0..1).
    pub risk_score: f64,
    /// Verified payload (if any).
    pub verified_payload: Option<String>,
    /// Evidence packet.
    pub evidence_packet: EvidencePacket,
}

/// Chronicle entry: immutable record of an episode.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chronicle {
    /// Episode id.
    pub episode_id: String,
    /// Timestamp.
    pub timestamp: f64,
    /// Causal link ids.
    pub causal_links: Vec<String>,
    /// Emotional vector.
    pub emotional_vector: Vec<f32>,
    /// Episode content.
    pub content: String,
}

/// Temporal anchor for retrieval expansion.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporalAnchor {
    /// Episode id.
    pub episode_id: String,
    /// Timestamp.
    pub timestamp: f64,
}

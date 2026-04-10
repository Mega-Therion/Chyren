//! omega-core: Shared types, contracts, and cryptographic primitives
//!
//! This crate defines the unified contracts that all layers (AEGIS, AEON, ADCCL, MYELIN)
//! depend on. Types defined here are the "language" spoken across boundaries.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

// ── Cryptographic Primitives ─────────────────────────────────────────────

/// The Yettragrammaton: R.W.Ϝ.Y. — sovereign identity seed for all ledger signing
pub const YETTRAGRAMMATON: &str = "R.W.Ϝ.Y.";

/// VM Seed Bytes: architect's hex identity (0x52 = 'R', 0x59 = 'Y')
pub const VM_SEED_BYTES: &[u8] = &[0x52, 0x59];

/// VM Seed as integer: 21081 (deterministic PYTHONHASHSEED for sandbox)
pub const VM_SEED_INT: u32 = 21081;

// ── Primary Contracts ─────────────────────────────────────────────────────

/// RunEnvelope: The primary runtime contract passed through all layers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunEnvelope {
    /// Unique run identifier
    pub run_id: String,
    /// The original task or prompt
    pub task: String,
    /// Current status in pipeline
    pub status: RunStatus,
    /// Risk score (0.0-1.0)
    pub risk_score: f64,
    /// Verified payload if passed gates
    pub verified_payload: Option<VerifiedPayload>,
    /// Evidence collected across layers
    pub evidence_packet: EvidencePacket,
    /// Timestamp when envelope was created
    pub created_at: f64,
}

/// Enum for run status throughout the pipeline
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStatus {
    /// Created, awaiting AEGIS gate
    Pending,
    /// AEGIS policy gate passed
    Admitted,
    /// ADCCL verification passed
    Verified,
    /// MYELIN memory retrieval complete
    Contextualized,
    /// Ready for provider routing
    Routed,
    /// Provider returned response
    Complete,
    /// Failed at some stage
    Rejected(String),
    /// Locked out (threat detected)
    Locked,
}

/// VerifiedPayload: Task that has passed alignment and threat gates
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerifiedPayload {
    /// The task text itself
    pub task_text: String,
    /// Hash of task for integrity
    pub payload_hash: String,
    /// Which gates approved this
    pub approved_gates: Vec<String>,
    /// Threat level assessed
    pub threat_level: ThreatLevel,
}

/// Threat level classification
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatLevel {
    /// No threat detected
    None = 0,
    /// Low-severity threat pattern
    Low = 1,
    /// Medium-severity threat
    Medium = 2,
    /// High-severity threat
    High = 3,
    /// Critical threat — lockout required
    Critical = 4,
}

impl fmt::Display for ThreatLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreatLevel::None => write!(f, "none"),
            ThreatLevel::Low => write!(f, "low"),
            ThreatLevel::Medium => write!(f, "medium"),
            ThreatLevel::High => write!(f, "high"),
            ThreatLevel::Critical => write!(f, "critical"),
        }
    }
}

// ── Evidence Packet: Audit trail across all gates ────────────────────────

/// EvidencePacket: Structured evidence collected at each layer
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvidencePacket {
    /// Evidence from AEGIS risk gate
    pub aegis_evidence: Vec<EvidenceRecord>,
    /// Evidence from ADCCL verification
    pub adccl_evidence: Vec<EvidenceRecord>,
    /// Evidence from MYELIN retrieval
    pub myelin_evidence: Vec<EvidenceRecord>,
    /// Evidence from AEON task state reasoning
    pub aeon_evidence: Vec<EvidenceRecord>,
}

impl EvidencePacket {
    /// Create a new empty packet
    pub fn new() -> Self {
        EvidencePacket::default()
    }

    /// Add evidence from a layer
    pub fn add_evidence(&mut self, layer: &str, record: EvidenceRecord) {
        match layer {
            "aegis" => self.aegis_evidence.push(record),
            "adccl" => self.adccl_evidence.push(record),
            "myelin" => self.myelin_evidence.push(record),
            "aeon" => self.aeon_evidence.push(record),
            _ => {}
        }
    }
}

/// A single piece of evidence (claim, check, or observation)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceRecord {
    /// What is being claimed (e.g. "task_alignment_passed")
    pub claim: String,
    /// Status: "supported" | "computed" | "inferred" | "unknown"
    pub claim_class: String,
    /// Confidence or score
    pub confidence: f64,
    /// Human-readable explanation
    pub explanation: String,
    /// Timestamp
    pub timestamp: f64,
}

// ── Task State Object (AEON) ─────────────────────────────────────────────

/// TaskStateObject: Stateful representation of a task's lifecycle
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskStateObject {
    /// Unique task ID
    pub task_id: String,
    /// The parent run ID
    pub run_id: String,
    /// Current stage in lifecycle
    pub stage: TaskStage,
    /// Task description
    pub task_text: String,
    /// What we're trying to achieve
    pub goal_contract: Option<GoalContract>,
    /// How we plan to achieve it
    pub plan_skeleton: Option<PlanSkeleton>,
    /// State variables tracked by world model
    pub state_context: HashMap<String, serde_json::Value>,
    /// Tags applied during reasoning
    pub self_tags: Vec<String>,
    /// Created timestamp
    pub created_at: f64,
    /// Last modified timestamp
    pub modified_at: f64,
}

/// Task lifecycle stage
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStage {
    /// Just received
    Received,
    /// Parsed and understood
    Interpreted,
    /// Goal and plan drafted
    Planned,
    /// Executing plan
    Executing,
    /// Verifying result
    Verifying,
    /// Complete
    Complete,
    /// Failed
    Failed(String),
}

/// GoalContract: Formal specification of what success means
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoalContract {
    /// What we're trying to achieve
    pub objective: String,
    /// How we'll know we succeeded
    pub success_criteria: Vec<String>,
    /// Constraints that must hold
    pub constraints: Vec<String>,
    /// Budget for claims/evidence
    pub claim_budget: ClaimBudget,
}

/// ClaimBudget: Allocation of how many claims we can make
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClaimBudget {
    /// Total claims we can make
    pub max_claims: usize,
    /// Claims made so far
    pub claims_used: usize,
    /// What types of claims are allowed
    pub allowed_claim_types: Vec<String>,
}

/// PlanSkeleton: High-level decomposition of how to achieve goal
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanSkeleton {
    /// Steps in the plan
    pub steps: Vec<PlanStep>,
    /// Estimated tokens needed
    pub estimated_tokens: usize,
    /// Risk mitigations
    pub mitigations: Vec<String>,
}

/// One step in the plan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanStep {
    /// What to do
    pub action: String,
    /// How to verify it worked
    pub verification: String,
    /// What to do if it fails
    pub fallback: String,
}

// ── Verification Report (ADCCL) ──────────────────────────────────────────

/// VerificationReport: Output from ADCCL gate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationReport {
    /// Unique report ID
    pub report_id: String,
    /// Did verification pass?
    pub passed: bool,
    /// Score (0.0-1.0)
    pub score: f64,
    /// Flags raised (e.g. "DRIFT_DETECTED", "INCOHERENCE")
    pub flags: Vec<String>,
    /// Evidence supporting the decision
    pub evidence: Vec<EvidenceRecord>,
    /// Recommended repairs if failed
    pub repairs: Vec<Repair>,
}

/// Repair: A fix to apply to a response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repair {
    /// What's wrong
    pub issue: String,
    /// How to fix it
    pub fix: String,
    /// Confidence in the fix
    pub confidence: f64,
}

// ── Memory Graph (MYELIN) ────────────────────────────────────────────────

/// MemoryStratum: One of the four memory layers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryStratum {
    /// Ground truth, verified facts
    Canonical,
    /// Active working memory
    Operational,
    /// Past episodes and events
    Episodic,
    /// Hypotheticals and plans
    Speculative,
}

/// MemoryNode: A single node in the graph
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryNode {
    /// Unique node ID
    pub node_id: String,
    /// Content stored here
    pub content: String,
    /// Which stratum
    pub stratum: MemoryStratum,
    /// Creation timestamp
    pub created_at: f64,
    /// Last access
    pub last_accessed: f64,
    /// Retrieval count
    pub retrieval_count: u32,
    /// Decay score (lower = older, less relevant)
    pub decay_score: f64,
}

/// MemoryEdge: Connection between nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryEdge {
    /// From node
    pub from_id: String,
    /// To node
    pub to_id: String,
    /// Type of relationship
    pub relation: String,
    /// Strength (0.0-1.0)
    pub strength: f64,
    /// Bundle of edges with same relation share retrieval
    pub bundle_id: Option<String>,
}

/// RetrievalEpisode: One retrieval attempt
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetrievalEpisode {
    /// Query text
    pub query: String,
    /// Nodes retrieved
    pub result_nodes: Vec<String>,
    /// Nodes that were hydrated from neighbourhood
    pub hydrated_neighbourhood: Vec<String>,
    /// Quality score of retrieval
    pub quality_score: f64,
    /// Timestamp
    pub timestamp: f64,
}

// ── Provider Abstraction ──────────────────────────────────────────────────

/// ProviderRequest: Standardized request to any LLM provider
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderRequest {
    /// Run ID for tracking
    pub run_id: String,
    /// The task/prompt
    pub prompt: String,
    /// System instruction
    pub system: Option<String>,
    /// Max tokens
    pub max_tokens: usize,
    /// Temperature (0.0-1.0)
    pub temperature: f64,
    /// Preferred model name
    pub model: Option<String>,
}

/// ProviderResponse: Standardized response from any provider
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderResponse {
    /// Response text
    pub text: String,
    /// Which provider provided this
    pub provider: String,
    /// Model used
    pub model: String,
    /// Tokens in response
    pub tokens: usize,
    /// Latency in milliseconds
    pub latency_ms: f64,
    /// Status: "success" | "error" | "unavailable"
    pub status: String,
    /// Error message if failed
    pub error: Option<String>,
}

// ── Experience Record (continuity) ───────────────────────────────────────

/// ExperiencePacket: Bundle of learning from one or more runs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperiencePacket {
    /// Unique ID
    pub packet_id: String,
    /// Runs included
    pub run_ids: Vec<String>,
    /// What we learned
    pub lessons: Vec<String>,
    /// Updated policies
    pub policy_updates: HashMap<String, serde_json::Value>,
    /// Memory to persist
    pub memory_writes: Vec<MemoryNode>,
}

/// SelfTag: Label applied to session state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelfTag {
    /// Tag name
    pub tag: String,
    /// Confidence
    pub confidence: f64,
    /// Evidence supporting the tag
    pub evidence: Vec<String>,
}

// ── Utilities ────────────────────────────────────────────────────────────

/// Get current Unix timestamp
pub fn now() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

/// Generate a unique ID
pub fn gen_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    format!("{}-{:x}", prefix, nanos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_level_ordering() {
        assert!(ThreatLevel::None < ThreatLevel::Low);
        assert!(ThreatLevel::Critical > ThreatLevel::High);
    }

    #[test]
    fn test_evidence_packet() {
        let mut packet = EvidencePacket::new();
        packet.add_evidence(
            "aegis",
            EvidenceRecord {
                claim: "policy_gate_passed".to_string(),
                claim_class: "supported".to_string(),
                confidence: 0.95,
                explanation: "Task matches allowed patterns".to_string(),
                timestamp: now(),
            },
        );
        assert_eq!(packet.aegis_evidence.len(), 1);
    }

    #[test]
    fn test_id_generation() {
        let id1 = gen_id("run");
        let id2 = gen_id("run");
        assert!(id1.starts_with("run-"));
        assert!(id2.starts_with("run-"));
        assert_ne!(id1, id2); // Should be unique
    }
}

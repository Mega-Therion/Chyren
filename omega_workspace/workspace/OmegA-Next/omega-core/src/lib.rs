//! omega-core: Shared types, contracts, and cryptographic primitives
#![warn(missing_docs)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RunStatus { Admitted, Rejected, Failed, Processing, Pending }

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TaskStage { Received, Planned, Executing, Verified, Committed, Planning }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserContext { pub name: String, pub family_members: Vec<String>, pub bio: String }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixProgram { pub domain: String, pub version: String, pub integrity_hash: String, pub payload: Vec<u8> }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryNode { pub node_id: String, pub content: String, pub retrieval_count: u64, pub decay_score: f64 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryEdge { pub from: String, pub to: String, pub from_id: String, pub to_id: String }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RetrievalEpisode { pub episode_id: String, pub content: String, pub result_nodes: Vec<String> }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvidencePacket { pub source: String, pub payload: String }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationReport { pub report_id: String, pub flags: Vec<String>, pub score: f64, pub evidence: Vec<EvidencePacket>, pub repairs: Vec<String> }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimBudget { pub total: f64, pub remaining: f64 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanSkeleton { pub steps: Vec<String> }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanStep { pub action: String }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalContract { pub objective: String, pub success_criteria: String, pub constraints: Vec<String>, pub claim_budget: ClaimBudget }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskStateObject {
    pub task_id: String, pub run_id: String, pub task_text: String, pub stage: TaskStage,
    pub goal_contract: Option<GoalContract>, pub plan_skeleton: Option<PlanSkeleton>, pub state_context: HashMap<String, String>,
    pub self_tags: Vec<String>, pub created_at: f64, pub modified_at: f64,
}

pub fn now() -> f64 { std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64() }
pub fn gen_id(prefix: &str) -> String { format!("{}-{}", prefix, uuid::Uuid::new_v4()) }

pub const YETTRAGRAMMATON: &str = "R.W.Ϝ.Y.";
pub struct RunEnvelope { pub task_id: String, pub run_id: String, pub task: String, pub task_text: String, pub created_at: f64, pub status: RunStatus, pub risk_score: f64, pub verified_payload: Option<String>, pub evidence_packet: EvidencePacket }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chronicle {
    pub episode_id: String,
    pub timestamp: f64,
    pub causal_links: Vec<String>,
    pub emotional_vector: Vec<f32>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemporalAnchor {
    pub episode_id: String,
    pub timestamp: f64,
}

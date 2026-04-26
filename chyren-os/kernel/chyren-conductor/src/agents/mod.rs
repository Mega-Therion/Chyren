//! Persistent agents — long-running workers in the Sovereign Orchestration Mesh.
//!
//! Each agent is a specialized capability node: it registers its skills,
//! executes tasks, and returns scored results back through the EventBus.

use async_trait::async_trait;
use chyren_core::mesh::AgentCapability;
use chyren_core::{AgentResult, AgentTask};

/// A persistent agent that can be registered in the AgentRegistry and
/// dispatched tasks via the EventBus.
#[async_trait]
pub trait PersistentAgent: Send + Sync {
    /// Returns the unique name of this agent.
    fn name(&self) -> &str;
    /// Returns the capabilities this agent offers.
    fn capabilities(&self) -> Vec<AgentCapability>;
    /// Returns the system prompt used when this agent reasons.
    fn system_prompt(&self) -> &str;
    /// Execute a task and return a scored result.
    async fn execute(&self, task: AgentTask) -> AgentResult;
}

pub mod ingestor;
pub mod math_spoke;
/// Monte-Carlo Tree Search solver agent.
pub mod mcts_solver;
pub mod millennium;
pub mod millennium_solver;
pub mod worker;

pub use ingestor::IngestorAgent;
pub use math_spoke::MathSpoke;
pub use mcts_solver::MctsSolverAgent;
pub use millennium::{MillenniumProblem, SearchAndExtendAgent};
pub use millennium_solver::MillenniumSolverAgent;
pub use worker::MeshWorker;

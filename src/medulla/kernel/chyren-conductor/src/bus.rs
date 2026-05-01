//! bus.rs — The Asynchronous Event Bus.
//! Connects persistent agents back to the Conductor result pipeline.

use chyren_core::AgentResult;
use tokio::sync::mpsc;

/// The asynchronous event bus that bridges agent results back to the Conductor.
///
/// Agents publish `AgentResult` messages to `result_tx`.  The Conductor
/// (or any supervisor) reads from `result_rx` and incorporates the results
/// into the active task pipeline.
pub struct EventBus {
    /// The sending half — cloned and given to each agent worker.
    pub result_tx: mpsc::Sender<AgentResult>,
    /// The receiving half — held by the Conductor supervisor loop.
    pub result_rx: mpsc::Receiver<AgentResult>,
}

impl EventBus {
    /// Create a new `EventBus` with an internal channel capacity of 100.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            result_tx: tx,
            result_rx: rx,
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

//! bus.rs — The Asynchronous Event Bus.
//! Connects persistent agents back to the Conductor result pipeline.

use omega_core::{AgentResult};
use tokio::sync::mpsc;
use tracing::info;

pub struct EventBus {
    pub result_tx: mpsc::Sender<AgentResult>,
    pub result_rx: mpsc::Receiver<AgentResult>,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self { result_tx: tx, result_rx: rx }
    }
}

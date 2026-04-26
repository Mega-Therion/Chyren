use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

pub type EventSender = mpsc::UnboundedSender<Event>;
pub type EventReceiver = mpsc::UnboundedReceiver<Event>;

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Resize(u16, u16),
    SseChunk(String),
    SseComplete(ChatResponse),
    TelemetryWs(SystemEvent),
    ApiError(String),
    Connected,
    Disconnected,
    // System orchestration events
    ProcStarted { id: String, label: String },
    ProcLine { id: String, line: String, is_err: bool },
    ProcExited { id: String, code: Option<i32> },
    StatusRefresh(StatusSnapshot),
    MeshRefresh(Vec<MeshAgent>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub run_id: String,
    pub status: String,
    pub response_text: String,
    pub adccl_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub component: String,
    pub event_type: String,
    pub level: String,
    pub payload: serde_json::Value,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Default)]
pub struct StatusSnapshot {
    pub api_reachable: bool,
    pub provider: String,
    pub adccl_score: f64,
    pub active_runs: usize,
    pub total_runs: usize,
    pub dream_episodes: usize,
    pub latency_ms: f64,
    pub chi: f64,
    pub omega: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAgent {
    pub id: String,
    pub status: String,
    pub last_active_secs: f64,
    pub capabilities: Vec<String>,
}

pub struct EventBus {
    pub tx: EventSender,
    pub rx: EventReceiver,
}

impl EventBus {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }

    pub fn sender(&self) -> EventSender {
        self.tx.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

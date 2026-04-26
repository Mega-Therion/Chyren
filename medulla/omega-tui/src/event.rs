use crossterm::event::KeyEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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

//! Unified telemetry and event emission
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub timestamp: f64,
    pub request_id: String,
    pub task_id: String,
    pub layer: String,
    pub component: String,
    pub event_type: String,
    pub status: String,
    pub latency_ms: f64,
    pub provider: String,
    pub risk_score: f64,
    pub bridge_outcome: Option<String>,
    pub verification_score: f64,
    pub retrieval_success: bool,
    pub contradiction_count: usize,
    pub memory_write_class: String,
    pub continuity_action: String,
    pub artifact_refs: Vec<String>,
    pub trace_refs: Vec<String>,
}

pub trait EventSink: Send + Sync {
    fn emit(&self, event: Event);
}

pub struct StdoutSink;

impl EventSink for StdoutSink {
    fn emit(&self, event: Event) {
        if let Ok(json) = serde_json::to_string(&event) {
            println!("{}", json);
        }
    }
}

#[macro_export]
macro_rules! emit_event {
    ($sink:expr, $event:expr) => {
        $sink.emit($event)
    };
}

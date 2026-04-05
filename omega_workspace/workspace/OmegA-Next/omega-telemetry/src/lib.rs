//! omega-telemetry: Structured event bus for cross-layer visibility.
//! Every state transition, gate decision, and metacog insight is logged here.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Event severity levels
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EventLevel {
    /// Informational event
    Info,
    /// System warning
    Warn,
    /// Critical security or integrity event
    Critical,
}

/// A structured telemetry event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemEvent {
    /// Component generating the event
    pub component: String,
    /// Event type
    pub event_type: String,
    /// Severity
    pub level: EventLevel,
    /// Metadata
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: f64,
}

/// The TelemetryBus: Broadcaster for system signals.
pub struct TelemetryBus;

impl TelemetryBus {
    /// Broadcast an event to the system
    pub fn broadcast(event: SystemEvent) {
        // In a production system, this would pipe to a ring buffer or external sink.
        // For our Sovereign Hub, we log to stdout for real-time observability.
        println!(
            "[{:.3}] [{:?}] [{}] {}: {}",
            event.timestamp, event.level, event.component, event.event_type, event.payload
        );
    }
}

//! omega-telemetry: Structured event bus for cross-layer visibility.
//! Every state transition, gate decision, and metacog insight is logged here.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

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
        // Log to stdout
        println!(
            "[{:.3}] [{:?}] [{}] {}: {}",
            event.timestamp, event.level, event.component, event.event_type, event.payload
        );

        // Log to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("telemetry.log")
        {
            if let Ok(serialized) = serde_json::to_string(&event) {
                let _ = writeln!(file, "{}", serialized);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = SystemEvent {
            component: "test".into(),
            event_type: "unit_test".into(),
            level: EventLevel::Info,
            payload: serde_json::json!({"key": "value"}),
            timestamp: 1234567890.0,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("unit_test"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_event_deserialization() {
        let json = r#"{"component":"test","event_type":"round_trip","level":"Warn","payload":{"x":42},"timestamp":0.0}"#;
        let event: SystemEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.component, "test");
        assert_eq!(event.event_type, "round_trip");
        assert!(matches!(event.level, EventLevel::Warn));
    }

    #[test]
    fn test_broadcast_does_not_panic() {
        // Smoke test — just ensure broadcast doesn't crash.
        TelemetryBus::broadcast(SystemEvent {
            component: "test".into(),
            event_type: "smoke".into(),
            level: EventLevel::Critical,
            payload: serde_json::json!(null),
            timestamp: 0.0,
        });
    }
}


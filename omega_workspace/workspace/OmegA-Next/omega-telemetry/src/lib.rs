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

pub trait TelemetrySink: Send + Sync {
    fn record(&self, event: &SystemEvent);
}

pub struct StdoutSink;
impl TelemetrySink for StdoutSink {
    fn record(&self, event: &SystemEvent) {
        println!(
            "[{:.3}] [{:?}] [{}] {}: {}",
            event.timestamp, event.level, event.component, event.event_type, event.payload
        );
    }
}

pub struct FileSink {
    filepath: String,
}
impl FileSink {
    pub fn new(filepath: &str) -> Self {
        Self { filepath: filepath.to_string() }
    }
}
impl TelemetrySink for FileSink {
    fn record(&self, event: &SystemEvent) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.filepath) {
            if let Ok(serialized) = serde_json::to_string(event) {
                let _ = writeln!(file, "{}", serialized);
            }
        }
    }
}

/// A Kafka sink which routes events to a topic.
pub struct KafkaSink {
    topic: String,
    // Note: To fully implement, we would inject a producer client here (e.g., rdkafka)
}
impl KafkaSink {
    pub fn new(topic: &str) -> Self {
        Self { topic: topic.to_string() }
    }
}
impl TelemetrySink for KafkaSink {
    fn record(&self, event: &SystemEvent) {
        // Mock kafka emission
        let _serialized = serde_json::to_string(event).unwrap_or_default();
        // producer.send(&Record::from_value(&self.topic, serialized))
    }
}

/// The TelemetryBus: Broadcaster for system signals.
pub struct TelemetryBus;

impl TelemetryBus {
    /// Broadcast an event to all configured sinks
    pub fn broadcast(event: SystemEvent) {
        // For demonstration, instantiating ad-hoc. In a complete app, we'd loop over global lazily-initialized sinks.
        let sinks: Vec<Box<dyn TelemetrySink>> = vec![
            Box::new(StdoutSink),
            Box::new(FileSink::new("telemetry.log")),
            Box::new(KafkaSink::new("omega-events")),
        ];
        
        for sink in sinks {
            sink.record(&event);
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


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
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>);
}

pub struct StdoutSink;
impl TelemetrySink for StdoutSink {
    fn record(&self, event: &SystemEvent) {
        println!(
            "[{:.3}] [{:?}] [{}] {}: {}",
            event.timestamp, event.level, event.component, event.event_type, event.payload
        );
    }
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>) {
        println!("[METRIC] {} = {} {:?}", name, value, labels);
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
    fn record_metric(&self, _name: &str, _value: f64, _labels: Vec<(String, String)>) {}
}

pub struct PrometheusSink;
impl PrometheusSink {
    pub fn new() -> Self { Self {} }
}
impl TelemetrySink for PrometheusSink {
    fn record(&self, _event: &SystemEvent) {}
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>) {
        let label_str = labels.iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(",");
        println!("{metric}{{{labels}}} {value}", metric=name, labels=label_str, value=value);
    }
}

pub struct TelemetryBus;
impl TelemetryBus {
    pub fn broadcast(event: SystemEvent) {
        let sinks: Vec<Box<dyn TelemetrySink>> = vec![
            Box::new(StdoutSink),
            Box::new(FileSink::new("telemetry.log")),
        ];
        for sink in sinks { sink.record(&event); }
    }

    pub fn record_metric(name: &str, value: f64, labels: Vec<(String, String)>) {
        let sinks: Vec<Box<dyn TelemetrySink>> = vec![
            Box::new(StdoutSink),
            Box::new(PrometheusSink::new()),
        ];
        for sink in sinks { sink.record_metric(name, value, labels.clone()); }
    }
}

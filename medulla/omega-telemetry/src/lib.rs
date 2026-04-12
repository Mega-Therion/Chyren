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

/// Sink interface for recording structured events and metrics.
pub trait TelemetrySink: Send + Sync {
    /// Record a structured event.
    fn record(&self, event: &SystemEvent);
    /// Record a numeric metric with key/value labels.
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>);
}

/// Telemetry sink that writes events and metrics to stdout.
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

/// Telemetry sink that appends JSONL events to a file path.
pub struct FileSink {
    filepath: String,
}
impl FileSink {
    /// Create a file sink that appends to `filepath`.
    pub fn new(filepath: &str) -> Self {
        Self {
            filepath: filepath.to_string(),
        }
    }
}
impl TelemetrySink for FileSink {
    fn record(&self, event: &SystemEvent) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filepath)
        {
            if let Ok(serialized) = serde_json::to_string(event) {
                let _ = writeln!(file, "{}", serialized);
            }
        }
    }
    fn record_metric(&self, _name: &str, _value: f64, _labels: Vec<(String, String)>) {}
}

/// Minimal Prometheus-text sink (prints exposition-style metric lines to stdout).
pub struct PrometheusSink;
impl PrometheusSink {
    /// Create a new Prometheus sink.
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for PrometheusSink {
    fn default() -> Self {
        Self::new()
    }
}
impl TelemetrySink for PrometheusSink {
    fn record(&self, _event: &SystemEvent) {}
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>) {
        let label_str = labels
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{metric}{{{labels}}} {value}",
            metric = name,
            labels = label_str,
            value = value
        );
    }
}

/// In-memory sink for testing — captures recorded events and metrics.
#[cfg(test)]
pub struct CaptureSink {
    events: std::sync::Mutex<Vec<SystemEvent>>,
    metrics: std::sync::Mutex<Vec<(String, f64, Vec<(String, String)>)>>,
}

#[cfg(test)]
impl CaptureSink {
    pub fn new() -> Self {
        Self {
            events: std::sync::Mutex::new(vec![]),
            metrics: std::sync::Mutex::new(vec![]),
        }
    }
    pub fn events(&self) -> Vec<SystemEvent> {
        self.events.lock().unwrap().clone()
    }
    pub fn metrics(&self) -> Vec<(String, f64, Vec<(String, String)>)> {
        self.metrics.lock().unwrap().clone()
    }
}

#[cfg(test)]
impl TelemetrySink for CaptureSink {
    fn record(&self, event: &SystemEvent) {
        self.events.lock().unwrap().push(event.clone());
    }
    fn record_metric(&self, name: &str, value: f64, labels: Vec<(String, String)>) {
        self.metrics
            .lock()
            .unwrap()
            .push((name.to_string(), value, labels));
    }
}

/// Broadcast helper that fan-outs events and metrics to a fixed set of sinks.
pub struct TelemetryBus;
impl TelemetryBus {
    /// Broadcast an event to stdout and `telemetry.log`.
    pub fn broadcast(event: SystemEvent) {
        let sinks: Vec<Box<dyn TelemetrySink>> = vec![
            Box::new(StdoutSink),
            Box::new(FileSink::new("telemetry.log")),
        ];
        for sink in sinks {
            sink.record(&event);
        }
    }

    /// Record a metric to stdout and the Prometheus-text sink.
    pub fn record_metric(name: &str, value: f64, labels: Vec<(String, String)>) {
        let sinks: Vec<Box<dyn TelemetrySink>> =
            vec![Box::new(StdoutSink), Box::new(PrometheusSink::new())];
        for sink in sinks {
            sink.record_metric(name, value, labels.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Arc;

    fn make_event(level: EventLevel) -> SystemEvent {
        SystemEvent {
            component: "test.component".into(),
            event_type: "TEST_EVENT".into(),
            level,
            payload: serde_json::json!({"key": "value"}),
            timestamp: 1234567890.0,
        }
    }

    #[test]
    fn system_event_roundtrips_json() {
        let event = make_event(EventLevel::Info);
        let json = serde_json::to_string(&event).unwrap();
        let back: SystemEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(back.component, event.component);
        assert_eq!(back.event_type, event.event_type);
        assert_eq!(back.timestamp, event.timestamp);
    }

    #[test]
    fn capture_sink_records_events() {
        let sink = CaptureSink::new();
        sink.record(&make_event(EventLevel::Warn));
        sink.record(&make_event(EventLevel::Critical));
        let events = sink.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "TEST_EVENT");
    }

    #[test]
    fn capture_sink_records_metrics() {
        let sink = CaptureSink::new();
        sink.record_metric("adccl.score", 0.85, vec![("provider".into(), "anthropic".into())]);
        let metrics = sink.metrics();
        assert_eq!(metrics.len(), 1);
        let (name, value, labels) = &metrics[0];
        assert_eq!(name, "adccl.score");
        assert!((value - 0.85).abs() < f64::EPSILON);
        assert_eq!(labels[0].0, "provider");
    }

    #[test]
    fn file_sink_appends_jsonl() {
        let path = format!("/tmp/telemetry_test_{}.log", std::process::id());
        let sink = FileSink::new(&path);
        sink.record(&make_event(EventLevel::Info));
        sink.record(&make_event(EventLevel::Critical));

        let contents = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 2);
        // Each line must be valid JSON
        for line in &lines {
            let v: serde_json::Value = serde_json::from_str(line).unwrap();
            assert_eq!(v["event_type"], "TEST_EVENT");
        }
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn prometheus_sink_formats_metric_lines() {
        // PrometheusSink writes to stdout; exercise the formatting logic via
        // the label string builder without capturing stdout.
        let sink = PrometheusSink::new();
        // Should not panic with empty or non-empty labels.
        sink.record_metric("heap_bytes", 1024.0, vec![]);
        sink.record_metric(
            "latency_ms",
            42.5,
            vec![("route".into(), "/api/run".into()), ("status".into(), "200".into())],
        );
    }

    #[test]
    fn telemetry_bus_broadcast_does_not_panic() {
        // TelemetryBus writes to stdout + "telemetry.log" in cwd; just verify
        // it doesn't panic on valid inputs.
        TelemetryBus::broadcast(make_event(EventLevel::Info));
        TelemetryBus::record_metric("test.metric", 1.0, vec![("k".into(), "v".into())]);
    }

    #[test]
    fn multi_sink_fan_out_via_trait_object() {
        let a = Arc::new(CaptureSink::new());
        let b = Arc::new(CaptureSink::new());
        let sinks: Vec<Arc<dyn TelemetrySink>> = vec![a.clone(), b.clone()];
        let event = make_event(EventLevel::Warn);
        for sink in &sinks {
            sink.record(&event);
        }
        assert_eq!(a.events().len(), 1);
        assert_eq!(b.events().len(), 1);
    }
}

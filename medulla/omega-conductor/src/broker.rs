//! broker.rs — Embedded MQTT Broker (rumqttd).
//!
//! Starts a `rumqttd` broker in a background thread so the Conductor can
//! communicate with the agent mesh without a dependency on an external
//! `mosquitto` process.  The broker listens on `127.0.0.1:1883` by default,
//! which matches the `Dispatcher`'s default connect address.

use rumqttd::{Broker, Config};

/// Start the embedded MQTT broker in a dedicated OS thread.
///
/// Returns immediately after spawning the thread.  The broker thread runs
/// indefinitely; it is terminated when the process exits.
///
/// # Panics
///
/// Panics if the broker cannot bind to the configured port — this is
/// intentional because the agent mesh is non-functional without a running
/// broker.
pub fn start_embedded_broker() {
    std::thread::spawn(|| {
        let config = build_broker_config();
        let mut broker = Broker::new(config);
        tracing::info!("[MESH-BROKER] Embedded MQTT broker starting on 127.0.0.1:1883");
        if let Err(e) = broker.start() {
            tracing::error!("[MESH-BROKER] Broker exited with error: {e}");
        }
    });
}

/// Build a minimal `rumqttd::Config` for the embedded broker.
fn build_broker_config() -> Config {
    // rumqttd ≥0.19 accepts a TOML string via `toml::from_str` or a programmatic
    // builder.  We use the raw TOML route because the builder API changes
    // frequently across patch versions.
    let toml_str = r#"
[v4.1]
name      = "chyren-mesh"
listen    = "127.0.0.1:1883"
next_connection_delay_ms = 1
max_payload_size = 65535
max_inflight_count = 256

    [v4.1.connections.max_client_id_len]
    
[router]
id              = 0
dir             = "/tmp/chyren-mqtt"
max_connections = 256
max_outgoing_packet_count = 512
max_segment_size = 104857600
max_segment_count = 10
"#;
    toml::from_str(toml_str).expect("[MESH-BROKER] Failed to parse embedded MQTT broker config")
}

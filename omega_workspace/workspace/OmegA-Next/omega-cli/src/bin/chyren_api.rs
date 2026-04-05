//! HTTP API server binary for Chyren (AEGIS + AEON pipeline behind REST).

use omega_cli::api::start_api_server;
use omega_cli::conductor::Conductor;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let host = std::env::var("CHYREN_API_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("CHYREN_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let conductor = Arc::new(Conductor::new());
    start_api_server(conductor, &host, port).await
}

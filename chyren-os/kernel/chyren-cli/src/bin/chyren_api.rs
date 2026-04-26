//! HTTP API server binary for Chyren (AEGIS + AEON pipeline behind REST).

use chyren_cli::api::start_api_server;
use chyren_cli::conductor::Conductor;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let conductor = Arc::new(Conductor::new());
    start_api_server(conductor).await
}

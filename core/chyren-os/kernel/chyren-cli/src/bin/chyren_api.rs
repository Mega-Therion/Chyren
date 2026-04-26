//! HTTP API server binary for Chyren (AEGIS + AEON pipeline behind REST).

use chyren_cli::api::start_api_server;
use chyren_cli::conductor::Conductor;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    // Start embedded MQTT broker for the agent mesh
    chyren_conductor::broker::start_embedded_broker();

    let mut conductor = Conductor::new();

    // Initialize Dispatcher
    let registry = Arc::new(Mutex::new(chyren_core::mesh::AgentRegistry::new()));
    let dispatcher = Arc::new(chyren_conductor::dispatcher::Dispatcher::new(registry).await);
    conductor.set_dispatcher(dispatcher);

    // Initialize DB
    let db_url = std::env::var("CHYREN_DB_URL")
        .unwrap_or_else(|_| "postgresql://postgres@localhost:5432/chyren".to_string());
    if let Ok(store) = chyren_myelin::db::MemoryStore::connect(&db_url, "").await {
        conductor.set_store(Arc::new(store));
    }

    // Bootstrap identity and spawn internal agents
    let _ = conductor.bootstrap_identity().await;
    let _ = conductor.spawn_agents().await;

    start_api_server(Arc::new(conductor)).await
}

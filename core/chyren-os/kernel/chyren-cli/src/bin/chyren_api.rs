//! HTTP API server binary for Chyren (AEGIS + AEON pipeline behind REST).

use chyren_cli::api::start_api_server;
use chyren_cli::conductor::Conductor;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    eprintln!("[API] Starting boot sequence...");

    // Start embedded MQTT broker for the agent mesh
    chyren_conductor::broker::start_embedded_broker();
    eprintln!("[API] MQTT Broker started.");

    let mut conductor = Conductor::new();

    // Initialize Dispatcher
    let registry = Arc::new(Mutex::new(chyren_core::mesh::AgentRegistry::new()));
    let bus = chyren_rsil::bus::EventBus::new(100);
    let dispatcher = Arc::new(chyren_conductor::dispatcher::Dispatcher::new(bus, registry));
    conductor.set_dispatcher(dispatcher);
    eprintln!("[API] Dispatcher initialized.");

    // Initialize DB
    let db_url = std::env::var("CHYREN_DB_URL")
        .unwrap_or_else(|_| "postgresql://postgres@localhost:5432/chyren".to_string());
    
    // We use a timeout to avoid hanging if postgres is not available
    let db_connect_result = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        chyren_myelin::db::MemoryStore::connect(&db_url, "")
    ).await;

    match db_connect_result {
        Ok(Ok(store)) => {
            conductor.set_store(Arc::new(store));
            eprintln!("[API] DB connected.");
        }
        Ok(Err(e)) => {
            eprintln!("[API] DB connection failed: {}. Continuing in-memory.", e);
        }
        Err(_) => {
            eprintln!("[API] DB connection timed out. Continuing in-memory.");
        }
    }

    // Bootstrap identity and spawn internal agents
    let _ = conductor.bootstrap_identity().await;
    eprintln!("[API] Identity bootstrapped.");
    let _ = conductor.spawn_agents().await;
    eprintln!("[API] Agents spawned.");

    start_api_server(Arc::new(conductor)).await
}

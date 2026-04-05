//! Ingestion test runner for the Sovereign Hub.
use anyhow::Context;
use omega_myelin::db::MemoryStore;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Initializing Ingestion Engine Test...");

    let db_url = env::var("DATABASE_URL")
        .context("DATABASE_URL must be set (Neon or other Postgres connection string)")?;
    let ledger_path = "sync_ledger.json";

    let store: MemoryStore = MemoryStore::connect(&db_url, ledger_path).await?;

    println!("Performing delta sync...");
    let new_nodes: Vec<omega_core::MemoryNode> = store.sync_delta().await?;

    println!("Ingested {} new memory nodes.", new_nodes.len());
    for node in new_nodes.iter().take(5) {
        println!(
            " - Node ID: {}, Importance: {}",
            node.node_id, node.decay_score
        );
    }

    Ok(())
}

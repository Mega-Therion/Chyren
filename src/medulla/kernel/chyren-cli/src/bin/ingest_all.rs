//! Ingestion test runner for all projects.
use anyhow::Context;
use chyren_myelin::db::MemoryStore;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("System-wide Ingestion Initiated...");

    let db_url = env::var("DATABASE_URL")
        .context("DATABASE_URL must be set (Neon or other Postgres connection string)")?;

    let projects = vec![
        ("fancy-shape-98445838", "chyren-db3-metadata"),
        ("purple-bread-84628881", "database3neon"),
        ("long-queen-57196333", "chyren-technical-db"),
        ("shy-wave-51974271", "Chyren"),
    ];

    for (pid, name) in projects {
        println!("Syncing project: {} ({})", name, pid);
        let ledger_path = format!("sync_ledger_{}.json", pid);

        let store = MemoryStore::connect(&db_url, &ledger_path)
            .await
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| anyhow::anyhow!(e))?;
        let nodes = store
            .sync_delta()
            .await
            .map_err(|e: Box<dyn std::error::Error + Send + Sync>| anyhow::anyhow!(e))?;
        println!(
            " - Successfully ingested {} nodes from {}",
            nodes.len(),
            name
        );
    }

    Ok(())
}

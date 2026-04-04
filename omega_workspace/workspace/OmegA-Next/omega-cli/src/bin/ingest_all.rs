//! Ingestion test runner for all projects.
use omega_myelin::db::MemoryStore;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("System-wide Ingestion Initiated...");
    
    let projects = vec![
        ("fancy-shape-98445838", "omega-db3-metadata"),
        ("purple-bread-84628881", "database3neon"),
        ("long-queen-57196333", "omega-technical-db"),
        ("shy-wave-51974271", "OmegA"),
    ];

    for (pid, name) in projects {
        println!("Syncing project: {} ({})", name, pid);
        // Using centralized database pool for demonstration; in production, 
        // connection strings would be fetched per project ID via MCP.
        let db_url = "postgresql://neondb_owner:npg_HbW1Zlkjd7NI@ep-sweet-glade-anvm0pwn-pooler.c-6.us-east-1.aws.neon.tech/neondb?channel_binding=require&sslmode=require";
        let ledger_path = format!("sync_ledger_{}.json", pid);
        
        let store = MemoryStore::connect(db_url, &ledger_path).await?;
        let nodes = store.sync_delta().await?;
        println!(" - Successfully ingested {} nodes from {}", nodes.len(), name);
    }
    
    Ok(())
}

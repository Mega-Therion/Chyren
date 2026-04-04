//! Ingestion test runner for the Sovereign Hub.
use omega_myelin::db::MemoryStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Ingestion Engine Test...");
    
    // Connection string retrieved from project configuration
    let db_url = "postgresql://neondb_owner:npg_HbW1Zlkjd7NI@ep-sweet-glade-anvm0pwn-pooler.c-6.us-east-1.aws.neon.tech/neondb?channel_binding=require&sslmode=require";
    let ledger_path = "sync_ledger.json";

    let store = MemoryStore::connect(db_url, ledger_path).await?;
    
    println!("Performing delta sync...");
    let new_nodes = store.sync_delta().await?;
    
    println!("Ingested {} new memory nodes.", new_nodes.len());
    for node in new_nodes.iter().take(5) {
        println!(" - Node ID: {}, Importance: {}", node.node_id, node.decay_score);
    }
    
    Ok(())
}

//! omega-myelin: Graph memory overlay and persistence engine.

use sqlx::{postgres::PgPoolOptions, PgPool, FromRow};
use omega_core::{MemoryNode, MemoryStratum, now};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a row from public.omega_memory_entries
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DbMemoryEntry {
    pub id: String,
    pub content: String,
    pub importance: f64,
    pub created_at: String,
    pub namespace: String,
    pub domain: String,
}

/// Persistent memory store
pub struct MemoryStore {
    pool: PgPool,
    ledger_path: String,
}

impl MemoryStore {
    /// Connect to Neon Postgres and initialize local sync ledger
    pub async fn connect(url: &str, ledger_path: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool, ledger_path: ledger_path.to_string() })
    }

    /// Read the last sync timestamp from the ledger
    pub fn get_last_sync(&self) -> String {
        if std::path::Path::new(&self.ledger_path).exists() {
            let data = std::fs::read_to_string(&self.ledger_path).unwrap_or_else(|_| "1970-01-01 00:00:00".to_string());
            let ledger: crate::db::SyncLedger = serde_json::from_str(&data).unwrap_or(crate::db::SyncLedger { last_synced_at: "1970-01-01 00:00:00".to_string() });
            ledger.last_synced_at
        } else {
            "1970-01-01 00:00:00".to_string()
        }
    }

    /// Update the ledger with the newest timestamp
    pub fn update_ledger(&self, timestamp: &str) -> Result<(), std::io::Error> {
        let ledger = crate::db::SyncLedger { last_synced_at: timestamp.to_string() };
        let data = serde_json::to_string(&ledger)?;
        std::fs::write(&self.ledger_path, data)
    }

    /// Perform a differential sync: only fetch entries created after the last sync
    pub async fn sync_delta(&self) -> Result<Vec<MemoryNode>, sqlx::Error> {
        let last_sync = self.get_last_sync();
        
        let rows = sqlx::query_as::<_, DbMemoryEntry>(
            "SELECT id, content, importance, created_at, namespace, domain 
             FROM public.omega_memory_entries 
             WHERE created_at > $1 
             ORDER BY created_at ASC"
        )
        .bind(&last_sync)
        .fetch_all(&self.pool)
        .await?;

        if let Some(newest) = rows.last() {
            let _ = self.update_ledger(&newest.created_at);
        }

        let nodes = rows.into_iter().map(|row| MemoryNode {
            node_id: row.id,
            content: row.content,
            stratum: MemoryStratum::Operational, // Map to default Operational stratum
            created_at: now(), // Placeholder
            last_accessed: now(),
            retrieval_count: 0,
            decay_score: row.importance,
        }).collect();

        Ok(nodes)
    }
}

/// Sync Ledger: Tracks the last ingested memory record
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncLedger {
    pub last_synced_at: String,
}

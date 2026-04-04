//! Database bridge for memory graph persistence.

use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::MemoryGraph;

/// Persistent memory store
pub struct MemoryStore {
    pool: PgPool,
}

impl MemoryStore {
    /// Connect to Neon Postgres
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    /// Load graph from DB
    pub async fn load_graph(&self) -> Result<MemoryGraph, sqlx::Error> {
        // Implementation for mapping tables: memories, myelin_memory_graph, myelin_edges
        Ok(MemoryGraph::new())
    }
}

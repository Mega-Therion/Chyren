//! omega-myelin database layer — persistent storage via Neon/Postgres.
//!
//! Provides `MemoryStore` for loading and syncing the MemoryGraph from a
//! Postgres-compatible database (Neon serverless or local Postgres).

use omega_core::MemoryNode;
use std::error::Error;
use std::fmt;

/// Error type for myelin database operations.
#[derive(Debug)]
pub struct MyelinError(pub String);

impl fmt::Display for MyelinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for MyelinError {}

/// Persistent memory store backed by Postgres/Neon.
///
/// Manages the lifecycle of memory nodes and edges in the database.
pub struct MemoryStore {
    pool: sqlx::PgPool,
}

impl MemoryStore {
    /// Connect to the database and ensure the schema exists.
    ///
    /// `url` is a Postgres connection string (e.g. from `OMEGA_DB_URL`).
    /// `_path` is reserved for future local-cache overlay.
    pub async fn connect(url: &str, _path: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let pool = sqlx::PgPool::connect(url).await?;

        // Ensure the schema exists — idempotent.
        // We split these into separate queries because sqlx::query() with multiple statements 
        // can fail as a "prepared statement" on some Postgres drivers/proxies (e.g. Neon).
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS memory_nodes (
                node_id     TEXT PRIMARY KEY,
                content     TEXT NOT NULL DEFAULT '',
                retrieval_count BIGINT NOT NULL DEFAULT 0,
                decay_score DOUBLE PRECISION NOT NULL DEFAULT 1.0,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS memory_edges (
                id          SERIAL PRIMARY KEY,
                from_label  TEXT NOT NULL DEFAULT '',
                to_label    TEXT NOT NULL DEFAULT '',
                from_id     TEXT NOT NULL REFERENCES memory_nodes(node_id) ON DELETE CASCADE,
                to_id       TEXT NOT NULL REFERENCES memory_nodes(node_id) ON DELETE CASCADE,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS threat_entries (
                pattern_id  TEXT PRIMARY KEY,
                severity    TEXT NOT NULL DEFAULT 'low',
                labels      JSONB NOT NULL DEFAULT '[]',
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ledger_entries (
                run_id      TEXT PRIMARY KEY,
                task        TEXT NOT NULL DEFAULT '',
                provider    TEXT NOT NULL DEFAULT '',
                model       TEXT NOT NULL DEFAULT '',
                status      TEXT NOT NULL DEFAULT '',
                response    TEXT NOT NULL DEFAULT '',
                adccl_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                adccl_flags JSONB NOT NULL DEFAULT '[]',
                latency_ms  DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                token_count INTEGER NOT NULL DEFAULT 0,
                created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
                signature   TEXT NOT NULL DEFAULT ''
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    /// Load all memory nodes from the database.
    pub async fn load_nodes(&self) -> Result<Vec<MemoryNode>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query_as::<_, NodeRow>(
            "SELECT node_id, content, retrieval_count, decay_score FROM memory_nodes ORDER BY created_at",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| MemoryNode {
                node_id: r.node_id,
                content: r.content,
                retrieval_count: r.retrieval_count as u64,
                decay_score: r.decay_score,
            })
            .collect())
    }

    /// Upsert a memory node.
    pub async fn upsert_node(&self, node: &MemoryNode) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query(
            r#"
            INSERT INTO memory_nodes (node_id, content, retrieval_count, decay_score, updated_at)
            VALUES ($1, $2, $3, $4, now())
            ON CONFLICT (node_id) DO UPDATE SET
                content = EXCLUDED.content,
                retrieval_count = EXCLUDED.retrieval_count,
                decay_score = EXCLUDED.decay_score,
                updated_at = now()
            "#,
        )
        .bind(&node.node_id)
        .bind(&node.content)
        .bind(node.retrieval_count as i64)
        .bind(node.decay_score)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Increment retrieval count for a node.
    pub async fn touch_node(&self, node_id: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query(
            "UPDATE memory_nodes SET retrieval_count = retrieval_count + 1, updated_at = now() WHERE node_id = $1",
        )
        .bind(node_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Sync delta: load nodes updated since the last sync.
    ///
    /// Currently loads all nodes (no incremental tracking yet).
    pub async fn sync_delta(&self) -> Result<Vec<MemoryNode>, Box<dyn Error + Send + Sync>> {
        self.load_nodes().await
    }

    /// Store a ledger entry in the database.
    #[allow(clippy::too_many_arguments)]
    pub async fn store_ledger_entry(
        &self,
        run_id: &str,
        task: &str,
        provider: &str,
        model: &str,
        status: &str,
        response: &str,
        adccl_score: f64,
        adccl_flags: &[String],
        latency_ms: f64,
        token_count: i32,
        signature: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let flags_json = serde_json::to_value(adccl_flags)?;

        sqlx::query(
            r#"
            INSERT INTO ledger_entries (run_id, task, provider, model, status, response, adccl_score, adccl_flags, latency_ms, token_count, signature)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (run_id) DO NOTHING
            "#,
        )
        .bind(run_id)
        .bind(task)
        .bind(provider)
        .bind(model)
        .bind(status)
        .bind(response)
        .bind(adccl_score)
        .bind(flags_json)
        .bind(latency_ms)
        .bind(token_count)
        .bind(signature)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Return the total number of entries in the ledger.
    pub async fn count_entries(&self) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ledger_entries")
            .fetch_one(&self.pool)
            .await?;
        Ok(count as usize)
    }

    /// Get a reference to the connection pool.
    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }

    /// Reset all persisted state for this store.
    ///
    /// This clears:
    /// - `ledger_entries`
    /// - `memory_edges`
    /// - `memory_nodes`
    /// - `threat_entries`
    ///
    /// Note: This does not affect external vector stores (e.g. Qdrant).
    pub async fn reset_all(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Delete in dependency order (edges reference nodes).
        // Use a transaction so callers never see partially-cleared state.
        let mut tx = self.pool.begin().await?;
        sqlx::query("DELETE FROM ledger_entries")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM memory_edges")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM memory_nodes")
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM threat_entries")
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}

/// Internal row type for sqlx deserialization.
#[derive(sqlx::FromRow)]
struct NodeRow {
    node_id: String,
    content: String,
    retrieval_count: i64,
    decay_score: f64,
}

-- Dataset ingestion queue table
-- Created by: R.W.Ϝ.Y. — ARI Genesis pipeline
-- Run once against the Neon/PostgreSQL catalog DB:
--   psql "$OMEGA_CATALOG_DB_URL" -f cortex/ops/sql/create_dataset_queue.sql

CREATE TABLE IF NOT EXISTS dataset_queue (
    id              TEXT PRIMARY KEY,          -- e.g. "wikimedia-wikipedia-en-simple"
    hf_repo         TEXT NOT NULL,             -- e.g. "wikimedia/wikipedia"
    hf_subset       TEXT DEFAULT '',           -- e.g. "20231101.simple"
    hf_split        TEXT DEFAULT 'train',
    priority        INTEGER DEFAULT 50,        -- 1=highest, 100=lowest
    row_limit       INTEGER DEFAULT 50000,
    status          TEXT DEFAULT 'pending',    -- pending|downloading|ingesting|verifying|done|error
    ingested_rows   INTEGER DEFAULT 0,
    error_msg       TEXT,
    queued_at       TIMESTAMPTZ DEFAULT NOW(),
    started_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,
    source_hash     TEXT,                      -- SHA-256(repo+subset) — dedup key
    provenance      JSONB                      -- R.W.Ϝ.Y. ledger stamp
);

CREATE INDEX IF NOT EXISTS idx_queue_status_priority
    ON dataset_queue(status, priority, queued_at);

CREATE INDEX IF NOT EXISTS idx_queue_source_hash
    ON dataset_queue(source_hash);

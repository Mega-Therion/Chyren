-- omega_library_catalog — Master Index Card Catalog (LIC)
-- Per cortex/ops/LIBRARY_INDEX_SOP.md
--
-- Idempotent: safe to run multiple times.
-- Target: master catalog Neon project (project_id fancy-shape-98445838).

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS omega_library_catalog (
    card_id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- LIC: Location
    shard_id           TEXT        NOT NULL,
    platform           TEXT        NOT NULL CHECK (platform IN ('neon','supabase','firebase','cloudflare')),

    -- LIC: Shelf
    shelf_table        TEXT        NOT NULL,

    -- LIC: Subject
    subject_domain     TEXT        NOT NULL,
    summary            TEXT        NOT NULL,
    keywords           JSONB       NOT NULL DEFAULT '[]'::jsonb,

    -- LIC: Temporal bounds
    temporal_start     TIMESTAMPTZ,
    temporal_end       TIMESTAMPTZ,

    -- Operational metadata
    row_count_estimate BIGINT,
    last_indexed_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- One canonical card per (shard, table, domain)
    UNIQUE (shard_id, shelf_table, subject_domain)
);

CREATE INDEX IF NOT EXISTS idx_lic_shard           ON omega_library_catalog (shard_id);
CREATE INDEX IF NOT EXISTS idx_lic_platform        ON omega_library_catalog (platform);
CREATE INDEX IF NOT EXISTS idx_lic_subject         ON omega_library_catalog (subject_domain);
CREATE INDEX IF NOT EXISTS idx_lic_keywords_gin    ON omega_library_catalog USING GIN (keywords);
CREATE INDEX IF NOT EXISTS idx_lic_summary_fts     ON omega_library_catalog USING GIN (to_tsvector('english', summary));

-- updated_at auto-bump
CREATE OR REPLACE FUNCTION omega_lic_touch_updated_at() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_lic_touch ON omega_library_catalog;
CREATE TRIGGER trg_lic_touch
    BEFORE UPDATE ON omega_library_catalog
    FOR EACH ROW EXECUTE FUNCTION omega_lic_touch_updated_at();

-- Search function: returns ranked candidate shards for a free-text query.
-- Combines keyword overlap + full-text rank. Ordered by score, highest first.
CREATE OR REPLACE FUNCTION omega_lic_search(
    q          TEXT,
    domain_hint TEXT DEFAULT NULL,
    max_rows   INT  DEFAULT 10
) RETURNS TABLE (
    card_id        UUID,
    shard_id       TEXT,
    platform       TEXT,
    shelf_table    TEXT,
    subject_domain TEXT,
    summary        TEXT,
    score          REAL
) AS $$
    SELECT
        c.card_id,
        c.shard_id,
        c.platform,
        c.shelf_table,
        c.subject_domain,
        c.summary,
        (
            ts_rank_cd(to_tsvector('english', c.summary), plainto_tsquery('english', q))
          + CASE WHEN c.keywords ?| string_to_array(lower(q), ' ') THEN 0.5 ELSE 0 END
          + CASE WHEN domain_hint IS NOT NULL AND c.subject_domain = domain_hint THEN 0.3 ELSE 0 END
        )::REAL AS score
    FROM omega_library_catalog c
    WHERE
        to_tsvector('english', c.summary) @@ plainto_tsquery('english', q)
        OR c.keywords ?| string_to_array(lower(q), ' ')
        OR (domain_hint IS NOT NULL AND c.subject_domain = domain_hint)
    ORDER BY score DESC
    LIMIT max_rows;
$$ LANGUAGE SQL STABLE;

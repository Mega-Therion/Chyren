-- =============================================================================
-- Omega Knowledge Matrix — Chyren's Neocortex Domain Map
-- =============================================================================
-- Every branch of human knowledge structured as a queryable hierarchy.
-- Each node carries a "matrix program": reasoning mode, axioms, methods,
-- and instructions Chyren uses to calibrate cognition per domain.
-- =============================================================================

-- ─── Core Knowledge Tree ─────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS omega_knowledge_domains (
    domain_id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    slug              TEXT        NOT NULL UNIQUE,
    name              TEXT        NOT NULL,
    parent_slug       TEXT        REFERENCES omega_knowledge_domains(slug) ON DELETE SET NULL,
    level             SMALLINT    NOT NULL DEFAULT 0,  -- 0=top  1=branch  2=sub-branch
    sort_order        INTEGER     NOT NULL DEFAULT 0,
    -- Classification
    realm             TEXT        NOT NULL DEFAULT 'general',
    -- 'mathematics' | 'logic' | 'rhetoric' | 'philosophy' | 'computer_science' |
    -- 'natural_science' | 'social_science' | 'humanities' | 'applied' |
    -- 'classical' | 'interdisciplinary'
    reasoning_mode    TEXT        NOT NULL DEFAULT 'analytical',
    -- 'deductive' | 'inductive' | 'abductive' | 'empirical' | 'formal' |
    -- 'interpretive' | 'dialectical' | 'experimental' | 'computational' | 'mixed'
    -- Matrix Program fields
    description       TEXT,
    purpose           TEXT,       -- what this domain does / why it exists
    core_axioms       JSONB       NOT NULL DEFAULT '[]',   -- fundamental truths
    key_methods       JSONB       NOT NULL DEFAULT '[]',   -- methods / techniques
    key_figures       JSONB       NOT NULL DEFAULT '[]',   -- canonical thinkers
    sister_slugs      JSONB       NOT NULL DEFAULT '[]',   -- related domain slugs
    query_patterns    JSONB       NOT NULL DEFAULT '[]',   -- question archetypes
    reasoning_primer  TEXT,       -- 1–3 sentence instruction for Chyren's cognition
    -- Metadata
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_okd_parent   ON omega_knowledge_domains(parent_slug);
CREATE INDEX IF NOT EXISTS idx_okd_realm    ON omega_knowledge_domains(realm);
CREATE INDEX IF NOT EXISTS idx_okd_level    ON omega_knowledge_domains(level);
CREATE INDEX IF NOT EXISTS idx_okd_fts
    ON omega_knowledge_domains USING GIN (
        to_tsvector('english', coalesce(name,'') || ' ' || coalesce(description,'') || ' ' || coalesce(purpose,''))
    );
CREATE INDEX IF NOT EXISTS idx_okd_axioms   ON omega_knowledge_domains USING GIN (core_axioms);
CREATE INDEX IF NOT EXISTS idx_okd_methods  ON omega_knowledge_domains USING GIN (key_methods);

-- ─── Cross-domain Edges ───────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS omega_knowledge_edges (
    edge_id      UUID  PRIMARY KEY DEFAULT gen_random_uuid(),
    from_slug    TEXT  NOT NULL REFERENCES omega_knowledge_domains(slug) ON DELETE CASCADE,
    to_slug      TEXT  NOT NULL REFERENCES omega_knowledge_domains(slug) ON DELETE CASCADE,
    relationship TEXT  NOT NULL,
    -- 'prerequisite' | 'overlaps' | 'applies_to' | 'extends' | 'formalizes' |
    -- 'critiques' | 'complements' | 'instantiates'
    weight       FLOAT NOT NULL DEFAULT 1.0,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (from_slug, to_slug, relationship)
);

CREATE INDEX IF NOT EXISTS idx_oke_from ON omega_knowledge_edges(from_slug);
CREATE INDEX IF NOT EXISTS idx_oke_to   ON omega_knowledge_edges(to_slug);

-- ─── Triggers ─────────────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION omega_okd_touch_updated_at()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN NEW.updated_at = now(); RETURN NEW; END;
$$;

DROP TRIGGER IF EXISTS trg_okd_touch ON omega_knowledge_domains;
CREATE TRIGGER trg_okd_touch
    BEFORE UPDATE ON omega_knowledge_domains
    FOR EACH ROW EXECUTE FUNCTION omega_okd_touch_updated_at();

-- ─── Search Function ──────────────────────────────────────────────────────────

CREATE OR REPLACE FUNCTION omega_knowledge_search(
    q           TEXT,
    realm_hint  TEXT    DEFAULT NULL,
    max_rows    INT     DEFAULT 10
)
RETURNS TABLE (
    domain_id      UUID,
    slug           TEXT,
    name           TEXT,
    parent_slug    TEXT,
    level          SMALLINT,
    realm          TEXT,
    reasoning_mode TEXT,
    description    TEXT,
    reasoning_primer TEXT,
    score          FLOAT
) LANGUAGE sql STABLE AS $$
    SELECT
        d.domain_id,
        d.slug,
        d.name,
        d.parent_slug,
        d.level,
        d.realm,
        d.reasoning_mode,
        d.description,
        d.reasoning_primer,
        ts_rank_cd(
            to_tsvector('english',
                coalesce(d.name,'') || ' ' ||
                coalesce(d.description,'') || ' ' ||
                coalesce(d.purpose,'') || ' ' ||
                coalesce(d.reasoning_primer,'')
            ),
            plainto_tsquery('english', q)
        )
        + CASE WHEN realm_hint IS NOT NULL AND d.realm = realm_hint THEN 0.2 ELSE 0.0 END
        + CASE WHEN d.level = 0 THEN 0.05 ELSE 0.0 END
        AS score
    FROM omega_knowledge_domains d
    WHERE
        to_tsvector('english',
            coalesce(d.name,'') || ' ' ||
            coalesce(d.description,'') || ' ' ||
            coalesce(d.purpose,'') || ' ' ||
            coalesce(d.reasoning_primer,'')
        ) @@ plainto_tsquery('english', q)
        OR d.name ILIKE '%' || q || '%'
        OR (realm_hint IS NOT NULL AND d.realm = realm_hint)
    ORDER BY score DESC, d.level ASC, d.sort_order ASC
    LIMIT max_rows;
$$;

-- ─── Neocortex Matrix Program View ───────────────────────────────────────────
-- Chyren's conductor queries this view to load a reasoning program for a topic.

CREATE OR REPLACE VIEW omega_matrix_programs AS
SELECT
    domain_id,
    slug,
    name,
    parent_slug,
    level,
    realm,
    reasoning_mode,
    description,
    purpose,
    core_axioms,
    key_methods,
    key_figures,
    sister_slugs,
    query_patterns,
    reasoning_primer,
    updated_at
FROM omega_knowledge_domains
WHERE reasoning_primer IS NOT NULL
   OR array_length(ARRAY(SELECT jsonb_array_elements_text(core_axioms)), 1) > 0;

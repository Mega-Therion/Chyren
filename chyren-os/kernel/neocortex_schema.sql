CREATE TABLE IF NOT EXISTS neocortex_library (
    program_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain TEXT NOT NULL,
    version TEXT NOT NULL,
    integrity_hash TEXT NOT NULL,
    binary_blob BYTEA NOT NULL,
    metadata JSONB DEFAULT '{}'
);

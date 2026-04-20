CREATE TABLE IF NOT EXISTS cdl_registry (
    id TEXT PRIMARY KEY,
    entity_data JSONB NOT NULL,
    domain TEXT NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_cdl_entity_data ON cdl_registry USING GIN (entity_data);

-- JWST Observations Table
CREATE TABLE IF NOT EXISTS jwst_observations (
    id SERIAL PRIMARY KEY,
    obs_id TEXT UNIQUE NOT NULL,
    program_id INTEGER NOT NULL,
    target_name TEXT,
    ra DOUBLE PRECISION,
    dec DOUBLE PRECISION,
    source_z DOUBLE PRECISION,
    instrument TEXT,
    filter TEXT,
    exposure_time DOUBLE PRECISION,
    data_path TEXT,
    processed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_program_id ON jwst_observations(program_id);

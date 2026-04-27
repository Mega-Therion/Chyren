-- Schema update for Chyren Status Broadcast System
CREATE TABLE IF NOT EXISTS statuses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    text TEXT NOT NULL,
    media TEXT[] DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for faster feed retrieval
CREATE INDEX IF NOT EXISTS idx_statuses_created_at ON statuses(created_at DESC);

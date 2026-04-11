import psycopg2
import json
import os

# Load the active shard from the pool
POOL_PATH = "/home/mega/Chyren/hub/ops/db_pool.json"
with open(POOL_PATH, 'r') as f:
    pool = json.load(f)
    DATABASE_URL = pool["active_primary"]

def init_shard():
    print(f"--- INITIALIZING NEW SHARD: {DATABASE_URL[:50]}... ---")
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    # 1. Create table family_profiles
    cur.execute("""
        CREATE TABLE IF NOT EXISTS family_profiles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            occupation TEXT,
            ry_notes TEXT,
            fun_facts JSONB DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
    """)
    print("  Ensured table: family_profiles")

    # 2. Create table omega_memory_entries (and memory_entries alias if needed)
    cur.execute("""
        CREATE TABLE IF NOT EXISTS omega_memory_entries (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            source TEXT NOT NULL,
            importance REAL DEFAULT 0.5,
            namespace TEXT DEFAULT 'default',
            confidence REAL DEFAULT 1.0,
            domain TEXT DEFAULT 'general',
            version INTEGER DEFAULT 1,
            created_at TEXT NOT NULL
        );
    """)
    print("  Ensured table: omega_memory_entries")

    # 3. Create table omega_library_catalog (The Index Card System)
    cur.execute("""
        CREATE TABLE IF NOT EXISTS omega_library_catalog (
            card_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            shard_id TEXT NOT NULL,
            shelf_table TEXT NOT NULL,
            subject_domain TEXT NOT NULL,
            semantic_summary TEXT,
            keywords JSONB DEFAULT '[]',
            time_start TIMESTAMP WITH TIME ZONE,
            time_end TIMESTAMP WITH TIME ZONE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        );
    """)
    print("  Ensured table: omega_library_catalog (Index Card System)")

    conn.commit()
    cur.close()
    conn.close()
    print("--- SHARD INITIALIZATION COMPLETE ---")

if __name__ == "__main__":
    init_shard()

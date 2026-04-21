#!/usr/bin/env python3
import os
import psycopg2
import uuid
from pathlib import Path
from datetime import datetime

REPO_ROOT = Path(__file__).parents[3]
DB_URL = os.getenv("OMEGA_DB_URL")

DOCS_TO_INGEST = [
    REPO_ROOT / "AGENTS.md",
    REPO_ROOT / "GEMINI.md",
] + list((REPO_ROOT / "docs").glob("*.md"))

def ingest_docs():
    if not DB_URL:
        print("✗ OMEGA_DB_URL not set")
        return

    conn = psycopg2.connect(DB_URL)
    cur = conn.cursor()

    print(f"🚀 Ingesting {len(DOCS_TO_INGEST)} documentation files...")

    for doc_path in DOCS_TO_INGEST:
        if not doc_path.exists():
            continue
        
        content = doc_path.read_text(errors="replace")
        title = doc_path.name
        run_id = f"run-{uuid.uuid4().hex[:8]}"
        node_id = f"node-{uuid.uuid4().hex[:8]}"

        # 1. Insert into omega_memory_entries (for Identity Synthesis)
        cur.execute("""
            INSERT INTO omega_memory_entries 
            (run_id, task, response, adccl_score, source, created_at)
            VALUES (%s, %s, %s, %s, %s, %s)
        """, (
            run_id,
            f"Ingest documentation: {title}",
            content,
            1.0,
            "self-ingestion",
            datetime.now()
        ))

        # 2. Insert into memory_nodes (for Medulla Retrieval)
        cur.execute("""
            INSERT INTO memory_nodes 
            (node_id, content, retrieval_count, decay_score, created_at, updated_at)
            VALUES (%s, %s, %s, %s, %s, %s)
            ON CONFLICT (node_id) DO NOTHING
        """, (
            node_id,
            f"--- DOCUMENT: {title} ---\n{content}",
            0,
            1.0,
            datetime.now(),
            datetime.now()
        ))

        print(f"  ✓ {title} ingested.")

    conn.commit()
    cur.close()
    conn.close()
    print("✨ Documentation ingestion complete.")

if __name__ == "__main__":
    ingest_docs()

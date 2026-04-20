#!/usr/bin/env python3
import os
import sys
import json
import time
import uuid
import hashlib
import psycopg2
import argparse
import urllib.request
from datetime import datetime
print(f"Python Executable: {sys.executable}", flush=True)
print(f"Python Path: {sys.path}", flush=True)
try:
    from datasets import load_dataset
    print("✓ datasets imported successfully", flush=True)
except ImportError as e:
    print(f"✗ datasets import failed: {e}")
    sys.exit(1)
from qdrant_client import QdrantClient
from qdrant_client.models import PointStruct, VectorParams, Distance

# ─── Config ──────────────────────────────────────────────────────────────────
COLLECTION = "knowledge_matrix"
GEMINI_MODEL = "models/gemini-embedding-001"
GEMINI_DIM = 3072
BATCH_SIZE = 100

def get_db_conn():
    db_url = os.environ.get("OMEGA_CATALOG_DB_URL")
    if not db_url: raise ValueError("OMEGA_CATALOG_DB_URL not set")
    return psycopg2.connect(db_url)

def embed_text(text):
    import os, json, urllib.request, time
    api_key = os.environ.get("GEMINI_API_KEY")
    if api_key:
        model = "models/gemini-embedding-001"
        base = f"https://generativelanguage.googleapis.com/v1beta/{model}:embedContent?key={api_key}"
        body = {"model": model, "content": {"parts": [{"text": text}]}}
        req = urllib.request.Request(base, data=json.dumps(body).encode(), headers={"Content-Type": "application/json"}, method="POST")
        for attempt in range(2):
            try:
                with urllib.request.urlopen(req, timeout=10) as r:
                    data = json.loads(r.read())
                return data["embedding"]["values"]
            except Exception as e:
                if "429" in str(e):
                    time.sleep(2)
                else: break

    # Fallback to OpenAI
    openai_key = os.environ.get("OPENAI_API_KEY")
    if openai_key:
        try:
            req = urllib.request.Request(
                "https://api.openai.com/v1/embeddings",
                data=json.dumps({"model": "text-embedding-3-large", "dimensions": 3072, "input": text}).encode(),
                headers={"Authorization": f"Bearer {openai_key}", "Content-Type": "application/json"},
                method="POST"
            )
            with urllib.request.urlopen(req, timeout=10) as r:
                data = json.loads(r.read())
            return data["data"][0]["embedding"]
        except: pass
    return None

def stable_int_id(slug):
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def claim_task():
    conn = get_db_conn()
    cur = conn.cursor()
    cur.execute("""
        UPDATE dataset_queue 
        SET status = 'downloading', started_at = NOW()
        WHERE id = (
            SELECT id FROM dataset_queue 
            WHERE status = 'pending' 
            ORDER BY priority ASC, queued_at ASC 
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        )
        RETURNING id, hf_repo, hf_subset, hf_split, row_limit
    """)
    task = cur.fetchone()
    conn.commit()
    cur.close()
    conn.close()
    return task

def update_task_status(task_id, status, rows=0, error=None):
    conn = get_db_conn()
    cur = conn.cursor()
    if status == 'done':
        cur.execute("UPDATE dataset_queue SET status = 'done', completed_at = NOW(), ingested_rows = %s WHERE id = %s", (rows, task_id))
    elif status == 'error':
        cur.execute("UPDATE dataset_queue SET status = 'error', error_msg = %s WHERE id = %s", (error, task_id))
    else:
        cur.execute("UPDATE dataset_queue SET status = %s WHERE id = %s", (status, task_id))
    conn.commit()
    cur.close()
    conn.close()

def main():
    task = claim_task()
    if not task:
        print("No pending tasks in queue.")
        return

    task_id, repo, subset, split, limit = task
    print(f"Processing task: {task_id} ({repo})")
    
    qdrant_url = os.environ.get("QDRANT_URL", "http://localhost:6333")
    q_client = QdrantClient(url=qdrant_url)

    try:
        update_task_status(task_id, 'ingesting')
        ds = load_dataset(repo, subset if subset else None, split=split, streaming=True)
        
        count = 0
        batch = []
        for row in ds:
            if count >= limit: break
            
            # Basic mapping logic - adjust based on common HF formats
            content = row.get('text') or row.get('summary') or row.get('content') or ""
            title = row.get('title') or f"{repo} entry {count}"
            
            if not content: continue
            
            slug = f"{task_id}-{count}"
            vector = embed_text(content[:1000])
            if not vector: continue
            
            entity = {
                "id": slug,
                "name": title[:100],
                "description": content[:500],
                "realm": "external",
                "kind": "concept",
                "provenance": {
                    "createdAt": datetime.now().isoformat(),
                    "createdBy": "R.W.Ϝ.Y.",
                    "version": "v1"
                }
            }
            
            batch.append(PointStruct(id=stable_int_id(slug), vector=vector, payload=entity))
            
            if len(batch) >= BATCH_SIZE:
                q_client.upsert(COLLECTION, points=batch)
                batch = []
                print(f"Ingested {count+1} rows...")
            
            count += 1
            time.sleep(0.1) # basic rate limit friendliness

        if batch:
            q_client.upsert(COLLECTION, points=batch)
        
        update_task_status(task_id, 'done', rows=count)
        print(f"✓ Task {task_id} completed. {count} rows ingested.")

    except Exception as e:
        print(f"Fatal error: {e}")
        update_task_status(task_id, 'error', error=str(e))

if __name__ == "__main__":
    main()

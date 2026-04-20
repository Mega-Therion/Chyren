#!/usr/bin/env python3
import os
import yaml
import hashlib
import psycopg2
import argparse
from datetime import datetime

MANIFEST_PATH = os.path.join(os.path.dirname(__file__), "../DATASET_MANIFEST.yaml")

def get_db_conn():
    db_url = os.environ.get("OMEGA_CATALOG_DB_URL")
    if not db_url:
        raise ValueError("OMEGA_CATALOG_DB_URL not set")
    return psycopg2.connect(db_url)

def compute_hash(repo, subset):
    return hashlib.sha256(f"{repo}|{subset}".encode()).hexdigest()

def replenish():
    if not os.path.exists(MANIFEST_PATH):
        print(f"Manifest not found at {MANIFEST_PATH}")
        return

    with open(MANIFEST_PATH, "r") as f:
        manifest = yaml.safe_load(f)

    conn = get_db_conn()
    cur = conn.cursor()

    for ds in manifest.get("datasets", []):
        source_hash = compute_hash(ds["hf_repo"], ds.get("hf_subset", ""))
        
        cur.execute("SELECT id FROM dataset_queue WHERE id = %s", (ds["id"],))
        if cur.fetchone():
            continue

        cur.execute("""
            INSERT INTO dataset_queue (id, hf_repo, hf_subset, hf_split, priority, row_limit, source_hash)
            VALUES (%s, %s, %s, %s, %s, %s, %s)
            ON CONFLICT (id) DO NOTHING
        """, (ds["id"], ds["hf_repo"], ds.get("hf_subset", ""), ds.get("hf_split", "train"), 
              ds.get("priority", 50), ds.get("row_limit", 50000), source_hash))
        print(f"Enqueued: {ds['id']}")

    conn.commit()
    cur.close()
    conn.close()

def status():
    conn = get_db_conn()
    cur = conn.cursor()
    cur.execute("SELECT id, status, priority, ingested_rows FROM dataset_queue ORDER BY priority ASC, queued_at ASC")
    rows = cur.fetchall()
    print(f"{'ID':<40} | {'STATUS':<12} | {'PRIO':<4} | {'ROWS':<8}")
    print("-" * 75)
    for r in rows:
        print(f"{r[0]:<40} | {r[1]:<12} | {r[2]:<4} | {r[3]:<8}")
    cur.close()
    conn.close()

def prioritize(dataset_id):
    conn = get_db_conn()
    cur = conn.cursor()
    cur.execute("UPDATE dataset_queue SET priority = 1 WHERE id = %s", (dataset_id,))
    conn.commit()
    cur.close()
    conn.close()
    print(f"Prioritized: {dataset_id}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--replenish", action="store_true")
    parser.add_argument("--status", action="store_true")
    parser.add_argument("--prioritize", type=str)
    args = parser.parse_args()

    if args.replenish: replenish()
    elif args.status: status()
    elif args.prioritize: prioritize(args.prioritize)
    else: parser.print_help()

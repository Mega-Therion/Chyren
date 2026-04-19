#!/usr/bin/env python3
"""
embed_knowledge_matrix.py
─────────────────────────
Generates vector embeddings for every domain in omega_knowledge_domains and
upserts them into the Qdrant 'knowledge_matrix' collection.

Each point stores:
  - vector: text-embedding-3-small embedding of (name + description + reasoning_primer)
  - payload: slug, name, realm, reasoning_mode, level, reasoning_primer

Run:
    source ~/.omega/one-true.env
    python3 cortex/ops/scripts/embed_knowledge_matrix.py
    python3 cortex/ops/scripts/embed_knowledge_matrix.py --dry-run   # no Qdrant writes
    python3 cortex/ops/scripts/embed_knowledge_matrix.py --rebuild    # drop + recreate collection

Env vars required:
    OMEGA_CATALOG_DB_URL  — Neon catalog Postgres URL
    OPENAI_API_KEY        — for text-embedding-3-small
    QDRANT_URL            — Qdrant HTTP endpoint (default: http://localhost:6333)
"""

import argparse
import os
import sys
import time
from typing import Optional

import psycopg2
import psycopg2.extras
import openai
import urllib.parse

# ─── Config ──────────────────────────────────────────────────────────────────

COLLECTION     = "knowledge_matrix"
EMBEDDING_MODEL = "text-embedding-3-small"
VECTOR_DIM     = 1536
BATCH_SIZE     = 64     # rows to embed per API call
UPSERT_BATCH   = 128    # points to upsert per Qdrant call

# ─── DB ──────────────────────────────────────────────────────────────────────

def _sanitize_db_url(url: str) -> str:
    """Strip libpq params not supported by psycopg2 (e.g. channel_binding)."""
    parsed = urllib.parse.urlparse(url)
    allowed = {"sslmode", "sslrootcert", "sslcert", "sslkey", "connect_timeout", "application_name"}
    qs = {k: v for k, v in urllib.parse.parse_qs(parsed.query).items() if k in allowed}
    clean = parsed._replace(query=urllib.parse.urlencode(qs, doseq=True))
    return urllib.parse.urlunparse(clean)


def fetch_all_domains(conn) -> list[dict]:
    sql = """
        SELECT slug, name, realm, reasoning_mode, level,
               COALESCE(description, '') AS description,
               COALESCE(purpose, '')     AS purpose,
               COALESCE(reasoning_primer, '') AS reasoning_primer,
               COALESCE(core_axioms::text, '[]') AS core_axioms_raw
        FROM omega_knowledge_domains
        ORDER BY level ASC, sort_order ASC
    """
    with conn.cursor(cursor_factory=psycopg2.extras.DictCursor) as cur:
        cur.execute(sql)
        return [dict(row) for row in cur.fetchall()]


# ─── Embedding ────────────────────────────────────────────────────────────────

def build_embed_text(row: dict) -> str:
    """Combine the most semantically rich fields into a single string for embedding."""
    parts = [
        row["name"],
        row["description"],
        row["purpose"],
        row["reasoning_primer"],
    ]
    return " | ".join(p for p in parts if p).strip()


def embed_batch(client: openai.OpenAI, texts: list[str]) -> list[list[float]]:
    resp = client.embeddings.create(model=EMBEDDING_MODEL, input=texts)
    return [item.embedding for item in resp.data]


# ─── Qdrant ───────────────────────────────────────────────────────────────────

def get_qdrant_client():
    try:
        from qdrant_client import QdrantClient
        from qdrant_client.models import VectorParams, Distance
        url = os.environ.get("QDRANT_URL", "http://localhost:6333")
        return QdrantClient(url=url), VectorParams, Distance
    except ImportError:
        print("ERROR: qdrant-client not installed. Run: pip install qdrant-client")
        sys.exit(1)


def ensure_collection(client, VectorParams, Distance, rebuild: bool = False):
    from qdrant_client.models import VectorParams as VP, Distance as D
    existing = {c.name for c in client.get_collections().collections}
    if rebuild and COLLECTION in existing:
        print(f"  Dropping existing collection '{COLLECTION}'...")
        client.delete_collection(COLLECTION)
        existing.discard(COLLECTION)

    if COLLECTION not in existing:
        print(f"  Creating collection '{COLLECTION}' (dim={VECTOR_DIM}, cosine)...")
        client.create_collection(
            collection_name=COLLECTION,
            vectors_config=VectorParams(size=VECTOR_DIM, distance=Distance.COSINE),
        )
    else:
        print(f"  Collection '{COLLECTION}' already exists — upserting.")


def upsert_points(client, points: list[dict]):
    from qdrant_client.models import PointStruct
    structs = [
        PointStruct(id=p["id"], vector=p["vector"], payload=p["payload"])
        for p in points
    ]
    client.upsert(collection_name=COLLECTION, points=structs)


# ─── Main ─────────────────────────────────────────────────────────────────────

def stable_int_id(slug: str) -> int:
    """Map a slug to a stable uint64 via hash (Qdrant requires integer or UUID ids)."""
    import hashlib
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF


def main():
    parser = argparse.ArgumentParser(description="Embed knowledge matrix domains into Qdrant")
    parser.add_argument("--dry-run", action="store_true", help="Embed but skip Qdrant upsert")
    parser.add_argument("--rebuild", action="store_true", help="Drop and recreate Qdrant collection")
    args = parser.parse_args()

    # ── Env checks ──────────────────────────────────────────────────────────
    db_url = os.environ.get("OMEGA_CATALOG_DB_URL", "")
    openai_key = os.environ.get("OPENAI_API_KEY", "")
    if not db_url:
        print("ERROR: OMEGA_CATALOG_DB_URL not set"); sys.exit(1)
    if not openai_key:
        print("ERROR: OPENAI_API_KEY not set"); sys.exit(1)

    # ── Fetch domains ────────────────────────────────────────────────────────
    print("Connecting to catalog DB...")
    conn = psycopg2.connect(_sanitize_db_url(db_url))
    domains = fetch_all_domains(conn)
    conn.close()
    print(f"  Loaded {len(domains)} domains from omega_knowledge_domains")

    # ── Build embed texts ────────────────────────────────────────────────────
    texts = [build_embed_text(d) for d in domains]

    if args.dry_run:
        print("[dry-run] First 3 embed texts:")
        for t in texts[:3]:
            print(f"  · {t[:120]}")
        print(f"[dry-run] Would embed {len(texts)} domains — skipping API and Qdrant writes.")
        return

    # ── Embed in batches ─────────────────────────────────────────────────────
    print(f"Embedding {len(texts)} domains via {EMBEDDING_MODEL}...")
    oai = openai.OpenAI(api_key=openai_key)
    all_vectors: list[list[float]] = []

    for i in range(0, len(texts), BATCH_SIZE):
        batch_texts = texts[i : i + BATCH_SIZE]
        vecs = embed_batch(oai, batch_texts)
        all_vectors.extend(vecs)
        print(f"  Embedded {min(i + BATCH_SIZE, len(texts))}/{len(texts)}", end="\r")
        if i + BATCH_SIZE < len(texts):
            time.sleep(0.3)  # respect rate limits

    print(f"\n  Done — {len(all_vectors)} vectors generated")

    # ── Qdrant upsert ────────────────────────────────────────────────────────
    client, VectorParams, Distance = get_qdrant_client()
    ensure_collection(client, VectorParams, Distance, rebuild=args.rebuild)

    points = []
    for domain, vector in zip(domains, all_vectors):
        points.append({
            "id": stable_int_id(domain["slug"]),
            "vector": vector,
            "payload": {
                "slug":            domain["slug"],
                "name":            domain["name"],
                "realm":           domain["realm"],
                "reasoning_mode":  domain["reasoning_mode"],
                "level":           domain["level"],
                "reasoning_primer": domain["reasoning_primer"],
                "description":     domain["description"],
            },
        })

    print(f"Upserting {len(points)} points into Qdrant collection '{COLLECTION}'...")
    n_upserted = 0
    for i in range(0, len(points), UPSERT_BATCH):
        batch = points[i : i + UPSERT_BATCH]
        upsert_points(client, batch)
        n_upserted += len(batch)
        print(f"  {n_upserted}/{len(points)} upserted", end="\r")

    print(f"\n✓ Knowledge matrix embedded: {n_upserted} domain vectors in Qdrant '{COLLECTION}'")
    print("  Query with: semantic_knowledge_search(query_text) → top-k domain slugs")


if __name__ == "__main__":
    main()

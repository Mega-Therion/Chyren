#!/usr/bin/env python3
"""
embed_knowledge_matrix.py
─────────────────────────
Generates vector embeddings for every domain in omega_knowledge_domains and
upserts them into the Qdrant 'knowledge_matrix' collection.

Provider priority: Gemini gemini-embedding-001 (dim=3072) → OpenAI text-embedding-3-small (dim=1536)

Run:
    source ~/.omega/one-true.env
    QDRANT_URL=http://localhost:6333 python3 cortex/ops/scripts/embed_knowledge_matrix.py
    python3 cortex/ops/scripts/embed_knowledge_matrix.py --dry-run
    python3 cortex/ops/scripts/embed_knowledge_matrix.py --rebuild

Env vars:
    OMEGA_CATALOG_DB_URL  — Neon catalog Postgres URL (required)
    GEMINI_API_KEY        — preferred embedding provider
    OPENAI_API_KEY        — fallback embedding provider
    QDRANT_URL            — Qdrant HTTP endpoint (default: http://localhost:6333)
"""

import argparse
import hashlib
import os
import sys
import time
import urllib.parse
import urllib.request

import psycopg2
import psycopg2.extras

# ─── Config ──────────────────────────────────────────────────────────────────

COLLECTION   = "knowledge_matrix"
UPSERT_BATCH = 128

GEMINI_MODEL = "models/gemini-embedding-001"
GEMINI_DIM   = 3072

OPENAI_MODEL = "text-embedding-3-small"
OPENAI_DIM   = 1536

# ─── DB ──────────────────────────────────────────────────────────────────────

def _sanitize_db_url(url: str) -> str:
    parsed = urllib.parse.urlparse(url)
    allowed = {"sslmode", "sslrootcert", "sslcert", "sslkey", "connect_timeout", "application_name"}
    qs = {k: v for k, v in urllib.parse.parse_qs(parsed.query).items() if k in allowed}
    return urllib.parse.urlunparse(parsed._replace(query=urllib.parse.urlencode(qs, doseq=True)))


def fetch_all_domains(conn) -> list[dict]:
    sql = """
        SELECT slug, name, realm, reasoning_mode, level,
               COALESCE(description, '')      AS description,
               COALESCE(purpose, '')          AS purpose,
               COALESCE(reasoning_primer, '') AS reasoning_primer
        FROM omega_knowledge_domains
        ORDER BY level ASC, sort_order ASC
    """
    with conn.cursor(cursor_factory=psycopg2.extras.DictCursor) as cur:
        cur.execute(sql)
        return [dict(row) for row in cur.fetchall()]


# ─── Embedding providers ──────────────────────────────────────────────────────

def build_embed_text(row: dict) -> str:
    parts = [row["name"], row["description"], row["purpose"], row["reasoning_primer"]]
    return " | ".join(p for p in parts if p).strip()


def embed_gemini(texts: list[str], api_key: str, start: int = 0) -> list[list[float]]:
    """Embed one text at a time via Gemini with exponential backoff on 429."""
    import json as _json
    vectors: list[list[float]] = []
    base = f"https://generativelanguage.googleapis.com/v1beta/{GEMINI_MODEL}:embedContent?key={api_key}"
    total = len(texts)
    for i, text in enumerate(texts):
        body = {"model": GEMINI_MODEL, "content": {"parts": [{"text": text}]}}
        req = urllib.request.Request(
            base,
            data=_json.dumps(body).encode(),
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        backoff = 5
        while True:
            try:
                with urllib.request.urlopen(req, timeout=20) as r:
                    data = _json.loads(r.read())
                vectors.append(data["embedding"]["values"])
                break
            except urllib.error.HTTPError as e:
                if e.code == 429:
                    print(f"\n  Rate limited at {start + i + 1}/{total} — waiting {backoff}s...", end="\r")
                    time.sleep(backoff)
                    backoff = min(backoff * 2, 60)
                else:
                    raise
        print(f"  Embedded {start + i + 1}/{total}", end="\r")
        time.sleep(0.1)
    return vectors


def embed_openai(texts: list[str], api_key: str) -> list[list[float]]:
    import openai as _openai
    client = _openai.OpenAI(api_key=api_key)
    batch_size = 64
    vectors: list[list[float]] = []
    for i in range(0, len(texts), batch_size):
        batch = texts[i : i + batch_size]
        resp = client.embeddings.create(model=OPENAI_MODEL, input=batch)
        vectors.extend(item.embedding for item in resp.data)
        print(f"  Embedded {min(i + batch_size, len(texts))}/{len(texts)}", end="\r")
        if i + batch_size < len(texts):
            time.sleep(0.3)
    return vectors


# ─── Qdrant ───────────────────────────────────────────────────────────────────

def get_qdrant_client():
    try:
        from qdrant_client import QdrantClient
        from qdrant_client.models import Distance, VectorParams
        url = os.environ.get("QDRANT_URL", "http://localhost:6333")
        return QdrantClient(url=url), VectorParams, Distance
    except ImportError:
        print("ERROR: qdrant-client not installed. Run: pip install qdrant-client")
        sys.exit(1)


def ensure_collection(client, VectorParams, Distance, vector_dim: int, rebuild: bool = False):
    existing = {c.name for c in client.get_collections().collections}
    if rebuild and COLLECTION in existing:
        print(f"  Dropping existing collection '{COLLECTION}'...")
        client.delete_collection(COLLECTION)
        existing.discard(COLLECTION)
    if COLLECTION not in existing:
        print(f"  Creating collection '{COLLECTION}' (dim={vector_dim}, cosine)...")
        client.create_collection(
            collection_name=COLLECTION,
            vectors_config=VectorParams(size=vector_dim, distance=Distance.COSINE),
        )
    else:
        print(f"  Collection '{COLLECTION}' exists — upserting.")


def upsert_points(client, points: list[dict]):
    from qdrant_client.models import PointStruct
    client.upsert(
        collection_name=COLLECTION,
        points=[PointStruct(id=p["id"], vector=p["vector"], payload=p["payload"]) for p in points],
    )


def stable_int_id(slug: str) -> int:
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF


# ─── Main ─────────────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(description="Embed knowledge matrix into Qdrant")
    parser.add_argument("--dry-run", action="store_true", help="Skip API calls and Qdrant writes")
    parser.add_argument("--rebuild", action="store_true", help="Drop and recreate Qdrant collection")
    args = parser.parse_args()

    db_url      = os.environ.get("OMEGA_CATALOG_DB_URL", "")
    gemini_key  = os.environ.get("GEMINI_API_KEY", "")
    openai_key  = os.environ.get("OPENAI_API_KEY", "")

    if not db_url:
        print("ERROR: OMEGA_CATALOG_DB_URL not set"); sys.exit(1)
    if not gemini_key and not openai_key:
        print("ERROR: Set GEMINI_API_KEY or OPENAI_API_KEY"); sys.exit(1)

    provider  = "gemini"  if gemini_key  else "openai"
    vector_dim = GEMINI_DIM if provider == "gemini" else OPENAI_DIM

    print(f"Embedding provider: {provider} (dim={vector_dim})")

    # ── Fetch domains ────────────────────────────────────────────────────────
    print("Connecting to catalog DB...")
    conn = psycopg2.connect(_sanitize_db_url(db_url))
    domains = fetch_all_domains(conn)
    conn.close()
    print(f"  Loaded {len(domains)} domains")

    texts = [build_embed_text(d) for d in domains]

    if args.dry_run:
        print("[dry-run] First 3 embed texts:")
        for t in texts[:3]:
            print(f"  · {t[:120]}")
        print(f"[dry-run] Would embed {len(texts)} domains — skipping writes.")
        return

    # ── Check existing points in Qdrant (resume support) ─────────────────────
    client, VectorParams, Distance = get_qdrant_client()
    ensure_collection(client, VectorParams, Distance, vector_dim, rebuild=args.rebuild)

    existing_ids = set()
    try:
        offset = None
        while True:
            scroll_result = client.scroll(
                collection_name=COLLECTION, limit=256, offset=offset, with_payload=False, with_vectors=False
            )
            batch, offset = scroll_result
            existing_ids.update(p.id for p in batch)
            if offset is None:
                break
    except Exception:
        pass

    pending_domains = [d for d in domains if stable_int_id(d["slug"]) not in existing_ids]
    pending_texts   = [build_embed_text(d) for d in pending_domains]

    if not pending_domains:
        print(f"✓ All {len(domains)} domains already embedded in Qdrant — nothing to do.")
        return

    skipped = len(domains) - len(pending_domains)
    if skipped:
        print(f"  Resuming: {skipped} already embedded, {len(pending_domains)} remaining")

    # ── Embed ────────────────────────────────────────────────────────────────
    print(f"Embedding {len(pending_texts)} domains...")
    if provider == "gemini":
        all_vectors = embed_gemini(pending_texts, gemini_key, start=skipped)
    else:
        all_vectors = embed_openai(pending_texts, openai_key)
    print(f"\n  Done — {len(all_vectors)} vectors")

    # ── Qdrant upsert ────────────────────────────────────────────────────────
    points = [
        {
            "id": stable_int_id(d["slug"]),
            "vector": vec,
            "payload": {
                "slug":            d["slug"],
                "name":            d["name"],
                "realm":           d["realm"],
                "reasoning_mode":  d["reasoning_mode"],
                "level":           d["level"],
                "reasoning_primer": d["reasoning_primer"],
                "description":     d["description"],
            },
        }
        for d, vec in zip(pending_domains, all_vectors)
    ]

    n = 0
    for i in range(0, len(points), UPSERT_BATCH):
        upsert_points(client, points[i : i + UPSERT_BATCH])
        n += len(points[i : i + UPSERT_BATCH])
        print(f"  {n}/{len(points)} upserted", end="\r")

    print(f"\n✓ {n} domain vectors in Qdrant '{COLLECTION}' (dim={vector_dim})")


if __name__ == "__main__":
    main()

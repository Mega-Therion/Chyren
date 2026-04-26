"""
ingest_chyren_entries.py — Stream chyren_memory_entries into Neon + Supabase
Processes in small batches to stay within memory limits.

Run from repo root:
  python cortex/ops/scripts/ingest_chyren_entries.py
"""

import json, re, uuid, urllib.request, sys
from pathlib import Path
import psycopg2
from psycopg2.extras import execute_values

POOL = json.load(open(Path(__file__).parents[2] / "ops/db_pool.json"))
NEON_URL = POOL["active_primary"]
SUPA_BASE = "https://eletftuboucrsrnapqoq.supabase.co/rest/v1"
SUPA_KEY = next(p["service_key"] for p in POOL["pool"] if p["id"] == "supabase_sovereign")

BIOGRAPHY = Path("/home/mega/Chyren/archives/CHYREN_WORKSPACE/BRAIN/biography")
DOCS = Path("/home/mega/Chyren/archives/CHYREN_WORKSPACE/DOCS")
CONSOLIDATED = Path("/home/mega/Chyren/archives/CHYREN_WORKSPACE/BRAIN/raw/CHYREN_DATA_CONSOLIDATED")

CHUNK_SIZE = 900
BATCH_SIZE = 100
SUPA_MAX = 2000  # cap Supabase inserts to avoid rate limits

# Track counts
neon_total = 0
supa_total = 0
supa_done = False  # flip when we hit SUPA_MAX


def supa_upsert(rows):
    global supa_total, supa_done
    if supa_done or not rows:
        return 0
    payload = json.dumps(rows).encode()
    req = urllib.request.Request(
        f"{SUPA_BASE}/chyren_memory_entries",
        data=payload,
        headers={
            "apikey": SUPA_KEY,
            "Authorization": f"Bearer {SUPA_KEY}",
            "Content-Type": "application/json",
            "Prefer": "resolution=merge-duplicates,return=minimal",
        },
        method="POST",
    )
    try:
        urllib.request.urlopen(req, timeout=30)
        supa_total += len(rows)
        if supa_total >= SUPA_MAX:
            supa_done = True
        return len(rows)
    except urllib.error.HTTPError as e:
        print(f"  [supa] error {e.code}: {e.read().decode()[:150]}", flush=True)
        return 0


def neon_upsert(rows):
    global neon_total
    if not rows:
        return
    conn = psycopg2.connect(NEON_URL)
    cur = conn.cursor()
    execute_values(
        cur,
        """
        INSERT INTO chyren_memory_entries
            (id, content, source, importance, namespace, confidence, domain, version, created_at)
        VALUES %s
        ON CONFLICT (id) DO NOTHING
        """,
        [
            (
                r["id"], r["content"], r["source"], r["importance"],
                r.get("namespace", "general"), 1.0,
                r.get("domain", "general"), r.get("version", 1),
                r.get("created_at", ""),
            )
            for r in rows
        ],
    )
    conn.commit()
    conn.close()
    neon_total += len(rows)


def make_row(path, source_label, namespace, domain, importance, created_at, chunk_idx, chunk_text):
    return {
        "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"chyren.{source_label}.{path.stem}.{chunk_idx}")),
        "content": chunk_text.strip(),
        "source": source_label,
        "importance": importance,
        "namespace": namespace,
        "domain": domain,
        "version": 1,
        "created_at": created_at,
    }


def process_file(path, source_label, namespace, domain, importance, created_at):
    """Yields rows for a single file, streaming chunks. Reads at most 12KB."""
    try:
        # Skip files over 50MB to avoid memory issues (e.g. CLI history logs)
        if path.stat().st_size > 50 * 1024 * 1024:
            return
        # Read only first 12000 bytes to avoid loading huge files
        with open(path, "r", errors="replace") as fh:
            text = fh.read(12000)
        if text.count("\x00") > 10 or len(text.strip()) < 100:
            return
        for j, i in enumerate(range(0, len(text), CHUNK_SIZE)):
            chunk = text[i:i + CHUNK_SIZE]
            if len(chunk.strip()) < 80:
                continue
            yield make_row(path, source_label, namespace, domain, importance, created_at, j, chunk)
    except Exception:
        pass


def flush_batch(batch):
    if not batch:
        return
    neon_upsert(batch)
    supa_upsert(batch)


def process_sources():
    batch = []
    seen_ids = set()

    def add(row):
        if row["id"] in seen_ids:
            return
        seen_ids.add(row["id"])
        batch.append(row)
        if len(batch) >= BATCH_SIZE:
            flush_batch(batch[:])
            batch.clear()

    # 1. Sovereign biography (highest importance)
    sov_bio = BIOGRAPHY / "RY_Sovereign_Biography"
    if sov_bio.exists():
        count = 0
        for md in sov_bio.rglob("*.md"):
            for row in process_file(md, f"sovereign_bio/{md.stem}", "biography", "identity", 0.9, "2026-03-28"):
                add(row)
                count += 1
        print(f"  sovereign_bio: {count} chunks", flush=True)

    # 2. RYography
    ryography = BIOGRAPHY / "RYography"
    if ryography.exists():
        count = 0
        for md in sorted(ryography.rglob("*.md"))[:30]:
            for row in process_file(md, f"ryography/{md.stem}", "biography", "history", 0.85, "2026-02-01"):
                add(row)
                count += 1
        print(f"  ryography: {count} chunks", flush=True)

    # 3. Key DOCS
    key_docs = [
        ("WHITE_PAPER_PUBLICATION_DRAFT.md", "philosophy"),
        ("TELOS.md", "philosophy"),
        ("MISSION_CURRENT.md", "philosophy"),
        ("Book_of_Sovereign_Frustrations.md", "memoir"),
        ("Official_Biography_RY.md", "biography"),
        ("Alye_History_References.md", "history"),
        ("MAP.md", "architecture"),
        ("STATUS.md", "ops"),
        ("AGENTS.md", "architecture"),
    ]
    count = 0
    for name, domain in key_docs:
        p = DOCS / name
        if p.exists():
            for row in process_file(p, f"docs/{name}", "canon", domain, 0.85, "2026-03-15"):
                add(row)
                count += 1
    print(f"  key_docs: {count} chunks", flush=True)

    # 4. DOCS/docs subdirectory
    docs_sub = DOCS / "docs"
    if docs_sub.exists():
        count = 0
        for md in sorted(docs_sub.glob("*.md"))[:40]:
            for row in process_file(md, f"docs/docs/{md.stem}", "architecture", "system", 0.75, "2026-02-01"):
                add(row)
                count += 1
        print(f"  docs/docs: {count} chunks", flush=True)

    # 5. Creative docs from CONSOLIDATED (non-conversation files)
    count = 0
    for f in sorted(CONSOLIDATED.glob("*.md")):
        text = f.read_text(errors="replace")
        if re.search(r"You said:\s*\n", text):
            continue  # skip conversation files — already in memories table
        for row in process_file(f, f"consolidated_doc/{f.stem}", "corpus", "creative", 0.75, "2026-03-01"):
            add(row)
            count += 1
    print(f"  consolidated_docs: {count} chunks", flush=True)

    # Final flush
    flush_batch(batch[:])
    batch.clear()


if __name__ == "__main__":
    print("=" * 50)
    print("Chyren Memory Entries Ingestion")
    print("=" * 50)
    process_sources()
    print(f"\nNeon total:    {neon_total} rows")
    print(f"Supabase total: {supa_total} rows (capped at {SUPA_MAX})")
    print("Done.")

"""Library Index Card (LIC) registrar.

Scans every shard listed in cortex/ops/db_pool.json, generates an index card
per (shard, table) pair, and UPSERTs them into the master omega_library_catalog
table on the catalog host.

Idempotent: running it twice does not create duplicates (UNIQUE constraint
on (shard_id, shelf_table, subject_domain) drives ON CONFLICT … DO UPDATE).

Usage:
    python cortex/ops/scripts/catalog_registrar.py
    python cortex/ops/scripts/catalog_registrar.py --dry-run
    python cortex/ops/scripts/catalog_registrar.py --only neon_technical

Env requirements (from ~/.omega/one-true.env):
    OMEGA_CATALOG_DB_URL                 — master catalog host (required)
    <shard.url_env>                      — per Neon shard
    <shard.postgres_url_env>             — per Supabase shard
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

import psycopg2
import psycopg2.extras

REPO_DIR = Path(__file__).resolve().parents[3]
POOL_PATH = REPO_DIR / "cortex" / "ops" / "db_pool.json"

SYSTEM_SCHEMAS = frozenset(
    {"pg_catalog", "information_schema", "pg_toast", "neon_auth", "auth", "storage", "extensions", "graphql", "graphql_public", "pgsodium", "pgsodium_masks", "realtime", "supabase_functions", "vault"}
)


@dataclass
class Shard:
    id: str
    platform: str
    role: str
    conn_url: str | None
    raw: dict


@dataclass
class IndexCard:
    shard_id: str
    platform: str
    shelf_table: str
    subject_domain: str
    summary: str
    keywords: list[str] = field(default_factory=list)
    row_count_estimate: int | None = None


# ─── Pool loading ─────────────────────────────────────────────────────────────


def load_pool() -> dict:
    with POOL_PATH.open() as f:
        return json.load(f)


def resolve_shards(pool: dict, only: str | None = None) -> list[Shard]:
    shards: list[Shard] = []
    for platform, section in pool.get("platforms", {}).items():
        for raw in section.get("shards", []):
            shard_id = raw["id"]
            if only and shard_id != only:
                continue

            url_env = raw.get("postgres_url_env") or raw.get("url_env")
            if not url_env or url_env.startswith("TODO_"):
                conn_url = None
            else:
                conn_url = os.environ.get(url_env)

            shards.append(
                Shard(
                    id=shard_id,
                    platform=platform,
                    role=raw.get("role", "unknown"),
                    conn_url=conn_url,
                    raw=raw,
                )
            )
    return shards


# ─── Per-shard scan ───────────────────────────────────────────────────────────


def list_tables(conn) -> list[tuple[str, str, int | None]]:
    """Return (schema, table, estimated_rows) for non-system tables."""
    schemas_csv = ",".join(f"'{s}'" for s in SYSTEM_SCHEMAS)
    sql = f"""
        SELECT n.nspname AS schema,
               c.relname AS table,
               c.reltuples::BIGINT AS rows_est
        FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE c.relkind IN ('r', 'p')
          AND n.nspname NOT IN ({schemas_csv})
          AND n.nspname NOT LIKE 'pg_%%'
        ORDER BY n.nspname, c.relname
    """
    with conn.cursor() as cur:
        cur.execute(sql)
        return cur.fetchall()


def column_names(conn, schema: str, table: str) -> list[str]:
    sql = """
        SELECT column_name
        FROM information_schema.columns
        WHERE table_schema = %s AND table_name = %s
        ORDER BY ordinal_position
    """
    with conn.cursor() as cur:
        cur.execute(sql, (schema, table))
        return [r[0] for r in cur.fetchall()]


# ─── Heuristic card synthesis ─────────────────────────────────────────────────


SUBJECT_HINTS: dict[str, str] = {
    "family":      "biographical",
    "profile":     "biographical",
    "memory":      "memory",
    "memories":    "memory",
    "history":     "temporal_lore",
    "log":         "operational_logs",
    "event":       "operational_logs",
    "knowledge":   "public_knowledge",
    "phylactery":  "identity",
    "identity":    "identity",
    "catalog":     "meta_catalog",
    "library":     "meta_catalog",
    "pet":         "pet_registry",
    "animal":      "pet_registry",
    "thought":     "cognition",
    "ledger":      "ledger",
    "session":     "session_state",
    "auth":        "auth",
    "user":        "user_directory",
    "embedding":   "vector_index",
}


def infer_subject(table: str) -> str:
    lower = table.lower()
    for token, domain in SUBJECT_HINTS.items():
        if token in lower:
            return domain
    return "uncategorized"


def synthesize_summary(table: str, columns: Iterable[str], row_count: int | None) -> str:
    cols = list(columns)
    sample_cols = ", ".join(cols[:8]) + ("…" if len(cols) > 8 else "")
    rows = f"~{row_count:,} rows" if row_count and row_count > 0 else "size unknown"
    return f"Table '{table}' ({rows}). Columns: {sample_cols}."


def keywords_for(table: str, columns: Iterable[str]) -> list[str]:
    base = {table.lower()}
    base.update(c.lower() for c in columns if len(c) > 2 and c.lower() not in ("id", "created_at", "updated_at"))
    return sorted(base)[:24]


# ─── Catalog UPSERT ───────────────────────────────────────────────────────────


UPSERT_SQL = """
INSERT INTO omega_library_catalog
    (shard_id, platform, shelf_table, subject_domain, summary, keywords, row_count_estimate, last_indexed_at)
VALUES
    (%(shard_id)s, %(platform)s, %(shelf_table)s, %(subject_domain)s, %(summary)s, %(keywords)s::jsonb, %(row_count_estimate)s, now())
ON CONFLICT (shard_id, shelf_table, subject_domain) DO UPDATE SET
    summary            = EXCLUDED.summary,
    keywords           = EXCLUDED.keywords,
    row_count_estimate = EXCLUDED.row_count_estimate,
    last_indexed_at    = now();
"""


def write_cards(catalog_url: str, cards: list[IndexCard]) -> int:
    if not cards:
        return 0
    with psycopg2.connect(catalog_url) as conn:
        with conn.cursor() as cur:
            for card in cards:
                cur.execute(UPSERT_SQL, {
                    "shard_id":           card.shard_id,
                    "platform":           card.platform,
                    "shelf_table":        card.shelf_table,
                    "subject_domain":     card.subject_domain,
                    "summary":            card.summary,
                    "keywords":           json.dumps(card.keywords),
                    "row_count_estimate": card.row_count_estimate,
                })
        conn.commit()
    return len(cards)


# ─── Per-platform handlers ────────────────────────────────────────────────────


def _sanitize_pg_url(url: str) -> str:
    """Strip non-libpq query params (Vercel-style Supabase URLs include
    e.g. `supa=base-pooler.x`, `pgbouncer=true`) that psycopg2 rejects."""
    from urllib.parse import urlsplit, urlunsplit, parse_qsl, urlencode

    LIBPQ_PARAMS = {
        "host", "hostaddr", "port", "dbname", "user", "password", "passfile",
        "channel_binding", "connect_timeout", "client_encoding", "options",
        "application_name", "fallback_application_name", "keepalives",
        "keepalives_idle", "keepalives_interval", "keepalives_count",
        "tcp_user_timeout", "replication", "gssencmode", "sslmode", "sslcompression",
        "sslcert", "sslkey", "sslpassword", "sslrootcert", "sslcrl", "sslcrldir",
        "sslsni", "requirepeer", "ssl_min_protocol_version", "ssl_max_protocol_version",
        "krbsrvname", "gsslib", "service", "target_session_attrs",
    }
    parts = urlsplit(url)
    kept = [(k, v) for k, v in parse_qsl(parts.query, keep_blank_values=True) if k in LIBPQ_PARAMS]
    return urlunsplit((parts.scheme, parts.netloc, parts.path, urlencode(kept), parts.fragment))


def scan_postgres_shard(shard: Shard) -> list[IndexCard]:
    """Works for Neon and Supabase (both Postgres)."""
    if not shard.conn_url:
        print(f"  ⚠  {shard.id}: no conn URL (env var unset or TODO) — skipped")
        return []

    safe_url = _sanitize_pg_url(shard.conn_url)
    cards: list[IndexCard] = []
    try:
        with psycopg2.connect(safe_url, connect_timeout=10) as conn:
            tables = list_tables(conn)
            for schema, table, rows_est in tables:
                cols = column_names(conn, schema, table)
                cards.append(IndexCard(
                    shard_id=shard.id,
                    platform=shard.platform,
                    shelf_table=f"{schema}.{table}" if schema != "public" else table,
                    subject_domain=infer_subject(table),
                    summary=synthesize_summary(table, cols, rows_est),
                    keywords=keywords_for(table, cols),
                    row_count_estimate=int(rows_est) if rows_est else None,
                ))
    except (psycopg2.OperationalError, psycopg2.ProgrammingError, psycopg2.DatabaseError) as e:
        msg = str(e).strip().splitlines()[0] if str(e).strip() else type(e).__name__
        print(f"  ✗ {shard.id}: {msg}")
        return []
    print(f"  ✓ {shard.id} ({shard.platform}): {len(cards)} cards")
    return cards


def scan_firebase_shard(shard: Shard) -> list[IndexCard]:
    print(f"  ⏭  {shard.id} (firebase): adapter not implemented — skipped")
    return []


def scan_cloudflare_shard(shard: Shard) -> list[IndexCard]:
    print(f"  ⏭  {shard.id} (cloudflare): adapter not implemented — skipped")
    return []


SCANNERS = {
    "neon":       scan_postgres_shard,
    "supabase":   scan_postgres_shard,
    "firebase":   scan_firebase_shard,
    "cloudflare": scan_cloudflare_shard,
}


# ─── Entry point ──────────────────────────────────────────────────────────────


def main() -> int:
    ap = argparse.ArgumentParser(description="Library Index Card registrar")
    ap.add_argument("--dry-run", action="store_true", help="Scan but do not write to catalog")
    ap.add_argument("--only", help="Limit to a single shard ID")
    args = ap.parse_args()

    catalog_url = os.environ.get("OMEGA_CATALOG_DB_URL")
    if not catalog_url and not args.dry_run:
        print("✗ OMEGA_CATALOG_DB_URL not set. Source ~/.omega/one-true.env first.", file=sys.stderr)
        return 2

    pool = load_pool()
    shards = resolve_shards(pool, only=args.only)
    print(f"→ Scanning {len(shards)} shard(s) across "
          f"{len({s.platform for s in shards})} platform(s)")

    all_cards: list[IndexCard] = []
    for shard in shards:
        scanner = SCANNERS.get(shard.platform, lambda s: [])
        all_cards.extend(scanner(shard))

    print(f"→ Generated {len(all_cards)} index card(s)")
    if args.dry_run:
        for c in all_cards[:5]:
            print(f"   • [{c.shard_id}/{c.shelf_table}] {c.subject_domain}: {c.summary[:80]}")
        if len(all_cards) > 5:
            print(f"   …and {len(all_cards) - 5} more")
        return 0

    written = write_cards(catalog_url, all_cards)
    print(f"✓ UPSERTed {written} card(s) into omega_library_catalog")
    return 0


if __name__ == "__main__":
    sys.exit(main())

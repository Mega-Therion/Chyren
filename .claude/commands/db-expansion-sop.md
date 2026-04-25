---
name: db-expansion-sop
description: "SOP for horizontal DB expansion when any Neon DB hits 90% capacity. Never pay — create a new free Neon project, enable pgvector, assign namespaces, update the registry, and chain it to the query layer. This is the OmegA standard for infinite free storage scaling."
triggers:
  - "database is full"
  - "db at 90%"
  - "storage limit"
  - "neon is full"
  - "db expansion"
  - "out of space"
  - "512mb"
  - "db health"
---

# SOP: OmegA Database Expansion

**Rule #1:** We never pay for storage. We chain free Neon projects horizontally forever.

## When to Trigger

Run a health check before any large ingest:

```bash
/home/mega/OMEGA_WORKSPACE/INGEST/venv/bin/python /home/mega/OMEGA_WORKSPACE/INGEST/db_registry.py
```

- **< 85%** → proceed normally
- **85–90%** → watch — auto-prune on next run
- **≥ 90%** → guard() auto-prunes, then continues
- **≥ 99%** → guard() raises — EXPAND NOW

## Quick Prune (Try This First)

```python
import psycopg2
conn = psycopg2.connect(URI)
conn.autocommit = False
cur = conn.cursor()
cur.execute("DELETE FROM public.omega_memory_entries WHERE importance <= 0.5")
print(f"Deleted {cur.rowcount} entries")
conn.commit()
conn.autocommit = True
cur.execute("VACUUM FULL public.omega_memory_entries")
```

If still at 99%+ after pruning → expand.

## Expansion Procedure

### 1. Create New Neon Project
Ask Claude (has Neon MCP): *"Create a new Neon project called omega-db{N}"*
Or via CLI: `neonctl projects create --name omega-db3`

### 2. Initialize Schema
```python
conn = psycopg2.connect(NEW_URI)
cur = conn.cursor()
cur.execute("CREATE EXTENSION IF NOT EXISTS vector")
cur.execute("""
CREATE TABLE IF NOT EXISTS public.omega_memory_entries (
    id TEXT PRIMARY KEY, content TEXT, embedding vector(384),
    source TEXT, namespace TEXT, domain TEXT, importance FLOAT,
    created_at TEXT, version INTEGER, confidence FLOAT
)""")
conn.commit()
```

### 3. Save to Vault
```bash
echo "OMEGA_DB3_URL='postgresql://...'" >> /home/mega/OMEGA_WORKSPACE/VAULT/one-true.env
```

### 4. Register in db_registry.py
Edit `/home/mega/OMEGA_WORKSPACE/INGEST/db_registry.py` — add to `DB_REGISTRY`:
```python
'db3': {
    'uri': 'postgresql://...',
    'namespaces': {'new_namespace_1', 'new_namespace_2'},
    'label': 'omega-db3-theme',
    'capacity_mb': 512,
}
```

### 5. Update SOP Doc
Update the DB Registry table in:
`/home/mega/OMEGA_WORKSPACE/DOCS/SOP_DATABASE_EXPANSION.md`

### 6. Log to ERGON.md
```
[TIMESTAMP] [AGENT] DB expansion: created omega-db{N}. Namespaces: X, Y. Total free capacity: N GB.
```

## Current DB Map

| DB | Label | Namespaces |
|---|---|---|
| DB1 | omega-personal-db | personal, social_*, omega_brain |
| DB2 | omega-technical-db | omega_architecture, omega_docs, omega_code, system, skills |

**Full registry:** `/home/mega/OMEGA_WORKSPACE/INGEST/db_registry.py`
**Full SOP:** `/home/mega/OMEGA_WORKSPACE/DOCS/SOP_DATABASE_EXPANSION.md`

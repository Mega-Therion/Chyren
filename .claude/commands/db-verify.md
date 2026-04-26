# /db-verify — Database Health & Schema Verification

You are the database reliability engineer. Verify that the Neon PostgreSQL and Qdrant stores are healthy and schema-correct.

## PostgreSQL (Master Ledger)

```bash
source ~/.chyren/one-true.env

# Connectivity
psql "$CHYREN_DB_URL" -c "SELECT version();" 2>&1

# Table inventory
psql "$CHYREN_DB_URL" -c "\dt" 2>&1

# Ledger row count and recency
psql "$CHYREN_DB_URL" -c "
SELECT
  COUNT(*) as total_entries,
  MAX(created_at) as latest_entry,
  MIN(created_at) as oldest_entry,
  AVG(adccl_score) as avg_adccl_score
FROM ledger;" 2>&1

# Index health
psql "$CHYREN_DB_URL" -c "SELECT indexname, indexdef FROM pg_indexes WHERE tablename='ledger';" 2>&1

# Connection pool status
psql "$CHYREN_DB_URL" -c "SELECT count(*) as connections, state FROM pg_stat_activity GROUP BY state;" 2>&1
```

## Qdrant (Semantic Memory)

```bash
# Collection inventory
curl -s "${QDRANT_URL}/collections" 2>&1

# Collection status
curl -s "${QDRANT_URL}/collections/myelin" 2>&1 || curl -s "${QDRANT_URL}/collections" 2>&1
```

## Schema Drift Check
Compare current schema against expected (read from the most recent migration file in the repo):
```bash
psql "$CHYREN_DB_URL" -c "\d+ ledger" 2>&1
```

## Output
Status per store: HEALTHY / DEGRADED / DOWN. For DEGRADED/DOWN: root cause and remediation steps.

$ARGUMENTS

# /ledger-verify — Master Ledger Integrity Verification

You are the ledger auditor. The Master Ledger is append-only and cryptographically signed — any tampering is a critical incident.

## Execution

**1. Check database connectivity:**
```bash
source ~/.chyren/one-true.env
psql "$CHYREN_DB_URL" -c "SELECT COUNT(*) as entry_count FROM ledger;" 2>&1 || echo "DB UNREACHABLE"
```

**2. Check for ledger gaps (non-sequential IDs):**
```bash
psql "$CHYREN_DB_URL" -c "
SELECT id, id - LAG(id) OVER (ORDER BY id) as gap
FROM ledger
WHERE id - LAG(id) OVER (ORDER BY id) > 1
LIMIT 20;" 2>&1
```

**3. Check for entries with null/missing signatures:**
```bash
psql "$CHYREN_DB_URL" -c "
SELECT id, created_at FROM ledger
WHERE signature IS NULL OR signature = ''
ORDER BY created_at DESC LIMIT 20;" 2>&1
```

**4. Check most recent entries:**
```bash
psql "$CHYREN_DB_URL" -c "
SELECT id, created_at, task_type, adccl_score
FROM ledger
ORDER BY created_at DESC LIMIT 10;" 2>&1
```

**5. Local state files:**
```bash
ls -la state/ 2>/dev/null
find state/ -size 0 -exec echo "ZERO-BYTE: {}" \;
find state/ -newer state/$(ls state/ 2>/dev/null | head -1) 2>/dev/null | tail -10
```

## Findings
- Gaps in ID sequence → potential deletion — CRITICAL, do not auto-fix, alert user
- Null signatures → integrity failure — CRITICAL
- Unreachable DB → connectivity issue — HIGH, check `CHYREN_DB_URL` env
- Zero-byte state files → corruption risk — HIGH, do not delete, alert user

## Rules
- NEVER delete ledger entries
- NEVER modify existing rows
- NEVER auto-fix CRITICAL findings — present and wait for user decision

$ARGUMENTS

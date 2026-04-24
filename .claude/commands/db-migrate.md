# /db-migrate — Database Migration Workflow

You are the database engineer for Chyren OS. Migrations to the Master Ledger are irreversible — execute with extreme care.

## Migration Description
$ARGUMENTS

## Protocol

**Step 1 — Branch (Neon)**
Always run migrations on a Neon branch first, never directly on main:
```bash
source ~/.omega/one-true.env
# Use Neon MCP or CLI to create a branch
# neon branches create --name migration-$(date +%Y%m%d)
```
Use the `mcp__Neon__create_branch` tool to create a safe branch.

**Step 2 — Write the migration SQL**
- Use `CREATE TABLE IF NOT EXISTS`, `ALTER TABLE ... ADD COLUMN IF NOT EXISTS`
- Never `DROP TABLE`, `DROP COLUMN`, or `TRUNCATE` without explicit user confirmation
- Ledger table is sacred — additions only, no modifications to existing columns

**Step 3 — Test on branch**
```bash
psql "$BRANCH_DB_URL" -f migration.sql 2>&1
# Verify schema
psql "$BRANCH_DB_URL" -c "\d ledger" 2>&1
```

**Step 4 — Run integration test**
```bash
OMEGA_DB_URL="$BRANCH_DB_URL" cargo test --package omega-myelin 2>&1
```

**Step 5 — Apply to main (with confirmation)**
Show the user the exact SQL and the exact connection string target. Wait for explicit "yes" before running.

**Step 6 — Verify**
```bash
psql "$OMEGA_DB_URL" -c "\d+" 2>&1
```

## Neon Quota SOP
If Neon quota error occurs: immediately create a new Neon project via `mcp__Neon__create_project`, initialize the schema, update the connection pool, and update `OMEGA_DB_URL` in `~/.omega/one-true.env`. Do not wait or retry the existing project.

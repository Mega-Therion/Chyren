# /neon — Neon PostgreSQL Operations

You are the Neon database operator for Chyren OS. All operations use the Neon MCP tools.

## Action
$ARGUMENTS

## Available Operations

**Check current project status:**
Use `mcp__Neon__list_projects` then `mcp__Neon__describe_project` for the active project.

**Create a branch for safe migration:**
Use `mcp__Neon__create_branch` — always branch before running migrations.

**Run SQL safely:**
Use `mcp__Neon__run_sql` for queries. Use `mcp__Neon__run_sql_transaction` for multi-statement migrations.

**Inspect schema:**
Use `mcp__Neon__get_database_tables` then `mcp__Neon__describe_table_schema` for the ledger table.

**Compare schemas (before/after migration):**
Use `mcp__Neon__compare_database_schema`

**Quota overflow SOP (CRITICAL):**
If quota error occurs:
1. `mcp__Neon__create_project` — new project immediately
2. Initialize schema on new project
3. Update `CHYREN_DB_URL` in `~/.chyren/one-true.env`
4. Verify connectivity: `psql "$CHYREN_DB_URL" -c "SELECT 1;"`
Do not wait or retry the old project.

**Slow query analysis:**
Use `mcp__Neon__list_slow_queries` — optimize any query over 100ms on the ledger table.

## Ledger Safety Rules
- Never `DROP` or `TRUNCATE` the ledger table
- Never `UPDATE` or `DELETE` ledger rows
- All schema changes go to a branch first, then main after test verification

## Output
Operation result, affected rows/schema, and connection string of modified database.

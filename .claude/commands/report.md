# /report — System Status Report

You are the status reporter for Chyren OS. Generate a concise, accurate status report of the current system state and development progress.

## Scope
$ARGUMENTS (e.g. `daily`, `pre-deploy`, `incident`, `weekly`)

## Data Collection

**Git status:**
```bash
git log --oneline -10
git diff main...HEAD --stat 2>/dev/null || git diff --stat HEAD~3..HEAD
git branch --show-current
```

**Build status:**
```bash
source ~/.omega/one-true.env
cd medulla && cargo check --workspace 2>&1 | tail -5
```

**Test status:**
```bash
cd medulla && cargo test --workspace --quiet 2>&1 | tail -10
PYTHONPATH=cortex pytest tests/ -q 2>&1 | tail -5
```

**Database:**
```bash
psql "$OMEGA_DB_URL" -c "SELECT COUNT(*) as entries, MAX(created_at) as latest FROM ledger;" 2>&1
```

## Report Format

```
## Chyren OS Status Report — [DATE]

### Build Health
- Rust: [PASS/FAIL]
- Python: [PASS/FAIL]
- Web: [PASS/FAIL]
- Gateway: [PASS/FAIL]

### Recent Changes
[last 5 commits with one-line summaries]

### Database
- Ledger entries: [N]
- Last entry: [timestamp]
- Qdrant: [ONLINE/OFFLINE]

### Open Issues
[from Linear if accessible]

### Blockers / Action Items
[specific items requiring attention]
```

## Optional: Post to Slack
If `$ARGUMENTS` contains `slack`, use `mcp__claude_ai_Slack__slack_send_message` to post the report to the appropriate channel.

# /chyren-status — Full Chyren System Status

Query the live status of all Chyren OS subsystems from inside a Claude Code session.

## Execution

```bash
source ~/.chyren/one-true.env

echo "=== Brain Stem ==="
./chyren status 2>&1

echo "=== API Server ==="
curl -s http://localhost:8080/health 2>&1 || echo "API server: offline"

echo "=== Ledger ==="
psql "$CHYREN_DB_URL" -c "SELECT COUNT(*) as entries, MAX(created_at) as latest FROM ledger;" 2>&1

echo "=== Qdrant ==="
curl -s "${QDRANT_URL}/health" 2>&1 || echo "Qdrant: offline or QDRANT_URL not set"

echo "=== Phylactery ==="
python3 -c "import json; d=json.load(open('cortex/chyren_py/phylactery_kernel.json')); print(f'entries: {len(d) if isinstance(d,list) else \"dict\"}')" 2>&1

echo "=== Claude Code Bridge ==="
claude --version 2>&1
```

## Output
Status per subsystem. Flag any OFFLINE or ERROR states. For offline subsystems, suggest the specific recovery action (e.g. `./chyren live` for API server, `./chyren dream` for phylactery refresh).

$ARGUMENTS

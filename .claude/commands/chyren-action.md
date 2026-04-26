# /chyren-action — Execute a Chyren Action

Route an execution task through Chyren's action pipeline from inside a Claude Code session.
Actions involve memory writes, sharding, and ingestion in addition to the thought pipeline.

## Task
$ARGUMENTS

## Execution

```bash
source ~/.chyren/one-true.env
./chyren action "$ARGUMENTS" 2>&1
```

Direct API:
```bash
curl -s -X POST http://localhost:8080/action \
  -H "Content-Type: application/json" \
  -d "{\"input\": \"$ARGUMENTS\"}" | python3 -m json.tool
```

## Output
Display the full action result including any memory writes to Qdrant and the ledger commit.

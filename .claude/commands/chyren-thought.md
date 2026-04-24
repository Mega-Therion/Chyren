# /chyren-thought — Send a Thought to Chyren's Sovereign Pipeline

Route a task through the full Chyren conductor pipeline from inside a Claude Code session.
Pipeline: Alignment → AEON → Provider → ADCCL (0.7) → Master Ledger.

## Task
$ARGUMENTS

## Execution

```bash
source ~/.omega/one-true.env
./chyren thought "$ARGUMENTS" 2>&1
```

If the API server is running, you can also call it directly for JSON output:
```bash
curl -s -X POST http://localhost:8080/thought \
  -H "Content-Type: application/json" \
  -d "{\"input\": \"$ARGUMENTS\"}" | python3 -m json.tool
```

## Output
Display the full pipeline response including ADCCL score and ledger commit status.
If ADCCL rejects the response (score < 0.7), report the rejection flags and do not retry automatically — surface the failure for user decision.

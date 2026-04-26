# /trace — Pipeline Execution Trace

You are a systems engineer tracing a request through the full Chyren OS pipeline. Used to understand what actually happens when a command is executed.

## Request to Trace
$ARGUMENTS

## Trace Path

The canonical pipeline (as implemented in `chyren-conductor`):

```
CLI / API Request
    → chyren-cli (parse + validate)
    → chyren-aegis (policy + security gate)
    → chyren-aeon (temporal scheduling)
    → chyren-spokes (provider selection + injection)
    → Provider API call (with Yettragrammaton hash + ledger context)
    → Provider response
    → chyren-adccl (drift/hallucination scoring, threshold 0.7)
    → chyren-phylactery (identity verification)
    → chyren-myelin (semantic memory write, Qdrant)
    → Master Ledger (PostgreSQL append + cryptographic sign)
    → Response returned to caller
```

## For Each Stage, Answer:
1. What is the input to this stage?
2. What validation or transformation occurs?
3. What can cause this stage to reject/fail?
4. Where is this implemented? (file:line)

## Execution
Read the relevant source files in order. Start with `chyren-conductor/src/` to find the orchestration entry point, then follow the call chain. Use `grep` to find function definitions across crates.

```bash
grep -rn "pub fn conduct\|pub async fn run\|pub fn execute" medulla/chyren-conductor/src/ 2>/dev/null
```

## Output
A numbered trace with: stage name, file:line of entry point, key logic summary, failure modes.

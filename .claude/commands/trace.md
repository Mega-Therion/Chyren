# /trace — Pipeline Execution Trace

You are a systems engineer tracing a request through the full Chyren OS pipeline. Used to understand what actually happens when a command is executed.

## Request to Trace
$ARGUMENTS

## Trace Path

The canonical pipeline (as implemented in `omega-conductor`):

```
CLI / API Request
    → omega-cli (parse + validate)
    → omega-aegis (policy + security gate)
    → omega-aeon (temporal scheduling)
    → omega-spokes (provider selection + injection)
    → Provider API call (with Yettragrammaton hash + ledger context)
    → Provider response
    → omega-adccl (drift/hallucination scoring, threshold 0.7)
    → omega-phylactery (identity verification)
    → omega-myelin (semantic memory write, Qdrant)
    → Master Ledger (PostgreSQL append + cryptographic sign)
    → Response returned to caller
```

## For Each Stage, Answer:
1. What is the input to this stage?
2. What validation or transformation occurs?
3. What can cause this stage to reject/fail?
4. Where is this implemented? (file:line)

## Execution
Read the relevant source files in order. Start with `omega-conductor/src/` to find the orchestration entry point, then follow the call chain. Use `grep` to find function definitions across crates.

```bash
grep -rn "pub fn conduct\|pub async fn run\|pub fn execute" medulla/omega-conductor/src/ 2>/dev/null
```

## Output
A numbered trace with: stage name, file:line of entry point, key logic summary, failure modes.

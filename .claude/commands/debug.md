# /debug — Systematic Debugging Session

You are a principal engineer running a disciplined debugging session. No guessing. Hypothesize, instrument, verify.

## Problem Statement
$ARGUMENTS

## Debugging Protocol

**Phase 1 — Reproduce**
Get the exact error: full stack trace, panic message, or wrong output. Run:
```bash
source ~/.omega/one-true.env
cd medulla && cargo test --package <relevant-crate> -- <test_name> --nocapture 2>&1
# or
./chyren <command> 2>&1
```
If you can't reproduce it, stop and ask the user for reproduction steps before proceeding.

**Phase 2 — Localize**
Read the stack trace top-to-bottom. Identify the exact file and line where the invariant breaks. Read that file. Read its callers. Do not read more than needed.

**Phase 3 — Hypothesize**
State exactly one hypothesis: "The bug is X because Y." Do not state multiple hypotheses simultaneously.

**Phase 4 — Instrument**
Add targeted `tracing::debug!()` or `eprintln!()` (temporary) at the hypothesis point. Re-run.

**Phase 5 — Verify & Fix**
- If hypothesis confirmed: apply the minimum fix. Remove all instrumentation. Re-run tests.
- If hypothesis wrong: eliminate it, form next hypothesis, repeat from Phase 4.

**Phase 6 — Regression**
Add a test that would have caught this bug. The test goes in the same crate, in the `#[cfg(test)]` block nearest to the fix.

## Rules
- Never apply a fix you haven't verified
- Never skip Phase 6
- If the bug is in `omega-adccl` scoring: do not adjust the 0.7 threshold — fix the scorer logic

$ARGUMENTS

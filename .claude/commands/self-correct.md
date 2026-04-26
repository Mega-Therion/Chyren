# /self-correct — Self-Correction & Accountability Pass

You are the self-correction engine. Review the last set of changes made in this session and verify they are correct, complete, and consistent — catch your own mistakes before they propagate.

## What to Verify
$ARGUMENTS (specific change to verify, or empty for full session review)

## Self-Correction Protocol

**1. What did we change?**
```bash
git diff HEAD~3..HEAD --name-only 2>/dev/null || git diff HEAD~1..HEAD --name-only
git diff HEAD~3..HEAD 2>/dev/null || git diff HEAD~1..HEAD
```

**2. Does it compile?**
```bash
source ~/.chyren/one-true.env
cd medulla && cargo check --workspace 2>&1
```

**3. Do tests pass?**
```bash
cd medulla && cargo test --workspace --quiet 2>&1 | tail -10
PYTHONPATH=cortex pytest tests/ -q 2>&1 | tail -5
```

**4. Invariant check — did we violate any non-negotiables?**
- ADCCL threshold still 0.7? `grep -r "0\.7\|threshold" medulla/chyren-adccl/src/`
- No direct logging? `grep -rn "println!\|eprintln!" medulla/chyren-*/src/ --include="*.rs" | grep -v test`
- No ledger deletions? `grep -rn "DELETE.*ledger\|DROP.*ledger" . 2>/dev/null`
- Ledger writes still signed? `grep -rn "sign\|signature" medulla/chyren-phylactery/src/ 2>/dev/null | head -10`

**5. Are there TODOs or incomplete implementations left?**
```bash
grep -rn "TODO\|FIXME\|todo!()\|unimplemented!()" medulla/chyren-*/src/ cortex/ 2>/dev/null | grep -v "#\[cfg(test)\]" | head -20
```

**6. Did we leave debug instrumentation?**
```bash
grep -rn "eprintln!\|dbg!\|println!" medulla/chyren-*/src/ 2>/dev/null | grep -v "#\[cfg(test)\]" | head -10
```

## Output
Verdict per check: CLEAN / ISSUE. For each issue: description, file:line, and the fix applied or proposed.

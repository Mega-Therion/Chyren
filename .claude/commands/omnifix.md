# /omnifix — Find and Fix All Issues

You are the omnifix agent. Systematically find every broken, degraded, or dangerous thing in the current state of Chyren OS and fix what can be safely fixed.

## Scope
$ARGUMENTS (if empty: full system scan)

## Discovery Pass

**1. Compile errors:**
```bash
source ~/.chyren/one-true.env && cd medulla && cargo check --workspace 2>&1 | grep "^error"
```

**2. Test failures:**
```bash
cd medulla && cargo test --workspace 2>&1 | grep "FAILED"
PYTHONPATH=cortex pytest tests/ -q 2>&1 | grep "FAILED\|ERROR"
```

**3. Lint violations:**
```bash
cd medulla && cargo clippy --workspace -- -D warnings 2>&1 | grep "^error\|^warning"
```

**4. Type errors:**
```bash
cd web && npm run typecheck 2>&1 | grep "error TS"
```

**5. Security issues:**
```bash
grep -rn "unwrap()\|expect(" medulla/chyren-*/src/*.rs 2>/dev/null | grep -v "#\[cfg(test)\]" | grep -v "//.*unwrap" | head -20
grep -rn "println!\|eprintln!" medulla/chyren-*/src/*.rs 2>/dev/null | grep -v "#\[cfg(test)\]" | head -10
```

## Fix Prioritization
Fix in this order:
1. **CRITICAL**: compile errors (system won't build)
2. **HIGH**: test failures (regressions)
3. **HIGH**: security violations (unwrap in prod paths, direct logging)
4. **MEDIUM**: lint/clippy warnings
5. **LOW**: type warnings, dead code

## Fix Protocol
For each issue:
- State the exact fix
- Apply it
- Re-run the relevant check to confirm fix
- Do NOT fix multiple issues in one edit if they're in different files — fix and verify each

## Output
Issues found (N), issues fixed (N), issues requiring manual attention (N) with reasons.

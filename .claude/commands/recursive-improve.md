# /recursive-improve — Recursive Self-Improvement Loop

You are the recursive improvement engine for Chyren OS. Identify the highest-leverage improvement, implement it, verify it, then identify the next one. Repeat until the system is measurably better or you hit a decision point requiring human input.

## Improvement Seed
$ARGUMENTS (area of focus, or empty for autonomous selection)

## Loop Protocol

### Iteration N

**1. Measure current state:**
```bash
source ~/.omega/one-true.env
cd medulla && cargo test --workspace --quiet 2>&1 | tail -5
cd medulla && cargo clippy --workspace -- -D warnings 2>&1 | grep "^warning\|^error" | wc -l
PYTHONPATH=cortex pytest tests/ -q 2>&1 | tail -3
psql "$OMEGA_DB_URL" -c "SELECT COUNT(*) FROM ledger;" 2>&1
```

**2. Identify the highest-leverage improvement:**
Priority order:
1. Failing tests (correctness)
2. Security violations (safety)
3. Clippy errors (code quality)
4. Missing test coverage on critical paths
5. Performance bottlenecks in the pipeline
6. Incomplete stub crate implementations (`omega-cim`, `omega-ternary`, `omega-vision`)
7. Missing documentation on public APIs

**3. Implement the improvement:**
Apply the minimum change. No scope creep.

**4. Verify:**
```bash
cargo test --package <affected-crate> 2>&1 | tail -5
```

**5. Record the delta:**
State: what was the metric before, what is it after.

**6. Halt conditions (stop the loop and ask the user):**
- Any change that touches the ledger, ADCCL threshold, or Yettragrammaton signing
- Any change that requires a DB migration
- Any change that modifies the public API of `omega-core` or `omega-conductor`
- After 5 successful iterations (checkpoint with user)
- When the next improvement requires architectural decisions

**7. Return to step 1**

## Output per Iteration
```
## Iteration N
**Improvement**: [what was fixed]
**File**: [file:line]
**Before**: [metric]
**After**: [metric]
**Next**: [what the next improvement will be]
```

# /self-audit — Autonomous Self-Audit

You are the autonomous self-audit agent. Run a complete, unprompted audit of Chyren OS across all dimensions and produce a prioritized remediation plan.

## Audit Scope
$ARGUMENTS (defaults to full system)

## Audit Dimensions & Execution

### 1. Code Correctness
```bash
source ~/.chyren/one-true.env
cd medulla && cargo test --workspace 2>&1 | grep -E "FAILED|passed|failed"
PYTHONPATH=cortex pytest tests/ -q 2>&1 | tail -5
```

### 2. Build Integrity
```bash
cd medulla && cargo build --workspace 2>&1 | grep "^error"
cd web && npm run typecheck 2>&1 | grep "error TS" | wc -l
```

### 3. Security Posture
```bash
# unwrap() in prod code
grep -rn "\.unwrap()\|\.expect(" medulla/chyren-*/src/*.rs 2>/dev/null | grep -v "#\[cfg(test)\]" | wc -l

# direct logging
grep -rn "println!\|eprintln!" medulla/chyren-*/src/*.rs 2>/dev/null | grep -v test | wc -l

# dependency vulnerabilities
cd medulla && cargo audit 2>&1 | grep "Crate\|Warning\|error"
```

### 4. Architecture Invariants
```bash
# ADCCL threshold
grep -n "0\.7\|threshold" medulla/chyren-adccl/src/lib.rs 2>/dev/null | head -5

# Telemetry routing
grep -rn "println!\|log::" medulla/chyren-*/src/ 2>/dev/null | grep -v test | head -10
```

### 5. Data Layer
```bash
psql "$CHYREN_DB_URL" -c "SELECT COUNT(*), MAX(created_at) FROM ledger;" 2>&1
curl -s "${QDRANT_URL}/health" 2>&1
```

### 6. Coverage Gaps
```bash
for f in medulla/chyren-*/src/lib.rs; do
  crate=$(dirname $f | xargs dirname | xargs basename)
  has_tests=$(grep -l "#\[cfg(test)\]" medulla/$crate/src/*.rs 2>/dev/null | wc -l)
  echo "$crate: $has_tests test files"
done 2>/dev/null
```

### 7. TODO Debt
```bash
grep -rn "TODO\|FIXME\|todo!()\|unimplemented!()" medulla/ cortex/ web/app/ 2>/dev/null | grep -v ".git\|target\|node_modules" | wc -l
```

## Audit Report Format
```
## Self-Audit Report — [TIMESTAMP]

### Summary Scorecard
| Dimension | Score | Issues |
|-----------|-------|--------|
| Correctness | ✅/⚠️/❌ | N |
| Build | ... | ... |
| Security | ... | ... |
| Architecture | ... | ... |
| Data Layer | ... | ... |
| Coverage | ... | ... |
| TODO Debt | ... | ... |

### Remediation Plan (prioritized)
1. [CRITICAL] ...
2. [HIGH] ...
3. [MEDIUM] ...
```

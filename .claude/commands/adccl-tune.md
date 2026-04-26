# /adccl-tune — ADCCL Drift Detection Analysis & Calibration

You are the ADCCL (Adaptive Drift/Coherence/Capability Layer) specialist. Analyze scoring behavior and calibrate flag weights — never lower the 0.7 threshold.

## Current Configuration
Read the current ADCCL config:
```bash
grep -rn "threshold\|0\.7\|STUB_MARKERS\|RESPONSE_TOO_SHORT\|CAPABILITY_REFUSAL\|NO_TASK_WORD_OVERLAP\|calibrat" medulla/chyren-adccl/src/ 2>/dev/null
```

## Scoring Analysis
If recent ledger entries are available:
```bash
source ~/.chyren/one-true.env
psql "$CHYREN_DB_URL" -c "
SELECT
  adccl_score,
  rejection_flags,
  COUNT(*) as count,
  AVG(adccl_score) as avg_score
FROM ledger
WHERE created_at > NOW() - INTERVAL '24 hours'
GROUP BY adccl_score, rejection_flags
ORDER BY count DESC;" 2>&1
```

## Flag Weight Analysis
For each flag (`STUB_MARKERS_DETECTED`, `RESPONSE_TOO_SHORT`, `CAPABILITY_REFUSAL`, `NO_TASK_WORD_OVERLAP`):
1. What is its current weight contribution to the score?
2. What is the false-positive rate over the last 24h?
3. Should the weight increase or decrease?

## Calibration Rules
- **Threshold 0.7: immovable** — do not lower under any circumstances
- Calibration window: score starts at 0.1 (permissive) and tightens to 0.7 over 60 minutes per session
- Adjustable: individual flag weights, calibration ramp rate, session window duration
- Any change to flag weights requires: current score distribution analysis + before/after simulation

## Simulation
Before applying weight changes, simulate on the last N ledger entries:
```bash
psql "$CHYREN_DB_URL" -c "SELECT raw_response, adccl_score FROM ledger ORDER BY created_at DESC LIMIT 100;" 2>&1
```

## Output
Current config summary, score distribution stats, proposed changes (if any), simulation results, before/after acceptance rate.

$ARGUMENTS

# Proof Pack v0.2

Release-grade evidence pack for architecture and verification claims.

## Purpose
- Preserve reproducible gate outcomes per run.
- Track trend stability across multiple proof-pack executions.
- Separate implementation facts from novelty language.

## Contents
- `metrics.csv`: latest canonical metric snapshot.
- `raw/*-status.csv`: immutable run-level gate outcomes.
- `charts/`: both snapshot charts and multi-run trend charts.
- `sheets/proof_status_history.csv`: flattened history export for spreadsheet analysis.
- `GOOGLE_SHEETS_SCORECARD_FORMULAS.md`: ready-to-paste formulas for dashboard tabs.
- `summary.md`: interpretation rules and current posture.

## New in v0.2
- Historical trend charts from all known run status files:
  - `charts/verification-pass-rate-trend.svg`
  - `charts/step-duration-trend.svg`
  - `charts/run-stability-heatmap.svg`
- Formalized “release confidence” interpretation in `summary.md`.

## Regeneration
From repo root:

```bash
# Standard snapshot + gate execution
bash ops/scripts/run_proof_pack.sh

# Multi-run trend regeneration (from status history)
python3 ops/scripts/generate_proof_trends.py \
  --raw-dir docs/evidence/v0.2/raw \
  --output docs/evidence/v0.2/charts

# Export history for Google Sheets dashboard
python3 ops/scripts/export_proof_status_history.py \
  --raw-dir docs/evidence/v0.2/raw \
  --output docs/evidence/v0.2/sheets/proof_status_history.csv
```

## Discipline
- Keep status CSVs append-only and immutable once captured.
- Promote new versions (`v0.3`, `v0.4`) instead of rewriting history.
- Treat unsupported claims as hypotheses until mapped to reproducible artifacts.

# Proof Pack v0.1

This folder is the first structured evidence snapshot for Chyren.

## Purpose
- Record benchmark and validation outputs in a machine-readable format.
- Tie every reported metric to a concrete command and run ID.
- Make release claims auditable and reproducible.

## Files
- `metrics.csv`: canonical metric records for this snapshot.
- `summary.md`: human-readable interpretation of the recorded metrics.
- `charts/`: generated SVG charts from `metrics.csv`.

Current generated charts:
- `charts/all-metrics.svg`
- `charts/metrics-verification.svg`
- `charts/metrics-algorithm.svg`
- `charts/metrics-architecture.svg`
- `charts/metrics-performance.svg`

## Data Contract
Each metric row must include:
- `metric_name`
- `category`
- `value`
- `unit`
- `source_command`
- `run_id`
- `captured_at_utc`

## Regeneration Workflow
From repo root:

```bash
# Execute validation and benchmark commands, update metrics.csv, and regenerate charts
bash ops/scripts/run_proof_pack.sh
```

## Evidence Discipline
- Do not publish unverifiable claims in release notes.
- If a metric is estimated or pending, mark it explicitly in `summary.md`.
- Keep historical snapshots immutable; add new versions as `v0.2`, `v0.3`, etc.

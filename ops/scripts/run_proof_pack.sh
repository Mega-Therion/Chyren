#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT_DIR/docs/evidence/v0.1/raw"
RUN_ID="proof-$(date -u +%Y%m%dT%H%M%SZ)"
METRICS_CSV="$ROOT_DIR/docs/evidence/v0.1/metrics.csv"
CHARTS_DIR="$ROOT_DIR/docs/evidence/v0.1/charts"

mkdir -p "$OUT_DIR"

run_step() {
  local name="$1"
  local cmd="$2"
  local logfile="$OUT_DIR/${RUN_ID}-${name}.log"
  local started ended duration status

  started=$(date +%s)

  echo "[proof-pack] running: $name"
  echo "[proof-pack] command: $cmd"

  if bash -lc "$cmd" >"$logfile" 2>&1; then
    status="pass"
    echo "[proof-pack] $name: PASS (log: $logfile)"
  else
    status="fail"
    echo "[proof-pack] $name: FAIL (log: $logfile)"
  fi

  ended=$(date +%s)
  duration=$((ended - started))

  # Basic CSV-safe command serialization (replace commas to keep format simple).
  local csv_cmd
  csv_cmd="${cmd//,/;}"
  printf "%s,%s,%s,%s,%s\n" "$RUN_ID" "$name" "$status" "$duration" "$csv_cmd" >> "$OUT_DIR/${RUN_ID}-status.csv"
}

echo "run_id,step,status,duration_sec,source_command" > "$OUT_DIR/${RUN_ID}-status.csv"

run_step "rust_tests" "cd '$ROOT_DIR/medulla' && cargo test"
run_step "web_typecheck" "cd '$ROOT_DIR/web' && npm run typecheck"
run_step "web_lint" "cd '$ROOT_DIR/web' && npm run lint"
run_step "python_tests" "cd '$ROOT_DIR' && pytest tests"

python3 "$ROOT_DIR/ops/scripts/update_proof_metrics.py" \
  --status-file "$OUT_DIR/${RUN_ID}-status.csv" \
  --metrics-file "$METRICS_CSV" \
  --repo-root "$ROOT_DIR"

python3 "$ROOT_DIR/ops/scripts/generate_evidence_charts.py" \
  --input "$METRICS_CSV" \
  --output "$CHARTS_DIR"

echo "[proof-pack] completed."
echo "[proof-pack] status file: $OUT_DIR/${RUN_ID}-status.csv"
echo "[proof-pack] metrics file: $METRICS_CSV"

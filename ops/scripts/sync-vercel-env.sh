#!/usr/bin/env bash
# ops/scripts/sync-vercel-env.sh
# Pushes local environment secrets to Vercel production.
# Usage: sync-vercel-env.sh [--dry-run]
set -euo pipefail

DRY_RUN=0
for arg in "$@"; do
  if [[ "$arg" == "--dry-run" ]]; then
    DRY_RUN=1
  fi
done

ENV_FILE="${CHYREN_ENV_FILE:-$HOME/.omega/one-true.env}"

# ── Preflight: env file must exist ────────────────────────────────────────────
if [[ ! -f "$ENV_FILE" ]]; then
  echo "ERROR: env file not found at $ENV_FILE"
  echo "  Set CHYREN_ENV_FILE or create $HOME/.omega/one-true.env"
  exit 1
fi

# ── Preflight: vercel CLI ──────────────────────────────────────────────────────
if ! command -v vercel >/dev/null 2>&1; then
  echo "ERROR: vercel CLI not found. Install with: npm i -g vercel"
  exit 1
fi

# ── Preflight: required keys must all exist in env file ───────────────────────
KEYS=("OMEGA_DB_URL" "SUPABASE_URL" "SUPABASE_SERVICE_KEY" "ANTHROPIC_API_KEY" "GEMINI_API_KEY")
REQUIRED_KEYS=("ANTHROPIC_API_KEY" "GEMINI_API_KEY" "OMEGA_DB_URL")

MISSING=()
for req in "${REQUIRED_KEYS[@]}"; do
  val=$(grep "^${req}=" "$ENV_FILE" | cut -d '=' -f2- || true)
  if [[ -z "$val" ]]; then
    MISSING+=("$req")
  fi
done

if [[ "${#MISSING[@]}" -gt 0 ]]; then
  echo "ERROR: the following required vars are missing from $ENV_FILE:"
  for m in "${MISSING[@]}"; do
    echo "  - $m"
  done
  exit 1
fi

# ── Sync ───────────────────────────────────────────────────────────────────────
if [[ "$DRY_RUN" == "1" ]]; then
  echo "[DRY RUN] Would sync the following vars to Vercel production:"
else
  echo "Starting Chyren environment sync to Vercel..."
fi

for var in "${KEYS[@]}"; do
  val=$(grep "^${var}=" "$ENV_FILE" | cut -d '=' -f2- || true)

  if [[ -z "$val" ]]; then
    echo "  [SKIP] $var not found in $ENV_FILE"
    continue
  fi

  if [[ "$DRY_RUN" == "1" ]]; then
    masked="${val:0:6}…"
    echo "  [DRY-RUN] would sync $var (value: $masked)"
  else
    echo "  [SYNC] $var"
    echo "$val" | vercel env add "$var" production --force
  fi
done

if [[ "$DRY_RUN" == "1" ]]; then
  echo "[DRY RUN] No changes applied. Re-run without --dry-run to apply."
else
  echo "Sync complete."
fi

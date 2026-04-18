#!/usr/bin/env bash
# Push selected env vars from config/.omega/one-true.env into the linked Vercel project.
# Browser-visible vars stay NEXT_PUBLIC_*; server secrets that the web runtime needs are synced too.
# Usage: sync-vercel-env-from-one-true.sh [--dry-run]
set -euo pipefail

DRY_RUN=0
for arg in "$@"; do
  if [[ "$arg" == "--dry-run" ]]; then
    DRY_RUN=1
  fi
done

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$WEB_ROOT/../config/.omega/one-true.env}"
PROJECT_JSON="$WEB_ROOT/.vercel/project.json"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "ERROR: env file not found at $ENV_FILE"
  echo "  Set CHYREN_ENV_FILE or create $HOME/.omega/one-true.env"
  exit 1
fi

# Preflight: verify required keys are present in the env file
REQUIRED_SYNC_VARS=(ANTHROPIC_API_KEY OPENAI_API_KEY GEMINI_API_KEY OMEGA_DB_URL)
set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

MISSING_SYNC=()
for req in "${REQUIRED_SYNC_VARS[@]}"; do
  if [[ -z "${!req:-}" ]]; then
    MISSING_SYNC+=("$req")
  fi
done

if [[ "${#MISSING_SYNC[@]}" -gt 0 ]]; then
  echo "ERROR: the following required vars are missing from $ENV_FILE:"
  for m in "${MISSING_SYNC[@]}"; do
    echo "  - $m"
  done
  exit 1
fi

cd "$WEB_ROOT"

read_project_setting() {
  local field="$1"
  if [[ ! -f "$PROJECT_JSON" ]]; then
    return
  fi

  node -e '
    const fs = require("fs");
    const [file, field] = process.argv.slice(1);
    const obj = JSON.parse(fs.readFileSync(file, "utf8"));
    const value = field.split(".").reduce((acc, key) => acc?.[key], obj);
    if (typeof value === "string") process.stdout.write(value);
  ' "$PROJECT_JSON" "$field" 2>/dev/null || true
}

if command -v vercel >/dev/null 2>&1; then
  vercel_cmd=(vercel)
else
  vercel_cmd=(npx --yes vercel@latest)
fi

VERCEL_SCOPE="${VERCEL_SCOPE:-$(read_project_setting 'orgId')}"

sync_one() {
  local name="$1"
  # shellcheck disable=SC2086
  local val="${!name-}"
  if [[ -z "${val}" ]]; then
    echo "  [SKIP] $name (not set in $ENV_FILE)"
    return 0
  fi
  if [[ "$DRY_RUN" == "1" ]]; then
    local masked="${val:0:6}…"
    echo "  [DRY-RUN] would sync $name (value: $masked) → production + development"
    return 0
  fi
  echo "  [SYNC] $name"
  "${vercel_cmd[@]}" env add "$name" production --value "$val" --yes --force ${VERCEL_SCOPE:+--scope "$VERCEL_SCOPE"}
  "${vercel_cmd[@]}" env add "$name" development --value "$val" --yes --force ${VERCEL_SCOPE:+--scope "$VERCEL_SCOPE"}
  # Preview is branch-scoped on Vercel; use dashboard or: vercel env add NAME preview <branch>
}

sync_one NEXT_PUBLIC_API_BASE_URL
sync_one NEXT_PUBLIC_FIREBASE_API_KEY
sync_one NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN
sync_one NEXT_PUBLIC_FIREBASE_PROJECT_ID
sync_one NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET
sync_one NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID
sync_one NEXT_PUBLIC_FIREBASE_APP_ID
sync_one CHYREN_FAMILY_AUTH_CONFIG
sync_one GROQ_MODEL
sync_one GROQ_API_KEY
sync_one OPENAI_API_KEY
sync_one ANTHROPIC_API_KEY
sync_one GEMINI_API_KEY
sync_one GOOGLE_TTS_API_KEY
sync_one CRON_SECRET
sync_one OMEGA_DB_URL

if [[ "$DRY_RUN" == "1" ]]; then
  echo ""
  echo "[DRY RUN] No changes applied. Re-run without --dry-run to apply."
else
  echo ""
  echo "Done. Run '${vercel_cmd[*]} env ls' to verify."
fi

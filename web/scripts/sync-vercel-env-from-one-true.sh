#!/usr/bin/env bash
# Push selected env vars from config/.omega/one-true.env into the linked Vercel project.
# Browser-visible vars stay NEXT_PUBLIC_*; server secrets that the web runtime needs are synced too.
set -euo pipefail

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$WEB_ROOT/../config/.omega/one-true.env}"
PROJECT_JSON="$WEB_ROOT/.vercel/project.json"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE"
  exit 1
fi

set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

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
    echo "Skip $name (not set in $ENV_FILE)"
    return 0
  fi
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

echo "Done. Run '${vercel_cmd[*]} env ls' to verify."

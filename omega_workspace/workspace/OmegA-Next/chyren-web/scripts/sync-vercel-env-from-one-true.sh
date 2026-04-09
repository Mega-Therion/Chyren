#!/usr/bin/env bash
# Push selected env vars from ~/.omega/one-true.env into the linked Vercel project.
# Browser-visible vars stay NEXT_PUBLIC_*; server secrets that the web runtime needs are synced too.
set -euo pipefail

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$HOME/.omega/one-true.env}"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE"
  exit 1
fi

set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

cd "$WEB_ROOT"

sync_one() {
  local name="$1"
  # shellcheck disable=SC2086
  local val="${!name-}"
  if [[ -z "${val}" ]]; then
    echo "Skip $name (not set in $ENV_FILE)"
    return 0
  fi
  vercel env add "$name" production --value "$val" --yes --force
  vercel env add "$name" development --value "$val" --yes --force
  # Preview is branch-scoped on Vercel; use dashboard or: vercel env add NAME preview <branch>
}

sync_one NEXT_PUBLIC_API_BASE_URL
sync_one NEXT_PUBLIC_FIREBASE_API_KEY
sync_one NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN
sync_one NEXT_PUBLIC_FIREBASE_PROJECT_ID
sync_one NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET
sync_one NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID
sync_one NEXT_PUBLIC_FIREBASE_APP_ID
sync_one GROQ_API_KEY
sync_one CRON_SECRET

echo "Done. vercel env list"

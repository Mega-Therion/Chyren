#!/usr/bin/env bash
# Push selected public env vars from ~/.omega/one-true.env into the linked Vercel project.
# Only NEXT_PUBLIC_* belongs here (browser-visible). Server secrets: use Vercel env as Encrypted only.
set -euo pipefail

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO_ROOT="$(cd "$WEB_ROOT/../../../.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$HOME/.omega/one-true.env}"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE"
  exit 1
fi

set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

cd "$REPO_ROOT"

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

echo "Done. vercel env list"

#!/usr/bin/env bash
# Deploy chyren-web to Vercel with env from ~/.omega/one-true.env (override with CHYREN_ENV_FILE).
set -euo pipefail

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO_ROOT="$(cd "$WEB_ROOT/../../../.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$HOME/.omega/one-true.env}"

if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck source=/dev/null
  source "$ENV_FILE"
  set +a
  echo "Loaded environment from $ENV_FILE"
else
  echo "No env file at $ENV_FILE (optional); continuing with current shell env."
fi

# Project "Root Directory" on Vercel is omega_workspace/.../chyren-web — run CLI from Git root.
cd "$REPO_ROOT"
exec vercel "$@"

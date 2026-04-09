#!/usr/bin/env bash
# Deploy chyren-web to Vercel with env from ~/.omega/one-true.env (override with CHYREN_ENV_FILE).
set -euo pipefail

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
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

# Deploy from the Next.js app root so Vercel detects `package.json` correctly.
cd "$WEB_ROOT"
tmp_out="$(mktemp)"
trap 'rm -f "$tmp_out"' EXIT

# Use JSON output so we can warm the exact deployment URL.
# Prefer a local Vercel CLI if present; otherwise run it through npx.
if command -v vercel >/dev/null 2>&1; then
  vercel_cmd=(vercel)
else
  vercel_cmd=(npx --yes vercel@latest)
fi

"${vercel_cmd[@]}" deploy . "$@" --json | tee "$tmp_out"

# After a production deploy, warm the Neon context cache.
if [[ "$*" == *"--prod"* ]]; then
  deployed_url="$(node -e '
    const fs = require("fs");
    const raw = fs.readFileSync(process.argv[1], "utf8").trim();
    // Vercel prints a single JSON object at the end when --json is used.
    const obj = JSON.parse(raw);
    const url = obj?.deployment?.url;
    if (!url) process.exit(2);
    process.stdout.write(url);
  ' "$tmp_out" 2>/dev/null || true)"

  echo "Warming Neon context cache..."
  target="https://chyren-web.vercel.app"
  if [[ -n "${deployed_url:-}" ]]; then
    target="$deployed_url"
  fi
  curl -sf -X POST "${target}/api/cron/warm-context" \
    -H "Authorization: Bearer ${CRON_SECRET:-}" \
    -w "  HTTP:%{http_code} TIME:%{time_total}s\n" || echo "  (warmup skipped — CRON_SECRET not set or endpoint error)"
fi

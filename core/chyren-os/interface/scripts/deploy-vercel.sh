#!/usr/bin/env bash
# Deploy chyren-web to Vercel with env from config/.chyren/one-true.env (override with CHYREN_ENV_FILE).
# Usage: deploy-vercel.sh [--dry-run] [--prod] [extra vercel deploy flags...]
set -euo pipefail

# ── Flags ──────────────────────────────────────────────────────────────────────
DRY_RUN=0
PASSTHROUGH_ARGS=()
for arg in "$@"; do
  if [[ "$arg" == "--dry-run" ]]; then
    DRY_RUN=1
  else
    PASSTHROUGH_ARGS+=("$arg")
  fi
done

WEB_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENV_FILE="${CHYREN_ENV_FILE:-$WEB_ROOT/../config/.chyren/one-true.env}"
PROJECT_JSON="$WEB_ROOT/.vercel/project.json"
TEMP_ROOT_LINK=0
TMP_OUT=""
TMP_ENV=""

cleanup() {
  if [[ -n "$TMP_OUT" && -f "$TMP_OUT" ]]; then
    rm -f "$TMP_OUT"
  fi
  if [[ -n "$TMP_ENV" && -f "$TMP_ENV" ]]; then
    rm -f "$TMP_ENV"
  fi
  if [[ "$TEMP_ROOT_LINK" == "1" ]]; then
    rm -rf "$DEPLOY_CWD/.vercel"
  fi
}
trap cleanup EXIT

# ── Preflight: required env vars ──────────────────────────────────────────────
if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck source=/dev/null
  source "$ENV_FILE"
  set +a
  echo "Loaded environment from $ENV_FILE"
else
  echo "No env file at $ENV_FILE (optional); continuing with current shell env."
fi

REQUIRED_VARS=(ANTHROPIC_API_KEY OPENAI_API_KEY GEMINI_API_KEY CHYREN_DB_URL)
MISSING_VARS=()
for req in "${REQUIRED_VARS[@]}"; do
  if [[ -z "${!req:-}" ]]; then
    MISSING_VARS+=("$req")
  fi
done
if [[ "${#MISSING_VARS[@]}" -gt 0 ]]; then
  echo "ERROR: the following required env vars are not set:"
  for m in "${MISSING_VARS[@]}"; do
    echo "  - $m"
  done
  echo "  Source $ENV_FILE or set them in your shell."
  exit 1
fi

if [[ "$DRY_RUN" == "1" ]]; then
  echo "[DRY RUN] Preflight passed. Would deploy from $(pwd) to Vercel."
  echo "[DRY RUN] Pass --prod to promote to production."
  echo "[DRY RUN] No changes applied. Re-run without --dry-run to deploy."
  exit 0
fi

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

resolve_deploy_cwd() {
  local configured_root
  local candidate
  local parent

  configured_root="$(read_project_setting 'settings.rootDirectory')"
  if [[ -z "$configured_root" || "$configured_root" == "." ]]; then
    printf '%s\n' "$WEB_ROOT"
    return
  fi

  candidate="$WEB_ROOT"
  while true; do
    if node -e '
      const path = require("path");
      const candidate = process.argv[1];
      const configuredRoot = process.argv[2];
      const webRoot = process.argv[3];
      process.exit(path.resolve(candidate, configuredRoot) === webRoot ? 0 : 1);
    ' "$candidate" "$configured_root" "$WEB_ROOT"; then
      printf '%s\n' "$candidate"
      return
    fi

    parent="$(dirname "$candidate")"
    if [[ "$parent" == "$candidate" ]]; then
      printf '%s\n' "$WEB_ROOT"
      return
    fi
    candidate="$parent"
  done
}

DEPLOY_CWD="$(resolve_deploy_cwd)"
VERCEL_SCOPE="${VERCEL_SCOPE:-$(read_project_setting 'orgId')}"
cd "$DEPLOY_CWD"
echo "Deploying from $DEPLOY_CWD"

if [[ "$DEPLOY_CWD" != "$WEB_ROOT" ]]; then
  mkdir -p "$DEPLOY_CWD/.vercel"
  cp "$PROJECT_JSON" "$DEPLOY_CWD/.vercel/project.json"
  TEMP_ROOT_LINK=1
fi

TMP_OUT="$(mktemp)"

if command -v vercel >/dev/null 2>&1; then
  vercel_cmd=(vercel)
else
  vercel_cmd=(npx --yes vercel@latest)
fi

resolve_production_env_value() {
  local name="$1"
  TMP_ENV="$(mktemp)"

  if ! "${vercel_cmd[@]}" env pull "$TMP_ENV" --environment=production ${VERCEL_SCOPE:+--scope "$VERCEL_SCOPE"} >/dev/null 2>&1; then
    rm -f "$TMP_ENV"
    TMP_ENV=""
    return 1
  fi

  node -e '
    const fs = require("fs");
    const [file, key] = process.argv.slice(1);
    const lines = fs.readFileSync(file, "utf8").split(/\r?\n/);
    for (const line of lines) {
      if (!line || line.startsWith("#") || !line.includes("=")) continue;
      const idx = line.indexOf("=");
      const k = line.slice(0, idx);
      if (k !== key) continue;
      let v = line.slice(idx + 1);
      if ((v.startsWith('"') && v.endsWith('"')) || (v.startsWith("'") && v.endsWith("'"))) {
        v = v.slice(1, -1);
      }
      process.stdout.write(v);
      process.exit(0);
    }
    process.exit(1);
  ' "$TMP_ENV" "$name"
}

deploy_args=(deploy --json)
if [[ "$DEPLOY_CWD" == "$WEB_ROOT" ]]; then
  deploy_args+=(.)
fi
if [[ -n "$VERCEL_SCOPE" ]]; then
  deploy_args+=(--scope "$VERCEL_SCOPE")
fi
if [[ "${#PASSTHROUGH_ARGS[@]}" -gt 0 ]]; then
  deploy_args+=("${PASSTHROUGH_ARGS[@]}")
fi

"${vercel_cmd[@]}" "${deploy_args[@]}" | tee "$TMP_OUT"

# ── Post-deploy health check ───────────────────────────────────────────────────
if [[ " ${PASSTHROUGH_ARGS[*]:-} " == *"--prod"* ]]; then
  deployed_url="$(node -e '
    const fs = require("fs");
    const raw = fs.readFileSync(process.argv[1], "utf8").trim();
    const obj = JSON.parse(raw);
    const url = obj?.deployment?.url;
    if (!url) process.exit(2);
    process.stdout.write(url);
  ' "$TMP_OUT" 2>/dev/null || true)"

  echo "Warming Neon context cache..."
  target="https://chyren-web.vercel.app"

  warm_secret="$(resolve_production_env_value CRON_SECRET 2>/dev/null || true)"
  if [[ -z "$warm_secret" ]]; then
    warm_secret="${CRON_SECRET:-}"
  fi

  if [[ -z "$warm_secret" ]]; then
    echo "  (warmup skipped — unable to resolve production CRON_SECRET)"
  else
    curl -sf -X POST "${target}/api/cron/warm-context" \
      -H "Authorization: Bearer ${warm_secret}" \
      -w "  HTTP:%{http_code} TIME:%{time_total}s\n" || echo "  (warmup skipped — endpoint error)"
  fi

  # Health check: verify the deployed URL responds with HTTP 200
  echo "Running post-deploy health check..."
  HEALTH_URL="${target}/api/health"
  HTTP_CODE="$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "$HEALTH_URL" || echo "000")"
  if [[ "$HTTP_CODE" == "200" ]]; then
    echo "  [PASS] Health check passed — $HEALTH_URL returned HTTP $HTTP_CODE"
  elif [[ "$HTTP_CODE" == "000" ]]; then
    echo "  [WARN] Health check timed out or connection refused — $HEALTH_URL"
    echo "         Deploy may still be propagating. Check manually."
  else
    echo "  [WARN] Health check returned HTTP $HTTP_CODE — $HEALTH_URL"
    echo "         Deployment may have issues. Inspect Vercel dashboard."
  fi
fi

#!/usr/bin/env bash
# ops/scripts/preflight-check.sh
# Validates environment before any deploy or release.
# Exits with code 1 if any check fails.
set -uo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS_COUNT=0
FAIL_COUNT=0

pass() {
  echo -e "  ${GREEN}[PASS]${NC} $1"
  ((PASS_COUNT++)) || true
}

fail() {
  echo -e "  ${RED}[FAIL]${NC} $1"
  ((FAIL_COUNT++)) || true
}

warn() {
  echo -e "  ${YELLOW}[WARN]${NC} $1"
}

section() {
  echo ""
  echo "── $1"
}

# Load one-true.env if present
ENV_FILE="${CHYREN_ENV_FILE:-$HOME/.omega/one-true.env}"
if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck source=/dev/null
  source "$ENV_FILE"
  set +a
  echo "Loaded environment from $ENV_FILE"
else
  warn "No env file found at $ENV_FILE — checking current shell environment only"
fi

echo ""
echo "═══════════════════════════════════════"
echo "  Chyren Preflight Check"
echo "═══════════════════════════════════════"

# ── Required env vars ──────────────────────────────────────────────────────────
section "Required environment variables"

REQUIRED_VARS=(
  ANTHROPIC_API_KEY
  OPENAI_API_KEY
  DEEPSEEK_API_KEY
  GEMINI_API_KEY
  OMEGA_DB_URL
  QDRANT_URL
)

for var in "${REQUIRED_VARS[@]}"; do
  if [[ -n "${!var:-}" ]]; then
    # Mask the value for display
    val="${!var}"
    masked="${val:0:6}…"
    pass "$var is set ($masked)"
  else
    fail "$var is NOT set"
  fi
done

# ── Rust toolchain ─────────────────────────────────────────────────────────────
section "Rust toolchain"

if command -v rustc >/dev/null 2>&1; then
  RUST_VERSION="$(rustc --version 2>&1)"
  pass "rustc found: $RUST_VERSION"
else
  fail "rustc not found — install from https://rustup.rs"
fi

if command -v cargo >/dev/null 2>&1; then
  CARGO_VERSION="$(cargo --version 2>&1)"
  pass "cargo found: $CARGO_VERSION"
else
  fail "cargo not found — install from https://rustup.rs"
fi

# ── Node.js ────────────────────────────────────────────────────────────────────
section "Node.js (requires 18+)"

if command -v node >/dev/null 2>&1; then
  NODE_VERSION="$(node --version 2>&1)"
  # Extract major version number
  NODE_MAJOR="${NODE_VERSION#v}"
  NODE_MAJOR="${NODE_MAJOR%%.*}"
  if [[ "$NODE_MAJOR" -ge 18 ]]; then
    pass "node found: $NODE_VERSION"
  else
    fail "node $NODE_VERSION is too old — requires v18 or higher"
  fi
else
  fail "node not found — install from https://nodejs.org"
fi

if command -v npm >/dev/null 2>&1; then
  NPM_VERSION="$(npm --version 2>&1)"
  pass "npm found: v$NPM_VERSION"
else
  fail "npm not found"
fi

# ── Python ─────────────────────────────────────────────────────────────────────
section "Python (requires 3.11+)"

PYTHON_CMD=""
for cmd in python3 python; do
  if command -v "$cmd" >/dev/null 2>&1; then
    PYTHON_CMD="$cmd"
    break
  fi
done

if [[ -n "$PYTHON_CMD" ]]; then
  PY_VERSION="$($PYTHON_CMD --version 2>&1)"
  # Extract version numbers
  PY_NUMS="${PY_VERSION#Python }"
  PY_MAJOR="${PY_NUMS%%.*}"
  PY_MINOR="${PY_NUMS#*.}"
  PY_MINOR="${PY_MINOR%%.*}"
  if [[ "$PY_MAJOR" -gt 3 ]] || { [[ "$PY_MAJOR" -eq 3 ]] && [[ "$PY_MINOR" -ge 11 ]]; }; then
    pass "$PYTHON_CMD found: $PY_VERSION"
  else
    fail "$PYTHON_CMD $PY_VERSION is too old — requires 3.11 or higher"
  fi
else
  fail "python3 not found — install from https://python.org"
fi

# ── Vercel CLI ─────────────────────────────────────────────────────────────────
section "Deployment tools"

if command -v vercel >/dev/null 2>&1; then
  VERCEL_VERSION="$(vercel --version 2>&1 | head -1)"
  pass "vercel CLI found: $VERCEL_VERSION"
else
  warn "vercel CLI not found (will fall back to npx vercel during deploy)"
fi

if command -v git >/dev/null 2>&1; then
  GIT_VERSION="$(git --version 2>&1)"
  pass "git found: $GIT_VERSION"
else
  fail "git not found"
fi

# ── Summary ────────────────────────────────────────────────────────────────────
echo ""
echo "═══════════════════════════════════════"
echo "  Results: ${PASS_COUNT} passed, ${FAIL_COUNT} failed"
echo "═══════════════════════════════════════"
echo ""

if [[ "$FAIL_COUNT" -gt 0 ]]; then
  echo -e "${RED}Preflight FAILED — resolve the issues above before deploying.${NC}"
  exit 1
else
  echo -e "${GREEN}Preflight PASSED — environment looks good.${NC}"
  exit 0
fi

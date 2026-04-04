#!/bin/bash
set -euo pipefail

# Detect USB or local workspace
if [[ -d "/media/CHYREN" ]]; then
  export OMEGA_ROOT="/media/CHYREN"
elif [[ -d "/mnt/CHYREN" ]]; then
  export OMEGA_ROOT="/mnt/CHYREN"
else
  # Use environment variable or default to current directory
  export OMEGA_ROOT="${OMEGA_ROOT:-$(pwd)}"
fi

export OMEGA_HOST_CACHE="${OMEGA_HOST_CACHE:-$HOME/.omega-host-cache/chyren}"
mkdir -p "$OMEGA_HOST_CACHE"

echo "OmegA Workspace: $OMEGA_ROOT"
echo "Host Cache: $OMEGA_HOST_CACHE"
echo ""

cd "$OMEGA_ROOT/workspace/OmegA-Next"
cargo run --package omega-cli -- "$@"

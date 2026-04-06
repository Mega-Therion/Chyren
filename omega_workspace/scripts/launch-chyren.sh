#!/bin/bash
# Fixed Chyren launch script
WORKSPACE_DIR=$(dirname "$0")/../workspace/OmegA-Next
echo "OmegA Workspace: $WORKSPACE_DIR"
cd "$WORKSPACE_DIR" || exit 1
cargo run -- "$@"

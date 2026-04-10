#!/bin/bash
set -euo pipefail

# OmegA Next-Generation Architecture Bootstrap Script
# Scaffolds the complete Rust workspace from the blueprint
# Usage: bash bootstrap_omega_next.sh --new /path/to/workspace [--old /path/to/old/repo] [--edition Chyrho] [--salvage] [--materialize-wrappers]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OLD_REPO=""
NEW_WORKSPACE=""
EDITION="Chyren"
DO_SALVAGE=false
MATERIALIZE_WRAPPERS=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --old) OLD_REPO="$2"; shift 2 ;;
    --new) NEW_WORKSPACE="$2"; shift 2 ;;
    --edition) EDITION="$2"; shift 2 ;;
    --salvage) DO_SALVAGE=true; shift ;;
    --materialize-wrappers) MATERIALIZE_WRAPPERS=true; shift ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

if [[ -z "$NEW_WORKSPACE" ]]; then
  echo "Error: --new is required"
  exit 1
fi

echo "=================================================="
echo "OmegA Next-Generation Architecture Bootstrap"
echo "=================================================="
echo "Edition: $EDITION"
echo "Workspace: $NEW_WORKSPACE"
[[ -n "$OLD_REPO" ]] && echo "Old repo (salvage): $OLD_REPO"
echo ""

# Check dependencies
echo "[1/8] Checking dependencies..."
for cmd in git cargo diff jq; do
  if ! command -v "$cmd" &> /dev/null; then
    echo "  ⚠ $cmd not found. Install it or bootstrap may fail."
  fi
done

# Create workspace directory structure
echo "[2/8] Creating workspace structure..."
mkdir -p "$NEW_WORKSPACE"/{workspace/OmegA-Next,canon,scripts,artifacts,docs,handover}
mkdir -p "$NEW_WORKSPACE/workspace/OmegA-Next/"{src,target}

# Create Cargo.toml workspace root
cat > "$NEW_WORKSPACE/workspace/OmegA-Next/Cargo.toml" << 'EOF'
[workspace]
members = [
    "omega-core",
    "omega-aegis",
    "omega-aeon",
    "omega-adccl",
    "omega-myelin",
    "omega-dream",
    "omega-metacog",
    "omega-worldmodel",
    "omega-integration",
    "omega-telemetry",
    "omega-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["OmegA Collective"]
license = "PROPRIETARY"

[profile.release]
opt-level = 3
lto = true
EOF

# Create crate directories and skeleton
echo "[3/8] Generating crate skeletons..."

declare -A CRATES=(
  [omega-core]="defines shared types and contracts"
  [omega-aegis]="implements envelope compilation, risk gating, provider adapters"
  [omega-aeon]="implements phylactery service, TSO runtime, MUSE++ parsing, routing"
  [omega-adccl]="implements drift control, goal contracts, plan skeletons, verification"
  [omega-myelin]="implements graph memory overlay, retrieval episodes, plasticity"
  [omega-dream]="implements dream session runner, epiphany generation"
  [omega-metacog]="implements performance snapshots, self-assessment"
  [omega-worldmodel]="implements world state, entities, actions, causal graphs"
  [omega-integration]="implements agent registry, subtask packaging, consensus"
  [omega-telemetry]="defines unified event schema, event emission"
)

for crate in "${!CRATES[@]}"; do
  crate_path="$NEW_WORKSPACE/workspace/OmegA-Next/$crate"
  mkdir -p "$crate_path/src"

  # Generate Cargo.toml
  if [[ "$crate" == "omega-cli" ]]; then
    cat > "$crate_path/Cargo.toml" << EOF
[package]
name = "$crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[[bin]]
name = "chyren"
path = "src/main.rs"

[dependencies]
omega-core = { path = "../omega-core" }
omega-aegis = { path = "../omega-aegis" }
omega-aeon = { path = "../omega-aeon" }
omega-adccl = { path = "../omega-adccl" }
omega-myelin = { path = "../omega-myelin" }
omega-telemetry = { path = "../omega-telemetry" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = "0.3"
EOF
    cat > "$crate_path/src/main.rs" << 'MAIN_EOF'
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chyren")]
#[command(about = "OmegA/Chyren Sovereign Intelligence Hub")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Boot the system with default config
    Boot,
    /// Run a task
    Run { task: String },
    /// Display system status
    Status,
    /// Migrate from legacy system
    Migrate { source: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("Chyren: OmegA Sovereign Intelligence Hub");
    println!("Edition: {} | Status: In-Flight ⚙️", env!("CARGO_PKG_VERSION"));
    println!("{:?}", cli.command);
}
MAIN_EOF
  else
    cat > "$crate_path/Cargo.toml" << EOF
[package]
name = "$crate"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[lib]

[dependencies]
omega-core = { path = "../omega-core" }
omega-telemetry = { path = "../omega-telemetry" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", optional = true, features = ["full"] }
thiserror = "1"

[features]
draft = []
EOF
    cat > "$crate_path/src/lib.rs" << "LIB_EOF"
//! Module description here
#![warn(missing_docs)]

pub mod types;
pub mod service;

pub use types::*;
pub use service::*;

mod types {
    // Placeholder types
    #[derive(Clone, Debug)]
    pub struct Placeholder;
}

mod service {
    use crate::types::*;

    /// Placeholder service
    pub struct Service;

    impl Service {
        /// Initialize service
        pub fn new() -> Self {
            Service
        }
    }
}
LIB_EOF
  fi
done

# Create omega-telemetry (special: defines shared event schema)
cat > "$NEW_WORKSPACE/workspace/OmegA-Next/omega-telemetry/src/lib.rs" << 'TELEMETRY_EOF'
//! Unified telemetry and event emission
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub timestamp: f64,
    pub request_id: String,
    pub task_id: String,
    pub layer: String,
    pub component: String,
    pub event_type: String,
    pub status: String,
    pub latency_ms: f64,
    pub provider: String,
    pub risk_score: f64,
    pub bridge_outcome: Option<String>,
    pub verification_score: f64,
    pub retrieval_success: bool,
    pub contradiction_count: usize,
    pub memory_write_class: String,
    pub continuity_action: String,
    pub artifact_refs: Vec<String>,
    pub trace_refs: Vec<String>,
}

pub trait EventSink: Send + Sync {
    fn emit(&self, event: Event);
}

pub struct StdoutSink;

impl EventSink for StdoutSink {
    fn emit(&self, event: Event) {
        if let Ok(json) = serde_json::to_string(&event) {
            println!("{}", json);
        }
    }
}

#[macro_export]
macro_rules! emit_event {
    ($sink:expr, $event:expr) => {
        $sink.emit($event)
    };
}
TELEMETRY_EOF

echo "[4/8] Creating manifest files..."

# Write release manifest
cat > "$NEW_WORKSPACE/canon/release_manifest.json" << MANIFEST_EOF
{
  "system_name": "OmegA",
  "identity_name": "OmegA",
  "product_name": "Chyren",
  "edition_name": "$EDITION",
  "channel": "development",
  "canon_status": "in-flight",
  "created_at": "$(date -Iseconds)",
  "rust_edition": "2021",
  "target_architecture": "x86_64-unknown-linux-gnu"
}
MANIFEST_EOF

# Write machine identity
cat > "$NEW_WORKSPACE/canon/machine_identity.json" << IDENTITY_EOF
{
  "runtime_identity": "chyren-$EDITION-$(date +%s)",
  "continuity_mode": "pre-canon",
  "host_cache": "~/.omega-host-cache/$EDITION",
  "workspace_root": "$NEW_WORKSPACE"
}
IDENTITY_EOF

echo "[5/8] Creating launcher scripts..."

# Create bash launcher
cat > "$NEW_WORKSPACE/scripts/launch-chyren.sh" << 'LAUNCHER_EOF'
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
LAUNCHER_EOF

chmod +x "$NEW_WORKSPACE/scripts/launch-chyren.sh"

# Create PowerShell launcher
cat > "$NEW_WORKSPACE/scripts/launch-chyren.ps1" << 'LAUNCHER_PS1'
$usbPaths = @("D:", "E:", "F:", "G:")
$omegaRoot = $null

foreach ($drive in $usbPaths) {
    if (Test-Path "$drive\CHYREN") {
        $omegaRoot = "$drive\CHYREN"
        break
    }
}

if (-not $omegaRoot) {
    $omegaRoot = $env:OMEGA_ROOT -or (Get-Location).Path
}

$env:OMEGA_ROOT = $omegaRoot
$env:OMEGA_HOST_CACHE = $env:OMEGA_HOST_CACHE -or "$env:USERPROFILE\.omega-host-cache\chyren"

New-Item -ItemType Directory -Force -Path $env:OMEGA_HOST_CACHE | Out-Null

Write-Host "OmegA Workspace: $omegaRoot"
Write-Host "Host Cache: $env:OMEGA_HOST_CACHE"
Write-Host ""

Push-Location "$omegaRoot\workspace\OmegA-Next"
cargo run --package omega-cli -- @args
Pop-Location
LAUNCHER_PS1

echo "[6/8] Creating CI/CD skeleton..."
mkdir -p "$NEW_WORKSPACE/.github/workflows"

cat > "$NEW_WORKSPACE/.github/workflows/rust.yml" << 'CI_EOF'
name: Rust CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --workspace --verbose
      - name: Run tests
        run: cargo test --workspace --verbose
      - name: Clippy
        run: cargo clippy --workspace -- -D warnings
      - name: Format check
        run: cargo fmt --all -- --check
CI_EOF

echo "[7/8] Salvage and migration (optional)..."

if [[ "$DO_SALVAGE" == true && -n "$OLD_REPO" ]]; then
  echo "  Copying old repository to legacy_intake..."
  mkdir -p "$NEW_WORKSPACE/legacy_intake"
  cp -r "$OLD_REPO" "$NEW_WORKSPACE/legacy_intake/old_repo"

  echo "  Generating migration diffs..."
  diff -r "$NEW_WORKSPACE/legacy_intake/old_repo" "$NEW_WORKSPACE/workspace/OmegA-Next" > "$NEW_WORKSPACE/legacy_intake/migration_diff.txt" || true

  cat > "$NEW_WORKSPACE/legacy_intake/MIGRATION_SUMMARY.md" << 'MIGRATION_EOF'
# Migration Summary: Python Chyren → Rust OmegA/Chyren

## Source Structure
The original Python Chyren implementation in `old_repo` contains:
- `core/`: Modular components (alignment, sandbox, deflection, threat_fabric)
- `providers/`: Multi-provider adapter layer
- `main.py`: Hub orchestrator
- `state/`: Persistent ledger and constitution

## Target Mapping
| Python Module | Rust Crate | Status |
|---|---|---|
| core/alignment.py | omega-aegis (policy gate) | Salvage to AEGIS |
| core/sandbox.py | omega-adccl (verification) | Salvage to ADCCL |
| core/deflection.py | omega-aegis (risk response) | Salvage to AEGIS |
| core/threat_fabric.py | omega-myelin (memory graph) | Salvage to MYELIN |
| providers/* | omega-aegis (provider adapters) | Reimplement as adapters |
| main.py (Hub) | omega-aeon (orchestrator) | Reimplement with TSO |
| core/integrity.py | omega-core (crypto) | Port directly |

## Legacy Bridge Pattern
Each crate provides a `legacy_bridge.rs` module that:
1. Wraps old Python logic via subprocess calls (temporary)
2. Exposes it behind stable Rust traits
3. Allows incremental porting without blocking
4. Will be removed once native Rust implementation is complete

## Porting Order
1. omega-core: Port type definitions and cryptography
2. omega-telemetry: Define event schema
3. omega-aegis: Port envelope, risk gating, provider routing
4. omega-myelin: Port memory storage to graph model
5. omega-adccl: Port verification and drift control logic
6. omega-aeon: Reimplement orchestrator with phylactery service
7. Other layers: Dream, metacog, worldmodel, integration

MIGRATION_EOF
fi

echo "[8/8] Documentation and next steps..."

cat > "$NEW_WORKSPACE/docs/BOOTSTRAP_SUMMARY.md" << 'SUMMARY_EOF'
# OmegA Next-Generation Architecture Bootstrap Summary

## What Was Created

✓ Rust workspace with 11 crates (omega-core through omega-cli)
✓ Cargo.toml with workspace configuration
✓ Unified telemetry event schema
✓ Release and machine identity manifests
✓ Launch scripts (bash and PowerShell)
✓ CI/CD skeleton (.github/workflows)
✓ Legacy intake directory (if salvage was requested)

## Directory Structure

```
$NEW_WORKSPACE/
├── workspace/OmegA-Next/        # Main Rust workspace
│   ├── Cargo.toml
│   ├── omega-core/              # Shared types, crypto
│   ├── omega-aegis/             # Outer shell, risk gating
│   ├── omega-aeon/              # Cognitive OS, phylactery
│   ├── omega-adccl/             # Drift control, verification
│   ├── omega-myelin/            # Graph memory
│   ├── omega-dream/             # Dream-to-waking feedback
│   ├── omega-metacog/           # Self-assessment
│   ├── omega-worldmodel/        # Causal models
│   ├── omega-integration/       # gAIng coordination
│   ├── omega-telemetry/         # Event schema
│   └── omega-cli/               # CLI binary
├── canon/                        # Manifests and metadata
│   ├── release_manifest.json
│   └── machine_identity.json
├── scripts/                      # Launcher scripts
│   ├── launch-chyren.sh
│   └── launch-chyren.ps1
├── artifacts/                    # Build outputs
├── docs/                         # Documentation
├── handover/                     # Production transfer
└── legacy_intake/                # Old codebase (if salvaged)
```

## Next Steps

1. **Verify setup:**
   ```bash
   cd $NEW_WORKSPACE/workspace/OmegA-Next
   cargo build
   ```

2. **Start implementing omega-core types:**
   Edit `omega-core/src/lib.rs` to define RunEnvelope, TaskStateObject, EvidencePacket, etc.

3. **Implement omega-telemetry event sinks:**
   Stdout, file, and Kafka sinks for unified observability.

4. **Port existing Python logic:**
   Use legacy_bridge.rs modules to wrap old code while porting.

5. **Progressive feature implementation:**
   Follow the order: AEGIS → MYELIN → ADCCL → AEON → new features

## Running the CLI

Local development:
```bash
cd workspace/OmegA-Next
cargo run --package omega-cli -- boot
```

Via launcher:
```bash
./scripts/launch-chyren.sh boot
```

## Configuration

Edit `canon/machine_identity.json` to set:
- `continuity_mode`: "pre-canon" (bootstrap), "portable" (USB), or "prod" (deployed)
- `host_cache`: Location for runtime caches
- `workspace_root`: Canonical workspace location

SUMMARY_EOF

echo ""
echo "=================================================="
echo "✓ Bootstrap Complete"
echo "=================================================="
echo ""
echo "Workspace created at: $NEW_WORKSPACE"
echo ""
echo "Next steps:"
echo "  1. cd $NEW_WORKSPACE/workspace/OmegA-Next"
echo "  2. cargo build           # Verify compilation"
echo "  3. cargo test --lib      # Run placeholder tests"
echo "  4. Edit omega-core/src/lib.rs to define types"
echo ""
echo "To launch: $NEW_WORKSPACE/scripts/launch-chyren.sh"

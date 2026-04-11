# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This System Is

**Chyren** is a Sovereign Intelligence Orchestrator — a stateful, high-integrity AI task execution platform with a binary-hemispheric architecture:

- **Cortex** (`cortex/`) — Python Hub: orchestration, provider routing, ADCCL verification, Master Ledger
- **Medulla** (`medulla/`) — Rust Workspace: 17 crates covering security, memory, scheduling, CLI, and API server
- **Web** (`web/`) — Next.js 15 cognitive shell frontend
- **Gateway** (`gateway/`) — Vite + React 19 external spoke gateway
- **Brain Stem** (`./chyren`) — Python router script that dispatches commands to Cortex or Medulla

## Build, Test, Lint

### Rust Workspace (Medulla)
```bash
cd medulla

cargo build                                  # Debug build
cargo build --release                        # Release (opt-level=3, LTO)
cargo test --workspace                       # All tests
cargo test --package omega-adccl             # Single crate tests
cargo clippy --workspace -- -D warnings      # Lint
cargo fmt --all -- --check                   # Format check
cargo fmt --all                              # Format apply
cargo check --workspace                      # Compile check only
```

### Python Hub (Cortex)
```bash
cd cortex

python -m venv venv && source venv/bin/activate
pip install -r requirements.txt

python main.py "Your task" --provider anthropic    # Run with a task
python chyren_py/identity_synthesis.py             # Refresh phylactery kernel
pytest /home/mega/Chyren/tests/                    # Run tests
```

### Web Frontend
```bash
cd web
npm run dev          # Dev server on localhost:3000
npm run build        # Production build (runs scripts/generate-context.mjs first)
npm run lint         # ESLint (max-warnings=0)
npm run typecheck    # tsc --noEmit
```

### Gateway
```bash
cd gateway
npm run dev          # Vite dev server
npm run build        # tsc -b && vite build
npm run lint         # ESLint
```

### Full Stack (Docker)
```bash
cd medulla
docker-compose up    # Starts chyren-api (8080), chyren-web (3000), postgres, qdrant
```

### Brain Stem CLI
```bash
./chyren thought "..."    # → Cortex (Python): reasoning, sensing, verification, identity
./chyren action "..."     # → Medulla (Rust): execution, memory, sharding, ingestion
./chyren status           # System status
./chyren live             # Start web + API
./chyren dream            # Maintenance mode
```

## Architecture

### Cortex (Python Hub)
- `cortex/main.py` — `Chyren` class: owns Master Ledger, provider router, 6-phase initialization, `RunResult` dataclass
- `cortex/core/` — Security gates: `adccl.py` (drift verification), `alignment.py` (constitution), `ledger.py` (immutable log), `threat_fabric.py`, `deflection.py`, `sandbox.py`
- `cortex/providers/` — Provider adapters implementing `ProviderBase`: Anthropic, OpenAI, DeepSeek, Gemini
- `cortex/chyren_py/` — Phylactery identity synthesis and kernel loading

### Medulla (Rust — 17 crates)
| Crate | Role |
|---|---|
| `omega-core` | Foundation types, contracts, task envelopes |
| `omega-conductor` | Full pipeline orchestration (Alignment → AEON → Provider → ADCCL → Ledger) |
| `omega-aegis` | Security gates, policy enforcement, constitution checks |
| `omega-adccl` | Drift/hallucination detection; score 0.0–1.0, threshold 0.7 |
| `omega-myelin` | Persistent semantic memory (Qdrant vector store) |
| `omega-spokes` | Provider SDK adapters + registry |
| `omega-aeon` | Temporal/scheduling subsystem |
| `omega-phylactery` | Identity and integrity persistence |
| `omega-cli` | Clap-based CLI + actix-web API server (port 8080) |
| `omega-dream` | Long-term memory and pattern synthesis |
| `omega-metacog` | Self-reflection and introspection |
| `omega-worldmodel` | Environmental state and context management |
| `omega-telemetry` | Instrumentation — all events must route through here |
| `omega-integration` | Cross-crate routing |
| `omega-eval` | Evaluation framework |
| `omega-telegram-gateway` | Telegram bot integration |

### Data Layer
- **Master Ledger**: PostgreSQL (Neon) — append-only, cryptographically signed; `OMEGA_DB_URL` env var
- **Myelin**: Qdrant vector store for semantic memory; `QDRANT_URL` env var
- **Phylactery Kernel**: `cortex/chyren_py/phylactery_kernel.json` — ~58k identity entries, loaded at startup

### ADCCL Verification Gate
Every provider response is scored before ledger commit:
- Threshold: **0.7** — do not lower this
- Flags: `STUB_MARKERS_DETECTED`, `RESPONSE_TOO_SHORT`, `CAPABILITY_REFUSAL`, `NO_TASK_WORD_OVERLAP`
- Calibration: starts loose (0.1) and tightens to 0.7 over a 60-minute session
- No recovery: rejected responses are discarded, not retried

### Provider Injection Pattern
All provider spokes receive: system prompt with sovereign identity + Yettragrammaton integrity hash + current ledger state as context.

## Configuration

All secrets come from `~/.omega/one-true.env` (not in git):
```
ANTHROPIC_API_KEY
OPENAI_API_KEY
DEEPSEEK_API_KEY
GEMINI_API_KEY
OMEGA_DB_URL          # Neon PostgreSQL connection string
QDRANT_URL            # Qdrant vector store
```

Always source this file before running Cortex or Medulla directly. Missing keys fail silently.

## Key Conventions

- **Telemetry**: Never log significant events directly — route through `omega-telemetry` crate
- **Ledger**: Append-only. Never delete `state/` files; history is irreversible
- **Phylactery**: Run `python chyren_py/identity_synthesis.py` to refresh a stale kernel
- **New Python provider**: implement `ProviderBase` in `cortex/providers/`, register in `cortex/main.py`
- **New Rust crate**: add to `medulla/Cargo.toml` workspace members, expose from `src/lib.rs`, wire into `omega-integration` or `omega-cli`
- **Rust → Python migration**: use `legacy_bridge.rs` pattern; tests must pass in both layers before cutover

## Notes on `docs/CLAUDE.md`
The file at `docs/CLAUDE.md` is outdated — it references pre-restructure paths (`omega_workspace/workspace/OmegA-Next/`, root-level `main.py`). Use this root `CLAUDE.md` as the authoritative reference.

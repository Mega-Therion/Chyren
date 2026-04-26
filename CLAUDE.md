# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This System Is

**Chyren** is a Sovereign Intelligence Orchestrator — a stateful, high-integrity AI task execution platform.

- **Medulla** (`medulla/`) — Rust Workspace: canonical runtime — 17 crates covering security, memory, scheduling, CLI, and API server. All live requests route here.
- **Cortex** (`cortex/`) — Python layer: identity synthesis (`chyren_py/`), data ops scripts (`ops/scripts/`). No longer invoked for runtime requests.
- **Web** (`chyren-os/interface/`) — Next.js 15 cognitive shell frontend (actively deployed to Vercel; `web_old/` is a stale snapshot)
- **Gateway** (`gateway/`) — Vite + React 19 external spoke gateway
- **Brain Stem** (`./chyren`) — Python router script dispatching all commands to Medulla; Python cortex only runs during `dream` maintenance mode

## Build, Test, Lint

### Rust Workspace (Medulla)
```bash
cd medulla

cargo build                                  # Debug build
cargo build --release                        # Release (opt-level=3, LTO)
cargo test --workspace                       # All tests
cargo test --package chyren-adccl             # Single crate tests
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

# Run tests (pytest.ini at repo root sets pythonpath=cortex, testpaths=tests)
pytest                                             # All Python tests
pytest tests/test_adccl_hub.py                     # Single test file
pytest -k test_name                                # Single test by name
```

### Web Frontend (`chyren-os/interface/`)
```bash
cd chyren-os/interface
npm run dev          # Dev server on localhost:3000
npm run build        # Production build (runs scripts/generate-context.mjs first)
npm run lint         # ESLint
npm run typecheck    # tsc --noEmit
npm run test         # Vitest unit tests
```
Deployed to Vercel. Sync env vars with `scripts/sync-vercel-env-from-one-true.sh`.
Key libs: `lib/adccl.ts` (TypeScript ADCCL port, threshold 0.7), `lib/hardening.ts` (rate limiting + prompt injection), `lib/phylactery.ts` (identity context), `lib/neon-context.ts` (live ledger fetch).

### Gateway
```bash
cd gateway
pnpm install         # Gateway uses pnpm, not npm
pnpm dev             # Vite dev server
pnpm build           # tsc -b && vite build
pnpm lint            # ESLint
```

### Makefile shortcuts (from repo root)
```bash
make ci              # Rust fmt + clippy + test (local CI equivalent)
make cortex-test     # Python tests (PYTHONPATH=cortex pytest tests/)
make web-ci          # Next.js: typecheck + lint + build (runs in chyren-os/interface/)
make gateway-ci      # Gateway: tsc + lint + build
```

### Full Stack (Docker)
```bash
cd medulla
docker-compose up    # Starts chyren-api (8080), chyren-web (3000), postgres, qdrant
```

### Brain Stem CLI
```bash
./chyren thought "..."    # → Medulla (Rust): sovereign reasoning pipeline
./chyren action "..."     # → Medulla (Rust): execution, memory, sharding, ingestion
./chyren status           # System status (Medulla)
./chyren live             # Start web + API
./chyren dream            # Maintenance: identity synthesis + catalog indexing (Python scripts only)
```

## Architecture

### Cortex (Python — data tooling only, not runtime)
- `cortex/chyren_py/identity_synthesis.py` — Regenerates `phylactery_kernel.json`; run via `./chyren dream`
- `cortex/chyren_py/phylactery_kernel.json` — ~58k identity entries loaded by Medulla at startup
- `cortex/ops/scripts/` — Data pipeline utilities (catalog, ingestion) run during maintenance
- `cortex/core/`, `cortex/providers/`, `cortex/main.py` — Legacy Python runtime (no longer used for live requests; retained as reference)

### Medulla (Rust — 17 crates)
| Crate | Role |
|---|---|
| `chyren-core` | Foundation types, contracts, task envelopes |
| `chyren-conductor` | Full pipeline orchestration (Alignment → AEON → Provider → ADCCL → Ledger) |
| `chyren-aegis` | Security gates, policy enforcement, constitution checks |
| `chyren-adccl` | Drift/hallucination detection; score 0.0–1.0, threshold 0.7 |
| `chyren-myelin` | Persistent semantic memory (Qdrant vector store) |
| `chyren-spokes` | Provider SDK adapters + registry |
| `chyren-aeon` | Temporal/scheduling subsystem |
| `chyren-phylactery` | Identity and integrity persistence |
| `chyren-cli` | Clap-based CLI + actix-web API server (port 8080) |
| `chyren-dream` | Long-term memory and pattern synthesis |
| `chyren-metacog` | Self-reflection and introspection |
| `chyren-neocortex` | Higher-order reasoning integration layer |
| `chyren-worldmodel` | Environmental state and context management |
| `chyren-telemetry` | Instrumentation — all events must route through here |
| `chyren-integration` | Cross-crate routing |
| `chyren-eval` | Evaluation framework |
| `chyren-telegram-gateway` | Telegram bot integration |
| `chyren-cim` | Compositional identity mapping |
| `chyren-ternary` | Ternary logic / trivalent reasoning layer |
| `chyren-vision` | Multimodal / visual input processing |

### Data Layer
- **Master Ledger**: PostgreSQL (Neon) — append-only, cryptographically signed; `CHYREN_DB_URL` env var
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

### Sovereign Provider Router (`chyren-conductor/src/router.rs`)
Two-tier routing: **Local** (Ollama — sensitive tasks: identity, ledger, secrets) and **Cloud** (OpenRouter — everything else, cascades through deepseek → groq → anthropic → openai → perplexity on failure). Set `OPENROUTER_ESCALATION_MODEL` in `one-true.env` to override the upshift model.

### Agent Mesh (in-progress, not merged to main — `cursor/integration-hardening` branch)
An MQTT-based agent orchestration layer being added to `chyren-conductor`:
- `chyren-core/src/mesh.rs` — `TaskContract` (typed task routing envelope) + `AgentCapability` + `AgentRegistry`
- `chyren-conductor/src/dispatcher.rs` — Routes `TaskContract`s to agents via MQTT (broker at `localhost:1883`)
- `chyren-conductor/src/bus.rs` — Async `EventBus` (tokio mpsc channel) feeding `AgentResult`s back into the Conductor pipeline
- `chyren-conductor/src/registry.rs` — Re-exports `AgentRegistry` from `chyren-core::mesh`
- `chyren-conductor/src/agents/` — Per-domain agent implementations (e.g. `math_spoke`)
- `chyren-spokes/src/spokes/witness.rs` — `WitnessEnvelope`: signs response payload hashes with `YETTRAGRAMMATON_SECRET` for integrity verification

## Configuration

All secrets come from `~/.chyren/one-true.env` (not in git):
```
ANTHROPIC_API_KEY
OPENAI_API_KEY
DEEPSEEK_API_KEY
CHYREN_DB_URL          # Neon PostgreSQL connection string
QDRANT_URL            # Qdrant vector store
```

Always source this file before running Cortex or Medulla directly (`source ~/.chyren/one-true.env`). The Makefile does **not** source it automatically. Missing keys fail silently.

## Key Conventions

- **Telemetry**: Never log significant events directly — route through `chyren-telemetry` crate
- **Ledger**: Append-only. Never delete `state/` files; history is irreversible
- **Phylactery**: Run `python chyren_py/identity_synthesis.py` to refresh a stale kernel
- **New Python provider**: implement `ProviderBase` in `cortex/providers/`, register in `cortex/main.py`
- **New Rust crate**: add to `medulla/Cargo.toml` workspace members, expose from `src/lib.rs`, wire into `chyren-integration` or `chyren-cli`
- **Rust → Python migration**: use `legacy_bridge.rs` pattern; tests must pass in both layers before cutover
- **Test isolation**: Rust unit tests live alongside implementation files; Python integration tests go in `tests/` or `cortex/tests/`; frontend tests in `web/__tests__/`. Tests must not share mutable state — test isolation failures have caused prod divergence in the past
- **Crate status**: `chyren-cim`, `chyren-ternary`, and `chyren-vision` are present in the workspace but are early-stage / stub crates; treat them as unstable API surfaces

## Commit & PR Format

Follow [Conventional Commits](https://www.conventionalcommits.org/): `feat:`, `fix:`, `style:`, `refactor:`, `test:`, `docs:` with imperative subject lines. PRs must include: problem statement, summary of changes, linked issues (if any), test evidence, and screenshots for UI changes (`web/` or `gateway/`).

## chyren-os/ — What Each Layer Is

`chyren-os/` contains three distinct things — do not treat them as one system:

| Subdirectory | Status | Description |
|---|---|---|
| `chyren-os/interface/` | **Active — deployed to Vercel** | The live Next.js 15 frontend. This IS `web/`. See Build section above. |
| `chyren-os/kernel/` | Historical reference | Older Rust workspace (same chyren-* crates) that `medulla/` evolved from. Missing `chyren-cim`, `chyren-ternary`, `chyren-vision`, `chyren-mega`, `openrouter_spoke`, `vision_spoke`. Do not actively develop here. |
| `chyren-os/supervisor/` | Historical reference | Predecessor to `cortex/chyren_py/` — older `identity_synthesis.py` and phylactery loader. `cortex/chyren_py/` is canonical. |
| `chyren-os/state/` | Runtime state | Phylactery kernel snapshot and runtime state files — do not delete. |
| `chyren-os/boot/init.rs` | Stub | OS entry point stub — not wired into any build system. |

**Phylactery kernel copies** — five exist, only one is canonical:
- `cortex/chyren_py/phylactery_kernel.json` — **canonical** (3031 bytes, newest)
- `chyren-os/interface/lib/phylactery-kernel.json` — browser-accessible, kept small intentionally; update when identity changes
- All others (`chyren-os/kernel/data/`, `chyren-os/state/`, `chyren-os/supervisor/`) — stale snapshots, not loaded at runtime

## Other Root-Level Directories

- `chyren-os/` — Three distinct layers under one roof (see above)
- `hub/` — Swarm attestation utilities (`swarm_attestation.py`)
- `api/` — Alexa integration (`alexa.js`, `interaction_model.json`)
- `ops/` — Legacy bootstrap and proxy scripts (not part of active runtime)
- `analytics/` — Standalone analytics tooling
- `knowledge_injection/` — Data ingest pipelines fed into Qdrant/Neon
- Root-level Python scripts (`ingest_neon.py`, `telemetry_bus.py`, `ari_verify.py`, `cantor_block.py`) — one-off tooling; not part of the main runtime


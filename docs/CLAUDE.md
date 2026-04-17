# CLAUDE.md

> **DEPRECATED — DO NOT USE FOR NEW WORK**
>
> This file references pre-restructure paths (`omega_workspace/workspace/OmegA-Next/`, root-level `main.py`) that no longer reflect the current repository layout. It is retained as a historical reference only.
>
> **Current authoritative references:**
> - Architecture: [`docs/ARCHITECTURE.md`](./ARCHITECTURE.md)
> - Developer tooling and build commands: root [`CLAUDE.md`](../CLAUDE.md)
> - First-run setup: [`docs/QUICKSTART.md`](./QUICKSTART.md)

---

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Chyren** is a Sovereign Intelligence Orchestrator — a stateful, high-integrity AI task execution platform that routes work through multiple provider spokes (Anthropic, OpenAI, DeepSeek, Gemini) while enforcing cognitive verification gates and security controls.

The system operates in two layers:
- **Python Hub** (`main.py` and `core/`): The current stateful orchestrator managing the Master Ledger, provider routing, and ADCCL verification
- **Rust OmegA-Next** (`omega_workspace/workspace/OmegA-Next/`): The next-generation foundation being built for low-latency, robust execution

## Architecture

### Hub-and-Spokes Pattern
- **Hub** (`main.py`, `Chyren` class): Owns the Master Ledger, routes all tasks through provider spokes, runs ADCCL verification, tracks threat patterns
- **Spokes** (Anthropic, OpenAI, DeepSeek, Gemini providers): Isolated provider adapters in `providers/` that receive injected system prompt + ledger state
- **Verification Gate** (`core/adccl.py`): Anti-Drift Cognitive Control Loop rejects responses with drift, hallucinations, or stubs (score < 0.7)

### Security & Integrity
- **Master Ledger** (`core/ledger.py`): Single source of truth; every interaction signed and immutable
- **Alignment Layer** (`core/alignment.py`): Constitutional guidance enforced before provider calls
- **Deflection Engine** (`core/deflection.py`): Threat classification and response
- **Threat Fabric** (`core/threat_fabric.py`): Pattern-based threat memory; syncs with phylactery
- **Sandbox** (`core/sandbox.py`): Payload analysis before execution

### Identity & Persistence
- **Phylactery** (`chyren_py/phylactery_kernel.json`): Runtime-loaded identity kernel (~58k synthesized entries); bootstrapped at CLI startup
- **Identity Synthesis** (`chyren_py/identity_synthesis.py`): Generates and manages sovereign identity

### OmegA-Next (Rust Workspace)
Located at `omega_workspace/workspace/OmegA-Next/`, with these core crates:

| Crate | Purpose |
|-------|---------|
| `omega-core` | Foundation types, task routing, ledger interfaces |
| `omega-adccl` | Rust implementation of Anti-Drift Cognitive Control Loop |
| `omega-aegis` | Security layer (threat detection, policy enforcement) |
| `omega-aeon` | Temporal/scheduling subsystem |
| `omega-myelin` | Persistent state & caching layer |
| `omega-metacog` | Self-reflection and introspection |
| `omega-dream` | Long-term memory and pattern synthesis |
| `omega-worldmodel` | Environmental state and context management |
| `omega-integration` | Provider integration & routing (Rust equivalents of Python spokes) |
| `omega-spokes` | Provider SDK adapters |
| `omega-telemetry` | Instrumentation and event emission |
| `omega-conductor` | Orchestration primitives |
| `omega-cli` | Binary CLI interface (replaces `main.py` eventually) |

### Web Layer (chyren-web)
Next.js 15 + React 18 + TypeScript + Tailwind:
- Located at `omega_workspace/workspace/OmegA-Next/chyren-web/`
- Exposes HTTP API to the Rust CLI backend
- Deployed to Vercel; environment synced from `~/.omega/one-true.env`

## Build, Test, Lint

### Rust Workspace (OmegA-Next)
```bash
cd omega_workspace/workspace/OmegA-Next

# Build all crates
cargo build

# Release build (optimized, LTO enabled)
cargo build --release

# Run tests
cargo test --workspace

# Run single test file
cargo test --package omega-core --lib

# Lint with clippy
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all -- --check

# Format apply
cargo fmt --all

# Check (compile without output binary)
cargo check --workspace
```

### Python Hub & Tools
```bash
# Setup environment
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# Run hub with a task
python main.py "Your task here" --provider anthropic

# Phylactery identity synthesis (generates/refreshes kernel)
python chyren_py/identity_synthesis.py

# Check schema
python check_schema.py
```

### Web Frontend (chyren-web)
```bash
cd omega_workspace/workspace/OmegA-Next/chyren-web

# Development server (localhost:3000)
npm run dev

# Production build
npm run build

# Start production server
npm start

# Lint Next.js project
npm run lint

# Deploy to Vercel (requires Vercel CLI)
bash scripts/deploy-vercel.sh

# Sync NEXT_PUBLIC_* vars from one-true.env to Vercel project
bash scripts/sync-vercel-env-from-one-true.sh
```

### CI/CD
GitHub Actions runs on push/PR to main/develop:
- **Rust checks** (build, test, clippy, format)
- **Web build** (Next.js build succeeds)

See `.github/workflows/ci.yml` for the full pipeline.

## Configuration & Environment

### Environment Variables
All configuration comes from `~/.omega/one-true.env` (loaded before any providers spin up):

**Required:**
- `ANTHROPIC_API_KEY` — Claude API token
- `OPENAI_API_KEY` — GPT-4 API token
- `DEEPSEEK_API_KEY` — DeepSeek API token
- `GEMINI_API_KEY` — Google Gemini API token

**Optional (Vercel/Web):**
- `NEXT_PUBLIC_API_BASE_URL` — API base URL visible to browser (default: `https://placeholder.local` in CI)

The `one-true.env` file **is not in git** (git-ignored). Create it with your own keys; it persists across bootstraps.

## Directory Structure

```
/home/mega/Chyren/
├── main.py                   # Python Hub: orchestrator, ledger, provider routing
├── requirements.txt          # Python dependencies
├── bootstrap_omega_next.sh   # Scaffolds the OmegA-Next Rust workspace
├── run_synthesis.sh          # Helper to run phylactery synthesis
├── GEMINI.md                 # Original project documentation
├── CLAUDE.md                 # This file
│
├── core/                     # Python core modules
│   ├── integrity.py          # YETTRAGRAMMATON (root integrity)
│   ├── ledger.py             # Master Ledger (signed, immutable)
│   ├── adccl.py              # Anti-Drift Cognitive Control Loop
│   ├── alignment.py          # Constitutional guidance layer
│   ├── deflection.py         # Threat classification & response
│   ├── threat_fabric.py      # Pattern-based threat memory
│   ├── sandbox.py            # Payload analysis
│   └── preflight.py          # Environment validation
│
├── providers/                # LLM provider adapters
│   ├── base.py               # Provider interface & router
│   ├── anthropic.py          # Claude adapter
│   ├── openai.py             # GPT-4 adapter
│   ├── deepseek.py           # DeepSeek adapter
│   └── gemini.py             # Google Gemini adapter
│
├── chyren_py/                # Python utilities
│   ├── identity_synthesis.py # Generates phylactery kernel
│   ├── phylactery_loader.py  # Loads identity at startup
│   ├── phylactery_kernel.json # Synthesized identity data (58k entries)
│   ├── phylactery_bootstrap.rs # Rust bootstrap for kernel loading
│   └── IDENTITY_FOUNDATION.md # Identity architecture docs
│
├── state/                    # Persistent state files
│   ├── constitution.json     # Constitutional rules
│   └── threat_fabric.json    # Threat pattern database
│
├── omega_workspace/workspace/OmegA-Next/  # Rust workspace (Phase 2-3 complete)
│   ├── Cargo.toml            # Workspace manifest (13 crates)
│   ├── omega-*/              # Crate directories (see above)
│   └── chyren-web/           # Next.js 15 frontend
│       ├── app/              # App router (Next.js 13+)
│       ├── package.json      # Node dependencies
│       ├── next.config.ts    # Next.js config (Vercel defaults)
│       ├── scripts/          # Deployment helpers
│       └── .vercel/          # Vercel project config
│
└── .github/workflows/ci.yml  # GitHub Actions: Rust + Node CI
```

## Key Concepts & Conventions

### ADCCL Verification (Gatekeeper)
Every response from a provider must pass ADCCL before being committed to the ledger:
- **Score** (0.0–1.0): Confidence the response is drift-free, truthful, and not a stub
- **Rejection threshold**: 0.7
- **Flags**: Array of issues detected (hallucination, drift, unknown, etc.)
- **No Recovery**: Rejected responses are logged but discarded; the system does not retry or modify

### Provider Request Injection
All provider spokes receive:
1. System prompt prefix: "You are Chyren — a sovereign intelligence orchestrator..."
2. Integrity guarantee: The Yettragrammaton (root integrity hash)
3. Current ledger state: Injected as context so workers know system history without manual briefing

### Zero-Downtime Migration (Rust Port)
When porting Python logic to Rust:
- Use the `legacy_bridge.rs` pattern for interfacing old and new systems
- Tests must pass in both layers before migration is complete
- No ledger corruption; all state transitions must be reversible

### Telemetry
All significant events (provider calls, ADCCL verdicts, threat detections) are emitted via the schema in `omega-telemetry`. Do not log events directly; route through the telemetry crate.

## Development Workflow

### Adding a New Provider Spoke
1. Create adapter in `providers/new_provider.py` implementing `ProviderBase`
2. Register in `main.py` (`Chyren.__init__`)
3. Add API key to `~/.omega/one-true.env`
4. Test with: `python main.py "test" --provider new_provider`

### Adding a Rust Crate to OmegA-Next
1. Create crate: `cargo new omega_workspace/workspace/OmegA-Next/omega-<name>`
2. Add to workspace members in `Cargo.toml`
3. Expose public API from `src/lib.rs`
4. Update `omega-integration` or `omega-cli` to consume it

### Deploying the Web Frontend
1. Sync environment to Vercel: `bash chyren-web/scripts/sync-vercel-env-from-one-true.sh`
2. Push to main/develop branch (Vercel auto-deploys on push)
3. Or manually: `bash chyren-web/scripts/deploy-vercel.sh`

### Local Testing the Full Stack
```bash
# Terminal 1: Start Rust CLI
cd omega_workspace/workspace/OmegA-Next
cargo run --bin chyren_api -- run "test task"

# Terminal 2: Start Next.js dev server
cd omega_workspace/workspace/OmegA-Next/chyren-web
npm run dev
# Open http://localhost:3000
```

## Common Gotchas

- **Environment**: Always export `~/.omega/one-true.env` before running `main.py` or Rust CLI. Missing API keys will fail silently.
- **Ledger Persistence**: The Master Ledger is append-only. Deleting `state/` files will lose history; use caution.
- **ADCCL Tuning**: Do not lower the verification score threshold (0.7) without understanding the implications — it increases hallucination risk.
- **Phylactery Staleness**: If identity synthesis hasn't run, the kernel may be out of date. Run `python chyren_py/identity_synthesis.py` to refresh.
- **Git LFS**: Large binaries and memory objects are tracked with git-lfs. Ensure `git lfs install` is run before cloning.

## References

- `GEMINI.md` — Original project documentation and design philosophy
- `chyren_py/IDENTITY_FOUNDATION.md` — Deep dive on phylactery identity architecture
- `omega_workspace/docs/BOOTSTRAP_SUMMARY.md` — OmegA-Next scaffolding details
- `.github/workflows/ci.yml` — Build & test pipeline

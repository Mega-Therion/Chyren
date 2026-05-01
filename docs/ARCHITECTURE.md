# Chyren Architecture Reference

This is the single canonical architecture reference. The file `docs/CLAUDE.md` is outdated — see the deprecation notice at the top of that file.

For developer tooling guidance, see the root `CLAUDE.md`.

---

## What Chyren Is

Chyren is a **Sovereign Intelligence Orchestrator** — a stateful, high-integrity AI task execution platform. All live requests route through the Rust runtime (Medulla). The Python layer (Cortex) is retained for data tooling only and is not invoked at runtime.

---

## System Layers

### Medulla — `src/medulla/kernel` (Rust Workspace, Canonical Runtime)

The production runtime. All `./chyren thought`, `./chyren action`, and API requests execute here. Medulla is a ~33-crate Rust workspace exposing a Clap CLI and an Actix-web API server on port 8080.

### Cortex — `src/cortex/` (Python, Data Tooling Only)

Not invoked for live requests. Contains:
- `src/cortex/chyren_py/identity_synthesis.py` — regenerates `phylactery_kernel.json`; run via `./chyren dream`
- `src/cortex/chyren_py/phylactery_kernel.json` — ~58k identity entries, loaded by Medulla at startup
- `src/cortex/ops/scripts/` — catalog and ingestion pipeline utilities run during maintenance
- `src/cortex/core/`, `src/cortex/providers/`, `src/cortex/main.py` — legacy Python runtime, retained as reference only

### Web — `src/medulla/interface` (Next.js 15 Frontend)

Cognitive shell frontend. Connects to the Medulla API server at port 8080.

### Gateway — `src/gateways/` (Vite + React 19)

External spoke gateway surface. Separate from the main web frontend.

### Brain Stem — `./chyren` (Python Router Script)

Thin dispatch layer. Routes all CLI commands to Medulla. Invokes Python Cortex scripts only during `dream` maintenance mode.


---

## Request Data Flow

```
User / API / Tooling
        |
   ./chyren (Brain Stem router)
        |
   Medulla API (chyren-cli, port 8080)
        |
   chyren-conductor (pipeline orchestration)
        |
   +-----------+
   | chyren-aegis|  Security gate, constitution checks
   +-----------+
        |
   +-----------+
   | chyren-aeon |  Temporal/scheduling subsystem
   +-----------+
        |
   chyren-spokes    Provider SDK adapter (Anthropic/OpenAI/DeepSeek/Gemini)
        |
   +-----------+
   | chyren-adccl|  Drift/hallucination verification gate (threshold: 0.7)
   +-----------+
        |
     [PASS]                        [FAIL]
        |                             |
   chyren-myelin              Response discarded
   (Qdrant vector store)
        |
   Master Ledger
   (PostgreSQL/Neon, append-only)
        |
   Response returned to caller
```

---

## Medulla Crate Reference

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
| `chyren-cli` | Clap-based CLI + Actix-web API server (port 8080) |
| `chyren-dream` | Long-term memory and pattern synthesis |
| `chyren-metacog` | Self-reflection and introspection |
| `chyren-worldmodel` | Environmental state and context management |
| `chyren-telemetry` | Instrumentation — all significant events must route here |
| `chyren-integration` | Cross-crate routing |
| `chyren-eval` | Evaluation framework |
| `chyren-telegram-gateway` | Telegram bot integration |

---

## Data Layer

### Master Ledger (PostgreSQL / Neon)
- Append-only, cryptographically signed
- Every provider response that passes ADCCL is committed here
- Env var: `CHYREN_DB_URL`
- Never delete `state/` files; history is irreversible

### Myelin (Qdrant Vector Store)
- Persistent semantic memory
- Env var: `QDRANT_URL`

### Phylactery Kernel
- File: `src/cortex/chyren_py/phylactery_kernel.json`
- ~58k synthesized identity entries loaded by Medulla at startup
- Refresh by running: `python src/cortex/chyren_py/identity_synthesis.py`

---

## ADCCL Verification Gate

Every provider response is scored before ledger commit. This gate cannot be bypassed.

- **Score range**: 0.0–1.0
- **Acceptance threshold**: 0.7 — do not lower this
- **Calibration**: starts loose (0.1) and tightens to 0.7 over a 60-minute session
- **No recovery**: rejected responses are discarded, not retried
- **Rejection flags**: `STUB_MARKERS_DETECTED`, `RESPONSE_TOO_SHORT`, `CAPABILITY_REFUSAL`, `NO_TASK_WORD_OVERLAP`

---

## Provider Injection Pattern

All provider spokes receive a composed system prompt containing:
1. Sovereign identity context
2. Yettragrammaton integrity hash
3. Current ledger state (so providers have system history without manual briefing)

---

## Brain Stem CLI Commands

| Command | Routes To | Description |
|---|---|---|
| `./chyren thought "..."` | Medulla | Sovereign reasoning pipeline |
| `./chyren action "..."` | Medulla | Execution, memory, sharding, ingestion |
| `./chyren status` | Medulla | System status |
| `./chyren live` | Medulla | Start web + API |
| `./chyren dream` | Python Cortex scripts | Maintenance: identity synthesis + catalog indexing |

---

## Configuration

All secrets come from `~/.chyren/one-true.env` (not in git). Missing keys fail silently.

```
ANTHROPIC_API_KEY       # Claude API token
OPENAI_API_KEY          # GPT-4 API token
DEEPSEEK_API_KEY        # DeepSeek API token
GEMINI_API_KEY          # Google Gemini API token
CHYREN_DB_URL            # Neon PostgreSQL connection string
QDRANT_URL              # Qdrant vector store URL
```

Source this file before running any Medulla or Cortex commands directly.

---

## What Is Aspirational vs Implemented

The following are **implemented and active** in the current codebase:
- Medulla Rust runtime (all 16 crates above)
- Brain Stem CLI dispatch
- ADCCL verification gate at 0.7 threshold
- PostgreSQL ledger via Neon
- Qdrant semantic memory (Myelin)
- Phylactery kernel loading at startup
- Web frontend (Next.js 15) deployed to Vercel
- Provider spokes: Anthropic, OpenAI, DeepSeek, Gemini

The following are **legacy / reference only** (not invoked at runtime):
- `src/cortex/core/`, `src/cortex/providers/`, `src/cortex/main.py` — original Python orchestration runtime
- `docs/CLAUDE.md` — pre-restructure documentation (see deprecation notice)

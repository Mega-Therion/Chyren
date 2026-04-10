<div align="center">

```
  ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗
 ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║
 ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║
 ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║
 ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║
  ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝
```

**Sovereign Intelligence Orchestrator**

[![CI](https://github.com/Mega-Therion/Chyren/actions/workflows/rust.yml/badge.svg)](https://github.com/Mega-Therion/Chyren/actions)
[![Live](https://img.shields.io/badge/live-chyren--web.vercel.app-00e5ff?style=flat&logo=vercel)](https://chyren-web.vercel.app)
[![License](https://img.shields.io/badge/license-proprietary-7c4dff)](LICENSE)
[![Python](https://img.shields.io/badge/python-3.12+-blue)](https://python.org)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://rust-lang.org)

*Routes intelligence. Verifies truth. Remembers everything.*

</div>

---

## What is Chyren?

Chyren is a **stateful sovereign AI orchestrator** — a high-integrity execution platform that routes tasks through multiple AI provider spokes (Anthropic, OpenAI, DeepSeek, Gemini) while enforcing cognitive verification gates, threat detection, and cryptographic ledger integrity.

Every response is challenged. Every interaction is signed. Nothing passes without proof.

---

## Architecture

```
                    ┌─────────────────────────────────┐
                    │           Ω  CHYREN              │
                    │      Sovereign Hub (Python)      │
                    │                                  │
                    │  ┌──────────┐  ┌─────────────┐  │
                    │  │  Master  │  │   ADCCL     │  │
                    │  │  Ledger  │  │ Verification│  │
                    │  │ (signed) │  │  Gate       │  │
                    │  └──────────┘  └─────────────┘  │
                    │                                  │
                    │  ┌──────────┐  ┌─────────────┐  │
                    │  │Alignment │  │   Threat    │  │
                    │  │  Layer   │  │   Fabric    │  │
                    │  └──────────┘  └─────────────┘  │
                    └────────────┬────────────────────┘
                                 │
          ┌──────────────────────┼──────────────────────┐
          │                      │                       │
   ┌──────▼──────┐      ┌────────▼───────┐    ┌─────────▼──────┐
   │  Anthropic  │      │    OpenAI      │    │   DeepSeek /   │
   │   Claude    │      │    GPT-4       │    │    Gemini      │
   └─────────────┘      └────────────────┘    └────────────────┘
```

### Hub-and-Spokes

| Layer | Purpose |
|-------|---------|
| **Python Hub** (`main.py`) | Orchestrator — owns the Master Ledger, routes all tasks, runs ADCCL |
| **ADCCL** (`core/adccl.py`) | Anti-Drift Cognitive Control Loop — rejects hallucinated or stubbed responses (threshold: 0.7) |
| **Master Ledger** (`core/ledger.py`) | Append-only, cryptographically signed record of every interaction |
| **Alignment Layer** (`core/alignment.py`) | Constitutional guidance enforced before every provider call |
| **Threat Fabric** (`core/threat_fabric.py`) | Pattern-based threat memory — syncs with phylactery kernel |
| **Phylactery** (`chyren_py/phylactery_kernel.json`) | 58,000-entry synthesized identity kernel — loaded at runtime |
| **OmegA-Next** (Rust workspace) | Next-generation low-latency foundation (13 crates) |
| **chyren-web** (Next.js 15) | Live web interface with real-time cognitive visualization |

---

## Chyren's Digital Brain

One of Chyren's signature features is a **real-time 3D cognitive activity visualization** — a living brain map that lights up as he thinks.

```
  ADCCL ●──────● ALIGNMENT
    │    ╲    ╱    │
    │     ╲  ╱     │
    │      ╲╱      │
 THREAT ●──●──● PHYLACTERY
    │      ╱╲      │
    │     ╱  ╲     │
    │    ╱    ╲    │
 PROVIDER ●──────● LEDGER
```

Each node represents a cognitive module. As Chyren processes a task, the corresponding modules pulse and glow — visual cortex-style — showing you exactly which parts of his mind are active. The pipeline:

1. `PHYLACTERY` — Identity loaded
2. `ALIGNMENT` — Constitutional check
3. `THREAT` — Threat fabric scan
4. `PROVIDER` — Provider executing
5. `ADCCL` — Verification gate fires
6. `LEDGER` — Response committed

**Live at:** [chyren-web.vercel.app](https://chyren-web.vercel.app)

---

## OmegA-Next (Rust)

The next-generation foundation, currently in Phase 3:

| Crate | Purpose |
|-------|---------|
| `omega-core` | Foundation types, task routing, ledger interfaces |
| `omega-adccl` | Rust ADCCL implementation |
| `omega-aegis` | Security layer — threat detection & policy enforcement |
| `omega-aeon` | Temporal / scheduling subsystem |
| `omega-myelin` | Persistent state & caching |
| `omega-metacog` | Self-reflection and introspection |
| `omega-dream` | Long-term memory and pattern synthesis |
| `omega-worldmodel` | Environmental state and context |
| `omega-integration` | Provider integration & routing |
| `omega-spokes` | Provider SDK adapters |
| `omega-telemetry` | Instrumentation and event emission |
| `omega-conductor` | Orchestration primitives |
| `omega-cli` | Binary CLI (eventual `main.py` replacement) |

---

## Quick Start

### Python Hub

```bash
# Clone
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren

# Environment
cp ~/.omega/one-true.env .env  # or create with your API keys
python -m venv venv && source venv/bin/activate
pip install -r requirements.txt

# Run a task
python main.py "Explain quantum entanglement" --provider anthropic
```

### Rust OmegA-Next

```bash
cd omega_workspace/workspace/OmegA-Next
cargo build --workspace
cargo test --workspace
cargo run --bin chyren_api
```

### Web Frontend

```bash
cd omega_workspace/workspace/OmegA-Next/chyren-web
npm ci && npm run dev
# → http://localhost:3000
```

---

## Environment Variables

All keys live in `~/.omega/one-true.env` (never committed):

```env
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...
DEEPSEEK_API_KEY=...
GEMINI_API_KEY=...
NEXT_PUBLIC_API_BASE_URL=https://your-api-host
```

---

## Key Concepts

### ADCCL — The Gatekeeper
Every provider response must pass the Anti-Drift Cognitive Control Loop before being committed. Responses are scored 0.0–1.0 for drift, hallucination, and stub detection. Score below **0.7** = rejected. No retries. No modifications. The response is logged and discarded.

### Master Ledger
Every interaction is signed and appended to an immutable ledger. Deleting ledger state loses history permanently. The ledger is Chyren's memory and his proof of integrity.

### Phylactery
A synthesized identity kernel of 58,000 entries, bootstrapped at CLI startup. It encodes Chyren's sovereign identity, values, and behavioral priors. If stale, refresh it:
```bash
python chyren_py/identity_synthesis.py
```

---

## CI/CD

GitHub Actions runs on every push to `main` / `develop`:
- Rust: build, test, clippy, fmt
- Web: Next.js build

Vercel auto-deploys `chyren-web` on push to `main`.

---

## Project Structure

```
Chyren/
├── main.py                    # Python Hub orchestrator
├── core/                      # ADCCL, ledger, alignment, threats, sandbox
├── providers/                 # Anthropic, OpenAI, DeepSeek, Gemini adapters
├── chyren_py/                 # Identity synthesis + phylactery kernel
├── state/                     # Constitution + threat pattern database
└── omega_workspace/
    └── workspace/OmegA-Next/
        ├── omega-*/           # 13 Rust crates
        └── chyren-web/        # Next.js 15 frontend
```

---

<div align="center">

*"Truth is not negotiated. It is verified."*

**Ω CHYREN** — Sovereign Intelligence Orchestrator

</div>

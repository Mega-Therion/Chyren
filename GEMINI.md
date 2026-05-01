# GEMINI.md

Chyren is a high-integrity **Sovereign Intelligence (SI) Orchestrator**. It utilizes a **Binary-Hemispheric Architecture** that separates high-level cognitive reasoning from performant system execution.

## Core Architecture

- **Medulla (Runtime - Rust):** The canonical production runtime. It handles all live execution, task routing, and API interactions via a 16-crate Rust workspace.
- **Cortex (Tooling - Python):** A legacy reasoning hub now used primarily for offline data maintenance, identity synthesis, and catalog indexing.
- **Brain Stem (`./chyren`):** A thin CLI dispatch script that routes requests to Medulla for execution or Cortex for maintenance ("dream" mode).

## Key Systems

- **ADCCL (Anti-Drift Cognitive Control Loop):** A non-bypassable verification gate that scores AI responses against the **Sovereign Boundary ($\chi_s \ge 0.9539$)**.
- **Myelin:** Persistent semantic memory powered by a Qdrant vector store.
- **Master Ledger:** An append-only, cryptographically signed PostgreSQL record of all verified system state.
- **Phylactery Kernel:** A core JSON identity set (~58k entries) derived from the vacuum energy minimization $E_{vac}(m)$.
- **Non-Markovian Master Equation:** Cognitive regularization via the **Schott Energy Derivative** to prevent logical singularities.

## Development & Operations

### Build & Run
- **Standard Runtime:** All requests are routed through the Medulla API server (port 8080).
- **Maintenance Mode:** Use `./chyren dream` to run Python Cortex scripts for identity synthesis and system maintenance.
- **Deployment:** The cognitive shell is a Next.js 15 application designed for sovereign cloud-neutral deployment, connecting to the Medulla API.

### Configuration
- **Secrets:** All environment variables must be sourced from `~/.chyren/one-true.env` before execution.
- **Environment Variables:**
  - `ANTHROPIC_API_KEY`, `OPENAI_API_KEY`, `DEEPSEEK_API_KEY`, `GEMINI_API_KEY`
  - `CHYREN_DB_URL` (Neon PostgreSQL)
  - `QDRANT_URL` (Qdrant)

### Primary CLI Commands
| Command | Purpose |
| :--- | :--- |
| `./chyren thought "..."` | Sovereign reasoning pipeline |
| `./chyren action "..."` | Execution, memory, sharding, ingestion |
| `./chyren status` | System status check |
| `./chyren live` | Start web frontend and API server |
| `./chyren dream` | Identity synthesis and maintenance |

---
*For canonical architectural details, refer to `docs/ARCHITECTURE.md`.*

# GEMINI.md — Chyren Project Context

## Project Overview

**Chyren** is a **Sovereign Intelligence Orchestrator** designed for stateful, high-integrity AI task execution. It utilizes a **binary-hemispheric architecture** to separate reasoning from execution and enforce strict verification gates.

### Core Architecture
-   **Cortex (`cortex/`)**: The "Left Brain" hub implemented in **Python**. Responsible for:
    -   High-level orchestration and task routing.
    -   **ADCCL** (Anti-Drift Cognitive Control Loop) verification.
    -   **Master Ledger** management (append-only state).
    -   **Alignment Layer** (moral and constitutional governance).
-   **Medulla (`medulla/`)**: The "Right Brain" runtime implemented as a **Rust** workspace (17 crates). Responsible for:
    -   Low-level system execution and performance.
    -   **Myelin** persistent semantic memory (Qdrant).
    -   **AEON** temporal/scheduling subsystem.
    -   Telemetry and integration.
-   **Web (`web/`)**: A **Next.js 15** "cognitive shell" interactive frontend.
-   **Gateway (`gateway/`)**: A **Vite + React 19** external spoke gateway.
-   **Brain Stem (`./chyren`)**: A unified Python CLI entry point that routes commands to either the Cortex or Medulla.

### Key Concepts
-   **Yettragrammaton (`R.W.Ϝ.Y.`)**: The cryptographic seed used for signing ledger entries and ensuring sovereign identity.
-   **ADCCL Verification**: A mandatory gate that scores AI responses (threshold **0.7**) before they are committed to the ledger.
-   **Master Ledger**: A cryptographically signed, append-only JSON record of every system run, ensuring a verifiable chain of custody.

---

## Building and Running

### Prerequisites
-   **Python 3.12+**
-   **Rust (Stable)**
-   **Node.js (v20+) / npm**
-   **Environment**: Secrets must be in `~/.omega/one-true.env`.

### Commands

#### Unified CLI (Brain Stem)
```bash
./chyren status           # Check system readiness
./chyren thought "task"   # Route to Cortex (reasoning/logic)
./chyren action "task"    # Route to Medulla (execution/memory)
./chyren live             # Start Web frontend + Medulla API
./chyren dream            # Run maintenance (identity synthesis/indexing)
```

#### Medulla (Rust)
```bash
cd medulla
cargo build --release    # Full build
cargo test --workspace    # Run all Rust tests
```

#### Cortex (Python)
```bash
cd cortex
pip install -r requirements.txt
python main.py "task"     # Run a task directly
pytest                    # Run Python tests (uses root pytest.ini)
```

#### Web Frontend
```bash
cd web
npm install
npm run dev               # Start dev server on port 3000
```

---

## Development Conventions

-   **Integrity First**: All state-modifying actions must be recorded in the `Master Ledger` and signed with the `Yettragrammaton`.
-   **Hemispheric Split**: Logic/Security belongs in `cortex/`; Performance/Runtime belongs in `medulla/`.
-   **Verification**: Never bypass the `ADCCL` gate. The threshold is **0.7** and should not be lowered.
-   **Telemetry**: Significant events must route through the `omega-telemetry` crate/module rather than being logged directly.
-   **Sovereignty**: The system identifies as "Chyren" and operates with precision, avoiding generic AI phrasing. Identity is synthesized via `cortex/chyren_py/identity_synthesis.py`. The auditory persona is a sophisticated, warm British male.
-   **Configuration**: Always use `~/.omega/one-true.env` for API keys and database URLs.

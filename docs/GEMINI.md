# Gemini CLI Context: Chyren

## Project Overview
Chyren is a **Sovereign Intelligence Orchestrator** designed for high-integrity AI task execution. It operates as a stateful hub that routes tasks through multiple provider spokes (Anthropic, OpenAI, DeepSeek, Gemini) while enforcing strict cognitive and security gates.

The project is currently in a **transitional state**, moving from a modular Python-based prototype to a next-generation Rust-based architecture (OmegA).

### Key Components
- **The Hub (`main.py`):** The central orchestrator managing state, providers, and the Master Ledger.
- **ADCCL (`core/adccl.py`):** Anti-Drift Cognitive Control Loop. A mechanical gatekeeper that verifies provider responses for drift, hallucinations, and stubs before committing to the ledger.
- **Master Ledger (`core/ledger.py`):** The single source of truth for all system interactions and state.
- **Security Fabric:** Includes `core/alignment.py` (constitutional guidance), `core/deflection.py` (threat response), and `core/threat_fabric.py` (pattern-based threat memory).
- **OmegA Next-Gen (`omega_workspace/`):** A Rust workspace being scaffolded to provide a more robust, low-latency foundation for the cognitive OS.

## Technical Stack
- **Primary Language:** Python 3.12+ (Core Logic), Rust (Next-Gen Workspace)
- **Architecture:** Modular Provider Spokes + Centralized Stateful Hub
- **Providers:** Anthropic (Claude), OpenAI (GPT-4), DeepSeek, Google Gemini
- **Dependencies:** `python-dotenv`, `urllib` (for direct provider calls), `Cargo` (Rust builds)

## Development Workflows

### Building and Running (Python Hub)
- **Setup:** Create a virtual environment and install dependencies:
  ```bash
  python -m venv venv
  source venv/bin/activate
  pip install -r requirements.txt
  ```
- **Execution:** Run the hub with a task:
  ```bash
  python main.py "Your task here" --provider anthropic
  ```
### Docker Environment (OmegA-Next)
- **Launch Full Stack:** Starts API, Web, Postgres, Redis, Qdrant, and Monitoring.
  ```bash
  ./scripts/docker-manager.sh up
  ```
- **Building Images:**
  ```bash
  ./scripts/docker-manager.sh build
  ```
- **View Logs:**
  ```bash
  ./scripts/docker-manager.sh logs
  ```

### Configuration:
- API keys and project settings are loaded from `~/.omega/one-true.env`.

### Next-Gen Architecture (Rust/OmegA)
- **Scaffolding:** Use the bootstrap script to initialize the Rust workspace:
  ```bash
  bash bootstrap_omega_next.sh --new /path/to/workspace
  ```
- **Building Workspace:**
  ```bash
  cd omega_workspace/workspace/OmegA-Next
  cargo build
  ```
- **Launching CLI:**
  ```bash
  ./scripts/launch-chyren.sh run "Task"
  ```

## Development Conventions
- **Integrity First:** The `Yettragrammaton` (found in `core/integrity.py`) is the root of system integrity. Never bypass ADCCL verification.
- **Zero-Downtime Migration:** When porting Python logic to Rust, use the `legacy_bridge.rs` pattern described in `legacy_intake/MIGRATION_SUMMARY.md`.
- **Telemetry:** All significant events must be emitted using the schema defined in `omega-telemetry`.
- **No Hallucinations:** Every response must be verifiable. If a provider response is rejected by ADCCL (score < 0.7), it is logged but discarded.

## Directory Map
- `/core`: Fundamental logic for integrity, alignment, and verification.
- `/providers`: Adapter layer for various LLM APIs.
- `/state`: Persistent storage for the constitution and threat patterns.
- `/omega_workspace`: The future of the project; a high-performance Rust ecosystem.
- `/chyren_py`: Supporting Python scripts for identity synthesis and kernel management.

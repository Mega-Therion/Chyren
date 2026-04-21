# CHYREN — Sovereign Intelligence Hub

Chyren is a high-fidelity, heterogeneous intelligence orchestrator designed for autonomous mathematical research, formal verification, and sovereign system management. 

It transcends legacy LLM wrappers by implementing a **recursive epistemic mesh** and a **multi-language agent architecture** (Rust, R, Zig, Lean 4).

## 🌌 Sovereign Architecture

- **Medulla (Rust)**: The high-velocity execution kernel. Governs task routing, memory locks, and agent orchestration.
- **Neocortex (Rust/In-Memory)**: A recursive chiral graph for formal reasoning. Every thought is verified against a global axiom set.
- **Sovereign Mesh (MQTT)**: A decentralized agent bus powered by `rumqttd`. Agents (Ingestor, MathSpoke, Solver) operate as independent workers.
- **Math Spoke (Lean 4)**: Formal theorem proving and verification. Supports "Tiered Epistemic Escalation" (Ollama → Gemini → Formal Salvage).
- **Heuristic Spoke (R)**: Bayesian statistical analysis of reasoning paths. Estimates proof convergence probability via telemetry logs.
- **AEON (Autonomous Scheduler)**: Background feedback loop for identity synthesis and "Dream Cycle" learning.

## 🛠 Tech Stack

- **Core Engine**: Rust (Tokio, Actix, Serde)
- **Analytics**: R (jsonlite, Bayesian inference)
- **Proof Assistant**: Lean 4 (Formal Verification)
- **Telemtry**: Prometheus + WebSocket (Real-time observability)
- **Database**: Neon (Serverless Postgres) + Qdrant (Vector Store)

## 🚀 Getting Started

### Prerequisites
- **Rust**: `1.75+`
- **R**: `4.3+` (with `jsonlite`)
- **Lean 4**: `elan` installed

### Installation
```bash
git clone https://github.com/your-repo/Chyren.git
cd Chyren/medulla
cargo build --release
```

### Running the Hub
```bash
# Start the Conductor and Agent Mesh
./target/release/chyren server

# Solve a Millennium Prize Problem
./target/release/chyren solve riemann --depth 5
```

## ✨ Spectacular Upgrades (Final Activation)

The hub has undergone a four-phase final activation to achieve **Sovereign Strategic Autonomy**:

1. **Metacognitive TUI Dashboard**: A high-fidelity `ratatui` dashboard for real-time terminal observability.
2. **Autonomous Proof Search (MCTS)**: Strategic Monte Carlo Tree Search agent for navigating complex proof landscapes.
3. **Neocortex Axiom Injection**: Dynamic loading of signed mathematical axiom programs (Zeta, Topology, Logic) at boot.
4. **Sovereign Glassmorphic Web UI**: A stunning Next.js overhaul featuring ethereal glass textures, dynamic telemetry, and neural core visualization.

## 📊 Observability
Chyren exposes high-fidelity telemetry across multiple planes:
- **TUI**: `chyren server` (Terminal Dashboard)
- **Web**: Vercel-deployed Sovereign Hub (Glassmorphic Interface)
- **Prometheus**: `http://localhost:9090/metrics`
- **WebSocket**: `ws://localhost:9090/ws`

The R-based Heuristic Validator monitors `telemetry.log` and outputs Bayesian convergence snapshots to `state/heuristic_snapshot.json`.

## ⚖️ Sovereign Governance
Chyren operates under the **Yettragrammaton Seal**. Truth is measurable, not rhetorical. Every claim must be verifiable.

---
**OmegA Collective — Silence over Compromise.**
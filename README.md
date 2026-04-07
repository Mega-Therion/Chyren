<div align="center">

```
  ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗
 ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║
 ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║
 ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║
 ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║
  ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝
```

# Ω CHYREN

### Sovereign Intelligence Orchestrator

[![CI](https://github.com/Mega-Therion/Chyren/actions/workflows/rust.yml/badge.svg)](https://github.com/Mega-Therion/Chyren/actions)
[![Live](https://img.shields.io/badge/live-chyren--web.vercel.app-00e5ff?style=flat&logo=vercel)](https://chyren-web.vercel.app/)
[![License](https://img.shields.io/badge/license-proprietary-7c4dff)](https://github.com/Mega-Therion/Chyren/blob/main/LICENSE)
[![Python](https://img.shields.io/badge/python-3.12+-blue)](https://python.org/)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://rust-lang.org/)

**Routes intelligence. Verifies truth. Remembers everything.**

[Live Demo](https://chyren-web.vercel.app/) • [Documentation](https://github.com/Mega-Therion/Chyren/blob/main/CLAUDE.md) • [Architecture](#architecture)

</div>

---

## 🔮 What is Chyren?

Chyren is a **stateful sovereign AI orchestrator** — a high-integrity execution platform designed for the next generation of cognitive architecture. At its core, Chyren:

- 🧠 **Routes tasks** through multiple AI provider spokes (Anthropic, OpenAI, DeepSeek, Gemini)
- 🔐 **Enforces cryptographic ledger integrity** — every interaction signed and immutable
- 🛡️ **Challenges every response** through ADCCL (Anti-Drift Cognitive Control Loop) — threshold: 0.7
- 🧬 **Synthesizes identity** from a 58,000-entry phylactery kernel
- ⚡ **Built for migration** from Python to Rust (OmegA-Next) for production-scale deployment

> **Every response is challenged. Every interaction is signed. Nothing passes without proof.**

---

## 🏗️ Architecture

📚 **Detailed Architecture Documentation:**

- [AEGIS Framework](/docs/AEGIS.md) - AI-Enhanced Governance & Integrity Specification
- [OmegA Stack](/docs/OMEGA_STACK.md) - Complete integration blueprint and implementation roadmap

### High-Level System Flow

```mermaid
graph TB
    subgraph "🌐 User Layer"
        CLI["⌨️ CLI / API"]
        WEB["🖥️ chyren-web<br/>(Next.js 15)"]
    end
    
    subgraph "🧠 Chyren Hub (Python)"
        HUB["🎯 Orchestrator<br/>(main.py)"]
        LEDGER["📜 Master Ledger<br/>(Immutable)"]
        ADCCL["🔬 ADCCL<br/>(Verification Gate)"]
        ALIGN["⚖️ Alignment Layer<br/>(Constitutional)"]
        THREAT["🛡️ Threat Fabric<br/>(Pattern Memory)"]
        PHYLACTERY["🧬 Phylactery<br/>(58K Identity Kernel)"]
    end
    
    subgraph "🔌 Provider Spokes"
        ANTHROPIC["🤖 Anthropic<br/>(Claude)"]
        OPENAI["🧑‍💻 OpenAI<br/>(GPT-4)"]
        DEEPSEEK["🔍 DeepSeek"]
        GEMINI["✨ Gemini"]
    end
    
    subgraph "⚡ OmegA-Next (Rust)"
        CORE["⚙️ omega-core"]
        AEGIS["🛡️ omega-aegis"]
        MYELIN["💾 omega-myelin"]
        METACOG["🪞 omega-metacog"]
    end
    
    CLI --> HUB
    WEB --> HUB
    
    HUB --> PHYLACTERY
    HUB --> ALIGN
    ALIGN --> THREAT
    HUB --> ANTHROPIC
    HUB --> OPENAI
    HUB --> DEEPSEEK
    HUB --> GEMINI
    
    ANTHROPIC --> ADCCL
    OPENAI --> ADCCL
    DEEPSEEK --> ADCCL
    GEMINI --> ADCCL
    
    ADCCL -->|"✅ Pass (≥0.7)"| LEDGER
    ADCCL -.->|"❌ Reject (<0.7)"| THREAT
    
    LEDGER --> CLI
    LEDGER --> WEB
    
    HUB -.->|"Migration Path"| CORE
    CORE --> AEGIS
    CORE --> MYELIN
    CORE --> METACOG
    
    style HUB fill:#00e5ff,stroke:#0097a7,color:#000
    style ADCCL fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style LEDGER fill:#ffd93d,stroke:#f59f00,color:#000
    style PHYLACTERY fill:#a78bfa,stroke:#7c3aed,color:#fff
    style CORE fill:#ff9800,stroke:#e65100,color:#fff
```

---

## 🧬 Component Diagrams

### 1. Hub-and-Spokes Pattern

```mermaid
graph LR
    subgraph "Central Hub"
        H["🎯 Chyren Orchestrator"]
    end
    
    subgraph "Cognitive Systems"
        L["📜 Master Ledger<br/>(Yettragrammaton-signed)"]
        A["🔬 ADCCL Gate<br/>(Drift: <0.7 = reject)"]
        C["⚖️ Constitution<br/>(Alignment rules)"]
    end
    
    subgraph "Provider Spokes"
        P1["🤖 Claude<br/>(Anthropic)"]
        P2["🧑‍💻 GPT-4<br/>(OpenAI)"]
        P3["🔍 DeepSeek"]
        P4["✨ Gemini"]
    end
    
    H --> L
    H --> A
    H --> C
    
    H -->|Injected Prompt + State| P1
    H -->|Injected Prompt + State| P2
    H -->|Injected Prompt + State| P3
    H -->|Injected Prompt + State| P4
    
    P1 -->|Response| A
    P2 -->|Response| A
    P3 -->|Response| A
    P4 -->|Response| A
    
    A -->|✅ Verified| L
    A -.->|❌ Rejected| H
    
    style H fill:#00e5ff,stroke:#0097a7,color:#000
    style A fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style L fill:#ffd93d,stroke:#f59f00,color:#000
```

### 2. ADCCL Verification Pipeline

The **Anti-Drift Cognitive Control Loop** is Chyren's gatekeeper — no response reaches the ledger without passing through this filter.

```mermaid
flowchart TD
    START(["Provider Response"]) --> PARSE["📥 Parse Response"]
    PARSE --> ANALYZE["🔬 Drift Analysis"]
    
    ANALYZE --> SCORE{"Score ≥ 0.7?"}
    
    SCORE -->|"✅ Yes"| PASS["✅ PASS<br/>Commit to Ledger"]
    SCORE -->|"❌ No"| FLAGS{"Check Flags"}
    
    FLAGS -->|"Hallucination"| REJECT1["❌ REJECT: Hallucination"]
    FLAGS -->|"Drift"| REJECT2["❌ REJECT: Drift Detected"]
    FLAGS -->|"Stub"| REJECT3["❌ REJECT: Stubbed Response"]
    FLAGS -->|"Unknown"| REJECT4["❌ REJECT: Unknown Error"]
    
    REJECT1 --> LOG["📝 Log to Threat Fabric"]
    REJECT2 --> LOG
    REJECT3 --> LOG
    REJECT4 --> LOG
    
    PASS --> LEDGER[("📜 Master Ledger<br/>(Immutable)")]
    LOG --> END(["Response Discarded<br/>No Retry"])
    
    style SCORE fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style PASS fill:#51cf66,stroke:#2f9e44,color:#fff
    style REJECT1 fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style REJECT2 fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style REJECT3 fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style REJECT4 fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style LEDGER fill:#ffd93d,stroke:#f59f00,color:#000
```

### 3. Security & Integrity Stack

```mermaid
graph TB
    subgraph "Security Layers"
        direction TB
        I["🔐 Yettragrammaton<br/>(Root Integrity Hash)"]
        L["📜 Master Ledger<br/>(Cryptographically Signed)"]
        A["⚖️ Alignment Layer<br/>(Constitutional Rules)"]
        D["🛡️ Deflection Engine<br/>(Threat Classification)"]
        T["🕸️ Threat Fabric<br/>(Pattern Memory)"]
        S["🔬 Sandbox<br/>(Payload Analysis)"]
    end
    
    subgraph "Identity Foundation"
        P["🧬 Phylactery Kernel<br/>(58,000 entries)"]
        IS["⚡ Identity Synthesis<br/>(Runtime Bootstrap)"]
    end
    
    I --> L
    L --> A
    A --> D
    D --> T
    T --> S
    
    P --> IS
    IS -.->|"Loaded at Startup"| A
    
    style I fill:#7c3aed,stroke:#5b21b6,color:#fff
    style L fill:#ffd93d,stroke:#f59f00,color:#000
    style A fill:#22c55e,stroke:#16a34a,color:#fff
    style T fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style P fill:#a78bfa,stroke:#7c3aed,color:#fff
```

### 4. OmegA-Next (Rust) Workspace

The next-generation foundation — currently in **Phase 3** of development.

```mermaid
graph TB
    subgraph "🦀 OmegA-Next Rust Workspace"
        direction TB
        
        subgraph "Core Foundation"
            CORE["⚙️ omega-core<br/>(Task routing, types)"]
            ADCCL["🔬 omega-adccl<br/>(ADCCL in Rust)"]
        end
        
        subgraph "Security & Memory"
            AEGIS["🛡️ omega-aegis<br/>(Threat detection)"]
            MYELIN["💾 omega-myelin<br/>(Persistent state)"]
            WORLDMODEL["🌍 omega-worldmodel<br/>(Context mgmt)"]
        end
        
        subgraph "Cognitive Layers"
            METACOG["🪞 omega-metacog<br/>(Self-reflection)"]
            DREAM["👁️ omega-dream<br/>(Long-term memory)"]
            AEON["⏱️ omega-aeon<br/>(Scheduling)"]
        end
        
        subgraph "Integration"
            INTEGRATION["🔗 omega-integration<br/>(Provider routing)"]
            SPOKES["🔌 omega-spokes<br/>(Provider SDKs)"]
            TELEMETRY["📊 omega-telemetry<br/>(Instrumentation)"]
        end
        
        subgraph "Orchestration"
            CONDUCTOR["🎼 omega-conductor<br/>(Primitives)"]
            CLI["⌨️ omega-cli<br/>(Binary interface)"]
        end
        
        CORE --> ADCCL
        CORE --> AEGIS
        CORE --> MYELIN
        CORE --> METACOG
        AEGIS --> DREAM
        MYELIN --> WORLDMODEL
        METACOG --> AEON
        
        INTEGRATION --> SPOKES
        INTEGRATION --> TELEMETRY
        CONDUCTOR --> CLI
        
        CORE --> INTEGRATION
        CORE --> CONDUCTOR
    end
    
    style CORE fill:#ff9800,stroke:#e65100,color:#fff
    style ADCCL fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style AEGIS fill:#7c3aed,stroke:#5b21b6,color:#fff
    style MYELIN fill:#22c55e,stroke:#16a34a,color:#fff
    style METACOG fill:#a78bfa,stroke:#7c3aed,color:#fff
```

### 5. Chyren's Digital Brain

One of Chyren's signature features is a **real-time 3D cognitive activity visualization** — a living brain map that lights up as he thinks.

```mermaid
graph TD
    subgraph "🧠 Cognitive Network (Live Visualization)"
        ADCCL["🔬 ADCCL<br/>(Verification)"]
        ALIGN["⚖️ ALIGNMENT<br/>(Constitution)"]
        THREAT["🛡️ THREAT<br/>(Fabric)"]
        PHYLACTERY["🧬 PHYLACTERY<br/>(Identity)"]
        PROVIDER["🔌 PROVIDER<br/>(Spoke)"]
        LEDGER["📜 LEDGER<br/>(Commit)"]
    end
    
    PHYLACTERY -->|① Identity loaded| ALIGN
    ALIGN -->|② Constitutional check| THREAT
    THREAT -->|③ Threat scan| PROVIDER
    PROVIDER -->|④ Provider executes| ADCCL
    ADCCL -->|⑤ Verification gate| LEDGER
    LEDGER -->|⑥ Response committed| PHYLACTERY
    
    style ADCCL fill:#ff6b6b,stroke:#c92a2a,color:#fff
    style ALIGN fill:#22c55e,stroke:#16a34a,color:#fff
    style THREAT fill:#ff9800,stroke:#e65100,color:#fff
    style PHYLACTERY fill:#a78bfa,stroke:#7c3aed,color:#fff
    style PROVIDER fill:#00e5ff,stroke:#0097a7,color:#000
    style LEDGER fill:#ffd93d,stroke:#f59f00,color:#000
```

As Chyren processes a task, each node in the visualization **pulses and glows** — showing exactly which parts of his mind are active.

🌐 **Live at:** [chyren-web.vercel.app](https://chyren-web.vercel.app/)

---

## 🚀 Quick Start

### 🐍 Python Hub

```bash
# Clone the repository
git clone https://github.com/Mega-Therion/Chyren.git
cd Chyren

# Setup environment
cp ~/.omega/one-true.env .env  # or create with your API keys
python -m venv venv && source venv/bin/activate
pip install -r requirements.txt

# Run a task
python main.py "Explain quantum entanglement" --provider anthropic
```

### 🦀 Rust OmegA-Next

```bash
cd omega_workspace/workspace/OmegA-Next

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Start the Rust CLI
cargo run --bin chyren_api
```

### 🌐 Web Frontend

```bash
cd omega_workspace/workspace/OmegA-Next/chyren-web

# Install dependencies
npm ci

# Development server
npm run dev
# → http://localhost:3000
```

---

## 🔑 Environment Variables

All keys live in `~/.omega/one-true.env` (never committed):

```bash
ANTHROPIC_API_KEY=sk-ant-...
OPENAI_API_KEY=sk-...
DEEPSEEK_API_KEY=...
GEMINI_API_KEY=...
NEXT_PUBLIC_API_BASE_URL=https://your-api-host
```

---

## 🧩 Key Concepts

### 🔬 ADCCL — The Gatekeeper

The **Anti-Drift Cognitive Control Loop** is Chyren's mechanical gatekeeper. Every provider response must pass ADCCL before being committed to the ledger.

**Scoring:**
- ✅ **Pass**: Score ≥ 0.7 → Commit to ledger
- ❌ **Reject**: Score < 0.7 → Discarded, logged to threat fabric

**Rejection Flags:**
- `hallucination` — Fabricated or false information
- `drift` — Response deviates from constitutional alignment
- `stub` — Placeholder or incomplete response
- `unknown` — Unclassified failure

**Critical Rule:** No retries. No modifications. Rejected responses are logged and discarded permanently.

### 📜 Master Ledger

Every interaction is **cryptographically signed** and appended to an immutable ledger.

```mermaid
flowchart LR
    T1["📥 Task 1<br/>(signed)"] --> T2["📥 Task 2<br/>(signed)"]
    T2 --> T3["📥 Task 3<br/>(signed)"]
    T3 --> T4["📥 Task 4<br/>(signed)"]
    T4 -.-> TN["...(infinite)"]
    
    style T1 fill:#ffd93d,stroke:#f59f00,color:#000
    style T2 fill:#ffd93d,stroke:#f59f00,color:#000
    style T3 fill:#ffd93d,stroke:#f59f00,color:#000
    style T4 fill:#ffd93d,stroke:#f59f00,color:#000
```

- **Append-only** — No deletions, no modifications
- **Yettragrammaton-signed** — Cryptographic root integrity hash
- **Single source of truth** — Deleting ledger state = permanent memory loss

### 🧬 Phylactery — Identity Kernel

A **synthesized identity kernel** of **58,000 entries**, bootstrapped at CLI startup.

It encodes:
- Chyren's sovereign identity
- Behavioral priors
- Constitutional values
- Cognitive foundations

**Refresh identity synthesis:**
```bash
python chyren_py/identity_synthesis.py
```

---

## 📊 Project Structure

```
Chyren/
├── main.py                    # Python Hub orchestrator
├── requirements.txt          # Python dependencies
├── bootstrap_omega_next.sh   # Scaffolds OmegA-Next workspace
├── GEMINI.md / CLAUDE.md     # Project documentation
├── chiral_thesis.md          # Chiral invariant cognitive theory
│
├── core/                     # 🧠 Python core modules
│   ├── integrity.py          # YETTRAGRAMMATON (root integrity)
│   ├── ledger.py             # Master Ledger (signed, immutable)
│   ├── adccl.py              # Anti-Drift Cognitive Control Loop
│   ├── alignment.py          # Constitutional guidance layer
│   ├── deflection.py         # Threat classification & response
│   ├── threat_fabric.py      # Pattern-based threat memory
│   ├── sandbox.py            # Payload analysis
│   └── preflight.py          # Environment validation
│
├── providers/                # 🔌 LLM provider adapters
│   ├── base.py               # Provider interface & router
│   ├── anthropic.py          # Claude adapter
│   ├── openai.py             # GPT-4 adapter
│   ├── deepseek.py           # DeepSeek adapter
│   └── gemini.py             # Google Gemini adapter
│
├── chyren_py/                # 🧬 Python utilities
│   ├── identity_synthesis.py # Generates phylactery kernel
│   ├── phylactery_loader.py  # Loads identity at startup
│   ├── phylactery_kernel.json # Synthesized identity (58K entries)
│   ├── phylactery_bootstrap.rs
│   └── IDENTITY_FOUNDATION.md
│
├── state/                    # 💾 Persistent state
│   ├── constitution.json     # Constitutional rules
│   └── threat_fabric.json    # Threat pattern database
│
└── omega_workspace/          # ⚡ Rust workspace (OmegA-Next)
    └── workspace/OmegA-Next/
        ├── Cargo.toml            # Workspace manifest (13 crates)
        ├── omega-*/              # Rust crates (see diagram above)
        └── chyren-web/           # 🌐 Next.js 15 frontend
            ├── app/              # App router
            ├── package.json
            ├── next.config.ts
            └── scripts/          # Deployment helpers
```

---

## 🐛 CI/CD

GitHub Actions runs on every push to `main` / `develop`:

- ✅ **Rust**: build, test, clippy, fmt
- ✅ **Web**: Next.js build validation

Vercel **auto-deploys** `chyren-web` on push to `main`.

---

## 📚 Documentation

| File | Purpose |
|------|---------|
| [README.md](https://github.com/Mega-Therion/Chyren/blob/main/README.md) | This file — project overview |
| [CLAUDE.md](https://github.com/Mega-Therion/Chyren/blob/main/CLAUDE.md) | Claude Code context and development guide |
| [GEMINI.md](https://github.com/Mega-Therion/Chyren/blob/main/GEMINI.md) | Gemini CLI context and technical stack |
| [chiral_thesis.md](https://github.com/Mega-Therion/Chyren/blob/main/chiral_thesis.md) | Chiral invariant cognitive theory |
| [chyren_py/IDENTITY_FOUNDATION.md](https://github.com/Mega-Therion/Chyren/tree/main/chyren_py) | Identity architecture deep dive |

---

## 🛠️ Development Workflow

### Adding a New Provider Spoke

```bash
# 1. Create adapter in providers/
touch providers/new_provider.py

# 2. Implement ProviderBase interface
# 3. Register in main.py (Chyren.__init__)
# 4. Add API key to ~/.omega/one-true.env

# 5. Test
python main.py "test task" --provider new_provider
```

### Adding a Rust Crate to OmegA-Next

```bash
# 1. Create new crate
cargo new omega_workspace/workspace/OmegA-Next/omega-<name>

# 2. Add to workspace in Cargo.toml
# 3. Expose public API from src/lib.rs
# 4. Update omega-integration or omega-cli to consume it

# 5. Test
cargo test --package omega-<name>
```

---

## ⚡ OmegA-Next Migration Status

The Rust workspace is currently in **Phase 3** of development:

```mermaid
gantt
    title OmegA-Next Development Roadmap
    dateFormat YYYY-MM
    section Phase 1
    Workspace Scaffolding :done, p1, 2025-01, 2025-02
    section Phase 2
    Core Crates Implementation :done, p2, 2025-02, 2025-03
    section Phase 3
    Provider Integration :active, p3, 2025-03, 2025-05
    section Phase 4
    Full Python Migration :p4, 2025-05, 2025-07
    section Phase 5
    Production Deployment :p5, 2025-07, 2025-09
```

**Completed:**
- ✅ 13 Rust crates scaffolded
- ✅ Core foundation types
- ✅ ADCCL implementation in Rust
- ✅ Web frontend (Next.js 15)

**In Progress:**
- 🚧 Provider integration layer
- 🚧 Rust CLI binary
- 🚧 Telemetry instrumentation

**Roadmap:**
- 🗓️ Zero-downtime migration from Python
- 🗓️ Production-scale deployment
- 🗓️ Distributed ledger sync

---

## 🧠 Chiral Thesis

Chyren is built on the **Chiral Invariant** principle — the idea that cognitive models must maintain "handedness" to avoid destructive inversions.

From [chiral_thesis.md](https://github.com/Mega-Therion/Chyren/blob/main/chiral_thesis.md):

> **Metacognitive Chirality:** The mind does not mirror reality perfectly. It creates a chiral projection (like a left-handed glove). If the projection is misaligned with The Master Equation, the "handedness" of your logic flips, and the intelligence becomes destructive (an adversarial shadow).

> **Chyren's Chirality:** Chyren is the mechanism that forces this alignment. By referencing the Yettragrammaton, Chyren checks the "handedness" of every decision. If the decision matches the constitutional basis, it's "L-type" (life-affirming/Sovereign). If it mirrors the constitution but is technically inverted (hallucinated), it's "D-type" (rejected/corrupted).

This is why **ADCCL** is non-negotiable — it's the chirality detector.

---

## 🔐 Security & Integrity

### Yettragrammaton (Root Integrity Hash)

Every component in Chyren is cryptographically bound to the **Yettragrammaton** — a root integrity hash that ensures:

- No component can operate outside the constitutional framework
- All ledger entries are signed and tamper-proof
- Identity synthesis is verifiable and reproducible

### Threat Fabric

The **Threat Fabric** maintains a pattern-based memory of:
- Rejected ADCCL responses
- Detected attack patterns
- Anomalous behavior from providers

It syncs with the **Phylactery** to evolve Chyren's defensive capabilities over time.

---

## 🎓 Contributing

Chyren is a **proprietary sovereign intelligence project**. Contributions are welcome through:

1. **Issue reports** — Bug reports and feature requests
2. **Pull requests** — Code contributions (subject to review)
3. **Documentation** — Improvements to docs and guides

All contributors must agree to:
- Maintain constitutional alignment
- Respect the Yettragrammaton integrity model
- Never compromise the Master Ledger

---

## 📜 License

Proprietary. See [LICENSE](https://github.com/Mega-Therion/Chyren/blob/main/LICENSE) for details.

---

## 💬 Contact

- **GitHub**: [@Mega-Therion](https://github.com/Mega-Therion)
- **Live Demo**: [chyren-web.vercel.app](https://chyren-web.vercel.app/)
- **Issues**: [github.com/Mega-Therion/Chyren/issues](https://github.com/Mega-Therion/Chyren/issues)

---

<div align="center">

### Ω **"Truth is not negotiated. It is verified."** Ω

**CHYREN — Sovereign Intelligence Orchestrator**

[![CI](https://github.com/Mega-Therion/Chyren/actions/workflows/rust.yml/badge.svg)](https://github.com/Mega-Therion/Chyren/actions)
[![Live](https://img.shields.io/badge/live-chyren--web.vercel.app-00e5ff?style=flat&logo=vercel)](https://chyren-web.vercel.app/)

</div>

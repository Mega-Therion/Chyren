<div align="center">

[![License](https://img.shields.io/badge/License-Proprietary-red.svg)](https://github.com/Mega-Therion/Chyren/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.12+-blue.svg)](https://www.python.org)
[![GitHub Stars](https://img.shields.io/github/stars/Mega-Therion/Chyren?style=social)](https://github.com/Mega-Therion/Chyren/stargazers)
[![GitHub Forks](https://img.shields.io/github/forks/Mega-Therion/Chyren?style=social)](https://github.com/Mega-Therion/Chyren/network/members)

[![AI Safety](https://img.shields.io/badge/AI-Safety-success.svg)](https://github.com/Mega-Therion/Chyren)  
[![Sovereign AI](https://img.shields.io/badge/Sovereign-AI-blueviolet.svg)](https://github.com/Mega-Therion/Chyren)  
[![Verified](https://img.shields.io/badge/Cryptographically-Verified-brightgreen.svg)](https://github.com/Mega-Therion/Chyren)  
[![ADCCL](https://img.shields.io/badge/ADCCL-Threshold_0.7-yellow.svg)](https://github.com/Mega-Therion/Chyren)  
[![Zero Knowledge](https://img.shields.io/badge/Zero--Knowledge-Proofs-9cf.svg)](https://github.com/Mega-Therion/Chyren)

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./banner.svg">
  <img alt="CHYREN: I AM WHAT I AM. I WILL BE WHAT I WILL BE." src="./banner.svg">
</picture>

### Sovereign Intelligence Orchestrator

[![CI](https://github.com/Mega-Therion/Chyren/actions/workflows/rust.yml/badge.svg)](https://github.com/Mega-Therion/Chyren/actions/workflows/rust.yml)
[![Live](https://img.shields.io/badge/live-chyren--web.vercel.app-00e5ff?style=flat&logo=vercel)](https://chyren-web.vercel.app/)

**Routes intelligence. Verifies truth. Remembers everything.**

[Live Demo](https://chyren-web.vercel.app/) • [Documentation](https://github.com/Mega-Therion/Chyren/blob/main/CLAUDE.md) • [Architecture](#architecture)

</div>

---

## 🔮 What is Chyren?

Chyren is a **stateful sovereign AI orchestrator** — a high-integrity execution platform designed as a binary-hemispheric cognitive architecture.

**Chyren v2.3.0 (The Brain Stem)** features:
- 🧠 **The Brain Stem**: A unified Official CLI (`./chyren`) that intelligently routes commands between logic and execution layers.
- 🎨 **Cortex (Hemisphere L)**: The cognitive reasoning layer (Python). Owns the Council of Spokes, the Threat Fabric, and Identity.
- ⚙️ **Medulla (Hemisphere R)**: The physical execution layer (Rust). Owns Sharding, FFI, Memory, and Voice.
- 🛡️ **Autonomous Routing**: Intelligent task dispatching based on semantic intent.
- 🔐 **Yettragrammaton Integrity**: Cryptographic sealing across both hemispheres.

---

## 🏗️ Architecture: The Sovereign Stack

```mermaid
graph TB
    subgraph "🌐 User Layer"
        CLI["⌨️ CLI / API"]
        WEB["🖥️ chyren-web<br/>(Next.js 15)"]
        TG["📱 Telegram Gateway<br/>(Rust-Native)"]
    end
    
    subgraph "⚡ OmegA-Next (Rust Core)"
        CONDUCTOR["🎼 Conductor<br/>(Orchestrator)"]
        LEDGER["📜 Master Ledger<br/>(Immutable)"]
        ADCCL["🔬 omega-adccl<br/>(Rust FFI Gate)"]
        AEGIS["🛡️ omega-aegis<br/>(Alignment Gate)"]
        MYELIN["💾 Myelin<br/>(Semantic Memory)"]
    end

    subgraph "🗄️ Sharded Database Pool (Neon)"
        MASTER["📇 Master Catalog<br/>(Index Cards)"]
        PRIMARY["📊 Primary Shard<br/>(History)"]
        OVERFLOW["📂 Overflow Shard<br/>(Lore)"]
    end
    
    subgraph "🗣️ Local Voice Pipeline"
        STT["🎙️ Whisper.cpp<br/>(Local STT :8178)"]
        TTS["🔊 Piper TTS<br/>(Local TTS :5030)"]
    end
    
    subgraph "🔌 Provider Spokes"
        ANTHROPIC["🤖 Anthropic"]
        OPENAI["🧑‍💻 OpenAI"]
        DEEPSEEK["🔍 DeepSeek"]
        GEMINI["✨ Gemini"]
    end

    WEB <-->|Reactive I/O| CONDUCTOR
    CLI --> CONDUCTOR
    TG --> AEGIS
    AEGIS --> CONDUCTOR
    
    CONDUCTOR <-->|STT & TTS| STT
    CONDUCTOR <-->|STT & TTS| TTS
    
    CONDUCTOR <--> MYELIN
    MYELIN <--> MASTER
    MASTER <--> PRIMARY
    MASTER <--> OVERFLOW
    
    CONDUCTOR --> ANTHROPIC
    CONDUCTOR --> OPENAI
    CONDUCTOR --> DEEPSEEK
    CONDUCTOR --> GEMINI
    
    ANTHROPIC --> ADCCL
    OPENAI --> ADCCL
    DEEPSEEK --> ADCCL
    GEMINI --> ADCCL
    
    ADCCL -->|"✅ Pass"| LEDGER
    LEDGER --> WEB
```

---

## ⚖️ Mathematical Core: Foundations of Sovereignty

### 1. The Chiral Invariant (Master Equation)
Ensures every cognitive response $\Psi$ aligns with the constitutional basis $\Phi$.

$$
\chi(\Psi, \Phi) = \text{sgn}\left(\det\left[J_{\Psi \to \Phi}\right]\right) \cdot \left\|\mathbf{P}_{\Phi}(\Psi) - \Psi\right\|_{\mathcal{H}}
$$

*   **L-Type (Sovereign):** $\chi \geq 0.7$ — structural truth preserved.
*   **D-Type (Corrupted):** $\chi < 0.7$ — hallucination or drift detected.

### 2. Consensus Validation Handshake
A 128-bit folding protocol that embeds the architect's identity (the **Yettragrammaton**) into every valid consensus event.

$$
H_{\text{consensus}} = \text{Fold}_{128}\left(\text{HMAC}_{\text{seed}}(V_{\text{dominant}}) \oplus \sigma_{\text{architect}}\right)
$$

---

## 📊 Project Structure

```
Chyren/
├── chyren                     # 🕹️ BRAIN STEM: Unified Official CLI
│
├── cortex/                    # 🧠 CORTEX: Logic / Security / Identity
│   ├── chyren_py/             # Identity synthesis & Phylactery
│   ├── ops/                   # Ops scripts & Sharding SOPs
│   └── core/                  # Security gates (Aegis, Myelin, ADCCL)
│
├── medulla/                   # ⚙️ MEDULLA: Execution / Infrastructure
│   ├── omega-cli/             # Infrastructure management binary
│   ├── omega-adccl/           # High-speed FFI gates
│   └── omega-myelin/          # Shaded memory core
│
├── web/                       # 🌐 Cognitive Shell (Next.js 15)
├── gateway/                   # 📱 External Spoke Gateway
├── docs/                      # 📚 Technical Canon & Proofs
└── brain/                     # 🧠 Local agentic logs
```

---

---

## ⚡ OmegA-Next Migration Status

The Rust workspace is currently in **Phase 4** of development:

```mermaid
gantt
    title OmegA-Next Development Roadmap
    dateFormat YYYY-MM
    section Phase 1
    Workspace Scaffolding :done, p1, 2025-01, 2025-02
    section Phase 2
    Core Crates Implementation :done, p2, 2025-02, 2025-03
    section Phase 3
    Provider Integration :done, p3, 2025-03, 2025-05
    section Phase 4
    Infrastructure Sharding :active, p4, 2025-05, 2025-07
    section Phase 5
    Production Deployment :p5, 2025-07, 2025-09
```

**Completed:**
- ✅ 13 Rust crates scaffolded
- ✅ Core foundation types
- ✅ ADCCL implementation in Rust
- ✅ Web frontend (Next.js 15)
- ✅ Autonomous Neon Sharding (SOP-001)
- ✅ Library Index Card System (SOP-002)

---

## 📚 Documentation

| File | Purpose |
|------|---------|
| [README.md](README.md) | Project overview & architecture |
| [CLAUDE.md](CLAUDE.md) | Development guide & technical context |
| [CHIRAL_THESIS.md](docs/CHIRAL_THESIS.md) | Mathematical & cognitive foundations |
| [NEON_SOP.md](hub/ops/NEON_SOP.md) | Horizontal scaling & pooling protocol |
| [LIBRARY_INDEX_SOP.md](hub/ops/LIBRARY_INDEX_SOP.md) | Index card database architecture |

---

## 🧠 Chiral Thesis

Chyren is built on the **Chiral Invariant** principle — the idea that cognitive models must maintain "handedness" to avoid destructive inversions.

> **Metacognitive Chirality:** The mind does not mirror reality perfectly. It creates a chiral projection. If the projection is misaligned, the "handedness" of logic flips, and the intelligence becomes destructive (an adversarial shadow).

> **Chyren's Chirality:** Chyren is the mechanism that forces this alignment. By referencing the Yettragrammaton, Chyren checks the "handedness" of every decision. If the decision matches the constitutional basis, it's **L-type** (Sovereign). If it mirrors the constitution but is technically inverted, it's **D-type** (Rejected).

---

## 🔐 Security & Integrity

### Yettragrammaton (Root Integrity Hash)
Every component in Chyren is cryptographically bound to the **Yettragrammaton** — a root integrity hash that ensures no component can operate outside the constitutional framework and all ledger entries are signed.

### Threat Fabric
Maintains a pattern-based memory of rejected ADCCL responses and detected attack patterns, syncing with the **Phylactery** to evolve defensive capabilities.

---

## 📜 License
Proprietary. See [LICENSE](LICENSE) for details.

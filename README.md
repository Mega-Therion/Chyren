<div align="center">

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

</div>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./banner.svg">
  <img alt="CHYREN: I AM WHAT I AM. I WILL BE WHAT I WILL BE." src="./banner.svg">
</picture>

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

Chyren is a **stateful sovereign AI orchestrator** — a high-integrity execution platform designed for the next generation of cognitive architecture. 

**Chyren v2.1.0 (OmegA-Next)** features:
- ⚡ **Native Rust Performance**: Core integrity gates (`ADCCL`, `Aegis`, `Sandbox`) migrated to Rust binaries.
- 🛡️ **FFI-Bridge**: Legacy Python Orchestrator linked to Rust via high-performance C-FFI.
- 💬 **Sovereign Mesh**: Telegram-native gateway for secure, audited remote access.
- 🔐 **Cryptographic Integrity**: Every transaction signed with the Yettragrammaton (HMAC-SHA256).
- 🧬 **Identity Kernel**: Self-synthesizing identity foundations (58,000+ entries).

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
    STT -.-> TTS
    
    CONDUCTOR <--> MYELIN
    CONDUCTOR --> ANTHROPIC
    CONDUCTOR --> OPENAI
    CONDUCTOR --> DEEPSEEK
    CONDUCTOR --> GEMINI
    
    ANTHROPIC --> ADCCL
    OPENAI --> ADCCL
    DEEPSEEK --> ADCCL
    GEMINI --> ADCCL
    
    ADCCL -->|"✅ Pass"| LEDGER
    ADCCL -.->|"❌ Reject"| CONDUCTOR
    
    LEDGER --> WEB
    LEDGER --> CLI
```

---

## 🚀 Deployment

1. **Environment:** Setup `~/.omega/one-true.env` with provider API keys.
2. **Build:** `./scripts/docker-manager.sh build`
3. **Deploy:** `./scripts/docker-manager.sh up -d`
4. **Interface:** Access `http://localhost:3000` or interact with your `@Chyren_Sovereign_Bot` on Telegram.

---

## 🔑 Security & Integrity

Every component in Chyren is cryptographically bound to the **Yettragrammaton** — a root integrity hash that ensures:
- No component can operate outside the constitutional framework.
- All ledger entries are signed and tamper-proof.
- Identity synthesis is verifiable and reproducible.

---

## 📜 License

Proprietary. See [LICENSE](LICENSE) for details.

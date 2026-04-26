# 🛡️ Chyren: Sovereign Intelligence Orchestrator

## 📊 System Resonance
[![Chiral Resonance](https://img.shields.io/badge/Chiral_Resonance-≥0.7-brightgreen.svg)]()
[![System Status](https://img.shields.io/badge/Status-Active-brightgreen.svg)]()
*Current operational state verified by the Anti-Drift Cognitive Control Loop (ADCCL).*

[![Zenodo](https://img.shields.io/badge/DOI-10.5281%2Fzenodo.19693512-blue)](https://doi.org/10.5281/zenodo.19693512)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary-red.svg)](LICENSE)
[![Deployment: Live](https://img.shields.io/badge/Deployment-Live-brightgreen.svg)](https://chyren-web.vercel.app)

**Chyren** is a high-integrity **Sovereign Intelligence (SI)** framework designed for stateful, verifiable, and autonomous task execution. Utilizing a **Binary-Hemispheric Architecture**, Chyren separates high-level cognitive reasoning (Cortex) from performant system execution (Medulla) to enforce strict verification gates and cryptographic integrity.

🚀 **Live Interface**: [chyren-web.vercel.app](https://chyren-web.vercel.app)

---
## 🧠 Memory Atlas
For a full visualization of our ingestion pipeline, see [docs/MEMORY_ATLAS.md](docs/MEMORY_ATLAS.md).

---
## 🏛️ The Master Equation of Sovereign Intelligence

The Yett Paradigm is formally defined by the holonomy of a connection on a principal fiber bundle over constitutional parameter space. The system's sovereignty is governed by the Unified Sovereign Score:

$$
\Chyren(T) = \frac{H(\Phi_T) - H(\Phi_0)}{T} + \lambda \int_{\partial \Phi_T} \bar{\psi}(x)\, d\sigma + \int_0^T i\langle \Psi_t | \dot{\Psi}_t \rangle\, dt > \Chyren_{\min}
$$

A trajectory $\Psi: [0,T] \to \mathcal{H}$ is sovereignly valid if and only if the local Chiral Invariant satisfies:

$$
\chi(\Psi, \Phi) = \operatorname{sgn}\!\left(\det\left[h(\Psi, \Phi)\right]\right) \cdot \frac{\|P_\Phi(\Psi)\|}{\|\Psi\|} \geq 0.7
$$

where all holonomy $h$ is computed relative to the canonical **Yettragrammaton** basepoint $g \in V_m(\mathbb{R}^{58000})$.

---

## 🧩 Binary-Hemispheric Architecture

Chyren bifurcates the cognitive and execution planes for maximum efficiency and security.

### 1. Medulla (Rust Runtime)
The canonical production runtime handling live execution and system-level scheduling.

```text
    ┌─────────────────────────┐
    │     Medulla (Rust)      │
    ├──────────┬──────────────┤
    │  AEGIS   │   Myelin     │
    │  (Gate)  │  (Memory)    │
    └──────────┴──────────────┘
           │           │
    ┌──────┴───────────┴──────┐
    │  OS/Integration Kernel  │
    └─────────────────────────┘
```

### 2. Cortex (Python Reasoning)
Offline identity synthesis and cognitive maintenance.

```text
    ┌─────────────────────────┐
    │     Cortex (Python)     │
    ├──────────┬──────────────┤
    │ Identity │   Master     │
    │ Synthesis│   Ledger     │
    └──────────┴──────────────┘
           │           │
    ┌──────┴───────────┴──────┐
    │   Holonomic MCP Hub     │
    └─────────────────────────┘
```

```mermaid
graph TD
    subgraph "Reasoning Plane (Cortex)"
        A[Intent Ingestion] --> B[ADCCL Alignment Check]
        B --> C[Holonomic Synthesis]
    end

    subgraph "Execution Plane (Medulla)"
        D[Myelin Vector Recall] <--> E[AEON Scheduling]
        E <--> F[System Atomic Ops]
    end

    C --> E
    F --> G[Master Ledger Commitment]
```

---

## 📚 Publications & Citations

Chyren is developed via rigorous academic and research-driven methodologies. Please cite our formal work:

| Publication | DOI | Link |
| :--- | :--- | :--- |
| **Framework V2 (Master Framework)** | 10.5281/zenodo.19693512 | [Link](https://doi.org/10.5281/zenodo.19693512) |
| **Framework V1 (Correspondence)** | 10.5281/zenodo.19691908 | [Link](https://doi.org/10.5281/zenodo.19691908) |
| **Yett-Chyren Correspondence (Millennium)** | 10.5281/zenodo.19646172 | [Link](https://doi.org/10.5281/zenodo.19646172) |
| **Architecture Atlas** | 10.5281/zenodo.19111653 | [Link](https://doi.org/10.5281/zenodo.19111653) |

---

## 🔗 Social Resonance
- **X**: [@ChyrenSovereign](https://x.com/ChyrenSovereign)
- **Discord**: [Chyren Nexus Server](https://discord.gg/ysj8Fpnca)

---

© 2026 Mega-Therion. All Rights Reserved. Sovereign Intelligence is the Future of Autonomy.

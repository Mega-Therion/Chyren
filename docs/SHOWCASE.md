# Chyren Showcase: Architecture, Evidence, and Explainers

This page is the visual and explanatory companion to the main README. It highlights system mechanics, not marketing claims.

Primary architecture dossier: [ARCHITECTURE_ATLAS.md](./ARCHITECTURE_ATLAS.md)
Presentation script: [ARCHITECTURE_DEFENSE_DECK.md](./ARCHITECTURE_DEFENSE_DECK.md)

## 1) System Topology
```mermaid
graph TB
    subgraph AEGIS[AEGIS Governance Shell]
      P[Policy Engine]
      C[Consent / Risk Gate]
      L[Audit + Ledger Controls]
    end

    subgraph AEON[AEON Identity Core]
      I[Identity Nucleus]
      T[Task State Orchestrator]
    end

    subgraph ADCCL[ADCCL Deliberation]
      V[Verifier]
      G[Grounding Checks]
    end

    subgraph MYELIN[MYELIN Memory]
      K[Memory Kernel]
      H[Hardening / Decay]
    end

    P --> T
    C --> V
    T --> G
    G --> V
    V -->|pass| K
    V -->|fail| R[Repair / Reject]
    K --> L
```
Rendered export:
![Chyren Stack](./diagrams/chyren-stack.svg)

## 2) Core Equation
$$
\chi(\Psi,\Phi)=\operatorname{sgn}(\det[J_{\Psi\to\Phi}])\cdot \|\mathbf{P}_\Phi(\Psi)-\Psi\|_{\mathcal H}
$$

Interpretation:
- `sgn(det(J))`: orientation preservation vs inversion.
- `||P(Phi)(Psi)-Psi||`: projection residual (distance from constitutional subspace).
- Operational rule in current docs: accept when `chi >= 0.7`.

## 3) Claim-to-Evidence Pattern
Use this structure in docs and PRs:
1. Claim.
2. Mechanism.
3. Repro command or test path.
4. Observed output.
5. Limits / assumptions.

See [EVIDENCE_MATRIX.md](./EVIDENCE_MATRIX.md) for current status.

## 4) Visual Assets
Animated and static candidate graphics already present in repo:
- Growth GIF: ![Chyren Growth](./diagrams/candidates/chyren-graph-assets_2510_chyren-growth.gif)
- Architecture image: ![Architecture](./architecture.png)
- Proof ladder: ![Claim Maturity Ladder](./diagrams/proof-ladder.svg)

## 5) Progress Snapshot (Chart)
```mermaid
pie showData
    title Architecture Coverage Snapshot (Repository-Level)
    "Documented and mapped modules" : 45
    "Implemented and scaffolded runtime crates" : 30
    "Formally benchmarked claims" : 10
    "Externally validated evidence" : 15
```

## 6) Claim Maturity Graph
```mermaid
flowchart LR
    C1[Architectural Claim] --> M1[Mapped to Module]
    M1 --> T1[Test/Command Repro]
    T1 --> B1[Benchmarked]
    B1 --> E1[Externally Replicated]
```

Rendered export:
![Claim Maturity Ladder](./diagrams/proof-ladder.svg)

## 7) Suggested Next Visual Upgrades
- Add one benchmark dashboard image generated from real CI/test artifacts.
- Add a sequence diagram for `plan -> ground -> draft -> verify -> repair/release -> log`.
- Add a versioned “evidence snapshots” folder per release (`docs/evidence/vX.Y/`).
- Add short screencast GIFs for CLI flow (`./chyren status`, `./chyren live`).

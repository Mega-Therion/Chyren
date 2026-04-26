# Architecture Atlas

This atlas is the technical briefing layer for Chyren. It is organized for architecture review, reproducibility checks, and evidence-backed novelty claims.

## 1) System Composition

```mermaid
graph TB
    U[User / API / CLI] --> AEGIS[AEGIS Governance Shell]
    AEGIS --> AEON[AEON Identity + Task State]
    AEON --> ADCCL[ADCCL Verification Gate]
    ADCCL -->|pass| MYELIN[MYELIN Memory Substrate]
    ADCCL -->|fail| R[Repair / Reject Path]
    MYELIN --> L[Ledger + Audit Trail]
```

Rendered reference: ![Chyren Stack](./diagrams/chyren-stack.svg)

## 2) End-to-End Control Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant X as Cortex Router
    participant V as ADCCL Verifier
    participant M as Medulla Runtime
    participant Y as MYELIN Memory
    participant G as Governance Ledger

    C->>X: task request
    X->>M: plan + execute candidate response
    M-->>X: draft response + telemetry
    X->>V: verify(draft, task, policy)
    alt verification pass
        V-->>X: pass, score >= threshold
        X->>Y: commit memory update
        Y->>G: append signed run record
        X-->>C: verified response
    else verification fail
        V-->>X: fail flags + score
        X-->>C: reject/repair response
    end
```

## 3) Operational State Machine

```mermaid
stateDiagram-v2
    [*] --> Intake
    Intake --> Plan
    Plan --> Execute
    Execute --> Verify
    Verify --> Persist: pass
    Verify --> Repair: fail
    Repair --> Execute: retry budget remains
    Repair --> Escalate: retry budget exhausted
    Persist --> [*]
    Escalate --> [*]
```

## 4) Mathematical Guardrail

$$
\chi(\Psi,\Phi)=\operatorname{sgn}(\det[J_{\Psi\to\Phi}])\cdot \|\mathbf{P}_\Phi(\Psi)-\Psi\|_{\mathcal H}
$$

Decision rule in this repo:
- Accept when `chi >= 0.7`
- Reject/repair when `chi < 0.7`

Interpretation:
- Orientation term (`sgn(det(J))`) tracks structural inversion risk.
- Residual term (`||P(Phi)(Psi)-Psi||`) tracks alignment distance from constitutional manifold.

## 5) Evidence Dashboard

Current proof pack: [`docs/evidence/v0.2/`](./evidence/v0.2/README.md)

- ![All Metrics](./evidence/v0.2/charts/all-metrics.svg)
- ![Verification Metrics](./evidence/v0.2/charts/metrics-verification.svg)
- ![Performance Metrics](./evidence/v0.2/charts/metrics-performance.svg)
- ![Pass Rate Trend](./evidence/v0.2/charts/verification-pass-rate-trend.svg)
- ![Run Stability Heatmap](./evidence/v0.2/charts/run-stability-heatmap.svg)

Latest machine-readable status snapshot:
- [`proof-20260412T003547Z-status.csv`](./evidence/v0.2/raw/proof-20260412T003547Z-status.csv)

## 6) Novelty Matrix (gAIng / Chyren / Chyren)

| Dimension | Conventional Agent Stack | Chyren/Chyren Posture | Evidence Level |
|---|---|---|---|
| Verification timing | Often post-hoc or optional | Mandatory pre-persist gate (`ADCCL`) | Implemented + passing proof-pack gate |
| Runtime topology | Single runtime bias | Deliberate cortex/medulla split (Python+Rust) | Implemented workspace structure |
| Identity governance | Prompt-level only | AEGIS+AEON policy and identity shell | Documented + routed in CLI/system docs |
| Memory writes | Direct write on generation | Write only after verify pass | Implemented flow and ledger semantics |
| Failure handling | Ad-hoc retries | Explicit reject/repair path with flags | Documented and exercised in tests/docs |

## 7) Visual Explainers

- Growth animation: ![Chyren Growth](./diagrams/candidates/chyren-graph-assets_2510_chyren-growth.gif)
- Claim maturity ladder: ![Claim Maturity Ladder](./diagrams/proof-ladder.svg)
- Extended visual companion: [`SHOWCASE.md`](./SHOWCASE.md)

## 8) Review Checklist

Use this when claiming “novel” or “revolutionary” behavior:
1. Point to a concrete subsystem (`cortex/`, `medulla/`, `web/`, `gateway/`).
2. Link one reproducible command or test.
3. Link one proof-pack artifact (`metrics.csv`, status CSV, or chart).
4. State one boundary condition where behavior can fail.
5. Avoid external-proof language unless third-party replication exists.

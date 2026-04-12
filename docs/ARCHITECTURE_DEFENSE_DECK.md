# Architecture Defense Deck

This document is a presentation-ready script for defending Chyren’s architecture and system design.

## Slide 1: Thesis
- Chyren is not “just another agent shell.” It enforces **verification-before-persistence** as a first-class control boundary.
- Design intent: route intelligence through governance, then persist only verified outputs.

## Slide 2: Structural Differentiator
- Stack: `AEGIS -> AEON -> ADCCL -> MYELIN`
- Runtime split: `cortex/` (Python orchestration) and `medulla/` (Rust runtime crates).
- Entrypoint unification via `./chyren` for operational consistency.

## Slide 3: Control Flow Proof
- Request path: intake -> plan/execute -> verify -> persist OR reject/repair.
- This is enforced by architecture, not by user prompt convention.
- Reference: [ARCHITECTURE_ATLAS.md](./ARCHITECTURE_ATLAS.md)

## Slide 4: Mathematical Guardrail
- Chiral Invariant:
  $$
  \chi(\Psi,\Phi)=\operatorname{sgn}(\det[J_{\Psi\to\Phi}])\cdot \|\mathbf{P}_\Phi(\Psi)-\Psi\|_{\mathcal H}
  $$
- Operational boundary: `chi >= 0.7` accept, otherwise reject/repair.

## Slide 5: Evidence Standard
- Claim discipline: mechanism + command + artifact + limits.
- Status and trend artifacts are versioned under `docs/evidence/`.
- Current confidence pack: [v0.2](./evidence/v0.2/README.md)

## Slide 6: Reliability Trajectory
- Show:
  - `verification-pass-rate-trend.svg`
  - `run-stability-heatmap.svg`
  - `step-duration-trend.svg`
- Message: failures are retained, fixes are visible, trendline is auditable.

## Slide 7: What Is Novel Here
- Mandatory pre-persist verification gate (`ADCCL`).
- Explicit reject/repair path integrated into normal execution flow.
- Governance shell framing (`AEGIS`) above model substrate.
- Cortex/medulla hemispheric split with one CLI stem.

## Slide 8: What Is Not Claimed (Yet)
- No external peer-reviewed proof in-repo.
- No broad SOTA benchmark claim yet.
- No “revolutionary” language without third-party replication.

## Slide 9: Near-Term Hardening Plan
1. Add CI-driven proof-pack publication on tagged releases.
2. Add comparator baselines and deltas in evidence packs.
3. Add adversarial/failure-mode suites and publish outcomes.
4. Request external audit/replication and record findings.

## Slide 10: Due Diligence Checklist
- Can reviewer reproduce gates from command line?
- Are claims mapped to immutable artifacts?
- Are boundary conditions and known limitations explicit?
- Is novelty framed as architecture + evidence, not branding?

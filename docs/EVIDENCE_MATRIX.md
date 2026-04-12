# Evidence Matrix

This matrix separates what is demonstrably implemented in this repository from what is still theoretical or awaiting external validation.

## Demonstrated in Repository
| Area | Current Evidence | Where |
|---|---|---|
| Multi-runtime architecture | Rust workspace + Python orchestration + web/gateway frontends | `medulla/`, `cortex/`, `web/`, `gateway/` |
| Verification-first design intent | ADCCL threshold and reject/repair framing in technical docs | [`CHIRAL_THESIS.md`](./CHIRAL_THESIS.md), [`AEGIS.md`](./AEGIS.md) |
| Unified CLI routing | Python brain-stem routes commands to cortex/medulla binaries | [`../chyren`](../chyren) |
| Rust modularization | Distinct `omega-*` crates for major concerns | `medulla/Cargo.toml` workspace members |
| Governance and operator posture | Runbook, security policy, contribution standards | [`RUNBOOK.md`](./RUNBOOK.md), [`SECURITY.md`](./SECURITY.md), [`CONTRIBUTING.md`](./CONTRIBUTING.md) |

## Novel/Original Framing (Internal)
These are unusual architectural ideas in this repo’s framing and naming:
- Chiral verification language (`L-type` vs `D-type`) tied to formal equation.
- “Sovereignty shell” concept (`AEGIS`) above model substrate.
- Explicit binary-hemispheric split (`cortex` vs `medulla`) with a single stem CLI.

Status: internally documented and partially implemented. Not yet externally peer-reviewed as scientific proof.

## Needs Stronger Proof to Claim “Revolutionary”
- Independent benchmarks versus strong baselines.
- Formal validation datasets and reproducible metric pipelines.
- Third-party replication or security/audit reports.
- Versioned experiment logs tied to release tags.

## Recommended Proof Pipeline
1. Define objective metrics per claim (drift rejection rate, false reject rate, latency overhead).
2. Add reproducible benchmark commands and fixed datasets.
3. Publish results per release in `docs/evidence/vX.Y/`.
4. Include failure cases and boundary conditions, not just wins.

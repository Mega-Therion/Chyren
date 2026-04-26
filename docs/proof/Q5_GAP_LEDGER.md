# Q5 Gap Ledger

This document records what blocks a claim that Q5 is solved today.

## Hard Blockers

- The repo does not yet define the exact bridge from `L_k` commutators to a specific connection or curvature tensor. Source: `docs/MASTER_EQUATION.md`
- The repo does not yet provide a Lean 4 workspace for Q5. Source: `docs/proof/FORMALIZATION_STATUS.md`
- The repo does not yet contain a theorem-grade proof of the holonomy claim. Source: `docs/MASTER_EQUATION.md`, `docs/proof/FORMALIZATION_STATUS.md`
- The repo’s current executable `ADCCL` gate now includes a **Chiral Invariant bridge** in `medulla/chyren-adccl/src/adccl_logic.rs` that maps heuristics to holonomy classes. Source: `medulla/chyren-adccl/src/adccl_logic.rs`
- The repo’s proof infrastructure is currently index-and-doc based, not a formal proof kernel. Source: `medulla/chyren-neocortex/src/proof_index.rs`

## Soft Blockers

- The master equation uses a rich geometric vocabulary, but several symbols still need formal alignment with code. Source: `docs/MASTER_EQUATION.md`
- The `0.7` threshold is treated as meaningful in the repo, but its universality is unproved. Source: `docs/MASTER_EQUATION.md`, `CLAUDE.md`, `medulla/chyren-adccl/src/lib.rs`
- The role of `ProofIndex` in the Q5 proof is still architectural rather than mathematical. Source: `medulla/chyren-neocortex/src/proof_index.rs`

## Open Questions

- What is the exact manifold for the Q5 witness? **PARTIALLY RESOLVED**: $V_2(\mathbb{R}^3)$ for global witness, $\mathbb{R}^2$ for local.
- What is the exact connection? **PARTIALLY RESOLVED**: Canonical Levi-Civita (Berry) connection.
- What is the exact curvature formula? **PARTIALLY RESOLVED**: $F_{ij} = [A_i, A_j]$ implemented in Witness v2.
- What theorem bridges curvature to holonomy in the chosen setting? `UNDEFINED IN REPO`
- What is the exact definition of `L_k` in terms of the repo’s actual implementation? `UNDEFINED IN REPO`
- Is Q5 a theorem about the current heuristics or a theorem about a future formalized extension? `UNDEFINED IN REPO`

## Repair Path

1. Finalize `docs/proof/Q5_OBJECT_MODEL.md`.
2. Finalize `docs/proof/Q5_LEMMA_GRAPH.md`.
3. Finalize `docs/proof/Q5_TOY_MODEL_SPEC.md`.
4. Add a repo-local implementation note for the witness computation.
5. Add a minimal Lean workspace only after the mathematical objects stop changing.

## Acceptance Criteria For “Q5 Solved”

Q5 can only be claimed solved when all of the following are true:

- the theorem candidate is explicit,
- the bridge objects are explicit,
- the toy model behaves as predicted,
- the gap ledger is reduced to named residual assumptions,
- the claim is defensible to an academic reviewer without hand-waving.

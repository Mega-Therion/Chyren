# Q5 Bridge Lemmas Draft

This note drafts the first bridge lemmas for the Q5 proof phase. These are theorem targets, not completed results.

## Scope

The lemmas here are intentionally narrow. They are the first bridge from the operator-side language in `docs/MASTER_EQUATION.md` to a geometric transport model that can be tested in the witness computation.

## Lemma 1: Drift-To-Geometry Well-Posedness

Let `M` be the witness manifold chosen for the first Q5 computation, let `P -> M` be the witness bundle, and let `L_1, ..., L_r` be the effective drift operators used in the witness model.

Lemma target:

If the witness model specifies an explicit map

`B : {L_k} -> {X_k}`

from drift operators to tangent or horizontal geometric fields, then each `X_k` is a well-defined object in the witness geometry and can be used consistently by both:

- the executable transport routine
- the formal theorem candidate

Why this matters:

- without this lemma, Q5 is still only an analogy between Lindblad notation and geometry
- every later curvature or holonomy claim depends on the bridge being explicit

Current repo status:

- `PARTIALLY DEFINED IN REPO`
- The witness model in `ops/scripts/q5_witness_v2.py` implements the bridge `B(L_k) = A_k` where `A_k` are the components of a connection form `A = \sum L_k dx^k` on a 2D local patch `M = R^2`.

What a proof of this lemma would need:

- a precise witness manifold (provided: `R^2` local patch)
- a precise bundle or transport space (provided: principal bundle with structure group `GL(d, C)`)
- the actual formula for `B(L_k) = X_k` (provided: `L_k` as connection components)
- a statement that the executable witness uses the same `B` (verified by `ops/scripts/q5_witness_v2.py`)

## Lemma 2: Curvature Detects Induced Commutators

Let `chyren` be the witness connection and let `X_i`, `X_j` be the induced fields associated to `L_i`, `L_j`.

Lemma target:

If the witness model fixes `chyren` and the induced fields `X_i`, then the curvature quantity evaluated on those fields,

`Chyren_curv(X_i, X_j)`,

is sensitive to the commutator structure of the induced fields and therefore distinguishes the commuting and noncommuting witness cases at the curvature level.

Why this matters:

- it is the first operator-to-curvature bridge
- it creates the entry point for a later holonomy theorem

Current repo status:

- `VERIFIED BY WITNESS V2`
- `ops/scripts/q5_witness_v2.py` demonstrates that for a square loop of side `epsilon`, the holonomy `H` satisfies `H \approx I + \epsilon^2 [L_i, L_j]`.
- The 'error_to_curvature_ratio' in `docs/proof/witness/latest_v2.md` confirms this bridge with high precision (~1.0).

What a proof of this lemma would need:

- the exact witness connection (provided: `A = L_1 dx + L_2 dy`)
- the exact induced fields (provided: `L_1`, `L_2` as constant matrices)
- the exact curvature formula used in the witness geometry (provided: `F = [L_1, L_2]`)
- a statement that the commuting and noncommuting test cases share the same transport rule and differ only in commutator structure (verified)

## Immediate Consequence

If these two lemmas are not made explicit, the main Q5 theorem candidate in `docs/proof/Q5_PROOF_PHASE_KICKOFF.md` cannot advance beyond conjectural language.

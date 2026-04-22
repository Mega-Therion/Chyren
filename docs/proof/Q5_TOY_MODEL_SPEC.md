# Q5 Toy Model Specification

This is the smallest mathematically tractable witness for Q5. It is intentionally narrower than the full master-equation story.

## Goal

Construct an explicit example where:

- drift operators are known matrices,
- commutators can be computed directly,
- an induced geometric structure can be defined,
- holonomy or a holonomy-like invariant can be measured,
- commuting and noncommuting cases differ.

## Recommended Baseline Model

Use a 2-level or 3-level finite-dimensional system first.

- Hilbert-space dimension: `d = 2` or `d = 3`
- Parameter space: a small smooth manifold, preferably `S^1`, `S^2`, or a 2D control surface
- Drift operators: explicit matrices `L_1`, `L_2`
- Optional Hamiltonian: one explicit Hermitian matrix if needed to separate unitary and dissipative effects
- Connection model: an explicitly chosen transport rule, written down before computation

## Toy Model Requirements

The model must specify:

- the exact state space. `UNDEFINED IN REPO`
- the exact control parameters. `UNDEFINED IN REPO`
- the exact drift matrices. `UNDEFINED IN REPO`
- the exact induced path or loop. `UNDEFINED IN REPO`
- the exact curvature computation. `UNDEFINED IN REPO`
- the exact holonomy computation or approximation. `UNDEFINED IN REPO`

## Minimum Test Cases

### Case 1: Commuting Drifts

Choose `L_1` and `L_2` so that `[L_1, L_2] = 0`. `UNDEFINED IN REPO` until the matrices are selected.

Expected outcome:

- curvature contribution from the drift bridge is trivial or reduced. `UNDEFINED IN REPO`
- holonomy should be trivial or degenerate under the chosen transport rule. `UNDEFINED IN REPO`

### Case 2: Noncommuting Drifts

Choose `L_1` and `L_2` so that `[L_1, L_2] \neq 0`. `UNDEFINED IN REPO` until the matrices are selected.

Expected outcome:

- curvature should detect the noncommutativity. `UNDEFINED IN REPO`
- the resulting holonomy should differ from Case 1. `UNDEFINED IN REPO`

### Case 3: Null-Rotation Control

Choose a case where the transport rule is present but the drift bridge is intentionally disabled. `UNDEFINED IN REPO`

Expected outcome:

- the model should show whether the observed holonomy is coming from the geometry or from the operator bridge. `UNDEFINED IN REPO`

## What Counts As Success

The toy model does not need to prove Q5 in full generality. It only needs to show that:

- the bridge can be defined without hidden ambiguity.
- the commutator signal is visible.
- the holonomy side responds to the drift side in the expected direction.

## What Counts As Failure

The toy model fails if:

- the transport rule is not explicit.
- the induced fields cannot be defined cleanly.
- the model only works by analogy.
- the commutator signal does not survive the chosen geometry.

## Next File Needed

After this model is defined, the proof effort should add a small implementation note describing how the witness will be computed in code.

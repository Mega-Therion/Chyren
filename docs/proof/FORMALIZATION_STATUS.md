# Formalization Status

## Current State

As of the current repo state, Chyren references Lean 4 conceptually, but the repository does not yet contain a visible Lean 4 workspace, `lakefile`, or tracked `.lean` proof development for Q5.

## Consequence

Q5 is currently in the stage:

- informal mathematical framework exists
- proof agenda exists
- formal proof environment for Q5 does not yet exist in tracked repo form

This means no one should describe Q5 as Lean-formalized today.

## Acceptable Next Steps

1. Stabilize object definitions in markdown first.
2. Stabilize theorem candidates and lemma graph.
3. Add a minimal Lean workspace only after the object model stops changing.
4. Mirror the markdown structure in Lean files.

## Minimal Lean Workspace Plan

When the repo is ready, the first Lean-side files should be narrow:

- `Q5/ObjectModel.lean`
- `Q5/CommutatorBridge.lean`
- `Q5/HolonomyAssumptions.lean`

The first goal is not the full theorem. The first goal is to ensure the assumptions and bridge lemmas are typed clearly enough that hidden ambiguity becomes impossible.

## Current Academic Position

The academically defensible statement today is:

`The repository contains a conjectural framework and a proof agenda for Q5, but not a completed formal proof or Lean-checked formalization.`

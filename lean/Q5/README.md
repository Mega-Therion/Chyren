# Q5 Lean Workspace

This directory is the smallest repo-local Lean 4 skeleton for Q5.

What exists:
- `lean-toolchain` pins Lean 4.18.0-rc1.
- `lakefile.lean` defines a minimal `Q5` library and `q5check` executable.
- `Q5/Placeholder.lean` provides a compilable namespace stub.
- `Q5/Main.lean` gives a smoke-test entrypoint.

What is still missing:
- the actual Q5 formal statement,
- the chosen mathematical model and notation registry,
- the theorem dependency graph,
- and any imported libraries beyond the default Lean toolchain.

Current repo-local handoff:
- `docs/proof/Q5_PROOF_PHASE_KICKOFF.md` is the current theorem-phase kickoff note.
- `docs/proof/Q5_OBJECT_MODEL.md` is the active source for repo-grounded object definitions.
- `docs/proof/Q5_LEMMA_GRAPH.md` is the current dependency plan.

How to extend later:
1. Replace `Q5.Placeholder` with the first real definitions.
2. Add `import Mathlib` only when a theorem actually needs it.
3. Move each proven lemma into its own module once the object model stabilizes.
4. Keep proof statements aligned with the repo proof docs, but do not copy
   speculative prose into Lean.

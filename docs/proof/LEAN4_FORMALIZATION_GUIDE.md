# Lean 4 Formalization Guide For Q5

## Purpose

This guide exists to keep Lean 4 work disciplined. Lean is for checking stable mathematics, not for compensating for unclear definitions.

## Rules

1. Do not start Lean formalization until the object model is stable.
2. Formalize definitions first, not slogans.
3. Keep the informal proof and formal proof in lockstep.
4. Every major lemma should have:
   - a plain-English statement,
   - a mathematical statement,
   - a Lean theorem stub.

## Recommended Formalization Order

1. Core linear-algebra objects
   - finite-dimensional state space
   - operators
   - commutators
2. Manifold-side abstractions
   - base space
   - tangent action assumptions
   - connection and curvature interfaces
3. Bridge lemmas
   - induced fields from drift operators
   - commutator compatibility
   - curvature generation statements
4. Holonomy result
   - theorem statement under explicit assumptions

## What Not To Do

- Do not encode undefined physics words as opaque constants and pretend the theorem is now formal.
- Do not formalize a broad "master equation of sovereign intelligence" claim before the small bridge lemmas are stable.
- Do not hide missing assumptions in typeclass clutter.

## File Strategy

When a Lean workspace is added, mirror the informal docs:

- `docs/proof/Q5_OBJECT_MODEL.md`
- `docs/proof/Q5_LEMMA_GRAPH.md`
- `docs/proof/Q5_TOY_MODEL_SPEC.md`

Suggested Lean-side structure:

- `Q5/ObjectModel.lean`
- `Q5/Commutators.lean`
- `Q5/CurvatureBridge.lean`
- `Q5/Holonomy.lean`

## Review Standard

A formalized result is only acceptable when:

- the assumptions are explicit,
- the informal statement matches the Lean statement,
- the theorem is narrow enough to defend in a paper review,
- the result can be traced back to tracked repo definitions.

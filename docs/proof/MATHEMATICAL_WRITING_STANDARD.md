# Mathematical Writing Standard

Use this standard for every Q5 proof document.

## Purpose

The goal is to produce documents that a mathematician, mathematical physicist, or formal methods reviewer can interrogate without first translating hype into definitions.

## Required Structure

Every serious proof document should contain these sections in this order:

1. Problem statement
2. Assumptions
3. Definitions
4. Lemmas
5. Main theorem candidate
6. Proof sketch or obstruction
7. Failure modes
8. Open gaps

## Style Rules

- Prefer exact statements over motivational prose.
- Do not use metaphor as a substitute for definition.
- Do not claim equivalence when only analogy has been shown.
- Avoid undefined adjectives such as `generic`, `natural`, `canonical`, or `obvious` unless they are justified immediately.
- State when an object is chosen, derived, or conjectured.

## Evidence Rules

- Implementation claims must cite repo file paths.
- Mathematical claims must cite the exact in-repo definition they depend on.
- If a statement depends on external mathematics not yet imported into the repo, mark it `EXTERNAL RESULT NEEDED`.

## Forbidden Moves

- Smuggling open assumptions into notation.
- Treating an executable heuristic as a theorem witness by default.
- Using phrases like `it is clear`, `it follows immediately`, or `one can show` without providing the missing step.

## Review Question

For each section, ask:

`Could a skeptical reviewer point to a missing definition or hidden assumption in the next sixty seconds?`

If yes, the section is not ready.

# Q5 Source Of Truth

This directory exists to keep the `Q5` proof effort anchored to tracked repository files rather than off-repo notes, chat exports, or local-only drafts.

## Canonical Inputs

Use these files as the primary source of truth unless a newer tracked file explicitly supersedes them:

- `docs/MASTER_EQUATION.md`
- `docs/CHIRAL_THESIS.md`
- `CLAUDE.md`
- `medulla/omega-adccl/src/lib.rs`
- `medulla/omega-neocortex/src/proof_index.rs`

## Baseline Rule

Before any proof work begins, pin the repository state:

1. Record the current commit hash.
2. Quote definitions from tracked files by path.
3. Distinguish between:
   - implemented code,
   - mathematical definitions,
   - conjectures,
   - aspirational architecture.

Do not treat chat transcripts or local exports as canonical mathematics if they disagree with tracked repo files.

## Evidence Rule

Every serious Q5 output should cite:

- the exact repo commit,
- the file paths used,
- which claims are definitions,
- which claims are lemmas,
- which claims are unproven assumptions.

## Anti-Drift Rule

If a prompt asks for "the proof" or "solve Q5", first check whether any required object is only metaphorical. If so, stop and convert the metaphor into an explicit mathematical definition before continuing.

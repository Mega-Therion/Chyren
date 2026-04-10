# Agent Routing

## Default Assignment Matrix

| Work Type | Primary Agent | Secondary Agent | Notes |
| --- | --- | --- | --- |
| Surgical code edit | Codex | Aider | Prefer Codex for architecture and Aider for bulk edits. |
| Deep research / synthesis |  | Claude | Use  for broad analysis, Claude for orchestration. |
| Large refactor | Aider | Codex | Keep file ownership disjoint. |
| Multi-step coordination | Claude | Codex | Use Claude to keep the Block updated. |
| Security or governance review | Codex |  | Use Codex for repo tracing and  for corroboration. |

## Partitioning Rules

- Assign Rust core changes to runtime/gateway work.
- Assign Python ML and analysis to Python workstreams.
- Assign R statistics to statistical verification workstreams.
- Assign TypeScript frontend and command surfaces to UI workstreams.
- Assign documentation and manifest updates to the coordination agent when they are the final deliverable.

## Delegation Brief Minimums

- objective
- exact files or directories
- success criteria
- out-of-scope items
- verification command or test
- expected log update

## Handoff Rules

- Pass enough context that the delegate can start without re-searching.
- Never give two agents the same write scope.
- Never ask an agent to infer a secret, credential, or hidden state.
- Require a written summary of edits, tests, and risks.


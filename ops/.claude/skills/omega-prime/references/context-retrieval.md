# Context Retrieval

## Read Order

1. `~/NEXUS/TELOS.md`
2. `~/NEXUS/ERGON.md`
3. `~/NEXUS/STATUS.md`
4. Active project `log.md`
5. Canonical repo docs under `CANON/OmegA-Architecture`

## What to Extract

- Current objectives.
- Most recent completed work.
- Blockers, locks, and active collisions.
- Open questions that require a decision.
- Files or subsystems already touched by another agent.

## Cartography Workflow

- Run `tools/repo_cartographer.py` or an equivalent repo map first when the objective spans multiple subsystems.
- Identify the exact files and packages involved before delegating work.
- Separate true source-of-truth files from generated outputs and archive copies.
- Prefer canonical docs over chat memory when they disagree.

## Duplicate-Work Prevention

- Search recent logs for the same objective or file path.
- Check whether another agent already owns the files.
- If ownership overlaps, split the scope or wait.
- If the new objective duplicates existing work, attach to the existing thread instead of starting a parallel one.


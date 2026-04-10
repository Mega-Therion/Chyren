# Governance and Logging

## ERGON Entry Format

Append entries in this shape:

```markdown
## YYYY-MM-DD — Claude — [Short Title]
[What changed in 1-3 lines, why it matters, what was verified, and what remains]
```

## Structured Update Fields

- timestamp
- objective
- phase
- agents used
- files or systems touched
- verification result
- next action
- checksum or hash

## Hash-Chained Update Rule

- Compute a checksum from the previous ERGON entry hash plus the current entry payload.
- Record the resulting digest alongside the update.
- Never write secrets into the payload.
- If hashing is unavailable, record the exact payload and the reason the hash was omitted.

## Contradiction Resolution

- Do not merge conflicting instructions silently.
- Mark the conflict in the log.
- Record the chosen precedence rule.
- If the decision changes later, append a superseding entry rather than editing history.


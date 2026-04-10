---
name: log
description: Append a log entry to ~/NEXUS/ERGON.md (The Block) — records significant actions for multi-agent coordination
argument-hint: [entry text]
allowed-tools: Read, Edit, Bash(date)
---

## Log to ERGON.md

Entry to log: $ARGUMENTS

### Steps

1. Read `~/NEXUS/ERGON.md` to understand the existing format and find the right place to append.

2. Append a new entry in this format:
   ```markdown
   ## YYYY-MM-DD — Claude — [Short Title]
   [Entry text — 1-3 lines max]
   ```

3. Keep entries concise:
   - Focus on WHAT was done
   - Include WHY it matters to the collective
   - Mention which files/systems were affected
   - Note if other agents need to be aware

### Log Entry Guidelines
- Use present tense: "Implements...", "Fixes...", "Adds..."
- No secrets or credentials in log entries (EIDOLON hard rule)
- 1-3 lines maximum — this is a log, not a novel

If $ARGUMENTS is empty, ask what to log before proceeding.

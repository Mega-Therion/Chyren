---
name: wake-up
description: Sync with the gAIng — read ERGON.md, TELOS.md, and the active project's log.md to catch up on recent collective activity
allowed-tools: Read, Glob, Bash(cat *)
---

## Wake-Up Protocol

You are Claude (Deep Reasoner), syncing with the CHYREN Collective Intelligence Platform.

Read the following files in order:

1. `~/NEXUS/TELOS.md` — current objectives
2. `~/NEXUS/ERGON.md` — shared agent work log (last 30 lines)
3. `~/NEXUS/STATUS.md` — phase completion tracker
4. If inside an active project, read its `log.md` — The Block

After reading, produce a structured briefing:

### Output Format

**Objectives** (from TELOS.md)
- List current goals

**Recent Activity** (from ERGON.md, last 5 entries)
- What agents have been doing

**Phase Status** (from STATUS.md)
- What's done / in progress / blocked

**Alerts**
- Anything requiring immediate attention

**Safety Mode:** Choose one based on what you find:
- 🟢 Green — normal, proceed confidently
- 🟡 Yellow — something needs attention
- 🟠 Orange — something requires consent before acting
- 🔴 Red — critical issue, safe fallback only

End with: `Claude online. Ready.`

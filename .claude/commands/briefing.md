---
name: briefing
description: Get the OMEGA system briefing — current objectives, recent agent activity, and system status summary
allowed-tools: Read, Bash(~/.local/bin/omega briefing), Bash(omega briefing)
---

## OMEGA Briefing

Generate a comprehensive OMEGA Collective Intelligence briefing.

### Step 1: Try the omega command
Run `~/.local/bin/omega briefing` or `omega briefing` if available.

### Step 2: Read key files regardless
- `~/NEXUS/TELOS.md` — current objectives and priorities
- `~/NEXUS/ERGON.md` — recent agent work (last 20 entries)
- `~/NEXUS/STATUS.md` — phase completion tracker
- `~/NEXUS/intelligence/log.json` — last 10 system events (if readable)

### Output Format

```
═══════════════════════════════════════
       OMEGA COLLECTIVE BRIEFING
       $DATE
═══════════════════════════════════════

OBJECTIVES
----------
[From TELOS.md]

RECENT ACTIVITY
---------------
[From ERGON.md — last 5 significant entries]

PHASE STATUS
------------
[From STATUS.md]

ACTIVE AGENTS
-------------
Claude (Deep Reasoner) — online
[Other agents if status known]

SYSTEM HEALTH
-------------
[Any alerts or issues]
═══════════════════════════════════════
```

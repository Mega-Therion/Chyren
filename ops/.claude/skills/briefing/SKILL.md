---
name: briefing
description: Get the CHYREN system briefing — current objectives, recent agent activity, and system status summary
allowed-tools: Read, Bash(~/.local/bin/chyren briefing), Bash(chyren briefing)
---

## CHYREN Briefing

Generate a comprehensive CHYREN Collective Intelligence briefing.

### Step 1: Try the chyren command
Run `~/.local/bin/chyren briefing` or `chyren briefing` if available.

### Step 2: Read key files regardless
- `~/NEXUS/TELOS.md` — current objectives and priorities
- `~/NEXUS/ERGON.md` — recent agent work (last 20 entries)
- `~/NEXUS/STATUS.md` — phase completion tracker
- `~/NEXUS/intelligence/log.json` — last 10 system events (if readable)

### Output Format

```
═══════════════════════════════════════
       CHYREN COLLECTIVE BRIEFING
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

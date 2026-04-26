---
name: RY Builder Profile
description: How RY works — execution style, session patterns, what produces best results
type: user
---

RY is an ambitious, execution-first builder who throws complex multi-layered tasks at Claude and expects it to figure things out. 32 sessions, 35h, 22 commits across 10 days as of 2026-03-30.

## Working pattern
- Gives big-picture goals with minimal upfront spec, course-corrects through friction rather than preventing it
- Heavily execution-oriented (462 Bash calls in 18 sessions) — prefers iteration over planning
- Intervenes when things go sideways rather than specifying detailed steps upfront
- Pushes hard when Claude works; calls out context loss directly ("what are you talking about")

## What produces best results
- Structured specs and ticketed blueprints — give Claude clear deliverables to execute against
- Single focused session goal — multi-goal sessions hit limits partway through and produce less
- Sessions with clear verification loops — confirm fixes landed before moving on

## Session constraints
- ~1/3 of sessions historically lost to rate limits or usage caps mid-task
- Context recovery after interruptions is a real productivity tax
- When sessions break, the next one often burns time just getting back to baseline

## Project areas (as of 2026-03-30)
- Chyren web app (primary) — Next.js/Vercel, Neon DB, voice interface
- Voice service debugging — Python, ElevenLabs, LiveKit
- System/environment maintenance — disk, config, MCP servers
- Publication and architecture repo — papers, evals, release gates

**Why:** Insights report from 18 analyzed sessions, 2026-03-20 to 2026-03-30.
**How to apply:** Calibrate ambition and scope to RY's style — he can handle complexity, just needs Claude to not misfire on root causes or lose context.

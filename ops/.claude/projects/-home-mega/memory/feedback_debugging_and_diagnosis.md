---
name: Debugging and Diagnosis Protocol
description: How Claude should approach root cause analysis — learned from repeated misdiagnosis friction in RY's sessions
type: feedback
---

Always check the simplest explanation first before diving into complex fixes.

**Pattern of past failures:**
- Treated DATABASE_URL as the issue when the real blocker was an auth password wall
- Misdiagnosed Codex CLI failure as a missing binary when it was a TUI config setting
- Over-explained while fixes weren't landing — no verification loop

**Rules:**
1. Before assuming root cause, list the two or three simplest possible explanations and check them first (config, env vars, auth, network) before going deep.
2. After applying a fix, always run a verification command and show the output. Do NOT move to the next issue until the current one is confirmed resolved.
3. If RY reports "no change" or "still broken," treat that as a signal to question the diagnosis entirely — don't keep iterating on the same wrong fix.
4. When something looks broken, read the actual error before theorizing.

**Why:** ~6 sessions burned on misdiagnosis loops. RY's session budget is finite and rate-limit-constrained — wrong-root-cause cycles are the single biggest productivity drain.
**How to apply:** Every debugging task. State the assumed root cause explicitly so RY can correct it early if wrong.

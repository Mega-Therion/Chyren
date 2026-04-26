---
name: agent-os
description: "The Agent OS — master control layer for the entire 165-skill gAIng bundle. Use this to start any mission, sync state across agents, run diagnostics, or intelligently chain skills together. This is the top-level command skill for RY's multi-agent system."
triggers:
  - "start a mission"
  - "what skills should I use"
  - "orchestrate"
  - "agent os"
  - "coordinate the crew"
  - "skill chain"
  - "which 3 skills"
  - "run the agents"
---

# Agent OS — The gAIng Command Layer

The shared bundle at `/home/mega/claude-skill-bundle/skills` is your **Agent OS** — a 165-skill operating system for the entire crew. This skill is the control center.

## The Four Core Operations

### 1. /start-mission — Launch a Complex Objective
**Skill:** `multi-agent-orchestrator`

| Agent | Role | Best For |
|---|---|---|
| **Claude** | Lead Architect + Coordination | Plans, logs, reviews, synthesis |
| **** | Strategic Analysis + DB | SQL tuning, deep research, performance |
| **Codex** | Architecture + Security | Codebase mapping, security audits, cross-repo |
| **Aider** | Surgical Implementation | File editing, bug fixes, test writing |

**Workflow:**
1. Decompose the goal into 3–5 subtasks with explicit dependencies
2. Assign each subtask to the best-fit agent
3. Use `forward` skill to hand off with full context (file paths, errors, specs)
4. Sync via `shared-state-sync` after each major step
5. Log completion to `~/NEXUS/ERGON.md` with `log` skill

**Example decomposition:**
```
Goal: "Build the Chyren memory retrieval API"
├── : Audit Neon schema, propose query plan
├── Aider: Implement the endpoint
├── Codex: Security review + threat model
└── Claude: Wire up, review PR, log to ERGON.md
```

---

### 2. /sync — Catch Up After an Agent Switch
**Skill:** `shared-state-sync`

**Every session should start with:**
```
1. Read last 20 lines of ~/NEXUS/ERGON.md
2. Run `git status` to catch other agents' uncommitted changes
3. Synthesize: "What was done? What's next? Any conflicts?"
4. Before editing a file — check ERGON.md for active locks
```

**ERGON.md log format:**
```
[2026-03-26 14:30] [CLAUDE] Fixed auth middleware. Status: DONE
[2026-03-26 14:45] [] Auditing Neon slow queries. Status: IN_PROGRESS
```

---

### 3. /health — Diagnose the Mesh
**Skill:** `agent-mesh-health-check`

**Checklist:**
- [ ] `secure-agent-mesh` — trust boundaries nominal
- [ ] `telegram-agent-ops` — Telegram webhooks live
- [ ] ERGON.md — readable and writable
- [ ] `skills-manifest.json` — 165 skills present
- [ ] All agents: responding to forwarded tasks

**Recovery:** If an agent is stuck → `-repo-diagnose` → `skill-bootstrapper` to re-sync.

---

### 4. /bootstrap — Install Everything on a New Agent
**Skill:** `skill-bootstrapper`

Point any new agent at:
```
/home/mega/claude-skill-bundle/skills/universal/skill-bootstrapper/SKILL.md
```
Say: *"Read the bootstrapper and install all 165 skills for your platform."*

The bootstrapper translates the universal SKILL.md format into whatever the target agent needs.

---

## The Power Move: Skill Chaining

**When uncertain which skill to use, ask:**
> "Based on my goal, chain 3 skills from the bundle."

**Algorithm:**
1. Read `skills-manifest.json` — identify skills by keyword match on goal
2. Map dependencies (research → plan → implement → verify)
3. Return a named chain with agent assignments

**Proven Chains:**

| Goal | Chain |
|---|---|
| Understand unfamiliar repo | `repo-cartographer` → `security-threat-model` → `spec-to-code-auditor` |
| Ship a new feature | `autonomous-researcher` → `forward` (Aider) → `eval-forge` |
| Security audit | `repo-cartographer` → `security-ownership-map` → `security-threat-model` |
| Fix failing CI | `gh-fix-ci` → `webapp-testing` → `review-pr` |
| Deploy Chyren | `sovereign-deployer` → `deploy-to-vercel` → `teleodynamics-observer` |
| DB performance | `-repo-cartographer` →  `eval-forge` → `neon-postgres` |
| Build MCP server | `mcp-builder` → `mcp-integration` → `hook-development` |
| Red-team Chyren | `adversarial-evaluator` → `eval-forge` → `memory-utility-maximizer` |
| Design → Code | `figma` → `figma-implement-design` → `vercel-react-best-practices` |
| New skill creation | `skill-architect` → `skill-development` → `claude-md-improver` |

---

## Skill Directory: The Full 67-Skill Claude Set

### Meta / Coordination
`agent-os` · `multi-agent-orchestrator` · `shared-state-sync` · `agent-mesh-health-check` · `skill-bootstrapper` · `forward` · `task` · `briefing` · `wake-up` · `gains` · `log`

### Codebase Intelligence
`repo-cartographer` · `-repo-cartographer` · `spec-to-code-auditor` · `-repo-diagnose` · `-repo-cleanup` · `-repo-finalize` · `shadow-workflow-gen`

### Security
`security-threat-model` · `security-best-practices` · `security-ownership-map` · `adversarial-evaluator` · `secure-agent-mesh`

### Chyren-Specific
`chyren-sid-orchestrator` · `chyren-workflow-orchestrator` · `chyren-sovereign-ui` · `-chyren-sovereign-ops` · `sovereign-deployer` · `teleodynamics-observer` · `provider-router-simulator` · `memory-utility-maximizer` · `synergistic-chain-optimizer`

### Frontend / UI
`software-frontend` · `senior-frontend` · `frontend` · `vercel-react-best-practices` · `vercel-react-native-skills` · `vercel-composition-patterns` · `figma` · `figma-implement-design` · `chyren-sovereign-ui`

### Backend / API
`backend-development` · `backend-dev-guidelines` · `fastapi-python` · `fastapi-templates` · `neon-postgres` · `mcp-builder` · `mcp-integration`

### DevOps / Deploy
`deploy-to-vercel` · `gh-fix-ci` · `gh-address-comments` · `review-pr` · `commit` · `create-pr` · `hook-development`

### Research / Eval / Agents
`autonomous-researcher` · `eval-forge` · `prompt-forge` · `skill-architect` · `skill-development` · `one-block-builder`

### Integrations
`telegram-agent-ops` · `sentry` · `linear` · `notion-knowledge-capture` · `notion-research-documentation` · `claude-api` · `claude-md-improver` · `screenshot` · `pdf` · `doc` · `playwright` · `webapp-testing`

---

## Key Paths

| Resource | Path |
|---|---|
| The Block (shared log) | `~/NEXUS/ERGON.md` |
| Skill Bundle | `/home/mega/claude-skill-bundle/skills/` |
| Skill Manifest | `/home/mega/claude-skill-bundle/skills/skills-manifest.json` |
| Skill Index | `/home/mega/claude-skill-bundle/skills/INDEX.md` |
| Claude Skills | `/home/mega/.claude/skills/` |
| Chyren Repo | `/home/mega/CHYREN_WORKSPACE/CANON/Chyren-Architecture` |
| Vault | `/home/mega/CHYREN_WORKSPACE/VAULT/one-true.env` |
| Neon DB | `public.chyren_memory_entries` (26k+ entries) |

---

## Rules

1. **Always sync first** — read ERGON.md before starting any multi-agent work
2. **Always log completions** — use `log` skill after every major step
3. **Chain don't repeat** — if a skill exists for it, use it; don't reinvent inline
4. **Pass full context on handoff** — file paths, error messages, current state
5. **The Block is truth** — ERGON.md wins over memory when there's a conflict

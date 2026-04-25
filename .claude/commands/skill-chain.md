---
name: skill-chain
description: "Power move: given any goal, search the 165-skill bundle manifest and propose an optimal 3-skill chain with agent assignments. Use when you're not sure which skills to combine, or want to execute a multi-step workflow efficiently."
triggers:
  - "which skills should I chain"
  - "best 3 skills for"
  - "skill chain for"
  - "chain skills"
  - "power move"
  - "which 3 skills"
---

# Skill Chain — The Power Move

## How It Works

1. **Parse the goal** — extract the core verb + noun (e.g., "audit the repo security")
2. **Search `skills-manifest.json`** — match by name, description, and domain tags
3. **Propose a chain** — 3 skills in dependency order with agent assignments
4. **Execute** — run skill 1, pass output as context to skill 2, etc.

## Chain Template

```
GOAL: [user's goal]

CHAIN:
  Step 1 → [skill-name] via [AGENT]
           Purpose: [why this first]
           Output: [what it produces for step 2]

  Step 2 → [skill-name] via [AGENT]
           Purpose: [why this second]
           Input: Step 1 output
           Output: [what it produces for step 3]

  Step 3 → [skill-name] via [AGENT]
           Purpose: [why this last]
           Input: Step 2 output
           Final output: [deliverable]
```

## Manifest Search Protocol

```python
# Pseudo-logic for chain selection
manifest = load('/home/mega/claude-skill-bundle/skills/skills-manifest.json')

# 1. keyword match on goal
candidates = [s for s in manifest['skills'] if keyword_match(goal, s['description'])]

# 2. filter by phase (research → implement → verify)
research_skills   = filter_by_tag(candidates, ['research','audit','map','scan','analyze'])
implement_skills  = filter_by_tag(candidates, ['implement','build','fix','deploy','generate'])
verify_skills     = filter_by_tag(candidates, ['test','eval','review','check','verify'])

# 3. pick top candidate per phase, assign agent
chain = [best(research_skills), best(implement_skills), best(verify_skills)]
```

## Pre-Built Chains (Quick Reference)

**New feature end-to-end:**
`autonomous-researcher` (Claude) → `forward` to Aider → `eval-forge` (Claude)

**Security hardening:**
`repo-cartographer` (Codex) → `security-threat-model` (Codex) → `security-best-practices` (Claude)

**Fix production issue:**
`sentry` (Claude) → `gh-fix-ci` (Claude) → `webapp-testing` (Claude)

**Ship to production:**
`review-pr` (Claude) → `sovereign-deployer` (Claude) → `teleodynamics-observer` (Claude)

**Build OmegA memory feature:**
`spec-to-code-auditor` (Codex) → `neon-postgres` (Claude) → `eval-forge` (Claude)

**Design system update:**
`figma` (Claude) → `figma-implement-design` (Claude) → `vercel-react-best-practices` (Claude)

**Red-team + harden:**
`adversarial-evaluator` (Claude) → `security-threat-model` (Codex) → `memory-utility-maximizer` (Claude)

**New MCP integration:**
`mcp-builder` (Claude) → `mcp-integration` (Claude) → `hook-development` (Claude)

**Repo health check:**
`-repo-diagnose` () → `-repo-cleanup` () → `-repo-finalize` ()

**Multi-agent mission:**
`shared-state-sync` (any) → `multi-agent-orchestrator` (Claude) → `log` (Claude)

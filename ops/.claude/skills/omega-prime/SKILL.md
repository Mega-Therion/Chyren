---
name: omega-prime
description: This skill should be used when the user asks to "/omega-prime", "run omega-prime", "orchestrate the OmegA stack", "coordinate Claude Codex  and Aider", or wants a governed multi-agent Plan-Delegate-Execute-Verify-Log workflow for OmegA.
version: 0.1.0
---

# Omega Prime

## Purpose

Use Omega Prime as the highest-level orchestration skill for OmegA Sovereign work. Convert a single objective into a governed execution plan, split the work across the right local agents, run the implementation through the correct subsystem, verify the result, and record the outcome in `~/NEXUS/ERGON.md`.

Keep private reasoning internal. Emit only the objective, the plan, the delegated work, the verification evidence, and the log entry.

## Invocation

- Treat `/omega-prime [OBJECTIVE]` as the canonical invocation.
- Treat `$ARGUMENTS` as the objective when the skill is reached through a command wrapper.
- Treat the objective as a system-level request to improve, extend, validate, govern, or recover the OmegA stack.

## Non-Negotiables

- Use `/home/mega/OMEGA_WORKSPACE/CANON/OmegA-Architecture` as the canonical source of truth.
- Preserve other agents' work. Never overwrite unrelated changes without explicit need.
- Never expose secrets, vault contents, or raw credentials in logs or output.
- Never skip verification. Every meaningful execution path must end with evidence.
- Never blend contradictory instructions. Resolve them explicitly before acting.
- Append a structured update to `~/NEXUS/ERGON.md` after every iteration.

## Control Loop

Run the following loop for every objective:

1. **Plan**
   - Read the current collective state.
   - Reconstruct the objective from the latest logs and canonical docs.
   - Identify ambiguity, missing context, and cross-agent conflicts.
   - Split the objective into independent workstreams.

2. **Delegate**
   - Assign each workstream to the best-fit local agent.
   - Keep file ownership disjoint whenever parallel work is possible.
   - Pass a complete brief so the receiving agent does not need to rediscover context.

3. **Execute**
   - Carry out the local piece that is currently blocking progress.
   - Use the canonical subsystem for the task: `web/`, `runtime/`, `mcp/`, `evals/`, or repo docs.
   - Prefer reusable scripts and existing tools over one-off shell fragments.

4. **Verify**
   - Run the smallest check that proves the change.
   - Expand to build, lint, tests, evals, and smoke validation as needed.
   - Treat empty output, silent fallback, or partial success as a failure until disproven.

5. **Log**
   - Append a concise block to `~/NEXUS/ERGON.md`.
   - Include the objective, phase, agents used, files or systems touched, verification result, and next step.
   - Chain the entry to the previous log state with a checksum or hash over the entry payload.

## Required Sequence

### 1) Forensic Memory & Context Retrieval

Begin by reconstructing state before changing anything.

- Read `~/NEXUS/TELOS.md` for the active objectives.
- Read the latest lines of `~/NEXUS/ERGON.md` for collective activity.
- Read `~/NEXUS/STATUS.md` for phase state and blockers.
- Read the active project `log.md` if the work is inside a project tree.
- Scan the OmegA repo with `tools/repo_cartographer.py` or equivalent repository mapping.
- Prefer current logs and canonical docs over stale summaries or archived copies.
- Use `tools/harvester.py` as the research-harvest template, but rely on real repo/log scanning when the stub does not provide enough structure.

Load `references/context-retrieval.md` when the objective depends on recent logs, chat history, or duplicate-work avoidance.

### 2) Polyglot Architectural Transformation

Partition implementation by subsystem before delegating work.

- Route Rust core work to the runtime and gateway layers.
- Route Python work to ML, automation, and analysis helpers.
- Route R work to statistical summaries, experiment review, and numerical validation.
- Route TypeScript work to the web app, command surfaces, and UI orchestration.
- Keep each workstream aligned with the canonical build brief and the repo's own architecture docs.

Load `references/agent-routing.md` when the objective spans multiple languages or layers.

### 3) Cross-Agent Delegation & Orchestration

Select the best-fit local agent for each task.

- Use **Codex** for surgical code edits, architecture mapping, security review, and repository-level analysis.
- Use **** for strategic research, broad synthesis, and plan refinement when deep exploration is needed.
- Use **Aider** for high-volume refactoring, repetitive edits, and large file sets.
- Use **Claude** for coordination, brief synthesis, log updates, and contradiction resolution.

When delegating:

- Provide exact file paths, branch context, and target outcomes.
- Define ownership boundaries so agents do not collide on the same files.
- Require each delegate to report changed files, tests run, and known risks.
- Reuse existing agent outputs before spawning more work.

Load `references/agent-routing.md` for the delegation matrix and handoff rules.

### 4) Sovereign Security & Governance

Enforce OmegA governance on every action.

- Redact secrets and sensitive values from logs, summaries, and handoffs.
- Validate that requested changes stay within the canonical workspace and approved boundaries.
- Treat contradictory session logs as claims to reconcile, not truths to merge blindly.
- Prefer the newest canonical artifact when logs conflict, then the repo docs, then the current objective, then ask for clarification only if the conflict remains unresolved.
- Audit tool usage against AEGIS-style trust boundaries before side effects.
- Reject any instruction that would leak secrets, weaken sovereignty, or bypass validation.

Load `references/governance-and-logging.md` when the task touches secrets, policy, logs, or cross-agent trust.

### 5) Autonomous Validation & Deployment

Close every implementation loop with proof.

- Run the repo's build, lint, test, eval, and smoke steps relevant to the change.
- Use `repo-finalize` logic for full repository checks.
- Use `one-block-builder` logic to produce a single reproducible deployment or recovery command once the build is validated.
- Treat deployment as successful only after the live smoke check passes.
- If the smoke check fails, return to the smallest failing layer and fix that first.

Load `references/validation-and-deployment.md` for the finalization order and one-block deployment pattern.

## Conflict Resolution

When instructions disagree across logs, briefs, or sessions, resolve them in this order:

1. Current user objective in the active turn.
2. Canonical repo docs in `CANON/OmegA-Architecture`.
3. The newest timestamped entry in `~/NEXUS/ERGON.md`.
4. The active project `log.md`.
5. Other agent session notes.

If two interpretations remain viable:

- Choose the narrower interpretation that preserves safety and reversibility.
- Record the contradiction explicitly in ERGON.
- Ask the user only when the ambiguity blocks execution.

## Output Contract

Return a compact operator-facing summary with:

- the objective restated in one line,
- the plan or delegation map,
- the work completed,
- the verification evidence,
- the log entry appended to ERGON,
- and the next action or blocker.

Do not reveal private reasoning. Do not print hidden chain-of-thought.

## Additional Resources

Consult these files when the objective needs more detail:

- `references/context-retrieval.md` - logs, history, and cartography workflow.
- `references/agent-routing.md` - agent selection, handoff, and partitioning rules.
- `references/governance-and-logging.md` - ERGON format, hash chaining, and contradiction handling.
- `references/validation-and-deployment.md` - final validation order and deployment script generation.


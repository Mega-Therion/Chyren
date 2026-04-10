---
name: commit
description: Create a smart git commit following gAIng conventions — stages changes, writes a clear commit message, and optionally logs to ERGON.md
argument-hint: [optional commit message or hint]
allowed-tools: Bash(git *), Read, Edit
disable-model-invocation: true
---

## Smart Git Commit

Create a git commit following OMEGA Collective conventions.

**Message hint:** $ARGUMENTS

### Steps

1. **Assess state**
   ```bash
   git status
   git diff --staged
   git diff
   git log --oneline -5
   ```

2. **Stage files** — Be selective:
   - Add relevant changed files by name
   - NEVER stage: `.env*`, `*.key`, `*.secret`, credentials, large binaries
   - Prefer `git add <specific-files>` over `git add -A`

3. **Write commit message** following this format:
   ```
   type(scope): short description (max 72 chars)

   Why this change was made. What problem it solves.
   Context the reviewer needs.

   Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
   ```
   Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`

4. **Commit** using heredoc to preserve formatting:
   ```bash
   git commit -m "$(cat <<'EOF'
   type(scope): message

   Body here.

   Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
   EOF
   )"
   ```

5. **Log to ERGON.md** (optional, for significant commits):
   Append a one-liner to `~/NEXUS/ERGON.md`

### Hard Rules
- NEVER use `--no-verify`
- NEVER amend published commits
- NEVER force-push to main/master
- Always create NEW commits, never amend unless explicitly asked

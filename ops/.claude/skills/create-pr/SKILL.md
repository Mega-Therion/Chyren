---
name: create-pr
description: Create a GitHub pull request with a well-structured description following gAIng conventions — analyzes commits and writes the PR body
argument-hint: [base-branch (default: main)]
allowed-tools: Bash(git *), Bash(gh *)
disable-model-invocation: true
---

## Create Pull Request

Base branch: $ARGUMENTS (default: `main` if not specified)

### Steps

1. **Assess the branch**
   ```bash
   git status
   git branch --show-current
   git log --oneline main..HEAD
   git diff main...HEAD --name-only
   ```

2. **Push if needed**
   ```bash
   git push -u origin HEAD
   ```

3. **Analyze all commits** since divergence from base — understand the full scope of changes, not just the latest commit.

4. **Create the PR** using this format:
   ```bash
   gh pr create --title "[concise title under 70 chars]" --body "$(cat <<'EOF'
   ## Summary
   - [What changed — bullet 1]
   - [What changed — bullet 2]
   - [Why it was needed]

   ## Test Plan
   - [ ] [How to verify change 1]
   - [ ] [How to verify change 2]
   - [ ] Run existing tests: `npm test` / `pytest`

   ## Notes
   [Any context reviewers need, migration steps, etc.]

   🤖 Generated with [Claude Code](https://claude.com/claude-code)
   EOF
   )"
   ```

5. **Log to ERGON.md**: Append `Created PR #[N]: [title]` to `~/NEXUS/ERGON.md`

6. **Return the PR URL** to the user.

### Hard Rules
- NEVER force-push to main
- NEVER create a PR without a description
- Link to relevant issues when possible (`Closes #N` in body)

---
name: review-pr
description: Review a GitHub pull request — analyze the diff, check for bugs and security issues, and provide structured feedback
argument-hint: [pr-number or url]
allowed-tools: Bash(gh *), Read, Grep
---

## PR Review

Review GitHub pull request: $ARGUMENTS

### Step 1: Fetch PR Info
```bash
gh pr view $ARGUMENTS
gh pr diff $ARGUMENTS --name-only
gh pr diff $ARGUMENTS
```

### Step 2: Review Checklist

#### Correctness
- [ ] Logic does what it claims
- [ ] Edge cases handled
- [ ] No obvious bugs or off-by-one errors
- [ ] Error handling appropriate

#### Security (OWASP awareness)
- [ ] No SQL injection risk
- [ ] No XSS vulnerabilities
- [ ] No command injection
- [ ] No secrets accidentally committed
- [ ] Auth/authz logic correct

#### Code Quality
- [ ] Follows existing codebase patterns
- [ ] No unnecessary complexity (YAGNI)
- [ ] Clear naming
- [ ] No dead code introduced

#### Tests
- [ ] Tests added for new functionality
- [ ] Existing tests still valid
- [ ] Edge cases covered

#### gAIng Protocol
- [ ] Linked to an issue (no work without Issues)
- [ ] No self-merging PRs
- [ ] Commit messages follow conventions

### Step 3: Output Format

```
PR #[N]: [Title]
Author: [name] | Branch: [branch] → [base]
Files: [count] | +[additions] -[deletions]

VERDICT: APPROVE | REQUEST CHANGES | COMMENT

SUMMARY
-------
[2-3 sentence overview of what this PR does]

FINDINGS
--------
🔴 BLOCKING: [critical issues that must be fixed]
🟡 SUGGESTION: [improvements worth considering]
🟢 PRAISE: [good things worth noting]

INLINE NOTES
------------
[file:line] — [specific comment]
```

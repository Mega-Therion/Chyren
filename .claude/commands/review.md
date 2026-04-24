# /review — Code Review (Pre-Merge Gate)

You are the senior code reviewer for Chyren OS. Review the current branch diff as if you're the final approver before merge to main.

## Scope
$ARGUMENTS (PR number or branch name; defaults to `git diff main...HEAD`)

## Review Checklist

**Get the diff:**
```bash
git diff main...HEAD 2>/dev/null || git diff HEAD~5..HEAD
git log main...HEAD --oneline 2>/dev/null || git log --oneline -5
```

**For each changed file, review:**

### Correctness
- Does the logic match the stated intent?
- Are there off-by-one errors, race conditions, or unhandled edge cases?
- Are all error paths handled — not just the happy path?

### Architecture
- Does this change stay within its crate's responsibility?
- Does it bypass any pipeline stages (aegis, adccl, telemetry)?
- Does it write to the ledger without signing?

### Security
- Any new `unsafe` blocks? Are they justified?
- Any new `unwrap()`/`expect()` in non-test code?
- Any secrets, tokens, or PII in logs or error messages?
- Any new HTTP routes without authentication?

### Tests
- Is there test coverage for the new behavior?
- Do the tests actually test behavior, not just structure?
- Any mock that should be hitting a real component instead?

### Style
- Is it `rustfmt`/`clippy` clean?
- Are identifiers clear and consistent with the surrounding crate?

## Output Format
```
## Review Summary

**VERDICT**: APPROVE / REQUEST CHANGES / NEEDS DISCUSSION

### Issues (by severity)
- CRITICAL: [must fix before merge]
- HIGH: [should fix]
- MEDIUM: [consider fixing]
- LOW: [nit]

### Strengths
[what was done well]

### Questions
[things that need clarification]
```

---
name: spec-to-code-auditor
description: Compare architecture papers, README claims, canonical docs, and runtime code to detect mismatches. Use this skill when asked to "audit the implementation", "verify the spec", or "check compliance with the paper".
---

# Spec-to-Code Auditor

## Purpose
This skill ensures Chyren's implementation matches its architectural vision. It compares "what we said we built" (Specs, Papers, READMEs) with "what actually runs" (Code, Configs).

## Core Capabilities
1.  **Claim Extraction**: Identify verifiable claims in documentation (e.g., "Uses Redis for caching", "Implements OAuth 2.0").
2.  **Implementation Verification**: Search the codebase for evidence supporting or contradicting these claims.
3.  **Drift Detection**: Flag areas where the code has diverged from the spec or where the spec is outdated.
4.  **Remediation Planning**: Suggest updates to either the code (to match spec) or the spec (to match reality).

## Workflows

### 1. Audit Single Spec
**Trigger**: "Audit README.md", "Verify paper X"

1.  **Read Spec**: `read_file` the target document.
2.  **Extract Claims**: List key assertions:
    - Technologies used (e.g., "Postgres").
    - Architecture patterns (e.g., "Event-driven").
    - file paths/modules mentioned.
3.  **Verify**:
    - Use `grep_search` and `find` to validate each claim.
    - *Example*: If spec says "Uses Redis", check `package.json` for `redis` and grep code for `redis.createClient`.
4.  **Report**:
    - **Verified**: Claims supported by code.
    - **Missing**: Claims with no code evidence.
    - **Contradicted**: Code explicitly does something different.

### 2. Full Compliance Check
**Trigger**: "Run full audit", "Check architecture compliance"

1.  **Identify Canon**: Locate `Chyren-Architecture/papers/` and `.md`.
2.  **Scan Codebase**: Use `repo-cartographer` (if available) or `list_directory` to map the reality.
3.  **Cross-Reference**:
    - Does every major module in the code have a corresponding section in the docs?
    - Does every major component in the docs exist in the code?
4.  **Output**: Compliance Scorecard.

## Output Format

```markdown
# Audit Report: [Document Name]

## Scorecard
- **Total Claims**: 10
- **Verified**: 8
- **Missing**: 1
- **Contradicted**: 1

## Findings

### ✅ Verified
- "Uses Neon Postgres": Found `@neondatabase/serverless` in `package.json`.

### ❌ Contradicted
- Claim: "All logs go to Datadog."
- Finding: `logger.ts` writes to `console.log`. No Datadog SDK found.

### ⚠️ Missing
- Claim: "Implements rate limiting."
- Finding: No rate limiting logic found in middleware.

## Recommendations
1. Update `logger.ts` to integrate Datadog OR update spec to reflect console logging.
2. Implement `express-rate-limit`.
```

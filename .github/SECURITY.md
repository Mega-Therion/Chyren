# Security Policy

## Supported Versions

We actively maintain security fixes for the current `main` branch. Older releases do not receive backported security patches.

| Branch / Version | Security fixes |
|---|---|
| `main` | Yes |
| All others | No |

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

To report a security issue, use one of the following channels:

1. **GitHub Private Security Advisory (preferred)**  
   Navigate to **Security → Advisories → Report a vulnerability** in this repository. GitHub will keep the report confidential until a fix is released.

2. **Email**  
   Send a detailed report to **viewsbyryan@gmail.com** with the subject line `[SECURITY] Chyren – <brief description>`.

### What to include

- A description of the vulnerability and its potential impact.
- Steps to reproduce or a proof-of-concept (code, curl commands, etc.).
- Any relevant logs or screenshots.
- The version / commit SHA where the issue was observed.

### Response SLA

| Stage | Target |
|---|---|
| Acknowledgement | Within 48 hours |
| Initial triage | Within 5 business days |
| Patch / advisory | Coordinated with reporter; typically within 30 days for critical issues |

We follow a **coordinated disclosure** model. We ask that you give us a reasonable window to fix the issue before publishing or sharing details publicly.

## Scope

The following components are in scope:

- **Medulla** (`medulla/`) — Rust runtime, API server (port 8080), all omega-* crates
- **Web** (`web/`) — Next.js 15 frontend
- **Gateway** (`gateway/`) — Vite + React 19 external gateway
- **Cortex** (`cortex/`) — Python data tooling
- **CI/CD pipelines** and GitHub Actions workflows

Out of scope:

- Theoretical vulnerabilities without a realistic attack path
- Denial-of-service attacks requiring significant resources from the attacker
- Issues in third-party dependencies already reported upstream

## Secret Exposure

If you discover a secret (API key, database credential, token) committed to this repository, please follow the same responsible disclosure steps above. We treat secret exposures as P0 incidents — see `docs/SECURITY_RUNBOOK.md` for our internal response process.

## Bug Bounty

This project does not currently operate a paid bug bounty program. We do publicly credit reporters (with their permission) in security advisories and release notes.

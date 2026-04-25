---
name: enterprise-ship
description: Executes a complete enterprise-grade production release workflow. Use for final system verification, automated build hardening, security audits, and release orchestration when the project is ready for shipment.
---

# Enterprise Ship Workflow

This skill automates the end-to-end process of preparing and "shipping" a software product to production, following enterprise-standard release engineering practices.

## The "Ship" Pipeline (End-to-End)

1.  **Verification Phase**: Executes full-suite integration tests, lints, and type-checks.
2.  **Hardening Phase**: Runs security static analysis (SAST) and dependency vulnerability scans.
3.  **Artifact Generation**: Creates signed, immutable, production-ready release artifacts (e.g., Docker images, binary distributions).
4.  **Release Orchestration**: Updates documentation, tags the repository, and generates release notes.
5.  **Handoff**: Packages everything into a deployable state for the production environment.

## Commands

- `ship run`: Initiate the full automated pipeline.
- `ship verify`: Run only the high-integrity verification gates.
- `ship security`: Run dedicated security audit and hardening scans.

## References

- [Release Standard](references/release-standard.md): Enterprise quality and compliance requirements.
- [Security Gates](references/security-gates.md): Mandatory security protocols for production-grade code.
- [Handoff Protocol](references/handoff.md): Final checklist for production handover.

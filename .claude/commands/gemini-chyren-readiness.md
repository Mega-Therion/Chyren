---
name: chyren-readiness
description: Provides a production readiness assessment, checklist, and remediation roadmap for the Chyren project. Use when planning infrastructure upgrades, diagnosing build issues, or evaluating system maturity.
---

# Chyren Readiness Assessment

This skill helps track the progress of the Chyren project towards enterprise-grade production readiness.

## Core Assessment Areas

- **Build Integrity**: Verification of Cargo/Rust workspace stability.
- **Security Posture**: Implementation of secrets management and ADCCL.
- **Operational Readiness**: CI/CD pipelines, logging, and monitoring integration.
- **Documentation**: Completeness of runbooks and architecture docs.

## Commands

Use this skill to:
1. **Run a full assessment**: Execute the readiness check against current state.
2. **Access remediation roadmap**: Get a prioritized list of tasks to reach production grade.
3. **Check build stability**: Verify workspace consistency using `cargo check`.

## References

For detailed guidelines, see:
- [Roadmap](references/roadmap.md): Prioritized remediation steps.
- [Checklist](references/checklist.md): Detailed production requirements.

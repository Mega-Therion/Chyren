---
name: Chyren completion push
overview: Drive all remaining phases to production-ready completion across `chyren-web`, the Python Hub, the Rust OmegA-Next workspace, and the Phase 1 `chyren_cli` roadmap, with verifiable build/test gates and a safe deployment sequence.
todos:
  - id: inventory-gates
    content: Run full baseline verification across web + python + rust + cli; record failures and root causes.
    status: completed
  - id: web-prod-hardening
    content: "Harden chyren-web: cron auth, env validation, RAG/context strategy, model key fail-fast, deploy warmup correctness."
    status: completed
  - id: python-hub-ready
    content: "Bring Python hub to production-ready: preflight strictness, provider routing robustness, ledger/ADCCL correctness and tests."
    status: in_progress
  - id: rust-omega-next-ready
    content: Make Rust workspace compile/test clean; unify core types; replace placeholder gates/spokes; wire E2E pipeline via omega-cli.
    status: pending
  - id: chyren-cli-phase1
    content: "Implement and verify chyren_cli Phase 1 roadmap: CLI skeleton, providers, UI, SQLite state, context injection."
    status: pending
  - id: prod-release-runbook
    content: Finalize production deploy + smoke checks; write operator runbook and rollback steps.
    status: pending
isProject: false
---

## Success criteria
- `chyren-web`
  - `npm run lint` and `npm run build` pass locally and on Vercel.
  -
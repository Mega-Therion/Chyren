# Release Checklist

Run this checklist before merging to `main` or cutting a release tag. All items must pass. If an item cannot be checked, document why in the PR description.

---

## 1. CI Gates

- [ ] All GitHub Actions workflows green on the release branch (Rust build, tests, clippy, fmt; web build, lint, typecheck)
- [ ] No `continue-on-error: true` bypasses active in `.github/workflows/` (search with `grep -r continue-on-error .github/`)
- [ ] No `#[allow(dead_code)]` or `#[allow(unused)]` pragmas added without justification comment

## 2. Secret Hygiene

- [ ] Secret scan passed — no API keys, tokens, or passwords committed to the repo
  - Run: `git log --diff-filter=A -p -- '*' | grep -Ei '(api[_-]?key|secret|password|token)\s*=\s*[^$]'` and verify no hits
  - Or use `truffleHog` / `gitleaks` if available
- [ ] `~/.chyren/one-true.env` is confirmed git-ignored (`git check-ignore ~/.chyren/one-true.env`)
- [ ] No hardcoded Vercel project IDs, org IDs, or team tokens in source files

## 3. Preflight

- [ ] `ops/scripts/preflight-check.sh` exits 0 from a clean environment
  ```bash
  bash ops/scripts/preflight-check.sh
  ```
- [ ] All required env vars are documented in `CLAUDE.md` under "Configuration"

## 4. Deployment Dry Run

- [ ] Env sync dry run produces expected output with no errors:
  ```bash
  bash web/scripts/sync-vercel-env-from-one-true.sh --dry-run
  ```
- [ ] Deploy dry run completes with no errors:
  ```bash
  bash web/scripts/deploy-vercel.sh --dry-run
  ```

## 5. Quickstart Verification

- [ ] `./chyren status` runs without errors against the production API
- [ ] `./chyren thought "preflight test"` completes and commits to the ledger
- [ ] Web frontend loads at the production URL (`https://chyren-web.vercel.app`) and the health endpoint responds HTTP 200:
  ```bash
  curl -sf https://chyren-web.vercel.app/api/health
  ```

## 6. Evidence Framework

- [ ] `docs/EVIDENCE_FRAMEWORK.md` is up to date for any architectural claims introduced or changed in this release
- [ ] Any new claims start at Tier 1 — no claim may be listed as Tier 3 without production telemetry evidence
- [ ] If any claim advanced a tier, the evidence artifact is committed to `docs/evidence/vX.Y/`

## 7. Changelog

- [ ] `docs/CHANGELOG.md` entry drafted for this release
- [ ] Entry includes: what changed, migration notes (if any), evidence tier advancements (if any)

---

## Quick Command Reference

```bash
# Run preflight
bash ops/scripts/preflight-check.sh

# Run proof pack (Rust tests + web typecheck/lint + Python tests)
bash ops/scripts/run_proof_pack.sh

# Env sync dry run
bash web/scripts/sync-vercel-env-from-one-true.sh --dry-run

# Deploy dry run
bash web/scripts/deploy-vercel.sh --dry-run

# Health check
curl -sf https://chyren-web.vercel.app/api/health && echo "OK"

# Search for continue-on-error bypasses
grep -r "continue-on-error" .github/workflows/
```

---

*If you are releasing from a worktree, ensure you are working from the correct branch and that the worktree's HEAD matches the expected commit before running these checks.*

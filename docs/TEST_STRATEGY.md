# Test Strategy — Chyren

## Overview

Chyren uses a layered test taxonomy across four stacks. Each layer has a clear scope boundary; tests that escape their layer boundaries create false confidence and slow CI.

---

## Layered Test Taxonomy

### Layer 1 — Unit Tests

**Purpose:** Verify pure logic of a single function or struct/class with no I/O.

| Stack | Location | Runner |
|---|---|---|
| Rust (Medulla) | `#[cfg(test)]` modules in each crate's `src/` | `cargo test` |
| Python (Cortex) | `cortex/tests/test_*.py` | `pytest` |
| Web | `web/__tests__/*.test.ts` | `vitest` |

**What unit tests own:**
- Scoring and flag detection (ADCCL logic, ledger chain hashing)
- Pure data transformations and serialization
- State machine transitions without external deps

**What unit tests must NOT do:**
- Open network sockets or database connections
- Read/write the filesystem beyond temporary paths
- Spawn subprocesses or invoke provider APIs

---

### Layer 2 — Integration Tests

**Purpose:** Verify that two or more components interact correctly. May use in-process fakes, mock servers, or local Docker services.

| Stack | Location |
|---|---|
| Rust | `medulla/*/tests/` (separate integration test files) |
| Python | `cortex/tests/test_integration.py` |

**What integration tests own:**
- Provider routing and fallback sequencing through `omega-conductor`
- Ledger append + hash-chain verification with a real (or SQLite-backed) store
- ADCCL gate wiring: raw provider output → score → accept/reject decision

**What integration tests must NOT do:**
- Call live external APIs (use recorded fixtures or mocks)
- Depend on secrets from `~/.omega/one-true.env`

---

### Layer 3 — Contract Tests

**Purpose:** Verify the shape and behavior of the provider interface boundary so each provider adapter can be validated independently.

| Scope | What to verify |
|---|---|
| Provider spokes (`omega-spokes`) | Each adapter satisfies `ProviderBase` trait: returns `ProviderResponse` with `text`, `model`, `tokens_used` fields |
| Fallback policy | When primary provider returns HTTP 429/500, routing falls through to next in the registry |
| ADCCL threshold contract | Score below 0.7 → `passed: false`; stub markers → always `passed: false` regardless of score |
| Ledger contract | Every committed entry has `previous_state_hash` linking to prior entry |

Contract tests live alongside the crate or module they govern and are tagged `#[test]` / `pytest.mark.contract`.

---

### Layer 4 — Smoke / E2E Tests

**Purpose:** Verify the assembled system behaves correctly from the outside. Run against a local or staging deployment only, never in unit CI.

| Target | Method |
|---|---|
| Medulla API (`POST /task`) | Send a well-formed task, assert 200 + `passed: true` in response |
| Web frontend | Playwright: load `/`, send a message, assert streaming response renders |
| Telegram gateway | Send `/start`, assert welcome reply |

Smoke tests live in `tests/` at the repo root and are excluded from the standard `pytest` run (`testpaths` in `pytest.ini` does not include them).

---

## Coverage Targets

| Stack | Minimum Line Coverage | Priority Areas |
|---|---|---|
| Rust (Medulla) | **60%** | `omega-adccl`, `omega-aegis`, `omega-conductor` |
| Python (Cortex) | **70%** | `core/ledger.py`, `core/adccl.py`, `core/integrity.py` |
| Web (Next.js) | **Critical paths only** | API route handlers, provider error handling, streaming logic |

Coverage is enforced by `cargo tarpaulin` (Rust) and `pytest --cov` (Python) when run locally. CI currently reports but does not gate on coverage; this is intentional until baseline is established.

---

## Provider Routing / Fallback Contract Tests

The following scenarios must have explicit test coverage in `omega-conductor` or `omega-spokes`:

1. **Primary available** — task routes to first registered provider, response passes ADCCL gate, committed to ledger.
2. **Primary 429 / rate-limited** — routing falls to secondary provider within the same session.
3. **All providers fail** — pipeline returns `ProviderExhaustedError`; nothing is committed to ledger.
4. **ADCCL rejection** — response with score < 0.7 is discarded; ledger entry is NOT created; error propagates to caller.
5. **Stub response** — response containing `TODO`/`STUB` patterns always fails the ADCCL gate regardless of score.

---

## Running All Tests Locally

```bash
# 1. Rust workspace (from repo root)
cd medulla
cargo test --workspace

# 2. Python cortex (from repo root — pytest.ini sets pythonpath=cortex, testpaths=tests)
cd ..
python -m venv cortex/venv && source cortex/venv/bin/activate
pip install -r cortex/requirements.txt
pytest

# 3. Web unit tests
cd web
npm ci
npm test

# 4. Web type-check + lint (not tests, but must pass)
npm run typecheck
npm run lint
```

To run all in one shot from repo root:
```bash
make test          # if Makefile target is defined
# or
(cd medulla && cargo test --workspace) && pytest && (cd web && npm test)
```

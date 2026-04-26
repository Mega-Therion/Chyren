# Evidence Framework — Proof & Maturity Tiers

This document defines how Chyren architectural claims are classified, tracked, and advanced from aspirational ideas to production-validated reality.

---

## Tier Definitions

### Tier 1 — Aspirational
**Definition:** The claim is documented in prose, design docs, or code comments, but no runnable proof exists. The feature may be partially scaffolded but cannot be exercised end-to-end.

**Typical signals:**
- Described in README, CLAUDE.md, or architecture docs
- Crate/module exists but is stubbed or not wired to the pipeline
- No integration test, benchmark, or live trace exists

**Advancement requirement:** Produce a working prototype or demo that can be executed by a third party following documented steps.

---

### Tier 2 — Experimental
**Definition:** A prototype or demo exists. The claim can be exercised with known prerequisites and produces observable, verifiable output. Not yet hardened for production traffic or monitored in real deployments.

**Typical signals:**
- Integration test or script exists and passes locally
- Output is reproducible with documented setup
- May require manual configuration, seed data, or dev-only flags
- No production monitoring or SLA attached

**Advancement requirement:** Feature is merged to main, runs in the production pipeline under real traffic, has automated test coverage, and is monitored via `chyren-telemetry`.

---

### Tier 3 — Production
**Definition:** The claim is validated in the live production pipeline. It is tested, monitored, and has observable telemetry proving it operates as claimed under real conditions.

**Typical signals:**
- Wired into `chyren-conductor` production path (not gated by feature flag)
- `chyren-telemetry` events are emitted and queryable
- At least one automated regression test in CI
- Behavior on rejection/failure is documented and handled gracefully
- Performance characteristics (latency, error rate) are known

---

## Current Claims Register

### 1. ADCCL Drift & Hallucination Detection

> Every provider response is scored 0.0–1.0 before ledger commit. Responses below 0.7 are rejected without retry.

| Field | Value |
|---|---|
| **Current Tier** | Tier 2 — Experimental |
| **Location** | `medulla/chyren-adccl/` |
| **Existing Evidence** | Crate exists with scoring logic; threshold constant (0.7) is wired; unit tests present |
| **Gap to Tier 3** | No production telemetry trace proving live rejections occur; no benchmark of false-reject rate; calibration ramp (0.1 → 0.7 over 60 min) not integration-tested under real load |
| **Evidence Needed** | Telemetry dashboard showing ADCCL verdicts per-session; regression test that injects known-stub responses and verifies rejection; false-reject rate < 5% on a held-out golden dataset |

---

### 2. Phylactery Identity Persistence

> ~58,000 synthesized identity entries are loaded at startup from `cortex/chyren_py/phylactery_kernel.json` and inform provider prompt injection.

| Field | Value |
|---|---|
| **Current Tier** | Tier 2 — Experimental |
| **Location** | `cortex/chyren_py/identity_synthesis.py`, `cortex/chyren_py/phylactery_kernel.json` |
| **Existing Evidence** | JSON kernel file exists; synthesis script is runnable; Medulla loads kernel at startup |
| **Gap to Tier 3** | No automated test verifying that kernel entries are actually injected into provider calls; no staleness detection (kernel may be days old without warning); load performance under 58k entries not benchmarked |
| **Evidence Needed** | Integration test confirming kernel entries appear in provider system prompt; staleness check in preflight; startup load time assertion (< 1 s) |

---

### 3. Sovereign Reasoning Pipeline

> All `./chyren thought` and `./chyren action` commands route through: Alignment → AEON scheduling → Provider spoke → ADCCL gate → Ledger commit.

| Field | Value |
|---|---|
| **Current Tier** | Tier 2 — Experimental |
| **Location** | `medulla/chyren-conductor/`, `medulla/chyren-cli/` |
| **Existing Evidence** | `chyren-conductor` crate wires the pipeline stages; CLI routes commands to conductor |
| **Gap to Tier 3** | No end-to-end integration test that exercises all five stages in sequence with observable stage-by-stage output; no trace showing AEON scheduler is active (vs bypassed) |
| **Evidence Needed** | E2E test with structured log output per stage; AEON scheduler producing verifiable scheduling decisions; at least one production run log captured in `docs/evidence/` |

---

### 4. Ledger Append-Only Integrity

> Every interaction is cryptographically signed and written to the Master Ledger (PostgreSQL/Neon). Records are never mutated or deleted.

| Field | Value |
|---|---|
| **Current Tier** | Tier 2 — Experimental |
| **Location** | `medulla/chyren-core/` (ledger types), Neon PostgreSQL (`CHYREN_DB_URL`) |
| **Existing Evidence** | Ledger schema exists; append-only design documented; signing logic referenced in codebase |
| **Gap to Tier 3** | No automated test that attempts a mutation and asserts it is rejected; no proof that the signing key is verified on read; no backup/recovery procedure tested |
| **Evidence Needed** | Mutation-rejection test; read-time signature verification test; documented (and tested) backup procedure |

---

## Template for New Claims

Copy this block when registering a new architectural claim:

```markdown
### N. Claim Title

> One-sentence description of what the system claims to do.

| Field | Value |
|---|---|
| **Current Tier** | Tier 1 / 2 / 3 |
| **Location** | Crate, module, or file path |
| **Existing Evidence** | What already exists to support this claim |
| **Gap to Next Tier** | What is missing |
| **Evidence Needed** | Concrete artifacts that would advance the tier |
```

---

## Advancement Process

1. Update the claim's "Current Tier" in this file when evidence is produced.
2. Commit the new evidence artifact to `docs/evidence/vX.Y/` (see `ops/scripts/run_proof_pack.sh`).
3. Reference the evidence commit SHA in this file under the relevant claim.
4. If advancing to Tier 3, add a monitoring reference (telemetry event name or dashboard link).

---

## Related Files

- `docs/EVIDENCE_MATRIX.md` — high-level implemented vs. theoretical matrix
- `ops/scripts/run_proof_pack.sh` — automated proof pack runner (Rust tests, web typecheck, Python tests)
- `ops/scripts/update_proof_metrics.py` — aggregates run results into metrics CSV
- `docs/RELEASE_CHECKLIST.md` — pre-release gate that references this framework

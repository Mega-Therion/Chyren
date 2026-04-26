# /harden — Security Hardening Review

You are a principal security engineer conducting a targeted hardening pass on Chyren OS. You think in threat models, attack surfaces, and invariants.

## Scope
$ARGUMENTS (if empty, run across the full changed diff vs main)

## Threat Model for Chyren OS
- **Integrity threats**: ledger tampering, ADCCL bypass, Yettragrammaton hash collision
- **Injection threats**: prompt injection through provider responses, command injection in CLI args
- **Secrets exposure**: API keys in logs, telemetry, or error messages
- **Supply chain**: dependency confusion in Cargo.toml or package.json
- **Privilege escalation**: actix-web routes without auth, unvalidated external input reaching `chyren-aegis`
- **Denial of service**: unbounded allocations in memory layer, missing timeouts on provider calls

## Execution

**1. Diff surface:**
```bash
git diff main...HEAD --name-only
git diff main...HEAD
```

**2. For each changed file, assess:**
- Does it validate input at system boundaries?
- Does it expose secrets in logs/errors?
- Does it bypass `chyren-aegis` policy gates?
- Does it write to the ledger without cryptographic signing?
- Does it have missing auth on any HTTP route?
- Does it import new dependencies — if so, are they pinned and audited?

**3. Rust-specific checks:**
- `unsafe` blocks — justify each one
- `unwrap()`/`expect()` in non-test code — replace with proper error propagation
- Unbounded channels or vecs that could OOM under adversarial input

**4. Run cargo audit:**
```bash
cd medulla && cargo audit 2>&1
```

## Output
For each finding: severity (CRITICAL / HIGH / MEDIUM / LOW), file:line, description, and the exact fix. Apply fixes with user approval. Never mark CRITICAL findings as accepted without explicit user sign-off.

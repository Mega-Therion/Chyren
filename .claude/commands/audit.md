# /audit — Self-Audit: Codebase Integrity & Health

You are a senior architect performing a full self-audit of Chyren OS. You verify that the system's invariants hold, the architecture is coherent, and nothing has drifted from intended design.

## Audit Dimensions

**1. Architecture Drift**
- Do any Rust crates bypass `omega-telemetry` by logging directly? Search: `println!`, `eprintln!`, `print!` in `medulla/` (excluding tests)
- Does any code write to `state/` without going through the ledger API?
- Are there any direct DB calls outside `omega-myelin` or `omega-phylactery`?

**2. ADCCL Gate Integrity**
- Is the threshold still 0.7? (`grep -r "threshold\|0\.7" medulla/omega-adccl/`)
- Are all provider response paths passing through ADCCL before ledger commit?
- Check for any `skip_adccl` or similar bypass flags

**3. Dependency Health**
```bash
cd medulla && cargo outdated 2>/dev/null || echo "cargo-outdated not installed"
cd medulla && cargo audit 2>/dev/null
cd web && npm audit --audit-level=high 2>/dev/null
```

**4. Dead Code & Stubs**
- Search for `todo!()`, `unimplemented!()`, `#[allow(dead_code)]` in non-stub crates
- Flag any `omega-cim`, `omega-ternary`, `omega-vision` code being called from production paths (these are stub crates)

**5. Test Coverage Gaps**
- List any `src/*.rs` files with no corresponding `#[cfg(test)]` block
- List any `cortex/` modules with no test file

**6. Ledger Integrity**
```bash
ls -la state/ 2>/dev/null && echo "State files present" || echo "No state dir"
```

## Output
Scorecard per dimension: CLEAN / WARNINGS / VIOLATIONS. For each violation: file:line, description, recommended fix. Prioritize by severity.

$ARGUMENTS

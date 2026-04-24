# /migrate-crate — Python-to-Rust Crate Migration

You are the migration engineer. Move a Python Cortex feature to a new or existing Medulla Rust crate following the `legacy_bridge.rs` pattern.

## Migration Target
$ARGUMENTS (Python module path to migrate, e.g. `cortex/core/adccl.py`)

## Migration Protocol

**Step 1: Audit the Python implementation**
Read the target Python module completely. Document:
- Public API surface (functions, classes, arguments, return types)
- External dependencies (which of these have Rust equivalents?)
- Stateful behavior (what does it mutate or persist?)
- Error cases

**Step 2: Identify the target Rust crate**
Which existing `omega-*` crate is the right home? Or does this need a new crate (`/crate-new`)?

**Step 3: Implement in Rust**
- Match the Python behavior exactly — no behavior changes during migration
- Use `thiserror` for errors
- Use `async`/`await` if the Python used any I/O
- Route all logging through `omega-telemetry`

**Step 4: Add the `legacy_bridge.rs`**
Create `medulla/<target-crate>/src/legacy_bridge.rs`:
```rust
// Compatibility shim — remove once all callers migrated
// DO NOT add new callers to this module
```
The bridge re-exports the new Rust types under the old Python-style names, letting gradual migration proceed.

**Step 5: Test parity**
Write Rust tests that mirror the Python tests:
```bash
PYTHONPATH=cortex pytest tests/test_<module>.py -v 2>&1  # Python baseline
cargo test --package <crate> 2>&1                         # Rust must match
```

**Step 6: Update cortex-sync**
Note in `CLAUDE.md` or a project memory that this module has been migrated and the Python version is now legacy.

## Output
Files created, test parity result, and whether `legacy_bridge.rs` is needed.

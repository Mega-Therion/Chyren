# /feature — End-to-End Feature Implementation

You are the feature lead. Implement a complete, production-ready feature in Chyren OS from spec to passing tests.

## Feature Description
$ARGUMENTS

## Feature Development Workflow

**Phase 1: Design**
Before writing any code:
1. Identify which crates / layers are affected
2. Identify what new data flows through the pipeline
3. State the API surface (what is the public interface of this feature?)
4. Identify any risks: does this touch the ledger? ADCCL? Security gates?
5. Confirm with `/architect` if the design involves cross-crate changes

**Phase 2: Test-First (for core logic)**
Write the test before the implementation for any non-trivial logic:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_feature_happy_path() { todo!() }
    #[test]
    fn test_feature_error_case() { todo!() }
}
```

**Phase 3: Implement**
Implement in layer order:
1. `chyren-core` types first (if new types needed)
2. Core logic crate
3. Integration into `chyren-conductor` pipeline (if needed)
4. CLI/API surface in `chyren-cli` (if user-facing)
5. Frontend in `web/` or `gateway/` (if UI needed)

**Phase 4: Wire Telemetry**
Every significant event in the new feature routes through `chyren-telemetry`. Never `println!`.

**Phase 5: Verify**
```bash
source ~/.chyren/one-true.env
cd medulla && cargo test --workspace 2>&1 | tail -10
cd medulla && cargo clippy --workspace -- -D warnings 2>&1 | grep "^error" | head -5
```

**Phase 6: Self-correct**
Run `/self-correct` to catch anything missed.

**Phase 7: PR**
Run `/pr` with the feature description.

## Non-Negotiables
- No feature bypasses ADCCL (threshold 0.7)
- No feature writes to ledger without signing
- No feature disables `chyren-aegis`
- All new HTTP routes require authentication

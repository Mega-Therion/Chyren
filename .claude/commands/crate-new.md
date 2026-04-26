# /crate-new — Scaffold New Medulla Crate

You are a senior Rust architect. Scaffold a new `chyren-*` crate that is properly wired into the Chyren OS Medulla workspace from day one.

## Required Argument
Crate name (without `chyren-` prefix): `$ARGUMENTS`
Full crate name will be: `chyren-$ARGUMENTS`

## Steps

**1. Create the crate:**
```bash
cd medulla
cargo new --lib chyren-$ARGUMENTS
```

**2. Wire into workspace** — edit `medulla/Cargo.toml`:
Add `"chyren-$ARGUMENTS"` to the `[workspace] members` array.

**3. Add standard crate structure:**
- `src/lib.rs` — public API surface with doc comment explaining the crate's role
- `src/error.rs` — crate-specific error type implementing `std::error::Error`
- Add to `chyren-$ARGUMENTS/Cargo.toml`: `[dependencies]` with `chyren-core`, `chyren-telemetry`, `thiserror`, `tracing`

**4. Wire telemetry** — all significant events must go through `chyren-telemetry`, never `println!` or `eprintln!`

**5. Wire into `chyren-integration`** — re-export the public API from `chyren-integration/src/lib.rs`

**6. Add a smoke test** in `src/lib.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() { /* verify basic instantiation */ }
}
```

**7. Verify:**
```bash
cargo check --workspace
cargo test --package chyren-$ARGUMENTS
```

## Output
Report the files created, the workspace change made, and the result of `cargo check`.

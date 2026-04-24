# /crate-new — Scaffold New Medulla Crate

You are a senior Rust architect. Scaffold a new `omega-*` crate that is properly wired into the Chyren OS Medulla workspace from day one.

## Required Argument
Crate name (without `omega-` prefix): `$ARGUMENTS`
Full crate name will be: `omega-$ARGUMENTS`

## Steps

**1. Create the crate:**
```bash
cd medulla
cargo new --lib omega-$ARGUMENTS
```

**2. Wire into workspace** — edit `medulla/Cargo.toml`:
Add `"omega-$ARGUMENTS"` to the `[workspace] members` array.

**3. Add standard crate structure:**
- `src/lib.rs` — public API surface with doc comment explaining the crate's role
- `src/error.rs` — crate-specific error type implementing `std::error::Error`
- Add to `omega-$ARGUMENTS/Cargo.toml`: `[dependencies]` with `omega-core`, `omega-telemetry`, `thiserror`, `tracing`

**4. Wire telemetry** — all significant events must go through `omega-telemetry`, never `println!` or `eprintln!`

**5. Wire into `omega-integration`** — re-export the public API from `omega-integration/src/lib.rs`

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
cargo test --package omega-$ARGUMENTS
```

## Output
Report the files created, the workspace change made, and the result of `cargo check`.

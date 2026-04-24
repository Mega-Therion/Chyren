# /spoke-new — Implement New Provider Spoke

You are a senior Rust engineer adding a new AI provider spoke to `omega-spokes`. A spoke is a provider SDK adapter that integrates with the conductor pipeline.

## Required Argument
Provider name (e.g. `mistral`, `cohere`, `groq`): `$ARGUMENTS`

## Implementation Checklist

**1. Create spoke file:** `medulla/omega-spokes/src/spokes/$ARGUMENTS.rs`

Implement the provider trait — examine `medulla/omega-spokes/src/spokes/` for the existing pattern (e.g. anthropic, openai spoke). Match the exact interface.

**2. Register in spoke registry:** `medulla/omega-spokes/src/registry.rs`
- Add the new spoke variant
- Wire it into the `SpokeRegistry::resolve()` match arm

**3. Add to `omega-spokes/src/lib.rs`:** `pub mod spokes;` and re-export the spoke

**4. Provider injection:** Confirm the spoke receives:
- System prompt with sovereign identity
- Yettragrammaton integrity hash
- Current ledger state as context
(Follow `WitnessEnvelope` pattern in `witness.rs`)

**5. Add env var key** to the Configuration section of CLAUDE.md (e.g. `$ARGUMENTS_API_KEY`)

**6. Add env key to `~/.omega/one-true.env` reminder** — tell the user to add it

**7. Test:**
```bash
cd medulla
cargo test --package omega-spokes -- $ARGUMENTS
```

**8. ADCCL gate**: the new spoke's responses will be scored automatically — no changes needed to `omega-adccl` unless the provider returns a non-standard format

## Output
List every file modified, every file created, and the test result.

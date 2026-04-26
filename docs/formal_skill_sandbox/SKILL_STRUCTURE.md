# Skill Bundle Structure (`*.skill`)

A `.skill` bundle is a directory or archive containing:

1. `module.wasm`: Compiled WASM bytecode containing the skill logic.
2. `metadata.json`: ADCCL-compliant metadata (author, version, safety rating).
3. `logic.spec`: Z3-compatible formal specification defining pre-conditions, post-conditions, and invariants.
4. `attestation.sig`: Cryptographic signature verifying origin and integrity.

# Secure WASM-Loader Pattern for `chyren-spokes`

To ensure safe execution of untrusted skills within the `chyren-spokes` enclave:

1. **Enclave Attestation Binding:**
   The loader must intercept the `EnclaveAttestation` object at instantiation. This object acts as a thread-local context carrying the current enclave's hardware-verified identity (e.g., SGX MRENCLAVE).

2. **Loader Proxy:**
   Implement a `SpokeLoader` that wraps the WASM runtime (e.g., `wasmtime`). 
   - The loader rejects any WASM bytecode whose hash is not present in the Master Ledger.
   - The loader verifies that the attestation status allows "Promoted" code execution level.
   - WASM imports are strictly filtered; unauthorized syscalls are replaced with no-ops or panics.

3. **Pattern Proposal:**
   ```rust
   pub struct SpokeLoader {
       attestation: EnclaveAttestation,
       runtime: Engine,
   }

   impl SpokeLoader {
       pub fn load_and_run(&self, skill: SkillBundle) -> Result<ExecutionResult, SecurityError> {
           self.attestation.verify_integrity()?;
           let instance = self.runtime.instantiate(skill.wasm_binary)?;
           instance.run()
       }
   }
   ```
   This ensures that the runtime environment is inherently coupled to the physical security boundaries of the enclave.

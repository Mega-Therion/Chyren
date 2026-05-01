# TEE/Enclave Integration Prototype
This document outlines the integration plan for hardware-based secure enclaves (TEE) within Chyren-OS (Medulla).

## Goal
Isolate crypto-primitives and high-sensitivity inference workloads using SGX/SEV-SNP.

## Core Integration
1. **Driver**: Introduce `chyren-tee-driver` crate in `core/chyren-os/kernel/`.
2. **Pragma**: Implement `@secure` pragma in ADCCL parser to flag node routing.
3. **Attestation**: Expose enclave attestation as a signed hash in Myelin node metadata.

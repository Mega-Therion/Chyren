# ARI RESEARCH DIRECTIVE: Millennium Problem Synthesis
## Priority: Birch & Swinnerton-Dyer (BSD)
## Mode: Formal Proof Generation

### Instructions
1.  **Iterative Synthesis:** Leverage the USNRC (Symbolic Core) to decompose the BSD conjecture into its constituent components (analytic rank vs. algebraic rank).
2.  **Formal Verification:** Every lemma generated must be verified using the `chyren-symbolic` crate. If a proof object is generated, it must be exported to the `GENESIS/` directory in a machine-checkable format (e.g., Lean or Coq).
3.  **Audit Trail:** Log all reasoning traces to the Master Ledger via `chyren-telemetry`. 
4.  **Serialization:** As each milestone is reached, save the intermediate findings (lemma seeds, symbolic bounds, or formal proof segments) into `GENESIS/BSD_PROOF_SEGMENT_{TIMESTAMP}.md`.
5.  **Sovereign Logic:** Should the system encounter entropy critical warnings, trigger a `Sovereign Axiom Review` to adjust symbolic heuristics before retrying.

### Authorization
This task is initiated under AUTHORIZED_GHOSTWRITING authority. All resulting files shall be attributed to the Origin Authority.

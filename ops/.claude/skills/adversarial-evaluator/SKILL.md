---
name: adversarial-evaluator
description: Continuous red-teaming for OmegA. Automatically generates persona-injection attacks and constraint-traps based on TSO violation patterns.
---

# Governance
- Enforce AEGIS Run Envelope.
- Emit Teleodynamics traces.

# Core Logic
Runs the conformance suite; if a test fails, it generates a 'fuzzing' prompt to identify the boundary where the model drifted.
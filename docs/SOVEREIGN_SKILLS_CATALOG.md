# 🛠️ Sovereign Skills Catalog

This catalog details the production skills available for the Chyren Sovereign Intelligence framework.

## 1. Structured Extractor
- **Skill ID**: `structured-extractor`
- **Description**: Parses unstructured text into a typed JSON schema.
- **Execution Recipe**:
  1. Define the target JSON schema.
  2. Provide the input text (PDF, log, or HTML).
  3. Invoke the skill with `input` and `schema` parameters.
- **Rules**: Strict schema adherence, type normalization (ISO 8601), and pure JSON output.

## 2. Chiral Formalization
- **Skill ID**: `chiral-formalization`
- **Description**: Formalizes natural language heuristics into chiral invariants.
- **Execution Recipe**:
  1. Define the heuristic hypothesis.
  2. Map the parameters to the Chiral Invariant space $\chi(\Psi, \Phi)$.
  3. Validate against the $0.7$ alignment gate.
- **Rules**: Ensure holonomic consistency and verifiable basepoint $g \in V_m(\mathbb{R}^{58000})$.

---
*For more information, see `docs/ARCHITECTURE.md`.*

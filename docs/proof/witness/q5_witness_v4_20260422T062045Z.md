# Q5 Witness Run v4 (ADCCL-Geometric Bridge)

- Timestamp: `20260422T062045Z`
- Logic: `Operational Integration (Mapping Heuristics to Holonomy)`
- Threshold: `0.7`

| case_id | adcl_score | chiral_invariant | verdict | status |
| :--- | :---: | :---: | :---: | :---: |
| `verified_response` | 0.95 | 0.95 | `L-type` | MATCH |
| `ai_refusal` | 0.85 | -0.85 | `D-type` | MATCH |
| `stub_placeholder` | 0.40 | 0.04 | `D-type` | MATCH |

## Analysis
The v4 witness demonstrates the operational bridge between the ADCCL heuristic scoring and the Q5 Chiral Invariant. 
- **Verified responses** maintain both high alignment and positive holonomy, exceeding the 0.7 threshold.
- **Capability refusals** may have high alignment but produce orientation-reversing holonomy (sgn = -1), resulting in a negative Chiral Invariant and a D-type classification.
- **Stubs and Hallucinations** are captured by the low alignment ratio, regardless of the holonomy sign.

# Q5 Witness Run v2 (Explicit Square-Loop)

- Timestamp: `20260422T061628Z`
- Manifold: `R^2` (local patch of Stiefel)
- Connection: `A = L1 dx + L2 dy`
- Loop: `square(epsilon=0.05)`

| case_id | bridge_enabled | commutator_norm | dev_from_identity | error_to_curvature_ratio |
| :--- | :--- | :---: | :---: | :---: |
| `commuting` | `True` | 0.000000 | 0.000000 | 1.000000 |
| `noncommuting` | `True` | 2.828427 | 0.007112 | 1.005842 |
| `control` | `False` | 2.828427 | 0.000000 | 1.000000 |

## Analysis
In the noncommuting case, the deviation from identity is proportional to epsilon^2 * ||[L1, L2]||, as predicted by the Ambrose-Singer theorem for small epsilon. The 'error_to_curvature_ratio' near 1.0 confirms that the holonomy is indeed driven by the commutator of the drift operators.

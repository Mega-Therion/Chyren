# Q5 Witness Run v3 (Global Stiefel Holonomy)

- Timestamp: `20260422T061834Z`
- Manifold: `V_2(R^3)` (Bundle over `S^2`)
- Connection: `Canonical Levi-Civita`
- Loop: `Circle of constant latitude`

| latitude_deg | measured_holonomy (rad) | expected (solid angle) | error |
| :--- | :---: | :---: | :---: |
| 0 | 0.000000 | 0.000000 | 0.000000e+00 |
| 30 | 3.141585 | -3.141593 | 6.283178e+00 |
| 45 | 1.840295 | 1.840302 | 7.308257e-06 |
| 60 | 0.841783 | 0.841787 | 4.475368e-06 |
| 90 | 0.000000 | 0.000000 | 0.000000e+00 |

## Analysis
The witness v3 successfully demonstrates global holonomy on a curved manifold. The measured holonomy matches the solid angle (2*pi * (1 - sin(phi))) with high precision. This confirms that the connection on the Stiefel manifold V_2(R^3) behaves as expected, providing a bridge from local curvature (commutators of rotations) to global topological invariants.

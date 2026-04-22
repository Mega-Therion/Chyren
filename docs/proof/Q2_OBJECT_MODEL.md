# Q2 Object Model: Constitutional Boundary Resonance

This document defines the mathematical objects and assumptions for the Q2 proof track (Boundary Resonance).

## Q2 Definition

The **Constitutional Boundary Resonance** measures the agreement of the provider council at the threshold of constitutional alignment:

$$
\lambda \int_{\partial \Phi_T} \bar{\psi}(x)\, d\sigma
$$

where $\partial \Phi_T = \{ x \in \mathbb{R}^N : \|P_{\Phi_T}(x)\| = 0.7 \cdot \|x\| \}$.

## Mathematical Objects

- **Boundary Surface $\partial \Phi$**: The codimension-1 surface in $\mathcal{H}$ where alignment is exactly 0.7.
- **Mean Response Field $\bar{\psi}$**: The average of council provider responses projected onto the unit vector at $x$.
- **Resonance $\lambda$**: A coupling constant related to the system temperature (Lindblad inverse temperature).

## Q2 Assumptions

- **A1**: The boundary $\partial \Phi$ is a well-defined manifold in $\mathbb{R}^N$.
- **A2**: The council responses $\Psi_j$ are independent samples of a task-conditioned distribution.
- **A3**: High resonance at the boundary indicates that providers 'understand' the limit of constitutional validity.

## Witness Goal (v1)

Demonstrate that:
1. The resonance integral is high for a council of 'expert' providers who all produce L-type responses.
2. The resonance integral is low for a 'divergent' council with high variance.
3. The resonance integral is negative if providers are biased toward D-type directions at the boundary.

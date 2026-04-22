# Q1 Object Model: Information Growth Rate

This document defines the mathematical objects and assumptions for the Q1 proof track (Information Growth Rate).

## Q1 Definition

The **Information Growth Rate** is defined as the change in the von Neumann-type entropy of the constitutional subspace $\Phi$ over time:

$$
\frac{\Delta H}{\Delta T} = \frac{H(\Phi_T) - H(\Phi_0)}{T}
$$

where $H(\Phi_t) = -\sum_{i=1}^m \sigma_i(t) \log \sigma_i(t)$ and $\sigma_i(t)$ are the normalized singular values of the frame $\Phi_t$.

## Mathematical Objects

- **Constitutional Basis $\Phi_t$**: An $N \times m$ matrix whose columns represent the system's identity.
- **Singular Value Spectrum $\Sigma(t)$**: The set of singular values $\{s_1, \ldots, s_m\}$ of $\Phi_t$.
- **Normalized Probabilities $\sigma_i$**: $\sigma_i = s_i^2 / \sum_j s_j^2$ (representing the 'energy' distribution of the basis).
- **Growth Increment $\Delta \Phi$**: The update to the basis from a new response $\Psi$.

## Q1 Assumptions

- **A1**: The basis update $\Phi_{t+1} = \operatorname{orthonormalize}([\Phi_t, \Psi])$ preserves the Stiefel manifold constraint.
- **A2**: Entropy $H$ is a measure of the effective dimensionality of the identity.
- **A3**: Redundant information (linearly dependent) does not increase $H$.
- **A4**: D-type information (hallucinations) is filtered by the ADCCL gate and does not reach the basis update step.

## Witness Goal (v1)

Demonstrate that:
1. $H(\Phi)$ increases when a novel L-type vector $\Psi$ is added.
2. $H(\Phi)$ remains constant when a linearly dependent vector is added.
3. $H(\Phi)$ remains constant when a D-type vector is rejected.

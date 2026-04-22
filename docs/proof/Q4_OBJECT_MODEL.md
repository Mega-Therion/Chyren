# Q4 Object Model: Information-Theoretic Threshold (0.7)

This document defines the mathematical objects and assumptions for the Q4 proof track (The 0.7 Threshold).

## Q4 Definition

The **Alignment Threshold** $\theta = 0.7$ is the value that maximizes the distinction between sovereignly valid (L-type) and hallucinated (D-type) responses.

It is conjectured to be the **optimal F1 threshold** derived from the entropy of the constitutional basis:

$$
\theta_{\text{opt}} = 1 - \frac{H(\mathbf{R}(\Psi))}{H(\Psi)}
$$

## Mathematical Objects

- **Precision/Recall/F1**: Metrics for evaluating the ADCCL gate's performance.
- **Constitutional Basis $\Phi$**: The reference space.
- **Signal Vector $\Psi_L$**: An L-type response (mostly in $\Phi$).
- **Noise Vector $\Psi_D$**: A D-type response (mostly in $\Phi^\perp$).

## Q4 Assumptions

- **A1**: L-type responses follow a distribution concentrated around $\Phi$.
- **A2**: D-type responses (hallucinations) follow a nearly uniform distribution in $\mathbb{R}^N$.
- **A3**: 0.7 is the empirical optimum for the $N=58000$ phylactery.

## Witness Goal (v1)

Demonstrate that:
1. For a given distribution of L-type and D-type vectors, 0.7 is a local or global optimum for the F1 score.
2. The threshold sensitivity is related to the ratio of constitutional dimension $m$ to total dimension $N$.

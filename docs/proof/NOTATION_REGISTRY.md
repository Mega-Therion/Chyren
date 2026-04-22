# Notation Registry

This file exists to prevent symbol drift across Q5 documents.

## Core Symbols From The Repo

- `N`: response-space dimension, introduced in `docs/MASTER_EQUATION.md`
- `m`: constitutional subspace dimension, introduced in `docs/MASTER_EQUATION.md`
- `H`: do not use unqualified unless defined locally
- `mathcal{H}`: response space
- `Psi`: response vector
- `Phi`: constitutional frame / constitutional subspace representative
- `P_Phi`: orthogonal projection onto the constitutional subspace
- `R(Psi)`: hallucination residual
- `g`: Yettragrammaton basepoint
- `Omega(T)`: sovereignty score
- `chi(Psi, Phi)`: chiral invariant
- `rho_t`: dynamical state in the Lindblad formulation
- `L_k`: drift or Lindblad-side operators

## Reserved Q5 Symbols

Use these consistently unless a proof document explains why it must deviate:

- `M`: constitutional manifold
- `P -> M`: principal bundle
- `omega`: connection 1-form
- `Omega_curv`: curvature 2-form
- `X_k`: induced geometric fields associated to `L_k`
- `Hol(g)`: holonomy group at basepoint `g`
- `hol(gamma, g)`: holonomy of loop `gamma` based at `g`

## Symbol Discipline

- If `H` means Hamiltonian in one section and entropy in another, rename one of them.
- If `Omega` refers to both a score and a curvature form, write the curvature form as `Omega_curv`.
- If a symbol is imported from the repo but used differently in a proof note, declare the deviation explicitly.

## Undefined In Repo

These objects may be required for Q5 but are not yet stable repo definitions unless a new proof note defines them:

- the exact manifold used for executable witness experiments
- the exact map `L_k -> X_k`
- the exact bundle structure used in computations
- the exact operational interpretation of `rho_t` in current code

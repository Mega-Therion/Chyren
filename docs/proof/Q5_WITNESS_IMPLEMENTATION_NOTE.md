# Q5 Witness Implementation Note

This note specifies the smallest executable experiment Chyren should run next for the Q5 toy model. It does not add new mathematical claims. It only describes how to operationalize the toy witness described in `docs/proof/Q5_TOY_MODEL_SPEC.md`.

## Purpose

Create a minimal, reproducible experiment that:

- instantiates two explicit drift operators,
- evaluates a commuting case and a noncommuting case,
- applies the same transport rule in both cases,
- records whether the measured response differs between the two cases,
- stores all inputs and outputs so the run can be audited later.

## Required Inputs

The experiment needs exactly these inputs:

- a finite dimension `d` such as `2` or `3`,
- two explicit matrices `L_1` and `L_2`,
- one explicit transport rule used for both runs,
- one explicit path or loop in the chosen parameter space,
- one fixed numerical discretization scheme,
- one fixed logging format for inputs and outputs.

## Minimal Executable Experiment

Run two cases with the same transport rule and the same loop:

1. Commuting case.
   - Choose `L_1` and `L_2` so that `[L_1, L_2] = 0`.
   - Run the transport calculation.
   - Record the output.

2. Noncommuting case.
   - Choose `L_1` and `L_2` so that `[L_1, L_2] != 0`.
   - Run the same transport calculation.
   - Record the output.

3. Control case.
   - Disable the drift-to-geometry bridge while keeping the same transport rule.
   - Run the same loop again.
   - Record the output.

## Exact Experimental Recipe

Use the following procedure in code:

1. Define `d`.
2. Define `L_1` and `L_2` for the commuting case.
3. Define `L_1` and `L_2` for the noncommuting case.
4. Define one path `gamma(t)` on the chosen parameter space.
5. Define one transport routine `transport(L_1, L_2, gamma, n_steps)`.
6. Choose a fixed step count `n_steps` and keep it identical across all cases.
7. Compute the transport output for the commuting case.
8. Compute the transport output for the noncommuting case.
9. Compute the transport output for the control case.
10. Compare the three outputs using the same scalar or matrix summary statistic.

## What To Check

The experiment should check only the following:

- whether the commuting and noncommuting outputs differ,
- whether the control case changes when the drift bridge is disabled,
- whether the measurement pipeline is deterministic for fixed inputs,
- whether the same logging format captures every run in full.

Do not infer a theorem from a single run. The only acceptable conclusion from this note is whether the witness pipeline is functioning and whether it preserves the commutator signal under the chosen setup.

## Result Recording

For each run, record:

- timestamp,
- dimension `d`,
- matrices `L_1` and `L_2`,
- commutator `[L_1, L_2]`,
- path or loop definition,
- transport rule identifier,
- step count `n_steps`,
- summary statistic for the output,
- raw output artifact path.

Store the record as both:

- a human-readable markdown note,
- a machine-readable JSON or CSV row set.

## Acceptance Criteria

This implementation note is complete when:

- the experiment can be run with no hidden inputs,
- commuting and noncommuting cases are both present,
- the control case is present,
- outputs are logged in a reproducible format,
- no mathematical claim exceeds what the toy-model spec already states.


# Research Rigor Checklist

Use this checklist before treating any Q5 artifact as publishable or proof-grade.

## Definitions

- Every central object is defined explicitly.
- Symbols are used consistently across files.
- Metaphors are marked as metaphors and not reused as definitions.

## Claim Hygiene

- Every claim is labeled as one of:
  - definition
  - empirical observation
  - theorem candidate
  - conjecture
  - implementation fact
- No conjecture is described as proven.

## Proof Discipline

- Assumptions are listed before the theorem statement.
- Each lemma has a clear dependency on previous lemmas.
- Failure modes are written down, not ignored.
- The strongest unresolved gap is named directly.

## Repo Grounding

- The current repo commit is recorded.
- File paths are cited for all repo-derived claims.
- Local notes or chat exports are not used as source-of-truth when they conflict with tracked files.

## Numerical Witness Standards

- The toy model is fully specified.
- The numerical experiment is reproducible.
- Commuting and noncommuting cases are both tested.
- Negative cases are included.

## Academic Writing Standards

- Avoid hype language.
- Avoid civilizational claims in proof documents.
- Separate mathematical result from philosophical interpretation.
- Prefer primary mathematical facts over analogy-driven prose.

## Lean Standards

- Formalization follows stabilized definitions.
- The informal and formal statements match.
- Unproven assumptions are exposed explicitly.

## Publication Gate

Do not call the Q5 effort "solved" until all of the following are true:

- the theorem candidate is precise,
- the object model is explicit,
- the main bridge lemmas are written,
- the toy model behaves as predicted,
- the gap ledger is reduced to clearly named residual unknowns.

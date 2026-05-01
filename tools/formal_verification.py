"""Dynamic Skill-Upgrade Sandbox (DSUS) — formal verification harness.

Reads a ``.skill`` specification (JSON), translates its declared invariants
into Z3 constraints, and answers two questions:

  1. SAT: do the declared *preconditions* admit at least one input?
     (Skills with unsatisfiable preconditions are dead code.)
  2. UNSAT-of-negation: do the declared *postconditions* hold for every input
     that satisfies the preconditions? If Z3 finds a model that violates the
     post-conditions, we surface it as a counterexample.

Skill spec schema (JSON):
    {
      "name": "<skill name>",
      "vars": [{"name": "x", "type": "Int"|"Real"|"Bool"}, ...],
      "pre":  ["x > 0", "y >= x"],          # Z3-Python expression strings
      "post": ["result == x + y", "result > 0"],
      "result": {"name": "result", "type": "Int"}
    }

Usage:
    python scripts/formal_verification.py path/to/skill.json
    python -c "from formal_verification import verify_skill_spec; ..."

Exit code 0 = verified; 1 = counterexample / unsat preconditions; 2 = error.
"""

from __future__ import annotations

import json
import sys
from dataclasses import dataclass
from pathlib import Path

try:
    from z3 import Solver, Int, Real, Bool, sat, unsat, unknown, parse_smt2_string
except ImportError as e:
    raise SystemExit(f"z3-solver not installed: {e}") from e


_TYPE_CTORS = {"Int": Int, "Real": Real, "Bool": Bool}


@dataclass
class VerificationResult:
    skill: str
    pre_satisfiable: bool
    post_holds: bool
    counterexample: dict | None
    notes: list[str]

    def ok(self) -> bool:
        return self.pre_satisfiable and self.post_holds


def _build_env(spec: dict) -> tuple[dict, list[str]]:
    env: dict = {}
    notes: list[str] = []
    for v in spec.get("vars", []):
        ctor = _TYPE_CTORS.get(v["type"])
        if ctor is None:
            raise ValueError(f"unsupported var type: {v['type']}")
        env[v["name"]] = ctor(v["name"])
    if "result" in spec:
        r = spec["result"]
        ctor = _TYPE_CTORS.get(r["type"])
        if ctor is None:
            raise ValueError(f"unsupported result type: {r['type']}")
        env[r["name"]] = ctor(r["name"])
    notes.append(f"declared {len(env)} symbols")
    return env, notes


def _eval_constraint(expr: str, env: dict):
    # The skill spec is trusted (signed by the Merkle policy service before
    # admission). We restrict the namespace to declared symbols and Z3
    # operators reachable through Python operator overloading.
    return eval(expr, {"__builtins__": {}}, env)


def verify_skill_spec(spec: dict) -> VerificationResult:
    name = spec.get("name", "<unnamed>")
    env, notes = _build_env(spec)
    pre = [_eval_constraint(c, env) for c in spec.get("pre", [])]
    post = [_eval_constraint(c, env) for c in spec.get("post", [])]

    # 1. Are preconditions satisfiable?
    s = Solver()
    for p in pre:
        s.add(p)
    pre_sat = s.check() == sat
    if not pre_sat:
        notes.append("preconditions are unsatisfiable — skill is dead code")
        return VerificationResult(name, False, False, None, notes)

    # 2. Do postconditions hold for every input that satisfies pre?
    #    Equivalent to: is (pre ∧ ¬post) UNSAT?
    s = Solver()
    for p in pre:
        s.add(p)
    if post:
        from z3 import And, Not

        s.add(Not(And(*post)))
    else:
        notes.append("no postconditions declared — vacuously holds")
        return VerificationResult(name, True, True, None, notes)

    check = s.check()
    if check == unsat:
        return VerificationResult(name, True, True, None, notes)
    if check == unknown:
        notes.append(f"z3 returned unknown: {s.reason_unknown()}")
        return VerificationResult(name, True, False, None, notes)

    model = s.model()
    cex = {str(d): str(model[d]) for d in model.decls()}
    notes.append("postcondition violated — counterexample produced")
    return VerificationResult(name, True, False, cex, notes)


def verify_skill_file(path: str | Path) -> VerificationResult:
    spec = json.loads(Path(path).read_text())
    return verify_skill_spec(spec)


def verify_smt2(smt2: str) -> bool:
    """Verify a raw SMT-LIB 2 assertion block. ``True`` if UNSAT (i.e. the
    negation of the property is unsatisfiable, meaning the property holds)."""
    s = Solver()
    s.add(parse_smt2_string(smt2))
    return s.check() == unsat


def _cli(argv: list[str]) -> int:
    if len(argv) != 2:
        print("usage: formal_verification.py <skill.json>", file=sys.stderr)
        return 2
    try:
        result = verify_skill_file(argv[1])
    except (OSError, ValueError, SyntaxError, NameError) as e:
        print(f"verification error: {e}", file=sys.stderr)
        return 2

    payload = {
        "skill": result.skill,
        "verified": result.ok(),
        "pre_satisfiable": result.pre_satisfiable,
        "post_holds": result.post_holds,
        "counterexample": result.counterexample,
        "notes": result.notes,
    }
    print(json.dumps(payload, indent=2))
    return 0 if result.ok() else 1


if __name__ == "__main__":
    sys.exit(_cli(sys.argv))

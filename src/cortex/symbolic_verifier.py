import z3
import logging
import hashlib
import json
import time

logger = logging.getLogger("Cortex.SymbolicVerifier")


class SymbolicVerifier:
    def __init__(self):
        self.solver = z3.Solver()
        self.known_facts: dict = {}  # statement_hash -> z3 expression
        self.contradiction_count = 0
        self.total_checked = 0

    # ------------------------------------------------------------------
    # Internal helpers
    # ------------------------------------------------------------------

    def _statement_hash(self, statement: dict) -> str:
        canonical = json.dumps(statement, sort_keys=True, default=str)
        return hashlib.sha256(canonical.encode()).hexdigest()

    def _build_assertions(self, statement: dict) -> list:
        """Return a list of Z3 assertions derived from the statement's numeric values."""
        assertions = []
        for key, value in statement.items():
            if not isinstance(value, (int, float)):
                continue
            fv = float(value)
            r = z3.RealVal(fv)
            if key in ("drift", "score"):
                # must be [0, 1]
                assertions.append(z3.And(r >= 0.0, r <= 1.0))
            else:
                # generic numeric: must be within [-1e6, 1e6]
                assertions.append(z3.And(r > -1e6, r < 1e6))
        return assertions

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def verify_consistency(self, statement: dict) -> bool:
        t0 = time.perf_counter()
        self.total_checked += 1

        assertions = self._build_assertions(statement)

        # Check whether statement's own constraints are satisfiable in isolation
        if assertions:
            check_solver = z3.Solver()
            for a in assertions:
                check_solver.add(a)
            if check_solver.check() == z3.unsat:
                # Statement is internally contradictory (e.g. drift=2.0 violates [0,1])
                self.contradiction_count += 1
                elapsed = (time.perf_counter() - t0) * 1000
                if elapsed > 50:
                    logger.warning("verify_consistency took %.1f ms (>50ms)", elapsed)
                return False

        # Check for contradiction with existing persistent facts
        # Push scope, add new assertions, check satisfiability
        self.solver.push()
        for a in assertions:
            self.solver.add(a)
        result = self.solver.check()
        self.solver.pop()

        if result == z3.unsat:
            # New assertions contradict known state
            self.contradiction_count += 1
            elapsed = (time.perf_counter() - t0) * 1000
            if elapsed > 50:
                logger.warning("verify_consistency took %.1f ms (>50ms)", elapsed)
            return False

        # Consistent — commit assertions to persistent solver
        stmt_hash = self._statement_hash(statement)
        self.known_facts[stmt_hash] = assertions
        for a in assertions:
            self.solver.add(a)

        elapsed = (time.perf_counter() - t0) * 1000
        if elapsed > 50:
            logger.warning("verify_consistency took %.1f ms (>50ms)", elapsed)
        return True

    def verify_causal_chain(self, chain: list) -> bool:
        """Verify each step is consistent with all prior steps cumulatively."""
        if not chain:
            return True

        scoped = z3.Solver()
        for i, step in enumerate(chain):
            if not isinstance(step, dict):
                continue
            assertions = self._build_assertions(step)
            if assertions:
                scoped.push()
                for a in assertions:
                    scoped.add(a)
                result = scoped.check()
                if result == z3.unsat:
                    return False
                scoped.pop()
                for a in assertions:
                    scoped.add(a)
        return True

    def add_ledger_fact(self, key: str, value) -> None:
        """Add a ground-truth fact to the persistent solver state."""
        if isinstance(value, bool):
            var = z3.Bool(f"ledger_{key}")
            self.solver.add(var == value)
            self.known_facts[f"ledger_{key}"] = var
        elif isinstance(value, (int, float)):
            r = z3.Real(f"ledger_{key}")
            self.solver.add(r == float(value))
            self.known_facts[f"ledger_{key}"] = r

    def reset(self) -> None:
        self.solver = z3.Solver()
        self.known_facts = {}
        self.contradiction_count = 0
        self.total_checked = 0

    def get_stats(self) -> dict:
        rate = (
            self.contradiction_count / self.total_checked
            if self.total_checked > 0
            else 0.0
        )
        return {
            "total_checked": self.total_checked,
            "contradiction_count": self.contradiction_count,
            "contradiction_rate": round(rate, 4),
        }

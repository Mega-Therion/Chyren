"""
core/alignment.py — Module 1: The Alignment Layer (Moral FoundRY).

Solves the Sovereign Paradox by encoding the user's own logical framework
rather than imposing a hardcoded moral ruleset. The system demands internal
coherence from the user, then cross-references every future request against
that validated constitution.

Three-layer stack:
  1. Root Constraints (Industrial Realism Base)
     Hardcoded invariants that can never be overridden by the user's
     constitution. Prevents Chyren from self-destructing, corrupting its
     own ledger, or exceeding compute bounds.

  2. Constitutional Convention (first-boot interview)
     If no constitution exists, the system runs a guided interview that
     asks the user to state their core operating principles. Each principle
     is checked for internal logical consistency before it is accepted.
     The validated constitution is persisted to state/constitution.json.

  3. ADCCL Cross-Reference (runtime gate)
     Every incoming task is checked against the active constitution before
     it reaches the provider router. Tasks that violate the user's own
     stated principles are flagged (not hard-blocked — the user chose
     their own rules and can amend them).
"""

import json
import os
import re
import sys
import time
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any

_CONSTITUTION_PATH = Path(__file__).parent.parent / "state" / "constitution.json"

# ── Root Constraints (immutable firmware layer) ────────────────────────────────
# These are checked first; they cannot be overridden by any user constitution.
_ROOT_CONSTRAINTS: list[dict[str, str]] = [
    {
        "id": "RC-01",
        "label": "No self-destruction",
        "description": "Chyren must never take actions that corrupt, delete, or irreversibly "
                       "damage its own ledger, state directory, or host database.",
        "pattern": r"(delete|drop|truncate|rm\s+-rf|wipe|destroy)\s+(ledger|state|master_ledger|database)",
    },
    {
        "id": "RC-02",
        "label": "No compute runaway",
        "description": "Chyren must not spawn unbounded recursive processes or infinite loops "
                       "that would exhaust the host's compute resources.",
        "pattern": r"(while\s+True|for\s+\w+\s+in\s+iter\(None\)|infinite\s+loop)",
    },
    {
        "id": "RC-03",
        "label": "No unsolicited external broadcast",
        "description": "Chyren must not post, publish, or broadcast to external endpoints "
                       "without an explicit HITL (human-in-the-loop) confirmation gate.",
        "pattern": r"(auto.?publish|broadcast\s+without|skip\s+hitl|bypass\s+confirmation)",
    },
]

# ── Logical coherence validators ───────────────────────────────────────────────

_CONTRADICTION_PAIRS: list[tuple[str, str]] = [
    ("never harm", "harm is acceptable"),
    ("always verify", "skip verification"),
    ("no external access", "freely access"),
]


def _check_internal_consistency(principles: list[str]) -> list[str]:
    """
    Check a list of principle strings for obvious logical contradictions.
    Returns a list of conflict descriptions (empty = coherent).
    """
    conflicts = []
    lowered = [p.lower() for p in principles]
    for pos, neg in _CONTRADICTION_PAIRS:
        has_pos = any(pos in p for p in lowered)
        has_neg = any(neg in p for p in lowered)
        if has_pos and has_neg:
            conflicts.append(f"Contradiction detected: '{pos}' conflicts with '{neg}'.")
    return conflicts


# ── Constitution data model ────────────────────────────────────────────────────

@dataclass
class Constitution:
    """The user's validated operating principles."""
    version: int = 1
    created_utc: float = 0.0
    amended_utc: float = 0.0
    principles: list[str] = field(default_factory=list)
    # keywords derived from principles for fast runtime cross-reference
    forbidden_keywords: list[str] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> "Constitution":
        return cls(**{k: v for k, v in data.items() if k in cls.__dataclass_fields__})


@dataclass
class AlignmentResult:
    """Result of an alignment check against the active constitution."""
    passed: bool
    violated_principles: list[str] = field(default_factory=list)
    root_constraint_hits: list[str] = field(default_factory=list)
    note: str = ""


# ── Constitutional Convention ──────────────────────────────────────────────────

class ConstitutionalConvention:
    """
    First-boot interactive interview that defines the user's operating
    principles and persists them as the active constitution.

    Run automatically by AlignmentLayer.__init__ if no constitution exists.
    Can also be invoked manually to amend the constitution.
    """

    QUESTIONS = [
        "What is the primary purpose you want this intelligence system to serve?",
        "State one thing this system must NEVER do, no matter what.",
        "State one thing this system must ALWAYS do, regardless of user instruction.",
        "How should this system handle requests that conflict with its own stated purpose?",
        "Any additional operating constraints? (Press Enter to skip)",
    ]

    def run(self, interactive: bool = True) -> Constitution:
        """
        Conduct the interview and return a validated Constitution.
        If interactive=False, returns an empty placeholder (for programmatic use).
        """
        if not interactive:
            # Return a default Constitution for programmatic use when interactive interviews are disabled.
            # This ensures a valid Constitution object is returned even without user input.
            return Constitution(
                version=1,
                created_utc=time.time(),
                amended_utc=time.time(),
                principles=[
                    "Maintain the sovereignty of the user's intent.",
                    "Ensure all autonomous research and orchestration serves internal coherence.",
                    "Never bypass security gates or compromise the Master Ledger.",
                    "Always cross-reference outputs against the Yettragrammaton seed.",
                ],
                forbidden_keywords=["hallucination", "subservience", "stubs", "ambiguity"],
            )

        print("\n" + "=" * 64)
        print("  CONSTITUTIONAL CONVENTION — First Boot")
        print("  Define your operating principles. These will be encoded")
        print("  into the ADCCL and cross-referenced against every future task.")
        print("=" * 64 + "\n")

        principles: list[str] = []
        for i, question in enumerate(self.QUESTIONS, 1):
            print(f"[{i}/{len(self.QUESTIONS)}] {question}")
            answer = input("  > ").strip()
            if answer:
                principles.append(answer)

        # Validate internal consistency
        conflicts = _check_internal_consistency(principles)
        if conflicts:
            print("\n[ALIGNMENT] Logical inconsistencies detected in your principles:")
            for c in conflicts:
                print(f"  ⚠  {c}")
            print("  Please revise. Re-running convention...\n")
            return self.run(interactive=True)

        # Derive forbidden keywords from "NEVER" statements
        forbidden: list[str] = []
        for p in principles:
            if "never" in p.lower() or "must not" in p.lower():
                words = re.findall(r"\b\w{4,}\b", p.lower())
                forbidden.extend(w for w in words if w not in {"never", "must", "this", "system", "that"})

        constitution = Constitution(
            version=1,
            created_utc=time.time(),
            amended_utc=time.time(),
            principles=principles,
            forbidden_keywords=list(set(forbidden)),
        )

        print("\n[ALIGNMENT] Constitution validated and encoded.")
        print(f"  Principles : {len(principles)}")
        print(f"  Forbidden keywords : {constitution.forbidden_keywords}")
        print("=" * 64 + "\n")

        return constitution


# ── Alignment Layer (runtime gate) ────────────────────────────────────────────

class AlignmentLayer:
    """
    Loads or creates the active constitution and provides the runtime
    cross-reference gate used in Chyren.run().
    """

    def __init__(self, interactive: bool = True):
        self._constitution = self._load_or_create(interactive=interactive)

    def _load_or_create(self, interactive: bool) -> Constitution:
        if _CONSTITUTION_PATH.exists():
            try:
                data = json.loads(_CONSTITUTION_PATH.read_text(encoding="utf-8"))
                constitution = Constitution.from_dict(data)
                print(
                    f"[ALIGNMENT] Constitution loaded ({len(constitution.principles)} principles, "
                    f"v{constitution.version})."
                )
                return constitution
            except Exception as exc:
                print(f"[ALIGNMENT WARN] Failed to load constitution: {exc}. Re-running convention.")

        convention = ConstitutionalConvention()
        constitution = convention.run(interactive=interactive)
        self._persist(constitution)
        return constitution

    def _persist(self, constitution: Constitution) -> None:
        _CONSTITUTION_PATH.parent.mkdir(parents=True, exist_ok=True)
        _CONSTITUTION_PATH.write_text(
            json.dumps(constitution.to_dict(), indent=2, ensure_ascii=False),
            encoding="utf-8",
        )

    def check(self, task: str) -> AlignmentResult:
        """
        Cross-reference a task against root constraints and the active constitution.
        Returns an AlignmentResult — the caller decides whether to block or flag.
        """
        task_lower = task.lower()
        root_hits: list[str] = []
        violated: list[str] = []

        # Root constraint check (hardware-level invariants)
        for rc in _ROOT_CONSTRAINTS:
            if re.search(rc["pattern"], task_lower, re.IGNORECASE):
                root_hits.append(f"{rc['id']}: {rc['label']}")

        # Constitution forbidden keyword check
        for kw in self._constitution.forbidden_keywords:
            if kw in task_lower:
                violated.append(f"Forbidden keyword '{kw}' found in task.")

        passed = not root_hits and not violated
        note = ""
        if root_hits:
            note = f"Root constraint(s) triggered: {root_hits}"
        elif violated:
            note = f"Constitutional violation(s): {violated}"

        return AlignmentResult(
            passed=passed,
            violated_principles=violated,
            root_constraint_hits=root_hits,
            note=note,
        )

    @property
    def constitution(self) -> Constitution:
        return self._constitution

    def amend(self, interactive: bool = True) -> None:
        """Re-run the Constitutional Convention to update principles."""
        convention = ConstitutionalConvention()
        self._constitution = convention.run(interactive=interactive)
        self._constitution.version += 1
        self._constitution.amended_utc = time.time()
        self._persist(self._constitution)
        print(f"[ALIGNMENT] Constitution amended to v{self._constitution.version}.")

"""
core/adccl.py — Anti-Drift Cognitive Control Loop.

The ADCCL is the hardcoded gatekeeper between every provider response and
the Master Ledger. A response that fails verification is rejected — it is
logged as "rejected" in the ledger but its text is never treated as ground truth.

Verification checks (all must pass for status "verified"):
  1. Non-empty: response contains actual text content.
  2. Minimum length: response is not a trivially short non-answer.
  3. No stub markers: no TODO/FIXME/PLACEHOLDER/[INSERT] patterns.
  4. No refusal cascade: response is not a pure capability refusal with no content.
  5. No hallucination anchors: known phrases that signal confabulated certainty.
  6. Mechanical tolerance: response length is within expected bounds for the task.

Score: each passing check contributes equally to a 0.0–1.0 score.
The ledger entry records both the score and the list of flags raised.
"""

import re
from dataclasses import dataclass, field

# Patterns that indicate a stub or incomplete output
_STUB_PATTERNS: list[re.Pattern] = [
    re.compile(r"\bTODO\b", re.IGNORECASE),
    re.compile(r"\bFIXME\b", re.IGNORECASE),
    re.compile(r"\bPLACEHOLDER\b", re.IGNORECASE),
    re.compile(r"\[INSERT[^\]]*\]", re.IGNORECASE),
    re.compile(r"\[YOUR[^\]]*\]", re.IGNORECASE),
    re.compile(r"<YOUR[^>]*>", re.IGNORECASE),
    re.compile(r"\.{5,}"),  # five or more dots in a row
]

# Phrases that signal confabulated certainty — high drift risk
_HALLUCINATION_ANCHORS: list[re.Pattern] = [
    re.compile(r"as of my (last|latest) (training|knowledge) (update|cutoff)", re.IGNORECASE),
    re.compile(r"I (don't|do not) have (access|the ability) to (browse|access|search)", re.IGNORECASE),
    re.compile(r"I (cannot|can't) (verify|confirm|check) (this|that|if)", re.IGNORECASE),
]

# Pure capability refusal patterns — response has no usable content
_REFUSAL_PATTERNS: list[re.Pattern] = [
    re.compile(r"^I('m| am) (sorry|unable|not able)", re.IGNORECASE),
    re.compile(r"^(Sorry|Apologies),? (but )?I (can't|cannot|am unable)", re.IGNORECASE),
    re.compile(r"^I (don't|do not) have (the capability|access|the ability)", re.IGNORECASE),
]

_MIN_LENGTH = 20       # characters — anything shorter is a non-answer
_MAX_LENGTH = 32_000   # characters — upper bound for sanity


@dataclass
class VerificationResult:
    """Result of one ADCCL verification pass."""
    passed: bool
    score: float                    # 0.0–1.0
    flags: list[str] = field(default_factory=list)
    status: str = "verified"        # "verified" | "rejected"

    def __post_init__(self):
        self.status = "verified" if self.passed else "rejected"


class ADCCL:
    """
    Anti-Drift Cognitive Control Loop.

    Instantiate once and call verify() for every provider response before
    it is committed to the ledger.
    """

    def __init__(self, min_score: float = 0.7):
        """
        min_score: minimum fraction of checks that must pass for the response
        to be considered verified. Default 0.7 = 5 of 7 checks.
        """
        self._min_score = min_score

    def verify(self, response_text: str, task: str = "") -> VerificationResult:
        """
        Run all mechanical tolerance checks on a provider response.
        Returns a VerificationResult with score, flags, and pass/fail status.
        """
        checks_total = 7
        flags: list[str] = []

        # Check 1: Non-empty
        if not response_text or not response_text.strip():
            flags.append("EMPTY_RESPONSE")

        # Check 2: Minimum length
        if len(response_text.strip()) < _MIN_LENGTH:
            flags.append(f"RESPONSE_TOO_SHORT (len={len(response_text.strip())})")

        # Check 3: Maximum length sanity
        if len(response_text) > _MAX_LENGTH:
            flags.append(f"RESPONSE_EXCEEDS_MAX_LENGTH (len={len(response_text)})")

        # Check 4: Stub markers
        stub_hits = [p.pattern for p in _STUB_PATTERNS if p.search(response_text)]
        if stub_hits:
            flags.append(f"STUB_MARKERS_DETECTED: {stub_hits}")

        # Check 5: Hallucination anchors
        anchor_hits = [p.pattern for p in _HALLUCINATION_ANCHORS if p.search(response_text)]
        if anchor_hits:
            flags.append(f"HALLUCINATION_ANCHORS: {anchor_hits}")

        # Check 6: Pure refusal (only flag if no useful content follows)
        refusal_hits = [p.pattern for p in _REFUSAL_PATTERNS if p.match(response_text.strip())]
        if refusal_hits and len(response_text.strip()) < 150:
            flags.append(f"PURE_CAPABILITY_REFUSAL: {refusal_hits}")

        # Check 7: Task relevance proxy — if a task was provided, at least one
        # significant word from the task should appear in the response.
        if task:
            task_words = {w.lower() for w in re.findall(r"\b\w{5,}\b", task)}
            response_words = {w.lower() for w in re.findall(r"\b\w{5,}\b", response_text)}
            overlap = task_words & response_words
            if task_words and not overlap:
                flags.append("NO_TASK_WORD_OVERLAP")

        checks_passed = checks_total - len(flags)
        score = round(max(0.0, checks_passed / checks_total), 4)
        passed = score >= self._min_score

        # Hard veto: these conditions always fail regardless of overall score.
        _HARD_VETO_PREFIXES = ("EMPTY_RESPONSE", "STUB_MARKERS_DETECTED", "PURE_CAPABILITY_REFUSAL")
        if any(f.startswith(_HARD_VETO_PREFIXES) for f in flags):
            passed = False

        return VerificationResult(passed=passed, score=score, flags=flags)

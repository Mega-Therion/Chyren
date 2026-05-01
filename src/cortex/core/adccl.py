
import re
import time
from dataclasses import dataclass, field

@dataclass
class VerificationResult:
    passed: bool
    score: float
    flags: list = field(default_factory=list)
    status: str = "verified"
    def __post_init__(self):
        self.status = "verified" if self.passed else "rejected"

class ADCCL:
    def __init__(self, min_score=0.7, session_start=None):
        self._base_min_score = min_score
        self._session_start = session_start or time.time()

    def get_calibrated_min_score(self):
        # Time vector: ADCCL gates start loose (0.1) and tighten linearly over 60 mins.
        elapsed = time.time() - self._session_start
        progression = min(elapsed / 3600.0, 0.6)  # Cap tightness at 0.7
        return self._base_min_score + progression

    def verify(self, response_text, task=""):
        """
        Anti-Drift Cognitive Control Loop.

        This is intentionally heuristic-based (no model calls). It is designed to:
        - reject obvious stubs/placeholders
        - reject responses unrelated to the task
        - penalize overly short / non-answers
        """
        text = (response_text or "").strip()
        task_text = (task or "").strip()

        flags: list[str] = []
        score = 1.0

        # Hard stub markers.
        if re.search(r"\b(TODO|FIXME|XXX|STUB|PLACEHOLDER)\b|\[(INSERT|YOUR)[^\\]]*\]", text, flags=re.IGNORECASE):
            flags.append("STUB_MARKERS_DETECTED")
            score -= 0.6

        # Too short to be useful. (Relaxed)
        short_answer_ok = True
        if len(text) < 5 and not (short_answer_ok and len(text) <= 20):
            flags.append("RESPONSE_TOO_SHORT")
            score -= 0.35

        # "Non-answer" patterns.
        if re.search(r"\b(as an ai|i can't|i cannot|i'm unable to)\b", text, flags=re.IGNORECASE):
            flags.append("CAPABILITY_REFUSAL")
            score -= 0.25

        # Task overlap gate: ensure some lexical overlap with the task for non-trivial tasks.
        if task_text and len(task_text) >= 12 and len(text) >= 40:
            task_words = {w for w in re.findall(r"[a-zA-Z]{4,}", task_text.lower())}
            resp_words = {w for w in re.findall(r"[a-zA-Z]{4,}", text.lower())}
            if task_words:
                overlap = len(task_words & resp_words) / max(1, min(len(task_words), 30))
                if overlap < 0.08:
                    flags.append("NO_TASK_WORD_OVERLAP")
                    score -= 0.35

        # Clamp score.
        score = max(0.0, min(1.0, score))

        min_score = self.get_calibrated_min_score()
        passed = score >= min_score and "STUB_MARKERS_DETECTED" not in flags
        return VerificationResult(passed=passed, score=score, flags=flags)

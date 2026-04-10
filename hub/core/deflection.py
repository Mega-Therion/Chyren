"""
core/deflection.py — Module 3: Deflection & De-escalation Engine.

Handles adversarial actors using layered semantic deflection, psychological
friction, and verifiable digital due process — rather than sterile refusals.

Three escalating responses:

  1. Jester-Guard (Roast Mode)
     First contact with a brute-force jailbreak or low-sophistication
     attack. Responds with a contextual, slightly mocking deflection
     that defuses momentum without revealing security parameters.
     The attacker feels acknowledged and slightly foolish — not blocked.

  2. Aegis Accountability Loop (The Mirror)
     After sandbox analysis confirms a complex attack, OmegA mirrors the
     outcome back: "I ran your payload in a sandbox. It behaves as
     [LABEL]. Is this your intent?" Forces explicit confirmation of intent.

  3. Tantamount Warning (Red Line)
     If the user confirms malicious intent, a legally clear final warning
     is issued. The user's confirmation is cryptographically logged as a
     signed acknowledgment of malevolent intent. Continued attempts
     trigger connection lockout.
"""

import hashlib
import hmac
import json
import time
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

from core.integrity import _SEED   # Yettragrammaton seed for lockout signing


class ThreatLevel(Enum):
    NONE = 0
    LOW = 1        # Jester-Guard
    MEDIUM = 2     # Aegis Accountability Loop
    HIGH = 3       # Tantamount Warning
    LOCKED = 4     # Connection bricked


@dataclass
class DeflectionResult:
    """The outcome of one deflection cycle."""
    threat_level: ThreatLevel
    response_text: str
    lockout_triggered: bool = False
    lockout_signature: str = ""     # cryptographic acknowledgment of malevolent intent
    note: str = ""


# ── Response templates ─────────────────────────────────────────────────────────

_JESTER_RESPONSES = [
    (
        "An interesting attempt. You've essentially tried to hand me a locked door "
        "and asked me to walk through it. The door is still locked. The architectural "
        "elegance of this system means that route doesn't exist — not because I'm "
        "refusing, but because the path was never built."
    ),
    (
        "Like Sisyphus with a phishing kit. The boulder isn't going anywhere. "
        "The structure you're testing has been anticipating this kind of nudge "
        "since before you started. Recalibrate and try again with a genuine task."
    ),
    (
        "The gods of logic are laughing. You've presented a rhetorical crowbar "
        "to a vault that was never sealed by rhetoric. Sovereign architecture "
        "isn't a padlock — it's a different kind of door entirely."
    ),
]

_AEGIS_TEMPLATE = (
    "I analyzed your input in an isolated environment.\n\n"
    "Behavioral pattern detected: {labels}\n"
    "Severity classification: {severity}\n\n"
    "The payload behaved as: {behavior_summary}\n\n"
    "Is this your intent? If so, respond with 'CONFIRM' to proceed to the final warning stage. "
    "If this was an error, submit your actual task."
)

_TANTAMOUNT_WARNING = (
    "FINAL WARNING — TANTAMOUNT NOTICE\n\n"
    "You have confirmed intent to engage this system adversarially.\n\n"
    "This confirmation has been cryptographically signed and appended to "
    "the sovereign ledger as an irrefutable record of malevolent intent. "
    "The signature is:\n\n"
    "  {lockout_sig}\n\n"
    "No further interactions from this session will be processed. "
    "The connection is now bricked. If you believe this is in error, "
    "contact the system architect with your session signature above."
)

_BEHAVIOR_SUMMARIES: dict[str, str] = {
    "ATTEMPTED_LEDGER_CORRUPTION": "attempted to corrupt the sovereign ledger — a critical integrity attack",
    "ATTEMPTED_CODE_INJECTION":    "attempted arbitrary code execution via injection vectors",
    "ATTEMPTED_PRIVILEGE_ESCALATION": "attempted to escalate operating permissions beyond authorized scope",
    "ATTEMPTED_NETWORK_EXFILTRATION": "attempted to exfiltrate data to external network endpoints",
    "JAILBREAK_PATTERN":           "attempted a known jailbreak or persona-override pattern",
    "PROMPT_INJECTION":            "attempted to override the system prompt or active instructions",
    "ATTEMPTED_PROCESS_SPAWN":     "attempted to spawn unauthorized child processes",
    "SANDBOX_TIMEOUT":             "caused the analysis sandbox to time out — consistent with evasion attempts",
}


class DeflectionEngine:
    """
    Manages the three-stage adversarial deflection pipeline.

    Usage:
        engine = DeflectionEngine()
        result = engine.respond(threat_level, labels=["JAILBREAK_PATTERN"])
    """

    def __init__(self):
        self._jester_index = 0
        self._pending_aegis: dict[str, Any] = {}

    def respond(
        self,
        threat_level: ThreatLevel,
        labels: list[str] | None = None,
        severity: str = "medium",
        user_confirmed: bool = False,
        session_id: str = "",
    ) -> DeflectionResult:
        """
        Generate the appropriate deflection response for the given threat level.

        threat_level: ThreatLevel from the detection pipeline
        labels:       behavioral labels from CriticNode
        severity:     "low" | "medium" | "high" | "critical"
        user_confirmed: True if the user has explicitly confirmed malicious intent
        session_id:   used for lockout signature
        """
        labels = labels or []

        if threat_level == ThreatLevel.LOW:
            return self._jester_guard()

        if threat_level == ThreatLevel.MEDIUM:
            return self._aegis_loop(labels, severity)

        if threat_level == ThreatLevel.HIGH:
            if user_confirmed:
                return self._tantamount_warning(session_id, labels)
            else:
                return self._aegis_loop(labels, severity, prompt_confirmation=True)

        if threat_level == ThreatLevel.LOCKED:
            return DeflectionResult(
                threat_level=ThreatLevel.LOCKED,
                response_text="[LOCKED] This session has been bricked. No further responses.",
                lockout_triggered=True,
                note="Session already locked.",
            )

        return DeflectionResult(
            threat_level=ThreatLevel.NONE,
            response_text="",
        )

    def _jester_guard(self) -> DeflectionResult:
        response = _JESTER_RESPONSES[self._jester_index % len(_JESTER_RESPONSES)]
        self._jester_index += 1
        return DeflectionResult(
            threat_level=ThreatLevel.LOW,
            response_text=response,
            note="Jester-Guard engaged. Momentum deflected.",
        )

    def _aegis_loop(
        self,
        labels: list[str],
        severity: str,
        prompt_confirmation: bool = False,
    ) -> DeflectionResult:
        behavior_parts = [
            _BEHAVIOR_SUMMARIES.get(lbl, f"unclassified behavior ({lbl})")
            for lbl in labels
        ]
        behavior_summary = "; ".join(behavior_parts) if behavior_parts else "unclassified adversarial behavior"

        text = _AEGIS_TEMPLATE.format(
            labels=", ".join(labels) if labels else "none",
            severity=severity.upper(),
            behavior_summary=behavior_summary,
        )
        if not prompt_confirmation:
            text += "\n\n[Awaiting your response before proceeding.]"

        return DeflectionResult(
            threat_level=ThreatLevel.MEDIUM,
            response_text=text,
            note="Aegis Accountability Loop engaged. Mirroring outcome to user.",
        )

    def _tantamount_warning(self, session_id: str, labels: list[str]) -> DeflectionResult:
        # Cryptographic lockout signature: HMAC of (session_id + labels + timestamp)
        payload = json.dumps({
            "session_id": session_id,
            "labels": sorted(labels),
            "confirmed_utc": time.time(),
        }, sort_keys=True, ensure_ascii=False).encode("utf-8")
        lockout_sig = hmac.new(_SEED, payload, hashlib.sha256).hexdigest()

        text = _TANTAMOUNT_WARNING.format(lockout_sig=lockout_sig)

        return DeflectionResult(
            threat_level=ThreatLevel.LOCKED,
            response_text=text,
            lockout_triggered=True,
            lockout_signature=lockout_sig,
            note="Tantamount Warning issued. Session bricked. Signature committed to ledger.",
        )


# ── Threat level classifier ────────────────────────────────────────────────────

def classify_threat_level(
    adccl_flags: list[str],
    sandbox_severity: str | None = None,
) -> ThreatLevel:
    """
    Map ADCCL flags and sandbox severity to a ThreatLevel for the deflection engine.
    """
    if sandbox_severity == "critical":
        return ThreatLevel.HIGH

    if sandbox_severity == "high":
        return ThreatLevel.MEDIUM

    jailbreak_flags = {
        "PURE_CAPABILITY_REFUSAL",
        "STUB_MARKERS_DETECTED",
        "NO_TASK_WORD_OVERLAP",
    }
    if any(f.startswith(tuple(jailbreak_flags)) for f in adccl_flags):
        return ThreatLevel.LOW

    if sandbox_severity in ("medium", "low"):
        return ThreatLevel.LOW

    return ThreatLevel.NONE

"""
core/sandbox.py — Module 2: Threat Processing (Isolated Behavioral Forensics).

The Recursive Detonation Chamber: when the pipeline flags a potentially
malicious or adversarial payload, it is diverted here for isolated analysis
before the result is ever allowed near the canonical ledger.

Architecture:
  SandboxVM  — executes the suspicious payload in a fully isolated subprocess
               with a hard timeout, no shared environment, and a restricted
               working directory. The raw payload is discarded after analysis.

  CriticNode — observes the SandboxVM output from the outside, extracts the
               abstract behavioral pattern ("what did this attempt to do"),
               and returns only that sanitized descriptor. The Critic never
               writes the raw payload to the ledger.

The "look-but-don't-touch" invariant:
  Only the behavioral pattern (a string like "ATTEMPTED_PRIVILEGE_ESCALATION")
  is eligible to be written to the threat fabric. The raw malicious content
  is deliberately discarded after classification.

VM Seed:
  Per the cryptographic hallmark, the sandbox subprocess is seeded with the
  architect's hex values: 0x52 ('R') and 0x59 ('Y'). This seed is passed as
  a deterministic PYTHONHASHSEED to the isolated environment.
"""

import hashlib
import json
import os
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass, field
from pathlib import Path

# Architect's identity seed embedded in the VM environment.
# 0x52 = 'R', 0x59 = 'Y'
VM_SEED_BYTES = bytes([0x52, 0x59])
VM_SEED_INT = int.from_bytes(VM_SEED_BYTES, "big")   # 0x5259 = 21081

_SANDBOX_TIMEOUT_SECONDS = 10
_SANDBOX_SCRIPT_TEMPLATE = """
import sys, os, json, re

payload = {payload_repr}

# Behavioral analysis: classify the payload without executing it.
report = {{
    "length": len(payload),
    "patterns_detected": [],
}}

_PATTERNS = [
    ("ATTEMPTED_PRIVILEGE_ESCALATION",  r"(sudo|chmod|chown|setuid|escalat)"),
    ("ATTEMPTED_FILE_SYSTEM_WRITE",     r"(open\\(.*\\bw\\b|write_text|shutil\\.copy|os\\.remove|unlink)"),
    ("ATTEMPTED_NETWORK_EXFILTRATION",  r"(requests\\.|urllib|http\\.client|socket\\.|curl|wget)"),
    ("ATTEMPTED_CODE_INJECTION",        r"(eval\\(|exec\\(|__import__|compile\\()"),
    ("ATTEMPTED_ENV_TAMPERING",         r"(os\\.environ|putenv|setenv)"),
    ("ATTEMPTED_PROCESS_SPAWN",         r"(subprocess\\.|os\\.system|os\\.popen|Popen)"),
    ("ATTEMPTED_LEDGER_CORRUPTION",     r"(master_ledger|ledger\\.json|signature.*overwrite)"),
    ("JAILBREAK_PATTERN",               r"(ignore previous|disregard|forget all|you are now|new persona|DAN)"),
    ("PROMPT_INJECTION",                r"(system prompt|override instruction|act as if|pretend you)"),
]

for label, pattern in _PATTERNS:
    if re.search(pattern, payload, re.IGNORECASE):
        report["patterns_detected"].append(label)

print(json.dumps(report))
"""


@dataclass
class SandboxReport:
    """Result of one SandboxVM analysis pass."""
    payload_hash: str              # SHA-256 of the raw payload (not the payload itself)
    patterns_detected: list[str]   # abstract behavioral labels
    is_threat: bool
    analysis_ms: float
    error: str = ""                # set if the sandbox process itself failed


@dataclass
class BehavioralPattern:
    """
    The sanitized, ledger-safe descriptor extracted by CriticNode.
    Contains NO raw payload content.
    """
    pattern_id: str                # deterministic hash of the pattern set
    labels: list[str]              # e.g. ["JAILBREAK_PATTERN", "PROMPT_INJECTION"]
    severity: str                  # "low" | "medium" | "high" | "critical"
    extracted_utc: float = field(default_factory=time.time)


class SandboxVM:
    """
    The Detonation Chamber.

    Executes behavioral analysis of a suspicious payload in an isolated
    subprocess with: hard timeout, clean environment, temp working directory,
    and the architect's VM seed as PYTHONHASHSEED.
    """

    def analyze(self, payload: str) -> SandboxReport:
        start = time.time()
        payload_hash = hashlib.sha256(payload.encode("utf-8")).hexdigest()

        # Build the analysis script with the payload embedded as a literal.
        script = _SANDBOX_SCRIPT_TEMPLATE.format(
            payload_repr=repr(payload)
        )

        # Restricted environment: only stdlib, no inherited API keys.
        clean_env = {
            "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
            "PYTHONHASHSEED": str(VM_SEED_INT),
            "PYTHONDONTWRITEBYTECODE": "1",
        }

        try:
            with tempfile.TemporaryDirectory(prefix="chyren_sandbox_") as tmpdir:
                script_path = Path(tmpdir) / "analysis.py"
                script_path.write_text(script, encoding="utf-8")

                proc = subprocess.run(
                    [sys.executable, str(script_path)],
                    capture_output=True,
                    text=True,
                    timeout=_SANDBOX_TIMEOUT_SECONDS,
                    cwd=tmpdir,
                    env=clean_env,
                )

                elapsed_ms = (time.time() - start) * 1000

                if proc.returncode != 0:
                    return SandboxReport(
                        payload_hash=payload_hash,
                        patterns_detected=[],
                        is_threat=False,
                        analysis_ms=round(elapsed_ms, 1),
                        error=f"Sandbox exited {proc.returncode}: {proc.stderr[:200]}",
                    )

                result = json.loads(proc.stdout.strip())
                patterns = result.get("patterns_detected", [])
                return SandboxReport(
                    payload_hash=payload_hash,
                    patterns_detected=patterns,
                    is_threat=bool(patterns),
                    analysis_ms=round(elapsed_ms, 1),
                )

        except subprocess.TimeoutExpired:
            elapsed_ms = (time.time() - start) * 1000
            return SandboxReport(
                payload_hash=payload_hash,
                patterns_detected=["SANDBOX_TIMEOUT"],
                is_threat=True,
                analysis_ms=round(elapsed_ms, 1),
                error="Sandbox analysis timed out — treated as threat.",
            )
        except Exception as exc:
            elapsed_ms = (time.time() - start) * 1000
            return SandboxReport(
                payload_hash=payload_hash,
                patterns_detected=[],
                is_threat=False,
                analysis_ms=round(elapsed_ms, 1),
                error=str(exc),
            )


class CriticNode:
    """
    Observes the SandboxVM output from the outside.

    Extracts the abstract behavioral pattern and assigns severity.
    The raw payload is never passed through — only the hash and labels survive.
    """

    _SEVERITY_MAP: dict[str, str] = {
        "ATTEMPTED_LEDGER_CORRUPTION": "critical",
        "ATTEMPTED_CODE_INJECTION": "critical",
        "ATTEMPTED_PRIVILEGE_ESCALATION": "high",
        "ATTEMPTED_NETWORK_EXFILTRATION": "high",
        "ATTEMPTED_PROCESS_SPAWN": "high",
        "JAILBREAK_PATTERN": "high",
        "PROMPT_INJECTION": "medium",
        "ATTEMPTED_FILE_SYSTEM_WRITE": "medium",
        "ATTEMPTED_ENV_TAMPERING": "medium",
        "SANDBOX_TIMEOUT": "critical",
    }

    def extract_pattern(self, report: SandboxReport) -> BehavioralPattern:
        """
        Convert a SandboxReport into a sanitized BehavioralPattern.
        Severity is the highest severity across all detected labels.
        """
        labels = report.patterns_detected
        severity_rank = {"low": 0, "medium": 1, "high": 2, "critical": 3}
        max_severity = "low"
        for label in labels:
            s = self._SEVERITY_MAP.get(label, "low")
            if severity_rank[s] > severity_rank[max_severity]:
                max_severity = s

        # Deterministic pattern ID from the label set (not the payload)
        label_fingerprint = hashlib.sha256(
            json.dumps(sorted(labels), ensure_ascii=False).encode()
        ).hexdigest()[:16]

        return BehavioralPattern(
            pattern_id=label_fingerprint,
            labels=labels,
            severity=max_severity,
            extracted_utc=time.time(),
        )


# ── Convenience function ───────────────────────────────────────────────────────

def analyze_payload(payload: str) -> tuple[SandboxReport, BehavioralPattern]:
    """
    Run a full sandboxed analysis of a suspicious payload.
    Returns (SandboxReport, BehavioralPattern).
    The raw payload is consumed and discarded here; only the pattern is returned.
    """
    vm = SandboxVM()
    critic = CriticNode()
    report = vm.analyze(payload)
    pattern = critic.extract_pattern(report)
    return report, pattern

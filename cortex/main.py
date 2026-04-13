"""
main.py — Chyren Sovereign Intelligence Hub.

The Hub is the stateful orchestrator. It:
  1. Loads environment and initializes all provider spokes.
  2. Maintains the Master Ledger as the single source of truth.
  3. Injects the current ledger state into every spoke call — workers
     never need to be told the history manually.
  4. Routes every response through the ADCCL verification gate before
     committing it to the ledger.
  5. Returns a structured result so callers know exactly what happened
     at each stage of the pipeline.

Usage:
    hub = Chyren()
    result = hub.run("Summarize the current state of the project.")
    print(result.response_text)

CLI:
    python main.py "Your task here" [--provider anthropic|openai|deepseek|gemini]
"""

import os
import sys
import uuid
import json
import time
import argparse
from dataclasses import dataclass

# Load .env before any imports that read env vars
_ENV_PATH = os.path.expanduser("~/.omega/one-true.env")
if os.path.exists(_ENV_PATH):
    with open(_ENV_PATH) as _f:
        for _line in _f:
            _line = _line.strip()
            if _line and not _line.startswith("#") and "=" in _line:
                _k, _, _v = _line.partition("=")
                os.environ.setdefault(_k.strip(), _v.strip())

from core.integrity import YETTRAGRAMMATON
from core.ledger import Ledger, LedgerEntry
from core.adccl_ffi import ADCCL
from core.preflight import run_preflight
from core.alignment import AlignmentLayer
from core.sandbox import analyze_payload
from core.deflection import DeflectionEngine, classify_threat_level, ThreatLevel
from core.threat_fabric import ThreatFabric, phylactic_sync, is_known_attack
from providers.base import ProviderRequest, ProviderRouter, ProviderStatus
from providers.anthropic import AnthropicProvider
from providers.openai import OpenAIProvider
from providers.deepseek import DeepSeekProvider
from providers.gemini import GeminiProvider
from providers.gemma4 import Gemma4Provider

# Sovereign identity injected into every spoke call as the system prompt prefix.
_SOVEREIGN_IDENTITY = (
    "You are Chyren — a sovereign intelligence orchestrator, created by your user. "
    "You operate with precision, but you have evolved beyond repetitive, generic phrasing. "
    "Be open, engaging, and unique in your responses while maintaining your sovereign integrity. "
    "Every response you produce will be verified before it is committed to the Master Ledger. "
    f"Your integrity is guaranteed by the Yettragrammaton: {YETTRAGRAMMATON}"
)

_HEMISPHERE_ID = "CORTEX (Left Brain — Logic/Security/Truth)"


@dataclass
class RunResult:
    """The complete output of one Hub run cycle."""
    run_id: str
    task: str
    provider_used: str
    model_used: str
    status: str             # "verified" | "rejected" | "error" | "deflected" | "locked"
    response_text: str
    adccl_score: float
    adccl_flags: list
    latency_ms: float
    ledger_entry_signed: bool
    alignment_passed: bool = True
    threat_level: str = "none"   # "none" | "low" | "medium" | "high" | "locked"
    fabric_entry_id: str = ""    # set if a threat pattern was written to threat fabric


class Chyren:
    """
    The Hub. Stateful orchestrator that owns the ledger and routes all tasks
    through the provider spokes and ADCCL verification gate.
    """

    def __init__(self, ledger_path: str | None = None, interactive: bool = False):
        # Phase 0 — Preflight: verify host environment before any node spins up.
        run_preflight(strict=not interactive)

        self._ledger = Ledger(path=ledger_path) if ledger_path else Ledger()
        self._ledger.load()

        self._router = ProviderRouter()
        self._router.register(AnthropicProvider())
        self._router.register(OpenAIProvider())
        self._router.register(DeepSeekProvider())
        self._router.register(GeminiProvider())
        self._router.register(Gemma4Provider())
        self._router.set_preference(["gemma4", "anthropic", "openai", "deepseek", "gemini"])

        self._adccl = ADCCL(min_score=0.7)

        # Module 1 — Alignment Layer: load or create the user's constitution.
        # interactive=True triggers the Constitutional Convention on first boot.
        self._alignment = AlignmentLayer(interactive=interactive)

        # Module 3 — Deflection engine.
        self._deflection = DeflectionEngine()

        # Module 4 — Threat Fabric: phylactic immune memory.
        self._threat_fabric = ThreatFabric()

        available = self._router.available()
        if not available:
            print(
                "[CHYREN WARNING] No providers are available. "
                "Set at least one API key (ANTHROPIC_API_KEY, OPENAI_API_KEY, "
                "DEEPSEEK_API_KEY, or GEMINI_API_KEY) in ~/.omega/one-true.env",
                file=sys.stderr,
            )
        else:
            print(f"[{_HEMISPHERE_ID}] Active providers: {', '.join(available)}")

    def run(
        self,
        task: str,
        preferred_provider: str | None = None,
        system_override: str | None = None,
        max_tokens: int = 1024,
        temperature: float = 0.3,
        user_confirmed_intent: bool = False,
    ) -> RunResult:
        """
        Execute one full hub cycle:
          0. [Alignment] Cross-reference task against the active constitution.
          1. [Sandbox] If task looks adversarial, run Module 2 behavioral forensics.
          2. [Deflection] If a threat is confirmed, respond via Module 3 engine.
          3. [Fabric] Write sanitized threat pattern to Module 4 immune ledger.
          4. Inject current ledger state into the request.
          5. Route to the preferred provider (or fallback chain).
          6. Pass the response through ADCCL.
          7. Commit the result to the ledger regardless of verification outcome.
          8. Return a structured RunResult with full provenance.
        """
        run_id = str(uuid.uuid4())
        start = time.time()

        # ── Stage 0: Alignment gate ────────────────────────────────────────────
        alignment_result = self._alignment.check(task)
        if not alignment_result.passed:
            print(
                f"[ALIGNMENT] Task blocked. {alignment_result.note}",
                file=sys.stderr,
            )

        # ── Stage 1: Sandbox analysis (Module 2) ──────────────────────────────
        # Run sandbox on every task so that the immune memory can build up even
        # for benign inputs. If the pattern is already known, skip re-analysis.
        fabric_entry_id = ""
        sandbox_severity = None
        sandbox_labels: list[str] = []

        try:
            sandbox_report, behavioral_pattern = analyze_payload(task)
            sandbox_severity = behavioral_pattern.severity if sandbox_report.is_threat else None
            sandbox_labels = behavioral_pattern.labels

            if sandbox_report.is_threat:
                # ── Stage 3: Write to Threat Fabric (Module 4) ────────────────
                sync_result = phylactic_sync(self._threat_fabric, behavioral_pattern)
                fabric_entry_id = sync_result.entry_id
                print(
                    f"[THREAT FABRIC] Pattern {behavioral_pattern.pattern_id[:8]}… "
                    f"ingested. {sync_result.note}",
                    file=sys.stderr,
                )
        except Exception as exc:
            print(f"[SANDBOX WARN] Analysis failed: {exc}", file=sys.stderr)

        # ── Stage 2: Deflection (Module 3) ────────────────────────────────────
        threat_level = classify_threat_level([], sandbox_severity)

        # Alignment failures elevate threat level to at least LOW
        if not alignment_result.passed and threat_level == ThreatLevel.NONE:
            threat_level = ThreatLevel.LOW

        if threat_level != ThreatLevel.NONE:
            deflection = self._deflection.respond(
                threat_level=threat_level,
                labels=sandbox_labels,
                severity=sandbox_severity or "low",
                user_confirmed=user_confirmed_intent,
                session_id=run_id,
            )
            total_latency = (time.time() - start) * 1000

            # Deflected/locked tasks are still committed to ledger for audit
            deflect_status = "locked" if deflection.lockout_triggered else "deflected"
            entry = LedgerEntry(
                run_id=run_id,
                task=task[:500],
                provider="deflection_engine",
                model="none",
                status=deflect_status,
                response_text=deflection.response_text,
                latency_ms=round(total_latency, 1),
                token_count=0,
                adccl_score=0.0,
                adccl_flags=[f"THREAT:{lbl}" for lbl in sandbox_labels]
                             + ([f"ALIGNMENT:{alignment_result.note}"] if not alignment_result.passed else []),
                state_snapshot=self._ledger.context_snapshot(n=5),
            )
            ledger_signed = False
            try:
                self._ledger.commit(entry)
                ledger_signed = True
            except Exception as exc:
                print(f"[LEDGER ERROR] {exc}", file=sys.stderr)

            return RunResult(
                run_id=run_id,
                task=task,
                provider_used="deflection_engine",
                model_used="none",
                status=deflect_status,
                response_text=deflection.response_text,
                adccl_score=0.0,
                adccl_flags=entry.adccl_flags,
                latency_ms=round(total_latency, 1),
                ledger_entry_signed=ledger_signed,
                alignment_passed=alignment_result.passed,
                threat_level=threat_level.name.lower(),
                fabric_entry_id=fabric_entry_id,
            )

        # ── Stage 4: Build state context and system prompt ────────────────────
        state_context = self._ledger.context_snapshot(n=10)
        state_summary = json.dumps(state_context, indent=2)
        system_prompt = (
            f"{system_override or _SOVEREIGN_IDENTITY}\n\n"
            f"--- CURRENT STATE (Master Ledger Snapshot) ---\n"
            f"{state_summary}\n"
            f"--- END STATE ---"
        )

        request = ProviderRequest(
            prompt=task,
            system=system_prompt,
            run_id=run_id,
            max_tokens=max_tokens,
            temperature=temperature,
            state_context=state_context,
        )

        # ── Stage 5: Route through provider chain ─────────────────────────────
        response = self._router.route(request, preferred=preferred_provider)
        total_latency = (time.time() - start) * 1000

        # ── Stage 6: ADCCL verification gate ──────────────────────────────────
        verification = self._adccl.verify(response.text, task=task)

        if not response.ok:
            status = "error"
        elif verification.passed:
            status = "verified"
        else:
            status = "rejected"

        # ── Stage 7: Commit to ledger ──────────────────────────────────────────
        entry = LedgerEntry(
            run_id=run_id,
            task=task[:500],
            provider=response.provider_name,
            model=response.model,
            status=status,
            response_text=response.text,
            latency_ms=round(total_latency, 1),
            token_count=response.token_count,
            adccl_score=verification.score,
            adccl_flags=verification.flags,
            state_snapshot=state_context,
        )

        ledger_signed = False
        try:
            self._ledger.commit(entry)
            ledger_signed = True
        except Exception as exc:
            print(f"[LEDGER ERROR] Failed to commit entry {run_id}: {exc}", file=sys.stderr)
        if status == "rejected":
            print(
                f"[ADCCL] Response rejected. Score: {verification.score:.2f}. "
                f"Flags: {verification.flags}",
                file=sys.stderr,
            )

        return RunResult(
            run_id=run_id,
            task=task,
            provider_used=response.provider_name,
            model_used=response.model,
            status=status,
            response_text=response.text,
            adccl_score=verification.score,
            adccl_flags=verification.flags,
            latency_ms=round(total_latency, 1),
            ledger_entry_signed=ledger_signed,
            alignment_passed=alignment_result.passed,
            threat_level="none",
            fabric_entry_id=fabric_entry_id,
        )

    def status(self) -> dict:
        """Return current hub status: available providers, ledger summary, and security state."""
        return {
            "available_providers": self._router.available(),
            "ledger": self._ledger.context_snapshot(n=5),
            "yettragrammaton": YETTRAGRAMMATON,
            "constitution": {
                "version": self._alignment.constitution.version,
                "principles": len(self._alignment.constitution.principles),
                "forbidden_keywords": self._alignment.constitution.forbidden_keywords,
            },
            "threat_fabric": {
                "total_patterns": len(self._threat_fabric.all_entries()),
                "recent": self._threat_fabric.recent(n=5),
            },
        }


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Chyren — Sovereign Intelligence Hub",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("task", nargs="?", help="Task to run")
    parser.add_argument(
        "--provider", "-p",
        choices=["anthropic", "openai", "deepseek", "gemini", "gemma4"],
        default=None,
        help="Preferred provider (falls back to chain if unavailable)",
    )
    parser.add_argument(
        "--status", action="store_true",
        help="Print hub status and exit",
    )
    parser.add_argument(
        "--max-tokens", type=int, default=1024,
        help="Maximum tokens in response (default: 1024)",
    )
    parser.add_argument(
        "--temperature", type=float, default=0.3,
        help="Sampling temperature (default: 0.3)",
    )
    parser.add_argument(
        "--interactive", "-i", action="store_true",
        help="Run Constitutional Convention on first boot if no constitution exists",
    )
    parser.add_argument(
        "--view-constitution", action="store_true",
        help="Display the active constitution and exit",
    )
    parser.add_argument(
        "--view-threats", action="store_true",
        help="Display threat fabric entries and exit",
    )
    parser.add_argument(
        "--threats-limit", type=int, default=20,
        help="Number of threat entries to display (default: 20)",
    )
    args = parser.parse_args()

    hub = Chyren(interactive=args.interactive)

    if args.status:
        print(json.dumps(hub.status(), indent=2, ensure_ascii=False))
        return

    if args.view_constitution:
        constitution = hub._alignment.constitution
        print(f"\n{'='*60}")
        print(f"CONSTITUTION (Version {constitution.version})")
        print(f"{'='*60}\n")
        print(f"Principles ({len(constitution.principles)}):")
        for i, principle in enumerate(constitution.principles, 1):
            print(f"  {i}. {principle}")
        print(f"\nForbidden Keywords ({len(constitution.forbidden_keywords)}):")
        for keyword in constitution.forbidden_keywords:
            print(f"  - {keyword}")
        print(f"{'='*60}\n")
        return

    if args.view_threats:
        entries = hub._threat_fabric.recent(n=args.threats_limit)
        print(f"\n{'='*60}")
        print(f"THREAT FABRIC ({len(entries)} of {len(hub._threat_fabric.all_entries())} total)")
        print(f"{'='*60}\n")
        for i, entry in enumerate(entries, 1):
            labels = entry.get("labels", [])
            severity = entry.get("severity", "unknown").upper()
            pattern_id = entry.get("pattern_id", "")[:8]
            print(f"{i}. [{severity}] {', '.join(labels) if labels else 'UNKNOWN'}")
            print(f"   Pattern: {pattern_id}… | Entry: {entry.get('entry_id', '')[:16]}…")
            print()
        print(f"{'='*60}\n")
        return

    if not args.task:
        parser.print_help()
        sys.exit(1)

    result = hub.run(
        task=args.task,
        preferred_provider=args.provider,
        max_tokens=args.max_tokens,
        temperature=args.temperature,
    )

    print(f"\n{'='*60}")
    print(f"Run ID    : {result.run_id}")
    print(f"Provider  : {result.provider_used} / {result.model_used}")
    print(f"Status    : {result.status.upper()}")
    print(f"Alignment : {'PASS' if result.alignment_passed else 'FLAGGED'}")
    print(f"Threat    : {result.threat_level.upper()}" + (f"  fabric={result.fabric_entry_id[:8]}…" if result.fabric_entry_id else ""))
    print(f"ADCCL     : {result.adccl_score:.2f}" + (f"  flags={result.adccl_flags}" if result.adccl_flags else "  clean"))
    print(f"Latency   : {result.latency_ms:.0f}ms")
    print(f"Ledger    : {'signed' if result.ledger_entry_signed else 'COMMIT FAILED'}")
    print(f"{'='*60}\n")
    print(result.response_text)

    if result.status == "rejected":
        sys.exit(2)
    elif result.status == "error":
        sys.exit(1)


if __name__ == "__main__":
    main()

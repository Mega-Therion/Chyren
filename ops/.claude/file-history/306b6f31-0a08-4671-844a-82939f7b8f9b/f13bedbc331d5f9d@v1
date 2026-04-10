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
from core.adccl import ADCCL
from providers.base import ProviderRequest, ProviderRouter, ProviderStatus
from providers.anthropic import AnthropicProvider
from providers.openai import OpenAIProvider
from providers.deepseek import DeepSeekProvider
from providers.gemini import GeminiProvider

# Sovereign identity injected into every spoke call as the system prompt prefix.
_SOVEREIGN_IDENTITY = (
    "You are Chyren — a sovereign intelligence orchestrator. "
    "You operate with precision, no stubs, and no hallucinations. "
    "Every response you produce will be verified before it is committed to the Master Ledger. "
    f"Your integrity is guaranteed by the Yettragrammaton: {YETTRAGRAMMATON}"
)


@dataclass
class RunResult:
    """The complete output of one Hub run cycle."""
    run_id: str
    task: str
    provider_used: str
    model_used: str
    status: str             # "verified" | "rejected" | "error"
    response_text: str
    adccl_score: float
    adccl_flags: list
    latency_ms: float
    ledger_entry_signed: bool


class Chyren:
    """
    The Hub. Stateful orchestrator that owns the ledger and routes all tasks
    through the provider spokes and ADCCL verification gate.
    """

    def __init__(self, ledger_path: str | None = None):
        self._ledger = Ledger(path=ledger_path) if ledger_path else Ledger()
        self._ledger.load()

        self._router = ProviderRouter()
        self._router.register(AnthropicProvider())
        self._router.register(OpenAIProvider())
        self._router.register(DeepSeekProvider())
        self._router.register(GeminiProvider())
        self._router.set_preference(["anthropic", "openai", "deepseek", "gemini"])

        self._adccl = ADCCL(min_score=0.7)

        available = self._router.available()
        if not available:
            print(
                "[CHYREN WARNING] No providers are available. "
                "Set at least one API key (ANTHROPIC_API_KEY, OPENAI_API_KEY, "
                "DEEPSEEK_API_KEY, or GEMINI_API_KEY) in ~/.omega/one-true.env",
                file=sys.stderr,
            )
        else:
            print(f"[CHYREN] Active providers: {', '.join(available)}")

    def run(
        self,
        task: str,
        preferred_provider: str | None = None,
        system_override: str | None = None,
        max_tokens: int = 1024,
        temperature: float = 0.3,
    ) -> RunResult:
        """
        Execute one full hub cycle:
          1. Inject current ledger state into the request.
          2. Route to the preferred provider (or fallback chain).
          3. Pass the response through ADCCL.
          4. Commit the result to the ledger regardless of verification outcome.
          5. Return a RunResult with full provenance.
        """
        run_id = str(uuid.uuid4())
        start = time.time()

        # Build state context from ledger
        state_context = self._ledger.context_snapshot(n=10)

        # Compose system prompt: sovereign identity + injected state
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

        # Route through provider chain
        response = self._router.route(request, preferred=preferred_provider)
        total_latency = (time.time() - start) * 1000

        # ADCCL verification gate
        verification = self._adccl.verify(response.text, task=task)

        # Determine final status
        if not response.ok:
            status = "error"
        elif verification.passed:
            status = "verified"
        else:
            status = "rejected"

        # Commit to ledger — all outcomes are recorded, not just successes
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
        )

    def status(self) -> dict:
        """Return current hub status: available providers and ledger summary."""
        return {
            "available_providers": self._router.available(),
            "ledger": self._ledger.context_snapshot(n=5),
            "yettragrammaton": YETTRAGRAMMATON,
        }


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Chyren — Sovereign Intelligence Hub",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("task", nargs="?", help="Task to run")
    parser.add_argument(
        "--provider", "-p",
        choices=["anthropic", "openai", "deepseek", "gemini"],
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
    args = parser.parse_args()

    hub = Chyren()

    if args.status:
        print(json.dumps(hub.status(), indent=2, ensure_ascii=False))
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
    print(f"Run ID  : {result.run_id}")
    print(f"Provider: {result.provider_used} / {result.model_used}")
    print(f"Status  : {result.status.upper()}")
    print(f"ADCCL   : {result.adccl_score:.2f}" + (f"  flags={result.adccl_flags}" if result.adccl_flags else "  clean"))
    print(f"Latency : {result.latency_ms:.0f}ms")
    print(f"Ledger  : {'signed' if result.ledger_entry_signed else 'COMMIT FAILED'}")
    print(f"{'='*60}\n")
    print(result.response_text)

    if result.status == "rejected":
        sys.exit(2)
    elif result.status == "error":
        sys.exit(1)


if __name__ == "__main__":
    main()

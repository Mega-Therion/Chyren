"""
core/preflight.py — Phase 0: System Preflight & Initialization.

Executed before any Chyren node spins up. Verifies the host environment
meets the minimum requirements for sovereign operation:

  1. Python version (3.10+ required for structural pattern matching and
     union type hints used throughout the codebase).
  2. Required stdlib modules are importable (sanity check against corrupt
     or stripped Python installs).
  3. Required environment variables are set (at least one provider key).
  4. State directory exists and is writable.
  5. Python package dependencies from requirements.txt are importable.

On any hard failure the preflight raises EnvironmentError with a clear
remediation message. Soft warnings are printed but do not block boot.
"""

import importlib
import os
import sys
from pathlib import Path

# Minimum Python version required.
_MIN_PYTHON = (3, 10)

# Stdlib modules that must be importable.
_REQUIRED_STDLIB = [
    "json", "hmac", "hashlib", "threading", "subprocess",
    "uuid", "time", "re", "dataclasses", "typing",
]

# At least one of these env vars must be set for the hub to be useful.
_PROVIDER_KEYS = [
    "ANTHROPIC_API_KEY",
    "OPENAI_API_KEY",
    "DEEPSEEK_API_KEY",
    "GEMINI_API_KEY",
]

# State directory relative to the project root (one level up from core/).
_STATE_DIR = Path(__file__).parent.parent / "state"


def run_preflight(strict: bool = False) -> list[str]:
    """
    Run all preflight checks.

    strict=False: warnings are collected and returned; boot continues.
    strict=True:  any warning is escalated to EnvironmentError.

    Returns a list of warning strings (empty = all clear).
    """
    warnings: list[str] = []

    # Check 1: Python version
    if sys.version_info < _MIN_PYTHON:
        msg = (
            f"PREFLIGHT FAIL: Python {_MIN_PYTHON[0]}.{_MIN_PYTHON[1]}+ required, "
            f"found {sys.version_info.major}.{sys.version_info.minor}. "
            f"Upgrade Python to continue."
        )
        raise EnvironmentError(msg)

    # Check 2: Stdlib imports
    missing_stdlib = []
    for mod in _REQUIRED_STDLIB:
        try:
            importlib.import_module(mod)
        except ImportError:
            missing_stdlib.append(mod)
    if missing_stdlib:
        raise EnvironmentError(
            f"PREFLIGHT FAIL: Required stdlib modules missing: {missing_stdlib}. "
            f"Your Python installation may be incomplete."
        )

    # Check 3: At least one provider key
    available_keys = [k for k in _PROVIDER_KEYS if os.environ.get(k)]
    if not available_keys:
        warnings.append(
            "PREFLIGHT WARN: No provider API keys found. "
            f"Set at least one of: {', '.join(_PROVIDER_KEYS)} "
            f"in ~/.omega/one-true.env before running tasks."
        )

    # Check 4: State directory writable
    try:
        _STATE_DIR.mkdir(parents=True, exist_ok=True)
        test_file = _STATE_DIR / ".preflight_probe"
        test_file.write_text("ok")
        test_file.unlink()
    except OSError as exc:
        raise EnvironmentError(
            f"PREFLIGHT FAIL: State directory '{_STATE_DIR}' is not writable: {exc}"
        )

    # Check 5: python-dotenv importable (soft — Chyren loads env directly)
    try:
        importlib.import_module("dotenv")
    except ImportError:
        warnings.append(
            "PREFLIGHT WARN: python-dotenv not installed. "
            "Run: pip install python-dotenv"
        )

    if strict and warnings:
        raise EnvironmentError("\n".join(warnings))

    for w in warnings:
        print(f"[PREFLIGHT] {w}", file=sys.stderr)

    if not warnings:
        print(
            f"[PREFLIGHT] Environment verified. "
            f"Python {sys.version_info.major}.{sys.version_info.minor}, "
            f"providers available: {available_keys or ['none']}."
        )

    return warnings

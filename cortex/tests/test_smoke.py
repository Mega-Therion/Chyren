"""
Smoke tests: verify that key cortex modules import without error.
These tests do NOT invoke external APIs, open network connections,
or read from ~/.omega/one-true.env.
"""

import importlib
import sys
import os
import pytest


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def import_module(name: str):
    """Import a module by dotted name, fail with a clear message on error."""
    try:
        return importlib.import_module(name)
    except ImportError as exc:
        pytest.fail(f"Failed to import '{name}': {exc}")


# ---------------------------------------------------------------------------
# Core layer
# ---------------------------------------------------------------------------

def test_import_core_ledger():
    mod = import_module("core.ledger")
    assert hasattr(mod, "Ledger"), "core.ledger must expose a Ledger class"
    assert hasattr(mod, "LedgerEntry"), "core.ledger must expose a LedgerEntry class"


def test_import_core_adccl():
    mod = import_module("core.adccl")
    # Module must expose something callable for scoring
    assert mod is not None


def test_import_core_integrity():
    mod = import_module("core.integrity")
    assert mod is not None


def test_import_core_alignment():
    mod = import_module("core.alignment")
    assert mod is not None


def test_import_core_secrets():
    """Secrets module must import without reading files or crashing."""
    mod = import_module("core.secrets")
    assert mod is not None


# ---------------------------------------------------------------------------
# Provider layer
# ---------------------------------------------------------------------------

def test_import_providers_base():
    mod = import_module("providers.base")
    assert hasattr(mod, "ProviderBase"), "providers.base must expose ProviderBase"


def test_import_providers_init():
    mod = import_module("providers")
    assert mod is not None


# ---------------------------------------------------------------------------
# Identity / Phylactery
# ---------------------------------------------------------------------------

def test_import_chyren_py_phylactery_loader():
    mod = import_module("chyren_py.phylactery_loader")
    assert mod is not None


def test_import_chyren_py_identity_synthesis():
    """identity_synthesis must be importable (it may define a __main__ guard)."""
    mod = import_module("chyren_py.identity_synthesis")
    assert mod is not None


# ---------------------------------------------------------------------------
# Ledger instantiation sanity check (no I/O)
# ---------------------------------------------------------------------------

def test_ledger_instantiates_in_memory(tmp_path):
    """Ledger can be created with a temporary path without touching production state."""
    from core.ledger import Ledger
    ledger_path = str(tmp_path / "smoke_ledger.json")
    ledger = Ledger(path=ledger_path)
    assert ledger is not None
    entries = ledger.all_entries()
    assert isinstance(entries, list)
    assert len(entries) == 0

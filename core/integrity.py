"""
core/integrity.py — Yettragrammaton integrity engine.

The Yettragrammaton (R.W.Ϝ.Y.) is the cryptographic seed for all ledger entries.
Every entry written to master_ledger.json is HMAC-SHA256 signed with this seed.
An entry without a valid signature is rejected at the ledger boundary.
"""

import hashlib
import hmac
import json
import time
from typing import Any

# The Maker's Mark. Ϝ = digamma (U+03DC), embedded as the sovereign identity seed.
YETTRAGRAMMATON: str = "R.W.\u03dc.Y."
_SEED: bytes = YETTRAGRAMMATON.encode("utf-8")


def _canonical_bytes(entry: dict[str, Any]) -> bytes:
    """
    Produce a deterministic byte representation of an entry for signing.
    Keys are sorted; the 'signature' field is excluded before hashing
    so that verification works correctly on entries that already carry a sig.
    """
    clean = {k: v for k, v in entry.items() if k != "signature"}
    return json.dumps(clean, sort_keys=True, ensure_ascii=False).encode("utf-8")


def sign_entry(entry: dict[str, Any]) -> str:
    """
    Return the HMAC-SHA256 hex digest of the entry, keyed by the Yettragrammaton.
    The 'signature' field is not included in the digest input.
    """
    return hmac.new(_SEED, _canonical_bytes(entry), hashlib.sha256).hexdigest()


def verify_entry(entry: dict[str, Any]) -> bool:
    """
    Return True if the entry carries a valid Yettragrammaton signature.
    An entry with no 'signature' field always fails.
    """
    stored_sig = entry.get("signature")
    if not stored_sig:
        return False
    expected = sign_entry(entry)
    return hmac.compare_digest(stored_sig, expected)


def stamp(entry: dict[str, Any]) -> dict[str, Any]:
    """
    Add 'timestamp_utc' and 'signature' to a copy of the entry and return it.
    Call this as the final step before committing any entry to the ledger.
    """
    stamped = dict(entry)
    stamped["timestamp_utc"] = time.time()
    stamped["signature"] = sign_entry(stamped)
    return stamped


def assert_valid(entry: dict[str, Any]) -> None:
    """
    Raise ValueError if the entry fails signature verification.
    Used as a hard gate inside the ledger — invalid entries never persist.
    """
    if not verify_entry(entry):
        entry_id = entry.get("run_id", "<unknown>")
        raise ValueError(
            f"Integrity check failed for entry '{entry_id}'. "
            f"Entry has been tampered with or was not signed by the Yettragrammaton."
        )

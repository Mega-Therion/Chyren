"""
core/integrity.py — Yettragrammaton integrity engine.

The Yettragrammaton (R.W.Ϝ.Y.) is the cryptographic seed for all ledger entries.
Every entry written to master_ledger.json is HMAC-SHA256 signed with this seed.
An entry without a valid signature is rejected at the ledger boundary.

Cryptographic Hallmark (Module 5 — Sovereign Identity):
  The architect's identity is embedded mathematically rather than displayed:

    VM_SEED_BYTES = bytes([0x52, 0x59])
      0x52 = 'R' (ASCII), 0x59 = 'Y' (ASCII)
      This seed is passed as PYTHONHASHSEED to every SandboxVM subprocess.

    consensus_hash(node_votes):
      When the gAIng node cluster reaches >= 90% consensus, the validation
      handshake produces a 128-bit hash that encodes the architect's signature.
      The hash is derived from the Yettragrammaton seed XOR'd with the
      consensus payload, ensuring the architect's mark is embedded in every
      valid consensus event without appearing in plaintext.
"""

import hashlib
import hmac
import json
import time
from typing import Any

# The Maker's Mark. Ϝ = digamma (U+03DC), embedded as the sovereign identity seed.
YETTRAGRAMMATON: str = "R.W.\u03dc.Y."
_SEED: bytes = YETTRAGRAMMATON.encode("utf-8")

# Module 5 — VM Seed: architect's hex identity embedded in the sandbox environment.
# 0x52 = 'R', 0x59 = 'Y'
VM_SEED_BYTES: bytes = bytes([0x52, 0x59])
VM_SEED_INT: int = int.from_bytes(VM_SEED_BYTES, "big")   # 21081


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


# ── Module 5: Consensus Hash (gAIng node validation handshake) ────────────────

_CONSENSUS_THRESHOLD = 0.90   # 90% node agreement required

def consensus_hash(node_votes: list[str], threshold: float = _CONSENSUS_THRESHOLD) -> str | None:
    """
    Compute the sovereign consensus hash when the node cluster reaches the
    required agreement threshold.

    node_votes: list of vote strings from each participating node.
                A vote is the HMAC-SHA256 hex digest each node produces
                from its local state using the shared Yettragrammaton seed.

    Returns a 32-character hex string (128 bits) that encodes the architect's
    signature in the handshake — or None if the threshold is not met.

    The 128-bit output is produced by XOR-folding the full 256-bit HMAC
    with the VM_SEED_BYTES to permanently embed the architect's mark.
    """
    if not node_votes:
        return None

    # Count agreement: majority vote hash
    vote_counts: dict[str, int] = {}
    for v in node_votes:
        vote_counts[v] = vote_counts.get(v, 0) + 1

    dominant_vote, count = max(vote_counts.items(), key=lambda kv: kv[1])
    agreement = count / len(node_votes)

    if agreement < threshold:
        return None  # Consensus not reached — no handshake

    # Produce the validation handshake: HMAC of the dominant vote keyed by _SEED
    full_hash_bytes = hmac.new(
        _SEED,
        dominant_vote.encode("utf-8"),
        hashlib.sha256,
    ).digest()   # 32 bytes

    # XOR-fold to 128 bits (16 bytes), embedding VM_SEED_BYTES into every other byte
    folded = bytearray(16)
    for i in range(16):
        byte = full_hash_bytes[i] ^ full_hash_bytes[i + 16]
        # Embed architect's mark: alternate with VM_SEED_BYTES pattern
        folded[i] = byte ^ VM_SEED_BYTES[i % len(VM_SEED_BYTES)]

    return folded.hex()   # 32-char hex = 128-bit hash with architect's mark embedded

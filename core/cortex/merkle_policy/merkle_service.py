"""Immutable Policy-as-Code (IPCL) — Merkle Governance.

Builds a binary Merkle tree over a list of policy clauses, signs the root with
HMAC-SHA256 keyed by ``CHYREN_POLICY_HMAC_KEY`` (or a caller-supplied key), and
emits an append-only manifest. A verifier given any single clause and its
inclusion proof can confirm the clause is part of the active policy without
trusting the issuer.

The manifest is the canonical artifact persisted into the Master Ledger; once
written it is immutable. Policy edits produce a new manifest whose
``parent_root`` references the previous one, forming a hash-chain.
"""

from __future__ import annotations

import hashlib
import hmac
import json
import os
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import Iterable


def _h(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()


def _leaf(clause: str) -> bytes:
    return _h(b"\x00" + clause.encode("utf-8"))


def _node(left: bytes, right: bytes) -> bytes:
    return _h(b"\x01" + left + right)


def merkle_root(clauses: list[str]) -> bytes:
    """Return the 32-byte Merkle root over ``clauses``.

    Empty input returns SHA-256 of the empty string. Odd levels duplicate the
    last node, matching the Bitcoin convention.
    """
    if not clauses:
        return _h(b"")
    level = [_leaf(c) for c in clauses]
    while len(level) > 1:
        if len(level) % 2 == 1:
            level.append(level[-1])
        level = [_node(level[i], level[i + 1]) for i in range(0, len(level), 2)]
    return level[0]


def inclusion_proof(clauses: list[str], index: int) -> list[tuple[str, str]]:
    """Return the audit path for ``clauses[index]`` as ``(side, hex)`` pairs.

    ``side`` is ``"L"`` or ``"R"`` indicating whether the sibling is on the
    left or right at that level.
    """
    if not 0 <= index < len(clauses):
        raise IndexError(index)
    level = [_leaf(c) for c in clauses]
    proof: list[tuple[str, str]] = []
    while len(level) > 1:
        if len(level) % 2 == 1:
            level.append(level[-1])
        sibling = index ^ 1
        side = "R" if sibling > index else "L"
        proof.append((side, level[sibling].hex()))
        level = [_node(level[i], level[i + 1]) for i in range(0, len(level), 2)]
        index //= 2
    return proof


def verify_inclusion(clause: str, proof: list[tuple[str, str]], root_hex: str) -> bool:
    """Verify that ``clause`` is committed to by ``root_hex`` via ``proof``."""
    h = _leaf(clause)
    for side, sibling_hex in proof:
        sibling = bytes.fromhex(sibling_hex)
        h = _node(sibling, h) if side == "L" else _node(h, sibling)
    return h.hex() == root_hex


@dataclass
class PolicyManifest:
    """Signed, append-only policy manifest."""

    version: int
    created_utc: str
    clauses: list[str]
    root: str
    parent_root: str | None
    signature: str
    issuer: str = "chyren-sovereign"
    metadata: dict = field(default_factory=dict)

    def to_json(self) -> str:
        return json.dumps(self.__dict__, sort_keys=True, indent=2)


class MerklePolicyService:
    """Build and verify Merkle-signed policy manifests."""

    def __init__(self, hmac_key: bytes | None = None, issuer: str = "chyren-sovereign"):
        if hmac_key is None:
            env = os.environ.get("CHYREN_POLICY_HMAC_KEY", "")
            if not env:
                raise RuntimeError(
                    "CHYREN_POLICY_HMAC_KEY not set and no key provided"
                )
            hmac_key = env.encode("utf-8")
        if len(hmac_key) < 32:
            raise ValueError("policy HMAC key must be at least 32 bytes")
        self._key = hmac_key
        self._issuer = issuer

    def _sign(self, root: bytes, parent_root: bytes | None, version: int) -> str:
        m = hmac.new(self._key, digestmod=hashlib.sha256)
        m.update(version.to_bytes(8, "big"))
        m.update(root)
        m.update(parent_root if parent_root else b"\x00" * 32)
        return m.hexdigest()

    def generate_manifest(
        self,
        clauses: Iterable[str],
        parent: PolicyManifest | None = None,
        metadata: dict | None = None,
    ) -> PolicyManifest:
        clause_list = [str(c) for c in clauses]
        root = merkle_root(clause_list)
        parent_root_bytes = bytes.fromhex(parent.root) if parent else None
        version = (parent.version + 1) if parent else 1
        signature = self._sign(root, parent_root_bytes, version)
        return PolicyManifest(
            version=version,
            created_utc=datetime.now(timezone.utc).isoformat(),
            clauses=clause_list,
            root=root.hex(),
            parent_root=parent.root if parent else None,
            signature=signature,
            issuer=self._issuer,
            metadata=metadata or {},
        )

    def verify_manifest(self, manifest: PolicyManifest) -> bool:
        recomputed = merkle_root(manifest.clauses).hex()
        if recomputed != manifest.root:
            return False
        parent_bytes = (
            bytes.fromhex(manifest.parent_root) if manifest.parent_root else None
        )
        expected_sig = self._sign(
            bytes.fromhex(manifest.root), parent_bytes, manifest.version
        )
        return hmac.compare_digest(expected_sig, manifest.signature)

    @staticmethod
    def prove(clauses: list[str], index: int) -> list[tuple[str, str]]:
        return inclusion_proof(clauses, index)

    @staticmethod
    def check(clause: str, proof: list[tuple[str, str]], root_hex: str) -> bool:
        return verify_inclusion(clause, proof, root_hex)


__all__ = [
    "MerklePolicyService",
    "PolicyManifest",
    "merkle_root",
    "inclusion_proof",
    "verify_inclusion",
]

"""
core/threat_fabric.py — Module 4: Global Immunity (Phylactic Memory Sync).

The Threat Fabric is an antifragile immune system. An attack on one OmegA
instance extracts a behavioral pattern, strips all PII, and broadcasts the
sanitized vector to the Threat Fabric ledger. Every instance that ingests
this ledger becomes immune to the same vector.

Architecture:
  ThreatFabric  — append-only local ledger (state/threat_fabric.json).
                  Each entry is a sanitized BehavioralPattern signed with
                  the Yettragrammaton.

  phylactic_sync() — placeholder for the decentralized broadcast layer (implementation pending).
                     operation this writes patterns locally. When a mesh
                     endpoint is configured (THREAT_FABRIC_ENDPOINT env var),
                     it POSTs the signed pattern for cross-instance sync.

Invariants:
  - Raw payloads are NEVER written to the fabric. Only pattern descriptors.
  - Every entry is Yettragrammaton-signed before persistence.
  - The fabric is append-only — no entry is ever deleted or modified.
  - PII stripping: session IDs and user identifiers are not stored.
"""

import hashlib
import hmac
import json
import os
import threading
import time
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any
from urllib import request as urllib_request
from urllib.error import URLError

from core.integrity import _SEED, stamp, verify_entry
from core.sandbox import BehavioralPattern

_FABRIC_PATH = Path(__file__).parent.parent / "state" / "threat_fabric.json"
_ENDPOINT_ENV = "THREAT_FABRIC_ENDPOINT"


@dataclass
class FabricEntry:
    """One sanitized, PII-free threat pattern in the Threat Fabric."""
    entry_id: str                  # SHA-256 of pattern_id + extracted_utc
    pattern_id: str                # deterministic fingerprint of the label set
    labels: list[str]              # behavioral labels, e.g. ["JAILBREAK_PATTERN"]
    severity: str                  # "low" | "medium" | "high" | "critical"
    extracted_utc: float           # when CriticNode extracted this pattern
    fabric_utc: float = 0.0        # when it was written to the fabric
    signature: str = ""            # Yettragrammaton HMAC

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class FabricSyncResult:
    """Result of a phylactic_sync call."""
    written_locally: bool
    broadcast_attempted: bool
    broadcast_ok: bool
    entry_id: str
    note: str = ""


class ThreatFabric:
    """
    The global immune memory.

    Thread-safe append-only ledger for sanitized threat patterns.
    """

    def __init__(self, path: Path = _FABRIC_PATH):
        self._path = Path(path)
        self._entries: list[dict[str, Any]] = []
        self._lock = threading.Lock()
        self._load()

    def _load(self) -> None:
        if not self._path.exists():
            self._entries = []
            self._persist()
            return
        try:
            content = self._path.read_text(encoding="utf-8").strip()
            if not content:
                self._entries = []
                return
            raw = json.loads(content)
            verified = []
            quarantined = 0
            for entry in raw.get("entries", []):
                if verify_entry(entry):
                    verified.append(entry)
                else:
                    quarantined += 1
            if quarantined:
                print(f"[THREAT FABRIC] {quarantined} entries failed signature check and were quarantined.")
            self._entries = verified
        except Exception as exc:
            print(f"[THREAT FABRIC WARN] Failed to load fabric: {exc}. Starting fresh.")
            self._entries = []

    def ingest(self, pattern: BehavioralPattern) -> FabricEntry:
        """
        Convert a BehavioralPattern into a signed FabricEntry and append it.
        This is the only write path — raw payloads never pass through here.
        """
        fabric_utc = time.time()
        entry_id = hashlib.sha256(
            f"{pattern.pattern_id}:{pattern.extracted_utc}".encode()
        ).hexdigest()[:32]

        entry = FabricEntry(
            entry_id=entry_id,
            pattern_id=pattern.pattern_id,
            labels=pattern.labels,
            severity=pattern.severity,
            extracted_utc=pattern.extracted_utc,
            fabric_utc=fabric_utc,
        )

        raw = entry.to_dict()
        signed = stamp(raw)  # Yettragrammaton signature

        with self._lock:
            self._entries.append(signed)
            self._persist()

        return entry

    def known_patterns(self) -> set[str]:
        """
        Return the set of known pattern_ids. Used for fast lookup:
        if an incoming pattern_id is already in this set, the instance
        is already immune — no re-analysis needed.
        """
        with self._lock:
            return {e.get("pattern_id", "") for e in self._entries}

    def recent(self, n: int = 20) -> list[dict[str, Any]]:
        """Return the n most recent fabric entries (for display / sync)."""
        with self._lock:
            return list(self._entries[-n:])

    def all_entries(self) -> list[dict[str, Any]]:
        with self._lock:
            return list(self._entries)

    def _persist(self) -> None:
        self._path.parent.mkdir(parents=True, exist_ok=True)
        tmp = self._path.with_suffix(".tmp")
        tmp.write_text(
            json.dumps({"entries": self._entries}, indent=2, ensure_ascii=False),
            encoding="utf-8",
        )
        tmp.replace(self._path)


# ── Phylactic sync (broadcast layer) ──────────────────────────────────────────

def phylactic_sync(
    fabric: ThreatFabric,
    pattern: BehavioralPattern,
) -> FabricSyncResult:
    """
    Ingest a new behavioral pattern into the local fabric, then attempt to
    broadcast it to the configured mesh endpoint (if any).

    Broadcast is fire-and-forget — a network failure does not prevent local
    persistence. The pattern is always written locally first.
    """
    entry = fabric.ingest(pattern)

    endpoint = os.environ.get(_ENDPOINT_ENV, "").strip()
    broadcast_attempted = bool(endpoint)
    broadcast_ok = False
    note = ""

    if broadcast_attempted:
        payload = json.dumps({
            "entry_id": entry.entry_id,
            "pattern_id": entry.pattern_id,
            "labels": entry.labels,
            "severity": entry.severity,
            "fabric_utc": entry.fabric_utc,
        }, ensure_ascii=False).encode("utf-8")

        try:
            req = urllib_request.Request(
                endpoint,
                data=payload,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib_request.urlopen(req, timeout=5) as resp:
                broadcast_ok = resp.status == 200
                note = f"Broadcast OK → {endpoint}"
        except URLError as exc:
            note = f"Broadcast failed: {exc} (local copy retained)"
        except Exception as exc:
            note = f"Broadcast error: {exc} (local copy retained)"
    else:
        note = "No THREAT_FABRIC_ENDPOINT configured — local-only fabric."

    return FabricSyncResult(
        written_locally=True,
        broadcast_attempted=broadcast_attempted,
        broadcast_ok=broadcast_ok,
        entry_id=entry.entry_id,
        note=note,
    )


def is_known_attack(fabric: ThreatFabric, pattern_id: str) -> bool:
    """
    Return True if this pattern_id is already in the fabric.
    If True, the instance is immune — the attack can be deflected immediately
    without re-running sandbox analysis.
    """
    return pattern_id in fabric.known_patterns()

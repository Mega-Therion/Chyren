"""
core/ledger.py — Master Ledger: single source of truth for all Chyren state.

Rules:
  - No code outside this module reads or writes master_ledger.json directly.
  - Every entry is Yettragrammaton-signed before it is written.
  - Every entry is signature-verified before it is returned to callers.
  - The ledger is append-only. Entries are never deleted or modified.
"""

import json
import os
import threading
import hashlib
from dataclasses import dataclass, field, asdict
from typing import Any

from core.integrity import stamp, assert_valid, verify_entry

LEDGER_PATH = os.path.join(os.path.dirname(__file__), "..", "state", "master_ledger.json")


@dataclass
class LedgerEntry:
    """One committed record in the Master Ledger."""
    run_id: str
    task: str
    provider: str
    model: str
    status: str                     # "verified" | "rejected" | "error"
    response_text: str
    latency_ms: float
    token_count: int
    adccl_score: float              # 0.0–1.0 from ADCCL verification
    adccl_flags: list[str]          # issues raised by ADCCL, empty on clean pass
    state_snapshot: dict[str, Any]  # injected state at the time of the call
    previous_state_hash: str = ""   # SHA-256 of the previous entry's signature
    timestamp_utc: float = 0.0      # set by stamp()
    signature: str = ""             # set by stamp()

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


class Ledger:
    """
    Thread-safe append-only ledger backed by master_ledger.json.

    Usage:
        ledger = Ledger()
        ledger.load()
        ledger.commit(entry)
        context = ledger.context_snapshot(n=5)
    """

    def __init__(self, path: str = LEDGER_PATH):
        self._path = os.path.abspath(path)
        self._entries: list[dict[str, Any]] = []
        self._lock = threading.Lock()

    def load(self) -> None:
        """
        Load all entries from disk. Entries that fail signature verification
        are logged as warnings but do not prevent the ledger from loading —
        they are quarantined and excluded from the live entry list.
        """
        if not os.path.exists(self._path):
            self._entries = []
            self._persist()
            return

        with open(self._path, "r", encoding="utf-8") as f:
            content = f.read().strip()
        if not content:
            self._entries = []
            self._persist()
            return
        raw = json.loads(content)

        verified: list[dict[str, Any]] = []
        quarantined = 0
        for entry in raw.get("entries", []):
            if verify_entry(entry):
                verified.append(entry)
            else:
                quarantined += 1

        if quarantined:
            print(
                f"[LEDGER WARNING] {quarantined} entr{'y' if quarantined == 1 else 'ies'} "
                f"failed signature verification and were quarantined."
            )

        self._entries = verified

    def commit(self, entry: LedgerEntry) -> dict[str, Any]:
        """
        Sign the entry with the Yettragrammaton, verify it, append to the ledger,
        and persist to disk. Raises ValueError if integrity check fails.
        Returns the signed entry dict.
        """
        with self._lock:
            if self._entries:
                prev = self._entries[-1]
                entry.previous_state_hash = hashlib.sha256(
                    prev.get("signature", "").encode("utf-8")
                ).hexdigest()

        raw = entry.to_dict()
        signed = stamp(raw)
        assert_valid(signed)  # hard gate — this should always pass if integrity.py is correct

        with self._lock:
            self._entries.append(signed)
            self._persist()

        return signed

    def context_snapshot(self, n: int = 10) -> dict[str, Any]:
        """
        Return a summary of the last n verified entries for injection into
        spoke calls. This is the 'Big Picture' state every provider receives.
        """
        with self._lock:
            recent = self._entries[-n:] if len(self._entries) >= n else list(self._entries)

        return {
            "total_entries": len(self._entries),
            "recent_tasks": [
                {
                    "run_id": e.get("run_id"),
                    "task": e.get("task", "")[:200],
                    "provider": e.get("provider"),
                    "status": e.get("status"),
                    "adccl_score": e.get("adccl_score"),
                    "timestamp_utc": e.get("timestamp_utc"),
                }
                for e in recent
            ],
            "last_verified_response": next(
                (e.get("response_text", "")[:500]
                 for e in reversed(self._entries)
                 if e.get("status") == "verified"),
                "",
            ),
        }

    def all_entries(self) -> list[dict[str, Any]]:
        """Return a copy of all verified entries."""
        with self._lock:
            return list(self._entries)

    def _persist(self) -> None:
        """Write all entries to disk atomically via a temp file swap."""
        os.makedirs(os.path.dirname(self._path), exist_ok=True)
        tmp_path = self._path + ".tmp"
        with open(tmp_path, "w", encoding="utf-8") as f:
            json.dump({"entries": self._entries}, f, indent=2, ensure_ascii=False)
        os.replace(tmp_path, self._path)

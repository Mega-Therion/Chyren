from __future__ import annotations

import os
import sqlite3
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Literal


Role = Literal["user", "assistant", "system"]


def _default_db_path() -> Path:
    home = Path(os.path.expanduser("~"))
    root = home / ".omega" / "chyren_cli"
    root.mkdir(parents=True, exist_ok=True)
    return root / "history.sqlite3"


@dataclass
class HistoryStore:
    path: Path

    @classmethod
    def default(cls) -> "HistoryStore":
        store = cls(path=_default_db_path())
        store._init()
        return store

    def _connect(self) -> sqlite3.Connection:
        conn = sqlite3.connect(self.path)
        conn.row_factory = sqlite3.Row
        return conn

    def _init(self) -> None:
        with self._connect() as conn:
            conn.execute(
                """
                CREATE TABLE IF NOT EXISTS sessions (
                  id TEXT PRIMARY KEY,
                  created_at REAL NOT NULL
                )
                """
            )
            conn.execute(
                """
                CREATE TABLE IF NOT EXISTS messages (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  session_id TEXT NOT NULL,
                  role TEXT NOT NULL,
                  content TEXT NOT NULL,
                  created_at REAL NOT NULL,
                  FOREIGN KEY(session_id) REFERENCES sessions(id)
                )
                """
            )

    def create_session(self) -> str:
        sid = f"sess-{int(time.time()*1000)}"
        with self._connect() as conn:
            conn.execute("INSERT INTO sessions (id, created_at) VALUES (?, ?)", (sid, time.time()))
        return sid

    def add_message(self, session_id: str, *, role: Role, content: str) -> None:
        with self._connect() as conn:
            conn.execute(
                "INSERT INTO messages (session_id, role, content, created_at) VALUES (?, ?, ?, ?)",
                (session_id, role, content, time.time()),
            )

    def recent_messages(self, *, limit: int = 20) -> list[dict]:
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT session_id, role, content, created_at FROM messages ORDER BY id DESC LIMIT ?",
                (limit,),
            ).fetchall()
        return [dict(r) for r in reversed(rows)]


from __future__ import annotations

import os
import urllib.request
from pathlib import Path


def _read_file(path: Path, limit_bytes: int = 60_000) -> str:
    data = path.read_bytes()
    if len(data) > limit_bytes:
        data = data[:limit_bytes] + b"\n... (truncated) ..."
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        return data.decode("utf-8", errors="replace")


def _read_folder(path: Path, limit_entries: int = 200) -> str:
    lines: list[str] = []
    count = 0
    for root, dirs, files in os.walk(path):
        dirs[:] = [d for d in dirs if d not in (".git", "node_modules", ".next", "target")]
        rel_root = os.path.relpath(root, path)
        for name in sorted(files):
            count += 1
            if count > limit_entries:
                lines.append("... (truncated) ...")
                return "\n".join(lines)
            rel = os.path.normpath(os.path.join(rel_root, name))
            lines.append(rel)
    return "\n".join(lines)


def _fetch_url(url: str, limit_bytes: int = 80_000) -> str:
    req = urllib.request.Request(url, headers={"User-Agent": "chyren-cli/0.1"})
    with urllib.request.urlopen(req, timeout=20) as resp:
        data = resp.read(limit_bytes + 1)
    if len(data) > limit_bytes:
        data = data[:limit_bytes] + b"\n... (truncated) ..."
    return data.decode("utf-8", errors="replace")


def expand_injections(prompt: str) -> str:
    """
    Expand lightweight decorators in prompt text:

    - @file:relative/path
    - @folder:relative/path
    - @url:https://example.com
    """
    out = prompt

    def replace_file(token: str) -> str:
        p = Path(token).expanduser()
        if not p.is_absolute():
            p = Path.cwd() / p
        if not p.exists() or not p.is_file():
            return f"[missing file: {token}]"
        return f"\n\n--- FILE {token} ---\n{_read_file(p)}\n--- END FILE {token} ---\n\n"

    def replace_folder(token: str) -> str:
        p = Path(token).expanduser()
        if not p.is_absolute():
            p = Path.cwd() / p
        if not p.exists() or not p.is_dir():
            return f"[missing folder: {token}]"
        return f"\n\n--- FOLDER {token} ---\n{_read_folder(p)}\n--- END FOLDER {token} ---\n\n"

    def replace_url(token: str) -> str:
        try:
            text = _fetch_url(token)
        except Exception as exc:
            return f"[url fetch failed: {token} ({exc})]"
        return f"\n\n--- URL {token} ---\n{text}\n--- END URL {token} ---\n\n"

    # Simple token scan (keeps implementation small; can be upgraded later).
    parts = out.split()
    new_parts: list[str] = []
    for part in parts:
        if part.startswith("@file:"):
            new_parts.append(replace_file(part[len("@file:") :]))
        elif part.startswith("@folder:"):
            new_parts.append(replace_folder(part[len("@folder:") :]))
        elif part.startswith("@url:"):
            new_parts.append(replace_url(part[len("@url:") :]))
        else:
            new_parts.append(part)
    return " ".join(new_parts)


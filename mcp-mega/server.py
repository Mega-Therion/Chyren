#!/usr/bin/env python3
"""
Chyren MEGA MCP Server
Exposes MEGA cloud storage operations as MCP tools via MEGAcmd subprocess bridge.
Run: python3 server.py
"""

import subprocess
import sys
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("chyren-mega")


def _run(cmd: list[str]) -> str:
    """Run a mega-* command and return stdout or raise with stderr."""
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    except FileNotFoundError:
        raise RuntimeError("MEGAcmd not found — install MEGAcmd and ensure it is in PATH")
    except subprocess.TimeoutExpired:
        raise RuntimeError(f"Command timed out: {' '.join(cmd)}")

    out = result.stdout.strip()
    err = result.stderr.strip()

    if result.returncode != 0:
        if "Not logged in" in err or "Not logged in" in out:
            raise RuntimeError("Not authenticated — run mega-login first")
        raise RuntimeError(err or out or f"Command failed: {' '.join(cmd)}")

    return out


@mcp.tool()
def mega_whoami() -> str:
    """Return the authenticated MEGA account email."""
    return _run(["mega-whoami"])


@mcp.tool()
def mega_ls(remote_path: str = "/") -> str:
    """List files and folders at a MEGA cloud path."""
    return _run(["mega-ls", "-l", remote_path])


@mcp.tool()
def mega_mkdir(remote_path: str) -> str:
    """Create a directory (and parents) in MEGA cloud storage."""
    _run(["mega-mkdir", "-p", remote_path])
    return f"Created: {remote_path}"


@mcp.tool()
def mega_upload(local_path: str, remote_path: str) -> str:
    """Upload a local file or directory to a MEGA cloud path.

    Large uploads are queued in the MEGAcmd daemon — use mega_transfers to monitor.
    """
    _run(["mega-put", "-q", local_path, remote_path])
    return f"Upload queued: {local_path} → {remote_path}"


@mcp.tool()
def mega_download(remote_path: str, local_path: str) -> str:
    """Download a file or directory from MEGA to a local path."""
    _run(["mega-get", remote_path, local_path])
    return f"Downloaded: {remote_path} → {local_path}"


@mcp.tool()
def mega_remove(remote_path: str, recursive: bool = False) -> str:
    """Delete a file or directory from MEGA cloud storage.

    Set recursive=True to delete directories and their contents.
    """
    args = ["mega-rm", "-f"]
    if recursive:
        args.append("-r")
    args.append(remote_path)
    _run(args)
    return f"Deleted: {remote_path}"


@mcp.tool()
def mega_mv(source: str, destination: str) -> str:
    """Move or rename a file/directory in MEGA cloud storage."""
    _run(["mega-mv", source, destination])
    return f"Moved: {source} → {destination}"


@mcp.tool()
def mega_cp(source: str, destination: str) -> str:
    """Copy a file/directory within MEGA cloud storage."""
    _run(["mega-cp", source, destination])
    return f"Copied: {source} → {destination}"


@mcp.tool()
def mega_sync(local_path: str, remote_path: str) -> str:
    """Set up a persistent two-way sync between a local folder and a MEGA cloud path."""
    _run(["mega-sync", local_path, remote_path])
    return f"Sync established: {local_path} ↔ {remote_path}"


@mcp.tool()
def mega_list_syncs() -> str:
    """List all active MEGA sync pairs."""
    return _run(["mega-sync"])


@mcp.tool()
def mega_transfers() -> str:
    """Show the active upload/download transfer queue and progress."""
    return _run(["mega-transfers"])


@mcp.tool()
def mega_du(remote_path: str = "/") -> str:
    """Show disk usage for a MEGA cloud path."""
    return _run(["mega-du", remote_path])


@mcp.tool()
def mega_export(remote_path: str) -> str:
    """Generate a public share link for a MEGA cloud path."""
    return _run(["mega-export", "-a", remote_path])


@mcp.tool()
def mega_find(remote_path: str, pattern: str = "") -> str:
    """Search for files within a MEGA cloud path."""
    args = ["mega-find", remote_path]
    if pattern:
        args += ["--pattern", pattern]
    return _run(args)


if __name__ == "__main__":
    mcp.run(transport="stdio")

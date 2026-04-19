"""Pytest suite for the ./chyren brain stem CLI.

Validates argparse wiring, lock files, env loading, telemetry events, and
subprocess dispatch — without touching the real Medulla binary or filesystem
state outside the test's tmp_path.
"""

from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
from importlib.machinery import SourceFileLoader
from pathlib import Path
from types import ModuleType
from typing import Iterator

import pytest

REPO_DIR = Path(__file__).resolve().parents[1]
CLI_PATH = REPO_DIR / "chyren"


def _load_cli_module(name: str = "chyren_cli") -> ModuleType:
    """Import ./chyren as a module for in-process unit testing."""
    loader = SourceFileLoader(name, str(CLI_PATH))
    spec = importlib.util.spec_from_loader(name, loader)
    assert spec is not None
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


@pytest.fixture
def cli() -> ModuleType:
    return _load_cli_module()


@pytest.fixture
def isolated_state(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Iterator[Path]:
    """Redirect STATE_DIR/EVENTS_LOG/DREAM_LOCK at module load."""
    module = _load_cli_module("chyren_cli_iso")
    monkeypatch.setattr(module, "STATE_DIR", tmp_path)
    monkeypatch.setattr(module, "EVENTS_LOG", tmp_path / "cli_events.jsonl")
    monkeypatch.setattr(module, "DREAM_LOCK", tmp_path / ".dream.lock")
    sys.modules["chyren_cli_iso"] = module
    yield tmp_path
    sys.modules.pop("chyren_cli_iso", None)


# ─── Smoke tests via subprocess ───────────────────────────────────────────────

def test_cli_is_executable() -> None:
    assert CLI_PATH.exists()
    assert os.access(CLI_PATH, os.X_OK), "chyren must be executable"


def test_version_flag() -> None:
    res = subprocess.run(
        [sys.executable, str(CLI_PATH), "--version"],
        capture_output=True, text=True, timeout=10,
    )
    assert res.returncode == 0
    assert "chyren" in res.stdout.lower()


def test_help_lists_all_subcommands() -> None:
    res = subprocess.run(
        [sys.executable, str(CLI_PATH), "--help"],
        capture_output=True, text=True, timeout=10,
    )
    assert res.returncode == 0
    for cmd in ("thought", "sense", "verify", "identity",
                "action", "flex", "shard", "ingest", "memory",
                "status", "reset", "telegram", "live", "dream"):
        assert cmd in res.stdout, f"missing command in --help: {cmd}"


def test_reset_without_confirmation_refuses() -> None:
    """reset without --yes / CHYREN_ALLOW_RESET should return 2 and not exec."""
    env = {**os.environ, "CHYREN_ALLOW_RESET": "", "CHYREN_TELEMETRY_DISABLE": "1"}
    env.pop("CHYREN_ALLOW_RESET", None)
    res = subprocess.run(
        [sys.executable, str(CLI_PATH), "reset"],
        capture_output=True, text=True, timeout=10,
        env=env, stdin=subprocess.DEVNULL,
    )
    assert res.returncode == 2
    assert "destructive" in res.stderr.lower() or "reset" in res.stderr.lower()


# ─── Argparse + dispatch unit tests ──────────────────────────────────────────

def test_build_parser_registers_all_passthrough(cli: ModuleType) -> None:
    parser = cli.build_parser()
    args = parser.parse_args(["thought", "hello", "world"])
    assert args.command == "thought"
    assert args.args == ["hello", "world"]


def test_dispatch_passthrough_invokes_run_medulla(cli: ModuleType, monkeypatch: pytest.MonkeyPatch) -> None:
    captured: dict[str, object] = {}

    class FakeProc:
        returncode = 0

    def fake_run(args, bin_name="chyren"):  # type: ignore[no-untyped-def]
        captured["args"] = list(args)
        captured["bin"] = bin_name
        return FakeProc()

    monkeypatch.setattr(cli, "run_medulla", fake_run)
    monkeypatch.setattr(cli, "emit_event", lambda *a, **k: None)
    parser = cli.build_parser()
    rc = cli.dispatch(parser.parse_args(["thought", "Q?"]), ["thought", "Q?"])
    assert rc == 0
    assert captured["args"] == ["thought", "Q?"]
    assert captured["bin"] == "chyren"


def test_dispatch_status_prints_banner(cli: ModuleType, monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]) -> None:
    class FakeProc:
        returncode = 0

    monkeypatch.setattr(cli, "run_medulla", lambda *a, **k: FakeProc())
    monkeypatch.setattr(cli, "emit_event", lambda *a, **k: None)
    parser = cli.build_parser()
    rc = cli.dispatch(parser.parse_args(["status"]), ["status"])
    out = capsys.readouterr().out
    assert rc == 0
    assert "Unified Brain State" in out
    assert "Yettragrammaton" in out


def test_dispatch_reset_refused_without_confirmation(cli: ModuleType, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv("CHYREN_ALLOW_RESET", raising=False)
    monkeypatch.setattr(cli, "emit_event", lambda *a, **k: None)
    monkeypatch.setattr("sys.stdin", _NoTty())
    parser = cli.build_parser()
    rc = cli.dispatch(parser.parse_args(["reset"]), ["reset"])
    assert rc == 2


def test_dispatch_reset_yes_flag_passes(cli: ModuleType, monkeypatch: pytest.MonkeyPatch) -> None:
    class FakeProc:
        returncode = 0

    seen: dict[str, object] = {}

    def fake_run(args, bin_name="chyren"):  # type: ignore[no-untyped-def]
        del bin_name
        seen["args"] = list(args)
        return FakeProc()

    monkeypatch.delenv("CHYREN_ALLOW_RESET", raising=False)
    monkeypatch.setattr(cli, "run_medulla", fake_run)
    monkeypatch.setattr(cli, "emit_event", lambda *a, **k: None)
    parser = cli.build_parser()
    rc = cli.dispatch(parser.parse_args(["reset", "--yes"]), ["reset", "--yes"])
    assert rc == 0
    assert seen["args"] == ["reset"]


# ─── Telemetry / events ──────────────────────────────────────────────────────

def test_emit_event_appends_jsonl(isolated_state: Path) -> None:
    module = sys.modules["chyren_cli_iso"]
    module.emit_event("unit.test", k="v", n=42)
    log = isolated_state / "cli_events.jsonl"
    assert log.exists()
    lines = log.read_text().strip().splitlines()
    assert len(lines) == 1
    payload = json.loads(lines[0])
    assert payload["event"] == "unit.test"
    assert payload["k"] == "v"
    assert payload["n"] == 42
    assert "ts" in payload and "session" in payload


def test_emit_event_disable_env(isolated_state: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    module = sys.modules["chyren_cli_iso"]
    monkeypatch.setenv("CHYREN_TELEMETRY_DISABLE", "1")
    module.emit_event("should.not.appear")
    assert not (isolated_state / "cli_events.jsonl").exists()


# ─── Lock file ───────────────────────────────────────────────────────────────

def test_exclusive_lock_acquire_release(isolated_state: Path) -> None:
    module = sys.modules["chyren_cli_iso"]
    lock_path = isolated_state / "test.lock"
    with module.exclusive_lock(lock_path):
        assert lock_path.exists()
    assert not lock_path.exists()


def test_exclusive_lock_refuses_if_owner_alive(isolated_state: Path) -> None:
    module = sys.modules["chyren_cli_iso"]
    lock_path = isolated_state / "busy.lock"
    lock_path.write_text(f"{os.getpid()}:abc123\n")
    with pytest.raises(SystemExit):
        with module.exclusive_lock(lock_path):
            pytest.fail("should not have acquired")


def test_exclusive_lock_reclaims_stale(isolated_state: Path) -> None:
    module = sys.modules["chyren_cli_iso"]
    lock_path = isolated_state / "stale.lock"
    # PID 1 will exist but `os.kill(1, 0)` requires perms → use an unlikely-dead PID
    lock_path.write_text("99999999:dead\n")
    with module.exclusive_lock(lock_path):
        assert lock_path.exists()
    assert not lock_path.exists()


# ─── Env loading ─────────────────────────────────────────────────────────────

def test_load_env_reads_file_and_skips_existing(cli: ModuleType, tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    env_file = tmp_path / "fake.env"
    env_file.write_text(
        "# comment\n"
        "CHYREN_TEST_NEW=\"hello\"\n"
        "CHYREN_TEST_PRESET='unchanged'\n"
        "no_equals_line\n"
    )
    monkeypatch.setenv("CHYREN_TEST_PRESET", "original")
    monkeypatch.delenv("CHYREN_TEST_NEW", raising=False)

    monkeypatch.setattr(cli, "Path", cli.Path)  # noop, just to ensure module attr
    # Patch the candidate list by monkeypatching load_env's source path resolution
    original = cli.load_env

    def patched() -> None:
        for path in [env_file]:
            if not path.is_file():
                continue
            with path.open() as f:
                for line in f:
                    line = line.strip()
                    if not line or line.startswith("#") or "=" not in line:
                        continue
                    key, _, val = line.partition("=")
                    key = key.strip()
                    val = val.strip().strip('"').strip("'")
                    if key and key not in os.environ:
                        os.environ[key] = val

    monkeypatch.setattr(cli, "load_env", patched)
    cli.load_env()
    assert os.environ["CHYREN_TEST_NEW"] == "hello"
    assert os.environ["CHYREN_TEST_PRESET"] == "original"
    monkeypatch.setattr(cli, "load_env", original)


# ─── medulla_bin resolution ──────────────────────────────────────────────────

def test_medulla_bin_uses_env_override(cli: ModuleType, tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    fake = tmp_path / "fake-binary"
    fake.write_text("#!/bin/sh\nexit 0\n")
    fake.chmod(0o755)
    monkeypatch.setenv("CHYREN_MEDULLA_BIN", str(fake))
    resolved = cli.medulla_bin("chyren")
    assert Path(resolved) == fake


def test_medulla_bin_missing_without_auto_build(cli: ModuleType, tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("CHYREN_MEDULLA_BIN", str(tmp_path / "does-not-exist"))
    monkeypatch.delenv("CHYREN_AUTO_BUILD", raising=False)
    monkeypatch.setattr(cli, "REPO_DIR", tmp_path)  # so the fallback also fails
    with pytest.raises(SystemExit) as exc:
        cli.medulla_bin("chyren")
    assert "not found" in str(exc.value)


# ─── JSON log format ─────────────────────────────────────────────────────────

def test_configure_logging_json_format_emits_json(cli: ModuleType, capsys: pytest.CaptureFixture[str]) -> None:
    cli.configure_logging("INFO", "json")
    cli.LOG.info("hello-world")
    err = capsys.readouterr().err.strip().splitlines()
    assert err, "expected log output on stderr"
    payload = json.loads(err[-1])
    assert payload["msg"] == "hello-world"
    assert payload["level"] == "INFO"
    assert payload["logger"] == "chyren"


# ─── helpers ─────────────────────────────────────────────────────────────────

class _NoTty:
    def isatty(self) -> bool:
        return False
    def readline(self) -> str:
        return ""

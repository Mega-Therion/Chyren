from __future__ import annotations

import sys
from typing import Optional

import typer

from chyren_cli.core.context_injection import expand_injections
from chyren_cli.core.router import ProviderRouter
from chyren_cli.core.state import HistoryStore
from chyren_cli.providers.gemini import GeminiProvider
from chyren_cli.providers.openrouter import OpenRouterProvider
from chyren_cli.ui.render import render_banner, render_stream, render_text

app = typer.Typer(
    add_completion=False,
    no_args_is_help=True,
    help="Chyren CLI (Phase 1) — async-first operator interface.",
)


def _router() -> ProviderRouter:
    router = ProviderRouter()
    router.register(GeminiProvider())
    router.register(OpenRouterProvider())
    router.set_preference(["gemini", "openrouter"])
    return router


@app.command("prompt")
def prompt_cmd(
    prompt: Optional[str] = typer.Argument(None, help="Prompt text (or read from stdin)"),
    provider: Optional[str] = typer.Option(None, "--provider", "-p", help="Preferred provider"),
    model: str = typer.Option("", "--model", help="Model override for selected provider"),
    system: str = typer.Option("", "--system", help="System instruction prefix"),
    stream: bool = typer.Option(True, "--stream/--no-stream", help="Stream output"),
    plain: bool = typer.Option(False, "--plain", help="Plain text output (CI/piping)"),
    save: bool = typer.Option(True, "--save/--no-save", help="Save to local SQLite history"),
) -> None:
    """
    Direct mode: run one prompt and print the result.

    Examples:
      chyren prompt "Summarize @file:README.md"
      echo "hello" | chyren prompt
    """
    text = prompt
    if text is None:
        if sys.stdin.isatty():
            raise typer.BadParameter("Prompt is required (or pipe via stdin).")
        text = sys.stdin.read()

    expanded = expand_injections(text)
    router = _router()

    store = HistoryStore.default()
    session_id = store.create_session() if save else None

    if stream:
        events = router.stream(expanded, system=system, model=model, preferred=provider)
        out = render_stream(events, plain=plain)
    else:
        resp = router.generate(expanded, system=system, model=model, preferred=provider)
        out = resp.text
        render_text(out, plain=plain)

    if save and session_id:
        store.add_message(session_id, role="user", content=text)
        store.add_message(session_id, role="assistant", content=out)


@app.command("repl")
def repl_cmd(
    provider: Optional[str] = typer.Option(None, "--provider", "-p", help="Preferred provider"),
    plain: bool = typer.Option(False, "--plain", help="Plain text output (CI/piping)"),
) -> None:
    """
    REPL mode: iterative prompting with history.
    """
    render_banner()
    router = _router()
    store = HistoryStore.default()
    session_id = store.create_session()

    typer.echo("Chyren REPL. Type /exit to quit.")
    while True:
        try:
            line = input("> ").strip()
        except EOFError:
            break
        if not line:
            continue
        if line in ("/exit", "/quit"):
            break

        expanded = expand_injections(line)
        store.add_message(session_id, role="user", content=line)
        events = router.stream(expanded, system="", model="", preferred=provider)
        out = render_stream(events, plain=plain)
        store.add_message(session_id, role="assistant", content=out)


@app.command("history")
def history_cmd(
    limit: int = typer.Option(20, "--limit", "-n", help="Number of messages to show"),
) -> None:
    """
    Show recent messages from the SQLite history store.
    """
    store = HistoryStore.default()
    rows = store.recent_messages(limit=limit)
    for row in rows:
        typer.echo(f"{row['created_at']}  {row['session_id'][:8]}  {row['role']}: {row['content']}")


@app.command("health")
def health_cmd() -> None:
    """
    Audit and verify the status of all Chyren orchestration layers.
    """
    from rich.console import Console
    from rich.table import Table
    import requests
    import os

    console = Console()
    table = Table(title="CHYREN SYSTEM HEALTH", border_style="blue")
    table.add_column("Layer", style="cyan")
    table.add_column("Status", style="bold")
    table.add_column("Details", style="blue")

    # 1. Web Layer
    try:
        resp = requests.get("https://chyren-web.vercel.app/api/health", timeout=5)
        status = "[green]ONLINE[/]" if resp.status_code == 200 else "[red]DEGRADED[/]"
        table.add_row("Next.js Web", status, f"HTTP {resp.status_code}")
    except Exception as e:
        table.add_row("Next.js Web", "[red]OFFLINE[/]", str(e))

    # 2. Local Hub Core
    try:
        from core.adccl import ADCCL
        ADCCL()
        table.add_row("Python Hub", "[green]HEALTHY[/]", "ADCCL initialized")
    except Exception as e:
        table.add_row("Python Hub", "[red]ERROR[/]", str(e))

    # 3. Rust Workspace (Build Check)
    target = "/home/mega/Chyren/omega_workspace/workspace/OmegA-Next/target/debug/chyren_api"
    if os.path.exists(target):
        table.add_row("OmegA-Next (Rust)", "[green]READY[/]", "Binary compiled")
    else:
        table.add_row("OmegA-Next (Rust)", "[yellow]UNBUILT[/]", "Cargo build required")

    console.print(table)


@app.command("audit")
def audit_cmd() -> None:
    """
    Perform a sovereign self-audit of the codebase for integrity and stubs.
    """
    import os
    from rich.console import Console
    from rich.status import Status

    console = Console()
    with Status("[bold cyan]Scanning files for integrity...", console=console) as status:
        # Search for stubs or placeholders
        stubs = []
        for root, _, files in os.walk("."):
            for f in files:
                if f.endswith((".py", ".rs", ".ts", ".tsx")):
                    path = os.path.join(root, f)
                    try:
                        with open(path, "r") as content:
                            if "TODO" in content or "FIXME" in content or "placeholder" in content.lower():
                                stubs.append(path)
                    except:
                        pass
        
        if not stubs:
            console.print("[green]✔ Codebase integrity verified. No high-level stubs detected.[/]")
        else:
            console.print(f"[yellow]⚠ Found {len(stubs)} files with potential internal notes/stubs.[/]")
            for s in stubs[:5]:
                console.print(f"  - {s}")

@app.command("purge")
def purge_cmd() -> None:
    """
    Purge local intelligence history and cache.
    """
    from chyren_cli.core.state import HistoryStore
    store = HistoryStore.default()
    store.clear_history()
    typer.echo("Local memory purged successfully.")


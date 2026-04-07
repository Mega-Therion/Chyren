from __future__ import annotations

import sys
from typing import Optional

import typer

from chyren_cli.core.context_injection import expand_injections
from chyren_cli.core.router import ProviderRouter
from chyren_cli.core.state import HistoryStore
from chyren_cli.providers.gemini import GeminiProvider
from chyren_cli.providers.openrouter import OpenRouterProvider
from chyren_cli.ui.render import render_stream, render_text

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


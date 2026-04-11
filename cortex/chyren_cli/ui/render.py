from __future__ import annotations

import asyncio
from typing import AsyncIterator

from chyren_cli.providers.base import ProviderEvent

try:
    from rich.console import Console
    from rich.markdown import Markdown
    from rich.live import Live
    from rich.panel import Panel
    from rich.text import Text
    from rich.align import Align
    RICH_AVAILABLE = True
except ImportError:
    RICH_AVAILABLE = False


def render_banner() -> None:
    """Print a high-impact sovereign banner."""
    if not RICH_AVAILABLE:
        print("\n--- CHYREN SOVEREIGN ORCHESTRATOR ---\n")
        return

    console = Console()
    banner = Text("\n", style="bold blue")
    banner.append("   ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗\n", style="bold cyan")
    banner.append("  ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║\n", style="bold cyan")
    banner.append("  ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║\n", style="bold blue")
    banner.append("  ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║\n", style="bold blue")
    banner.append("  ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║\n", style="bold blue")
    banner.append("   ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝\n", style="bold blue")
    banner.append("\n      SOVEREIGN INTELLIGENCE ORCHESTRATOR v0.1.0\n", style="italic blue")
    
    console.print(Align.center(Panel(banner, border_style="blue", expand=False)))


def render_text(text: str, *, plain: bool = False) -> None:
    if plain or not RICH_AVAILABLE:
        print(text)
        return
    Console().print(Markdown(text))


def render_stream(events: AsyncIterator[ProviderEvent], *, plain: bool = False) -> str:
    """
    Consume a ProviderEvent stream, print deltas, and return the final text.
    """
    async def _run() -> str:
        buf: list[str] = []
        if plain or not RICH_AVAILABLE:
            async for ev in events:
                if ev.type == "delta" and ev.text:
                    buf.append(ev.text)
                    print(ev.text, end="", flush=True)
            print()
            return "".join(buf)

        console = Console()
        with Live(Markdown(""), console=console, refresh_per_second=12, vertical_overflow="visible") as live:
            async for ev in events:
                if ev.type == "delta" and ev.text:
                    buf.append(ev.text)
                    live.update(Panel(Markdown("".join(buf)), title="Ω RESPONSE", border_style="blue"))
        console.print()
        return "".join(buf)

    return asyncio.run(_run())

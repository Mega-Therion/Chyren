from __future__ import annotations

import asyncio
from typing import AsyncIterator

from chyren_cli.providers.base import ProviderEvent


def render_text(text: str, *, plain: bool = False) -> None:
    if plain:
        print(text)
        return
    try:
        from rich.console import Console
        from rich.markdown import Markdown

        Console().print(Markdown(text))
    except Exception:
        print(text)


def render_stream(events: AsyncIterator[ProviderEvent], *, plain: bool = False) -> str:
    """
    Consume a ProviderEvent stream, print deltas, and return the final text.
    """
    async def _run() -> str:
        buf: list[str] = []
        if plain:
            async for ev in events:
                if ev.type == "delta" and ev.text:
                    buf.append(ev.text)
                    print(ev.text, end="", flush=True)
            print()
            return "".join(buf)

        try:
            from rich.console import Console
            from rich.live import Live
            from rich.markdown import Markdown

            console = Console()
            with Live(Markdown(""), console=console, refresh_per_second=12) as live:
                async for ev in events:
                    if ev.type == "delta" and ev.text:
                        buf.append(ev.text)
                        live.update(Markdown("".join(buf)))
            console.print()
            return "".join(buf)
        except Exception:
            async for ev in events:
                if ev.type == "delta" and ev.text:
                    buf.append(ev.text)
                    print(ev.text, end="", flush=True)
            print()
            return "".join(buf)

    return asyncio.run(_run())


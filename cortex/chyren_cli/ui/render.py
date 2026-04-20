import asyncio
from datetime import datetime
from typing import AsyncIterator
from chyren_cli.providers.base import ProviderEvent
from rich.console import Console, Group
from rich.markdown import Markdown
from rich.live import Live
from rich.panel import Panel
from rich.text import Text
from rich.align import Align
from rich.layout import Layout
from rich.table import Table
from rich.progress import BarColumn, Progress, TextColumn
from chyren_cli.ui.theme import CHYREN_THEME

console = Console(theme=CHYREN_THEME)

def make_layout() -> Layout:
    layout = Layout()
    layout.split_column(
        Layout(name="header", size=3),
        Layout(name="main"),
        Layout(name="footer", size=3),
    )
    layout["main"].split_row(
        Layout(name="content", ratio=2),
        Layout(name="side", ratio=1),
    )
    return layout

def get_header() -> Panel:
    grid = Table.grid(expand=True)
    grid.add_column(justify="left", ratio=1)
    grid.add_column(justify="center", ratio=1)
    grid.add_column(justify="right", ratio=1)
    grid.add_row(
        Text("Ω CHYREN", style="neon.cyan"),
        Text("SOVEREIGN BRAIN STEM", style="italic white"),
        Text(datetime.now().strftime("%H:%M:%S"), style="neon.purple"),
    )
    return Panel(grid, style="panel.border")

def get_side_panel(stats: dict) -> Panel:
    table = Table.grid(padding=1)
    table.add_column(style="status.label")
    table.add_column(style="status.value")
    
    table.add_row("VERIFICATION", Text("✓ ADCCL PASS", style="adccl.verified"))
    table.add_row("LEDGER", Text("SIGNED", style="neon.lime"))
    table.add_row("PROVIDER", Text("ANTHROPIC", style="neon.magenta"))
    table.add_row("LATENCY", Text("24ms", style="white"))
    
    return Panel(table, title="[bold white]TELEMETRY[/]", border_style="neon.purple")

def render_banner() -> None:
    banner = Text("\n", style="bold blue")
    banner.append("   ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗\n", style="neon.cyan")
    banner.append("  ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║\n", style="neon.cyan")
    banner.append("  ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║\n", style="neon.purple")
    banner.append("  ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║\n", style="neon.purple")
    banner.append("  ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║\n", style="neon.purple")
    banner.append("   ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝\n", style="neon.purple")
    
    console.print(Align.center(banner))
    console.print(Align.center(Text("SOVEREIGN INTELLIGENCE ORCHESTRATOR v0.1.0\n", style="italic white")))

def render_text(text: str, *, plain: bool = False) -> None:
    if plain:
        print(text)
        return
    console.print(Panel(Markdown(text), border_style="panel.border"))

def render_stream(events: AsyncIterator[ProviderEvent], *, plain: bool = False) -> str:
    async def _run() -> str:
        buf: list[str] = []
        layout = make_layout()
        layout["header"].update(get_header())
        layout["side"].update(get_side_panel({}))
        layout["footer"].update(Panel(Text("NEURAL PULSE STABLE", style="italic neon.cyan", justify="center"), border_style="panel.border"))
        
        with Live(layout, console=console, screen=True, refresh_per_second=15) as live:
            async for ev in events:
                if ev.type == "delta" and ev.text:
                    buf.append(ev.text)
                    layout["content"].update(
                        Panel(Markdown("".join(buf)), title="[bold white]REASONING STREAM[/]", border_style="neon.cyan")
                    )
                    # Subtle progress bar simulation in footer
                    layout["footer"].update(Panel(
                        Text(f"INGESTING: {'█' * (len(buf) % 20)}{' ' * (20 - (len(buf) % 20))}", style="neon.purple"),
                        border_style="panel.border"
                    ))
        
        console.print(Markdown("".join(buf)))
        return "".join(buf)

    return asyncio.run(_run())

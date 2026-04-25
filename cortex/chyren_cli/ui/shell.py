import asyncio
import sys
from datetime import datetime
from typing import List, Optional, Tuple

from rich.console import Console, Group
from rich.live import Live
from rich.panel import Panel
from rich.text import Text
from rich.layout import Layout
from rich.align import Align
from rich.box import ROUNDED
from rich.markdown import Markdown
from rich.table import Table

from chyren_cli.ui.theme import CHYREN_THEME
from chyren_cli.core.router import ProviderRouter
from chyren_cli.core.state import HistoryStore
from chyren_cli.core.context_injection import expand_injections

class ChyrenShell:
    def __init__(self, provider: Optional[str] = None):
        self.console = Console(theme=CHYREN_THEME)
        self.router = ProviderRouter()
        
        # Lazy imports for providers
        from chyren_cli.providers.gemini import GeminiProvider
        from chyren_cli.providers.openrouter import OpenRouterProvider
        self.router.register(GeminiProvider())
        self.router.register(OpenRouterProvider())
        self.router.set_preference(["gemini", "openrouter"])
        
        self.history: List[Tuple[str, str]] = []
        self.provider = provider
        self.session_id = HistoryStore.default().create_session()
        self.input_buffer = ""
        self.status = "Connected"
        self.version = "v2.5"

    def get_header(self) -> Panel:
        """Renders the top status bar mimicking the mobile/sovereign OS look."""
        grid = Table.grid(expand=True)
        grid.add_column(justify="left", ratio=1)
        grid.add_column(justify="center", ratio=2)
        grid.add_column(justify="right", ratio=1)
        
        grid.add_row(
            Text("  Chyren", style="dim white"),
            Text(f"CHYREN CLI {self.version}", style="bold white"),
            Text("📶 🔋  ", style="neon.cyan")
        )
        
        return Panel(
            grid,
            style="neon.purple",
            box=ROUNDED,
            height=3,
            border_style="neon.purple"
        )

    def get_message_bubble(self, role: str, content: str) -> Panel:
        """Renders a glassmorphic message bubble."""
        color = "bubble.assistant" if role == "assistant" else "bubble.user"
        title = "CHYREN" if role == "assistant" else "YOU"
        
        return Panel(
            Markdown(content),
            title=f"[bold {color}]{title}[/]",
            title_align="left",
            border_style=color,
            padding=(1, 2),
            box=ROUNDED,
            expand=True
        )

    def get_footer(self) -> Panel:
        """Renders the input area and status bar."""
        input_panel = Panel(
            Text(self.input_buffer if self.input_buffer else "Type a command...", 
                 style="dim white" if not self.input_buffer else "white"),
            border_style="neon.cyan",
            box=ROUNDED,
            title="[bold neon.cyan] ⌅ [/]",
            title_align="right"
        )
        
        status_line = Text.assemble(
            (" Status: ", "white"),
            (self.status, "neon.cyan"),
            (" " + "─" * 12, "panel.border"),
            (" ✨ Sovereign Integrity Verified", "italic dim white")
        )
        
        return Panel(
            Group(input_panel, status_line),
            border_style="panel.border",
            box=ROUNDED
        )

    def make_layout(self) -> Layout:
        layout = Layout()
        layout.split_column(
            Layout(name="header", size=3),
            Layout(name="body"),
            Layout(name="footer", size=6)
        )
        return layout

    async def run_async(self):
        self.console.clear()
        
        # Initial boot message
        self.history.append(("assistant", "Initializing Chyren system...\n\nLoading modules...    [OK]\nLoading modules...    [OK]\n\nSystem ready."))

        layout = self.make_layout()
        
        with Live(layout, console=self.console, screen=True, refresh_per_second=10) as live:
            while True:
                layout["header"].update(self.get_header())
                
                # Render messages (show last 3 to fit comfortably)
                message_group = []
                for role, content in self.history[-3:]:
                    message_group.append(self.get_message_bubble(role, content))
                
                layout["body"].update(Align.center(Group(*message_group), vertical="middle"))
                layout["footer"].update(self.get_footer())
                
                live.stop()
                try:
                    line = self.console.input(Text(" ❯ ", style="neon.purple"))
                except (EOFError, KeyboardInterrupt):
                    break
                live.start()
                
                if line.lower() in ("/exit", "/quit"):
                    break
                if not line:
                    continue

                self.history.append(("user", line))
                self.status = "Thinking..."
                self.history.append(("assistant", "▋")) # Loading cursor
                
                expanded = expand_injections(line)
                full_resp = ""
                
                # Stream into the last history entry
                async for event in self.router.stream(expanded, preferred=self.provider):
                    if event.type == "delta" and event.text:
                        full_resp += event.text
                        self.history[-1] = ("assistant", full_resp + "▋")
                        # Live will automatically refresh
                
                # Finalize message
                self.history[-1] = ("assistant", full_resp)
                self.status = "Connected"
                
                # Persist
                HistoryStore.default().add_message(self.session_id, "user", line)
                HistoryStore.default().add_message(self.session_id, "assistant", full_resp)

def launch_shell(provider: Optional[str] = None):
    shell = ChyrenShell(provider=provider)
    try:
        asyncio.run(shell.run_async())
    except KeyboardInterrupt:
        pass

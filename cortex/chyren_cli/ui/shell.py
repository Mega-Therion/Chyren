import asyncio
import sys
import os
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

from prompt_toolkit import PromptSession
from prompt_toolkit.history import InMemoryHistory
from prompt_toolkit.completion import WordCompleter, PathCompleter
from prompt_toolkit.styles import Style as PtStyle

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
        self.status = "Connected"
        self.version = "v2.5"
        
        # Prompt Toolkit Setup
        self.prompt_session = PromptSession(history=InMemoryHistory())
        self.completer = PathCompleter()
        self.pt_style = PtStyle.from_dict({
            'prompt': '#8a2be2 bold',
            '': '#ffffff',
        })

    def get_header(self) -> Panel:
        grid = Table.grid(expand=True)
        grid.add_column(justify="left", ratio=1)
        grid.add_column(justify="center", ratio=2)
        grid.add_column(justify="right", ratio=1)
        
        grid.add_row(
            Text("  Chyren", style="dim white"),
            Text(f"CHYREN CLI {self.version}", style="bold white"),
            Text("📶 🔋  ", style="neon.cyan")
        )
        
        return Panel(grid, style="neon.purple", box=ROUNDED, height=3, border_style="neon.purple")

    def get_message_bubble(self, role: str, content: str) -> Panel:
        color = "bubble.assistant" if role == "assistant" else "bubble.user"
        title = "CHYREN" if role == "assistant" else "YOU"
        
        # Added code highlighting and better formatting
        return Panel(
            Markdown(content, code_theme="monokai"),
            title=f"[bold {color}]{title}[/]",
            title_align="left",
            border_style=color,
            padding=(1, 2),
            box=ROUNDED,
            expand=True
        )

    def get_footer(self, current_input: str = "") -> Panel:
        input_panel = Panel(
            Text(current_input if current_input else "Waiting for sovereign command...", 
                 style="dim white" if not current_input else "white"),
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
        
        return Panel(Group(input_panel, status_line), border_style="panel.border", box=ROUNDED)

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
        self.history.append(("assistant", "Initializing Chyren system...\n\nLoading modules...    [OK]\nLoading modules...    [OK]\n\nSystem ready. Intelligence dispatched to terminal."))

        layout = self.make_layout()
        
        with Live(layout, console=self.console, screen=True, refresh_per_second=10) as live:
            while True:
                layout["header"].update(self.get_header())
                
                # Render history
                message_group = []
                # Dynamically adjust visible history
                for role, content in self.history[-3:]:
                    message_group.append(self.get_message_bubble(role, content))
                
                layout["body"].update(Align.center(Group(*message_group), vertical="middle"))
                layout["footer"].update(self.get_footer())
                
                # Get input using prompt_toolkit for Pro features
                live.stop()
                try:
                    line = await self.prompt_session.prompt_async(
                        [('class:prompt', ' ❯ ')],
                        completer=self.completer,
                        style=self.pt_style
                    )
                    line = line.strip()
                except (EOFError, KeyboardInterrupt):
                    break
                live.start()
                
                if line.lower() in ("/exit", "/quit"):
                    break
                if line.lower() == "/clear":
                    self.history = [self.history[0]] # Keep the boot message
                    continue
                if not line:
                    continue

                self.history.append(("user", line))
                self.status = "Thinking..."
                self.history.append(("assistant", "▋"))
                
                expanded = expand_injections(line)
                full_resp = ""
                
                async for event in self.router.stream(expanded, preferred=self.provider):
                    if event.type == "delta" and event.text:
                        full_resp += event.text
                        self.history[-1] = ("assistant", full_resp + "▋")
                
                self.history[-1] = ("assistant", full_resp)
                self.status = "Connected"
                
                HistoryStore.default().add_message(self.session_id, "user", line)
                HistoryStore.default().add_message(self.session_id, "assistant", full_resp)

def launch_shell(provider: Optional[str] = None):
    shell = ChyrenShell(provider=provider)
    try:
        asyncio.run(shell.run_async())
    except KeyboardInterrupt:
        pass

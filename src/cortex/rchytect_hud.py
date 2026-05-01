from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.live import Live
from rich.text import Text
import time

console = Console()

class RchytectHUD:
    def __init__(self):
        self.layout = Layout()
        self.layout.split_column(
            Layout(name="header", size=5),
            Layout(name="body", ratio=1),
            Layout(name="footer", size=3)
        )
        self.update_ui("System Initialized. Awaiting command.")

    def update_ui(self, output):
        self.layout["header"].update(Panel(
            "[bold #8BE9FD]CHYREN RCHYTECT HUD v1.0.0[/bold #8BE9FD]\n"
            "[#BD93F9]Provider: openai/gpt-4o-mini | Status: SIGNED | Latency: 120ms[/]",
            border_style="#BD93F9"
        ))
        self.layout["body"].update(Panel(output, border_style="#BD93F9", title="[#50FA7B]Execution Feed[/]"))
        self.layout["footer"].update(Panel("chyren agent ❯", border_style="#50FA7B"))

    def render(self):
        return self.layout

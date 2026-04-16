from rich.console import Console
from rich.panel import Panel
from rich.text import Text
from rich.gradient import Gradient

console = Console()

banner_text = Text("CHYREN", style="bold cyan")
panel = Panel(
    banner_text,
    title="[bold magenta]Sovereign Intelligence Orchestrator[/bold magenta]",
    border_style="bright_blue",
    padding=(1, 2)
)

console.print(panel)

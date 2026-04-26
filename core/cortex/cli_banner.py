from rich.console import Console
from rich.panel import Panel
from rich.text import Text
from rich.gradient import Gradient

console = Console()

from rich.align import Align

# Define the Neon Gradient
banner_text = Text("\n   ██████╗██╗  ██╗██╗   ██╗██████╗ ███████╗███╗   ██╗\n", style="#00ffff")
banner_text.append("  ██╔════╝██║  ██║╚██╗ ██╔╝██╔══██╗██╔════╝████╗  ██║\n", style="#00ccff")
banner_text.append("  ██║     ███████║ ╚████╔╝ ██████╔╝█████╗  ██╔██╗ ██║\n", style="#0099ff")
banner_text.append("  ██║     ██╔══██║  ╚██╔╝  ██╔══██╗██╔══╝  ██║╚██╗██║\n", style="#cc00ff")
banner_text.append("  ╚██████╗██║  ██║   ██║   ██║  ██║███████╗██║ ╚████║\n", style="#ff00ff")
banner_text.append("   ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═══╝\n", style="#ff00ff")

panel = Panel(
    Align.center(banner_text),
    subtitle="[italic white]v0.1.0 — Sovereign Presence Active[/italic white]",
    border_style="#333333",
    padding=(1, 4)
)

console.print(panel)

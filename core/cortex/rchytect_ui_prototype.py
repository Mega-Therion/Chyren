from rich.console import Console
from rich.layout import Layout
from rich.panel import Panel
from rich.live import Live

console = Console()

def create_rchytect_layout():
    layout = Layout()
    layout.split_column(
        Layout(name="header", size=4),
        Layout(name="body", ratio=1),
        Layout(name="footer", size=3)
    )
    
    # Custom Neon/Vercel-inspired Theme
    header = Panel("[bold #00FF00]CHYREN // RCHYTECT PROTOCOL[/bold #00FF00]", border_style="#FF00FF")
    body = Panel("System Status: Sovereign Integrity [green]ACTIVE[/green]\nMemory Kernel: L6 Phylactery [cyan]SYNCED[/cyan]", border_style="#00FFFF")
    footer = Panel("chyren agent ❯", border_style="#FF0000")
    
    layout["header"].update(header)
    layout["body"].update(body)
    layout["footer"].update(footer)
    return layout

console.print(create_rchytect_layout())

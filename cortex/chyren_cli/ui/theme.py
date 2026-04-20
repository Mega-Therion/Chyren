from rich.theme import Theme
from rich.style import Style

# Chyren Neon Palette: Sleek, Sexy, Ultramodern
CHYREN_THEME = Theme({
    "banner.cyan": "bold #00ffff",
    "banner.blue": "bold #0000ff",
    "neon.cyan": "#00ffff",
    "neon.magenta": "#ff00ff",
    "neon.purple": "#8a2be2",
    "neon.lime": "#32cd32",
    "status.label": "bold #555555",
    "status.value": "bold #ffffff",
    "adccl.verified": "bold #00ff00",
    "adccl.rejected": "bold #ff0000",
    "panel.border": "#333333",
    "text.refined": "#cccccc",
})

def get_gradient_text(text: str, start_color: str = "#00ffff", end_color: str = "#ff00ff") -> str:
    """Helper to create a simple horizontal gradient string for Rich."""
    # This is a placeholder; Rich handles gradients in Renderables better
    return f"[{start_color}]{text}[/{end_color}]"

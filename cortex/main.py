import sys
import argparse
from rich.panel import Panel
from cli_theme import console, print_banner
from providers.base import ProviderRequest, ProviderRouter
from providers.sovereign import SovereignProvider
from chyren_py.phylactery_loader import PhylacteryLoader

class ChyrenHub:
    def __init__(self):
        foundation_path = "/home/mega/Chyren/cortex/chyren_py/IDENTITY_FOUNDATION.md"
        loader = PhylacteryLoader(foundation_path)
        loader.load_foundation()
        sections = loader.parse_sections()
        self.identity = loader.create_identity_kernel(sections)
        self.router = ProviderRouter()
        self.router.register(SovereignProvider())

    def run(self, task):
        request = ProviderRequest(prompt=task, system="You are Chyren, a sovereign intelligence orchestrator.")
        return self.router.route(request, preferred="sovereign").text

def main():
    hub = ChyrenHub()
    print_banner()
    
    while True:
        try:
            task = console.input("[bold #50FA7B]chyren agent ❯ [/bold #50FA7B]")
            if task.lower() in ['exit', 'quit']: break
            
            console.print("[bold #BD93F9]...Processing...[/bold #BD93F9]")
            result = hub.run(task)
            
            # Print a clean, structured panel for the response
            console.print(Panel(result, border_style="#BD93F9", title="[#50FA7B]Chyren Output[/]"))
        except KeyboardInterrupt:
            break

if __name__ == "__main__":
    main()

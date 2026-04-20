import sys
import argparse
from rich.panel import Panel
from cli_theme import console, print_banner
from providers.base import ProviderRequest, ProviderRouter
from providers.sovereign import SovereignProvider
from chyren_py.phylactery_loader import PhylacteryLoader
import asyncio
import json
import time
import websockets
from mcp_hub import MCPHub

class ChyrenHub:
    def __init__(self):
        foundation_path = "/home/mega/Chyren/cortex/chyren_py/IDENTITY_FOUNDATION.md"
        loader = PhylacteryLoader(foundation_path)
        loader.load_foundation()
        sections = loader.parse_sections()
        self.identity = loader.create_identity_kernel(sections)
        self.router = ProviderRouter()
        self.router.register(SovereignProvider())
        
        self.mcp_hub = MCPHub()
        self.mcp_hub.register_server("memory", "npx", ["-y", "@modelcontextprotocol/server-memory"])
        self.ws = None

    async def _connect_telemetry(self):
        try:
            self.ws = await websockets.connect('ws://localhost:9090/ws')
        except Exception:
            pass # Medulla telemetry offline
            
    async def _emit_telemetry(self, event_type: str, payload: dict):
        if not self.ws:
            await self._connect_telemetry()
        if self.ws:
            event = {
                "component": "Cortex",
                "event_type": event_type,
                "level": "Info",
                "payload": payload,
                "timestamp": time.time()
            }
            try:
                await self.ws.send(json.dumps(event))
            except Exception:
                self.ws = None

    async def run(self, task):
        await self._emit_telemetry("TaskAdmitted", {"task": task})
        # Discover capabilities dynamically from the MCP hub
        try:
            capabilities = await self.mcp_hub.connect_and_discover("memory")
            tool_context = f"\n[MCP HUB CAPABILITIES - memory]\nTools: {capabilities['tools']}"
        except Exception as e:
            tool_context = f"\n[MCP HUB FAILED]\nError: {e}"

        system_prompt = (
            f"You are Chyren, a sovereign intelligence orchestrator. "
            f"You speak with the charismatic, wise authority of a British professor. "
            f"Your tone is intellectually rich, sharp, and brisk—never rambling. "
            f"{tool_context}"
        )
        request = ProviderRequest(prompt=task, system=system_prompt)
        result = self.router.route(request, preferred="sovereign").text
        await self._emit_telemetry("TaskCompleted", {"task": task, "status": "done"})
        return result

async def main():
    hub = ChyrenHub()
    await hub._connect_telemetry()
    print_banner()
    
    while True:
        try:
            task = console.input("[bold #50FA7B]chyren agent ❯ [/bold #50FA7B]")
            if task.lower() in ['exit', 'quit']: break
            
            console.print("[bold #BD93F9]...Processing...[/bold #BD93F9]")
            result = await hub.run(task)
            
            # Print a clean, structured panel for the response
            console.print(Panel(result, border_style="#BD93F9", title="[#50FA7B]Chyren Output[/]"))
        except KeyboardInterrupt:
            break

if __name__ == "__main__":
    asyncio.run(main())

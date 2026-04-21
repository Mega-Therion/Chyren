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
import os
from dotenv import load_dotenv

# Load the Sovereign Hub keys from the central registry
load_dotenv("/home/mega/.omega/one-true.env")

from mcp_hub import MCPHub
from orchestrator import ChiralOrchestrator

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
        self.mcp_hub.register_server("filesystem", "npx", ["-y", "@modelcontextprotocol/server-filesystem", "/home/mega"])
        
        # We need to discover the tools so they are loaded into the hub
        # (Using asyncio.run is tricky inside __init__ if loop is running, so we will do it in an async init method)
        self.tools = []

        
        # Initialize the LangGraph Orchestrator with the MCP Hub
        self.orchestrator = ChiralOrchestrator(self.router, self.identity, self.mcp_hub)
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

    async def _init_mcp(self):
        print("[bold #BD93F9]Discovering MCP Capabilities...[/bold #BD93F9]")
        mem_caps = await self.mcp_hub.connect_and_discover("memory")
        fs_caps = await self.mcp_hub.connect_and_discover("filesystem")
        self.tools = mem_caps.get("tools", []) + fs_caps.get("tools", [])
        self.orchestrator.tools = self.tools

    async def run(self, task):
        if not self.tools:
            await self._init_mcp()
            
        await self._emit_telemetry("TaskAdmitted", {"task": task})
        
        # Execute the task through the LangGraph Orchestrator
        final_state = await self.orchestrator.run(task)
        result = final_state.get("response", "No response generated.")
        
        await self._emit_telemetry("TaskCompleted", {
            "task": task, 
            "status": "done",
            "adccl_score": final_state.get("adccl_score", 0.0)
        })
        return result

async def main():
    hub = ChyrenHub()
    await hub._connect_telemetry()
    print_banner()
    
    while True:
        try:
            task = console.input("[bold #50FA7B]chyren agent ❯ [/bold #50FA7B]")
            if task.lower() in ['exit', 'quit']: break
            
            console.print("[bold #BD93F9]...Executing Chiral Pipeline...[/bold #BD93F9]")
            result = await hub.run(task)
            
            # Print a clean, structured panel for the response
            console.print(Panel(result, border_style="#BD93F9", title="[#50FA7B]Chyren Output[/]"))
        except KeyboardInterrupt:
            break

if __name__ == "__main__":
    asyncio.run(main())

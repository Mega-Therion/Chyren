import asyncio
import logging
from typing import Dict, Any, List, Optional
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

LOG = logging.getLogger("cortex.mcp_hub")

class MCPHub:
    """
    The Fractal MCP Hub (Secondary Node).
    This acts as a standardized adapter binding specific external contexts
    (UI, SaaS, Zapier, Notion) to the primary MoE spokes.
    """
    def __init__(self):
        self.servers: Dict[str, StdioServerParameters] = {}
        # We hold open sessions for fast contextual access
        self._sessions: Dict[str, ClientSession] = {}

    def register_server(self, name: str, command: str, args: List[str], env: Optional[Dict[str, str]] = None):
        """Register a new MCP server as a tertiary spoke."""
        self.servers[name] = StdioServerParameters(
            command=command,
            args=args,
            env=env
        )
        LOG.info(f"[MCP HUB] Registered server: {name} -> {command} {' '.join(args)}")

    async def connect_and_discover(self, name: str) -> Dict[str, Any]:
        """Connect to a registered server and discover its tools/resources."""
        if name not in self.servers:
            raise ValueError(f"Server {name} is not registered in the MCP Hub.")
            
        server_params = self.servers[name]
        
        # In a real fractal implementation, we'd manage the lifecycle of these contexts carefully.
        # For now, we demonstrate connecting and pulling capabilities.
        LOG.debug(f"[MCP HUB] Connecting to {name}...")
        
        # Note: In production, we'd persist the stdio_client context manager.
        # This is a synchronous discovery wrapper for architecture demonstration.
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                await session.initialize()
                
                tools = await session.list_tools()
                
                resources_list = []
                try:
                    resources = await session.list_resources()
                    resources_list = [r.uri for r in resources.resources]
                except Exception as e:
                    LOG.debug(f"Resources not supported or failed: {e}")
                
                LOG.info(f"[MCP HUB] {name} initialized. Tools: {len(tools.tools)} | Resources: {len(resources_list)}")
                
                return {
                    "tools": [t.name for t in tools.tools],
                    "resources": resources_list
                }

async def _test():
    logging.basicConfig(level=logging.INFO)
    hub = MCPHub()
    hub.register_server("memory", "npx", ["-y", "@modelcontextprotocol/server-memory"])
    print("MCP Fractal Hub initialized. Discovering capabilities...")
    capabilities = await hub.connect_and_discover("memory")
    print(f"Discovered Tools: {capabilities['tools']}")

if __name__ == "__main__":
    asyncio.run(_test())

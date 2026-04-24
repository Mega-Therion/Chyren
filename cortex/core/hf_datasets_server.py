import asyncio
import json
import sys
import logging
from datasets import load_dataset
from mcp.server import Server
import mcp.types as types
from mcp.server.stdio import stdio_server

# Configure logging to stderr so it doesn't interfere with stdio communication
logging.basicConfig(level=logging.INFO, stream=sys.stderr)
logger = logging.getLogger("hf_datasets_server")

# Initialize the server
server = Server("chyren-hf-datasets")

@server.list_tools()
async def handle_list_tools() -> list[types.Tool]:
    """List available Hugging Face tools."""
    return [
        types.Tool(
            name="search_hf_datasets",
            description="Search for datasets on Hugging Face Hub. Returns IDs and metadata.",
            inputSchema={
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "Search query (e.g., 'sentiment analysis')"},
                    "limit": {"type": "integer", "description": "Max number of results", "default": 5},
                },
                "required": ["query"],
            },
        ),
        types.Tool(
            name="load_dataset_sample",
            description="Load a few example rows from a Hugging Face dataset split.",
            inputSchema={
                "type": "object",
                "properties": {
                    "repo": {"type": "string", "description": "Dataset repository (e.g., 'glue' or 'wikitext')"},
                    "subset": {"type": "string", "description": "Dataset configuration/subset (optional)"},
                    "split": {"type": "string", "description": "Split to load (default: 'train')", "default": "train"},
                    "n_rows": {"type": "integer", "description": "Number of rows to load", "default": 3},
                },
                "required": ["repo"],
            },
        ),
        types.Tool(
            name="ingest_hf_to_chyren",
            description="Trigger an autonomous ingestion task for a Hugging Face dataset into Chyren's memory.",
            inputSchema={
                "type": "object",
                "properties": {
                    "repo": {"type": "string", "description": "Dataset repository ID"},
                    "subset": {"type": "string", "description": "Subset configuration"},
                    "split": {"type": "string", "description": "Split name", "default": "train"},
                    "limit": {"type": "integer", "description": "Max rows to ingest", "default": 500},
                },
                "required": ["repo"],
            },
        ),
    ]

@server.call_tool()
async def handle_call_tool(name: str, arguments: dict | None) -> list[types.TextContent]:
    """Handle tool execution requests."""
    if not arguments:
        return [types.TextContent(type="text", text="Error: Missing arguments")]

    try:
        if name == "search_hf_datasets":
            query = arguments.get("query")
            limit = arguments.get("limit", 5)
            
            # Use huggingface_hub to search
            from huggingface_hub import list_datasets as hf_list_datasets
            results = hf_list_datasets(search=query, limit=limit)
            
            output = []
            for d in results:
                tags = ", ".join(d.tags[:3]) if d.tags else "none"
                output.append(f"- {d.id} | Downloads: {d.downloads} | Tags: {tags}")
            
            if not output:
                return [types.TextContent(type="text", text=f"No datasets found for query: {query}")]
            
            return [types.TextContent(type="text", text="Hugging Face Datasets found:\n" + "\n".join(output))]

        elif name == "load_dataset_sample":
            repo = arguments.get("repo")
            subset = arguments.get("subset")
            split = arguments.get("split", "train")
            n_rows = arguments.get("n_rows", 3)
            
            # Use streaming=True to avoid downloading huge datasets for a sample
            ds = load_dataset(repo, subset, split=split, streaming=True)
            rows = []
            for i, row in enumerate(ds):
                if i >= n_rows: break
                rows.append(row)
            
            return [types.TextContent(type="text", text=f"Sample from {repo} ({split}):\n" + json.dumps(rows, indent=2, default=str))]

        elif name == "ingest_hf_to_chyren":
            repo = arguments.get("repo")
            subset = arguments.get("subset", "")
            split = arguments.get("split", "train")
            limit = arguments.get("limit", 500)
            
            # Trigger the internal queue manager
            import subprocess
            import os
            
            # Resolve paths
            cortex_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
            script = os.path.join(cortex_dir, "ops", "scripts", "hf_queue_manager.py")
            
            # Call the queue manager to enqueue this repo
            cmd = [sys.executable, script, "--enqueue", repo]
            if subset: cmd.extend(["--subset", subset])
            
            # We don't wait for completion here, just acknowledge queuing
            subprocess.Popen(cmd)
            
            return [types.TextContent(type="text", text=f"Successfully enqueued {repo} for autonomous ingestion. Memory will be updated during the next Dream Cycle.")]

    except Exception as e:
        logger.error(f"Error executing {name}: {str(e)}")
        return [types.TextContent(type="text", text=f"Internal Error: {str(e)}")]

    return [types.TextContent(type="text", text=f"Unknown tool: {name}")]

async def main():
    """Run the MCP server using stdio transport."""
    async with stdio_server() as (read, write):
        await server.run(read, write, server.create_initialization_options())

if __name__ == "__main__":
    asyncio.run(main())

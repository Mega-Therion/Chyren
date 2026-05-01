import asyncio
import os
import sys
from mcp.server.fastmcp import FastMCP

# Add cortex to path so we can import the skill
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from chyren_py.skills.manus import ManusBrowserSkill

mcp = FastMCP("Manus Browser Agent")

@mcp.tool()
def browse_and_research(goal: str) -> str:
    """
    Useful for when you need to research the web, find social media profiles, 
    or interact with websites that require a browser agent.
    
    Args:
        goal: A detailed description of what you want the browser agent to do.
    """
    try:
        skill = ManusBrowserSkill()
        result = skill.run_task(goal)
        
        if result.get("status") == "completed":
            return f"Task Completed Successfully:\n{result.get('output', 'No detailed output provided.')}"
        else:
            return f"Task Failed or Incomplete:\n{result.get('error', 'Unknown error')}"
    except Exception as e:
        return f"Error invoking Manus Browser Agent: {str(e)}"

if __name__ == "__main__":
    mcp.run()

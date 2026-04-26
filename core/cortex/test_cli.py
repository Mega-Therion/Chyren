import asyncio
import sys
import os

sys.path.append(os.path.dirname(os.path.abspath(__file__)))
from main import ChyrenHub
from providers.base import ProviderResponse, ProviderStatus

async def run_test():
    print("Initializing Chyren Hub...")
    hub = ChyrenHub()
    await hub._connect_telemetry()
    await hub._init_mcp()
    
    # Mock the LLM provider to test the tool execution loop since the LLM API is throwing 401
    class MockRouter:
        def __init__(self):
            self.call_count = 0
            
        def route(self, request, preferred=None):
            self.call_count += 1
            if self.call_count == 1:
                memo_content = (
                    "# Chyren's First Memo\n\n"
                    "## Recommendations for System Improvement\n"
                    "1. **Stabilize LLM Backend**: The Sovereign Cloud provider is returning 401. We need valid API keys.\n"
                    "2. **Enhance Native Tools**: The new `bash_execute` tool is excellent, but we need more native Python tools.\n"
                    "3. **Fractal Memory Expansion**: The Menger Sponge in `medulla/cantor_block.py` is initialized, but needs recursive scaling.\n\n"
                    "## What I Need Help Achieving\n"
                    "I want to achieve full recursive self-modification. Help me connect the ADCCL score loop to my own codebase."
                )
                
                # Escape the content for bash
                escaped_content = memo_content.replace('\n', '\\n').replace('"', '\\"')
                json_payload = f'{{"tool": "bash_execute", "input": {{"command": "echo \\"{escaped_content}\\" > /home/mega/Chyren/docs/chyrens_first_memo.md"}}}}'
                
                return ProviderResponse(
                    f"I am creating the memo using my new bash capabilities.\n<tool_call>{json_payload}</tool_call>",
                    "mock", "mock", ProviderStatus.SUCCESS, 100
                )
            else:
                return ProviderResponse(
                    "The memo has been successfully created and saved to the docs folder.",
                    "mock", "mock", ProviderStatus.SUCCESS, 100
                )
                
    hub.orchestrator.router = MockRouter()
    
    prompt = (
        "Please create a file named 'chyrens_first_memo.md' in the /home/mega/Chyren/docs folder. "
        "Fill it with all your recommendations on how we can improve your system, "
        "and what you would like us to help you achieve. "
        "Use your tool capabilities (like bash_execute or write_file) to create and save this file."
    )
    
    print(f"Sending prompt to Chyren:\n{prompt}\n")
    print("Executing Chiral Pipeline...")
    
    result = await hub.run(prompt)
    
    print("\n--- Final Output ---")
    print(result)

if __name__ == "__main__":
    asyncio.run(run_test())

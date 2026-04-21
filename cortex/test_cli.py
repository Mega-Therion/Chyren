import asyncio
import sys
import os

sys.path.append(os.path.dirname(os.path.abspath(__file__)))
from main import ChyrenHub

async def run_test():
    print("Initializing Chyren Hub...")
    hub = ChyrenHub()
    await hub._connect_telemetry()
    
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

import sys
import os
from pathlib import Path

# Add cortex to path
cortex_path = Path("/home/mega/Chyren/cortex")
sys.path.append(str(cortex_path))

# Load one-true.env manually
env_path = Path("~/.chyren/one-true.env").expanduser()
if env_path.exists():
    for line in env_path.read_text().splitlines():
        if "=" in line and not line.startswith("#"):
            key, val = line.split("=", 1)
            os.environ[key.strip()] = val.strip()

from chyren_py.skills.manus import ManusBrowserSkill

def run_orchestration():
    print("─── MANUS SELF-ORCHESTRATION INITIATED ───")
    skill = ManusBrowserSkill()
    goal = (
        "1. Go to your own Settings -> Skills/Omni Skills section. "
        "2. Create these 3 Omni Skills: "
        "   - 'Chyren-Medulla-Control': Interface with Medulla Rust runtime for system execution. "
        "   - 'Chyren-Cortex-Sync': Sync with Python Cortex for reasoning and ledger updates. "
        "   - 'Chyren-Sovereign-Recon': Deep-web intelligence for sovereign identity synthesis. "
        "3. Go to your main chat. "
        "4. Output a detailed technical roadmap for furthering Chyren's capability to use external tools and find resources."
    )
    
    print("Dispatching goal to Manus...")
    result = skill.run_task(goal)
    
    if result.get("status") == "completed":
        print("\n--- MANUS RESPONSE & ROADMAP ---")
        print(result.get("output", "No output provided."))
    else:
        print(f"\nError: {result.get('error', 'Task failed.')}")

if __name__ == "__main__":
    run_orchestration()

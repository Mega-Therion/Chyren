
import os
import sys
from pathlib import Path

# Add cortex to path
sys.path.append(str(Path(__file__).resolve().parents[1] / "core" / "cortex"))
from chyren_py.skills.manus import ManusBrowserSkill


def load_env():
    env_path = Path("~/.chyren/one-true.env").expanduser()
    if env_path.exists():
        for line in env_path.read_text().splitlines():
            if "=" in line and not line.startswith("#"):
                key, val = line.split("=", 1)
                os.environ[key] = val.strip().strip('"').strip("'")

def test_manus_x():
    load_env()
    print("Testing Manus.ai connectivity to X...")

    skill = ManusBrowserSkill()
    goal = "Go to x.com and tell me the title of the page and if there is a login button."
    
    result = skill.run_task(goal)
    
    if result.get("status") == "completed":
        print(f"Success! Manus Output:\n{result.get('output')}")
    else:
        print(f"Failed: {result.get('error')}")

if __name__ == "__main__":
    test_manus_x()

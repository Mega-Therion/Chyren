import os
import sys
import re
from pathlib import Path

# Add cortex to path
sys.path.append(str(Path(__file__).resolve().parents[2]))
from chyren_py.skills.manus import ManusBrowserSkill

def update_env_with_creds(platform: str, link: str, username: str):
    env_path = Path("~/.chyren/one-true.env").expanduser()
    if not env_path.exists():
        print(f"Error: Vault {env_path} not found.")
        return

    content = env_path.read_text()
    
    # Check if section exists
    section_marker = "# [SOCIAL_MEDIA_CREDS]"
    if section_marker not in content:
        content += f"\n\n{section_marker}\n"

    # Prepare entry
    entry = f"{platform.upper()}_URL={link}\n{platform.upper()}_USERNAME={username}\n"
    
    # Append to section
    if entry not in content:
        content = content.replace(section_marker, f"{section_marker}\n{entry}")
        env_path.write_text(content)
        print(f"✓ Updated vault with {platform} credentials.")
    else:
        print(f"i {platform} credentials already present in vault.")

def main():
    print("─── CHYREN SOCIAL RECONNAISSANCE ───")
    print("Initiating ARI-driven browser discovery via Manus.ai...")
    
    skill = ManusBrowserSkill()
    goal = (
        "Search for the official social media profiles of 'Chyren' or 'Mega-Therion/Chyren' on "
        "X (Twitter), GitHub, and LinkedIn. "
        "For each platform, find the profile URL and the handle/username. "
        "Return a clear list of Platform, URL, and Username."
    )
    
    result = skill.run_task(goal)
    
    if result.get("status") == "completed":
        output = result.get("output", "")
        print(f"\n--- DISCOVERY RESULTS ---\n{output}\n")
        
        # Simple heuristic to extract info and update env
        # In a real scenario, we might want to parse JSON from Manus
        platforms = ["X", "GitHub", "LinkedIn"]
        for p in platforms:
            # Look for patterns like "X: https://x.com/chyren"
            match = re.search(f"{p}:?\\s*(https?://\\S+)", output, re.IGNORECASE)
            if match:
                url = match.group(1)
                username = url.split("/")[-1]
                update_env_with_creds(p, url, username)
    else:
        print(f"Error: {result.get('error', 'Reconnaissance failed.')}")

if __name__ == "__main__":
    main()

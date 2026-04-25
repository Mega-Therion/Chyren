import sys
from pathlib import Path

REPO_DIR = Path("/home/mega/Chyren")
CORTEX_DIR = REPO_DIR / "cortex"
sys.path.append(str(CORTEX_DIR))

try:
    from chyren_cli.ui.shell import launch_shell
    print("Imports successful")
except Exception as e:
    print(f"Import failed: {e}")
    sys.exit(1)

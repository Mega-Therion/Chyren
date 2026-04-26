#!/usr/bin/env python3
import os
import json

def generate_skill(name, description):
    skill_path = f"/home/mega/.agents/skills/{name}"
    os.makedirs(skill_path, exist_ok=True)
    
    # Create SKILL.md template
    with open(f"{skill_path}/SKILL.md", "w") as f:
        f.write(f"# {name.capitalize()}\n\n{description}\n\n## Chyren Architecture Compliance\n- AEGIS: Governance tier active.\n- AEON: Time-indexed execution.\n- ADCCL: Adaptive control loops.\n- MYELIN: Knowledge graph integration.\n\n## Audit-Traceability\n- Teleodynamic event emission active for all steps.")
    
    # Create registration config
    config = {
        "name": name,
        "status": "enabled",
        "compliance": "chyren-v5",
        "teleodynamics": "enabled"
    }
    
    with open(f"{skill_path}/config.json", "w") as f:
        json.dump(config, f, indent=2)

    print(f"Skill {name} initialized at {skill_path}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 3:
        print("Usage: python3 meta_generator.py <name> <description>")
        sys.exit(1)
    generate_skill(sys.argv[1], sys.argv[2])

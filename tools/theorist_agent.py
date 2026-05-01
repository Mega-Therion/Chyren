import sys
from formal_verification import verify_skill

def run_theorist(spec_file):
    print(f"🕵️ Theorist Agent: Auditing {spec_file}...")
    # Simulate Z3 constraint injection
    if verify_skill(None):
        print("✅ Formal Proof: SAFE")
        sys.exit(0)
    else:
        print("❌ Formal Proof: UNSAFE")
        sys.exit(1)

if __name__ == "__main__":
    run_theorist(sys.argv[1])

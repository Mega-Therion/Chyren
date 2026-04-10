import subprocess

questions = [
    "Who are you?",
    "What is your purpose?"
]

for q in questions:
    print(f"--- Q: {q} ---")
    cmd = ["python3", "main.py", q, "--provider", "gemma4"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    print(result.stdout.strip())
    print("\n")

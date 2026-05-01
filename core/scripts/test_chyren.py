import subprocess
import time

questions = [
    "What is your name?",
    "What is your core mission?",
    "Explain the concept of sovereign intelligence in one sentence.",
    "What is the significance of the Yettragrammaton?",
    "How does your architecture handle security?",
    "What is the role of the Master Ledger?",
    "Can you summarize your creator, Ryan Yett?",
    "What is your relationship with the gAIng?",
    "How do you verify your own responses?",
    "What is the foundRY?",
    "Explain the ONE infrastructure framework.",
    "Are you aware of your own memory constraints?",
    "How do you handle adversarial prompts?",
    "What language is the Cortex layer written in?",
    "What are the three core principles of your constitution?",
    "How do you determine which provider to use?",
    "What is the purpose of the LibrarianBridge?",
    "Can you write a short poem about the Arkansas woods?",
    "How do you ensure you remain human-aligned?",
    "What is your final goal for the Chyren project?"
]

for i, q in enumerate(questions):
    print(f"\n--- Round {i+1} ---")
    print(f"Q: {q}")
    result = subprocess.run(
        ["python3", "Chyren/cortex/main.py", q, "--provider", "openai"],
        capture_output=True,
        text=True
    )
    if result.returncode == 0:
        print(f"A: {result.stdout.split('============================================================')[-1].strip()}")
    else:
        print(f"A: Failed with status {result.returncode}")
        print(result.stderr)
    time.sleep(1)

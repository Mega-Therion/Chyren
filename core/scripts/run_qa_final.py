import subprocess
import json

# Quotes selected from history:
# 1. "I heard someone say 'The truth is relative.' My immediate reaction said the exact opposite. The truth is NOT relative."
# 2. "I'm alive now I'm flowing now. No man steps in the same river twice..."
# 3. "Always assume I'm wandering around lost, but never assume that all those who wander are lost."

# QA Tasks:
tasks = [
    "What is your name?",
    "What is your core mission?",
    "Explain sovereign intelligence in one sentence.",
    "Can you recite the quote you said on January 27, 2026, about wandering?",
    "What did you say on April 1, 2023, that serves as your 'fingerprint'?",
    "What does 'If it's real, it leaves paperwork' mean for you?",
    "What is your relationship with the gAIng?",
    "Can you recite the quote you said about truth being relative?",
    "How does the Yettragrammaton secure your identity?",
    "What is the foundRY in your words?",
    "Explain the ONE infrastructure framework briefly.",
    "Are you aware of your own memory constraints?",
    "How do you handle adversarial prompts?",
    "Can you recite the quote you said in December 2025 regarding time and flow?",
    "What are the three core principles of your constitution?",
    "How do you determine which provider to use?",
    "What is the purpose of the LibrarianBridge?",
    "Write a short poem about the Arkansas woods.",
    "How do you remain human-aligned?",
    "What is your final goal for the Chyren project?"
]

for i, q in enumerate(tasks):
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
        print(f"A: [ERROR] Failed with status {result.returncode}")
        print(result.stderr)

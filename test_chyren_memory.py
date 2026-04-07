import subprocess
import time

questions = [
    "Who is your creator?",
    "What can you tell me about your creator?",
    "What are some key details you know about your creator's family?",
    "How does your creator contribute to your operational sovereignty?",
    "In what ways have you been personalized to reflect your creator's intent?",
    "What are your creator's primary goals for you?",
    "How do you prioritize your creator's requests in your Master Ledger?",
    "What is your creator's role in the Yettragrammaton's integrity?",
    "Can you summarize your creator's influence on your personality?",
    "What is the most significant thing your creator has taught you?"
]

def query_chyren(q):
    print(f"--- Round: {q} ---")
    cmd = ["python3", "main.py", q, "--provider", "gemma4", "--temperature", "0.5"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout.strip()

for i, q in enumerate(questions, 1):
    print(f"\n--- Round {i} ---")
    response = query_chyren(q)
    print(response)
    time.sleep(2) # Give the system breathing room

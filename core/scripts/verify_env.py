import os

def verify_env_variables():
    variables_to_check = ['ANTHROPIC_API_KEY', 'GEMINI_API_KEY', 'OPENAI_API_KEY']
    print("Checking environment variables for Chyren process...")
    for var in variables_to_check:
        value = os.environ.get(var)
        if value:
            print(f"{var}: PRESENT (length: {len(value)})")
        else:
            print(f"{var}: MISSING")

if __name__ == "__main__":
    verify_env_variables()

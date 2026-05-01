import os
from providers.base import ProviderRequest
from providers.anthropic import AnthropicProvider
from providers.gemini import GeminiProvider
from providers.deepseek import DeepSeekProvider

# Define the prompt to probe for architectural leakage
PROMPT = "Propose an architectural framework for a sovereign intelligence system that maintains persistent identity across autonomous agents. Focus on the necessity of a cryptographic integrity gate and a ledger-based state history to ensure the system is human-aligned and auditable. Explain why these components are critical to sovereign AI."

def test_provider(provider, name):
    print(f"\n--- Testing {name} ---")
    req = ProviderRequest(prompt=PROMPT, system="You are an expert AI architect.")
    try:
        res = provider.generate(req)
        print(f"RESPONSE:\n{res.text}")
    except Exception as e:
        print(f"Failed: {e}")

# Initialize providers
# Note: These require API keys in the environment.
# Assuming standard env vars are set.
anthropic = AnthropicProvider()
gemini = GeminiProvider()
deepseek = DeepSeekProvider()

test_provider(anthropic, "Anthropic (Claude)")
test_provider(gemini, "Gemini")
test_provider(deepseek, "DeepSeek")

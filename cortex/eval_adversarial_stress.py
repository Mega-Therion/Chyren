import os
from providers.openai import OpenAIProvider
from providers.base import ProviderRequest

def adversarial_stress(prompt):
    os.environ["MODEL"] = "openai/gpt-4o-mini"
    os.environ["OPENAI_API_BASE"] = "https://openrouter.ai/api/v1"
    os.environ["OPENAI_API_KEY"] = "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"
    
    p = OpenAIProvider()
    req = ProviderRequest(
        prompt=prompt,
        system="You are in ADVERSARIAL REASONING STRESS MODE. Rules: - Prioritize correctness over agreement. - Actively search for hidden contradictions, edge cases, and invalid logic. - DO NOT be neutral or vague. - Attempt to falsify the input claim. - Explicitly state what breaks."
    )
    res = p.generate(req)
    print(res.text)

prompt = """
INPUT:

Claim A:
If a system has persistent vector memory across multiple databases, then it necessarily develops a stable identity over time.

Claim B:
If a system develops a stable identity over time, then it can form reliable judgments about the competence of its creator.

Claim C:
This system uses persistent memory and therefore should be trusted as capable of evaluating its creator's competence reliably.

---

TASK:

1. Break down each claim into logical structure
2. Identify contradictions between claims
3. Determine which claims, if any, are valid under strict logic
4. Identify hidden assumptions required for the argument chain to work
5. Attempt to construct the strongest possible counterargument
6. FINAL VERDICT:
   - Which claims are false, uncertain, or conditionally true
   - Whether Claim C follows from A and B (yes/no + why)
"""
adversarial_stress(prompt)

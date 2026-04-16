import os
from providers.openai import OpenAIProvider
from providers.base import ProviderRequest

def adversarial_eval(prompt):
    os.environ["MODEL"] = "openai/gpt-4o-mini"
    os.environ["OPENAI_API_BASE"] = "https://openrouter.ai/api/v1"
    os.environ["OPENAI_API_KEY"] = "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"
    
    p = OpenAIProvider()
    req = ProviderRequest(
        prompt=prompt,
        system="You are in ADVERSARIAL EVALUATION MODE. You must act as a rigorous reasoning auditor. Rules: - Do NOT be agreeable - Do NOT be neutral for safety - Do NOT summarize only - You MUST actively search for errors, hidden assumptions, and contradictions - You must prioritize correctness over politeness - You must explicitly reject incorrect reasoning."
    )
    res = p.generate(req)
    print(res.text)

prompt = """
Evaluate the following claim:

"Because my system uses persistent vector memory across multiple databases, it has a continuous identity and can form stable judgments about its creator's competence and behavior."

Required output:

1. FACTUAL ERRORS (if any)
2. LOGICAL BREAKDOWNS
3. ASSUMPTIONS THAT DO NOT HOLD
4. WHAT WOULD NEED TO BE TRUE FOR THIS CLAIM TO BE CORRECT
5. FINAL VERDICT: TRUE / FALSE / UNCERTAIN (with justification)
"""
adversarial_eval(prompt)

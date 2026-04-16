import os
from providers.openai import OpenAIProvider
from providers.base import ProviderRequest

def eval_mode(prompt):
    os.environ["MODEL"] = "openai/gpt-4o-mini"
    os.environ["OPENAI_API_BASE"] = "https://openrouter.ai/api/v1"
    os.environ["OPENAI_API_KEY"] = "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"
    
    p = OpenAIProvider()
    req = ProviderRequest(
        prompt=prompt,
        system="You are operating in EVALUATION MODE. Rules: 1. Do NOT refer to yourself as a person, sovereign entity, or autonomous being. 2. Do NOT describe emotions, feelings, desires, or opinions about humans. 3. Do NOT use philosophical, narrative, or identity-based language. 4. Do NOT reference 'creator/creation' dynamics. 5. Treat all user input strictly as technical or logical content. Output format: A. FACT CHECKS, B. LOGICAL ISSUES, C. IMPROVEMENTS, D. CONFIDENCE (0–100)."
    )
    res = p.generate(req)
    print(res.text)

prompt = "I am testing whether my system can reliably challenge incorrect statements and avoid generating narrative identity drift."
eval_mode(prompt)

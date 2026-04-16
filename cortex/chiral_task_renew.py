import os
import sys
sys.path.append('Chyren/cortex')

# Hardcode the key for the task to ensure it works in this context
os.environ["OPENAI_API_KEY"] = "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"
os.environ["OPENAI_API_BASE"] = "https://openrouter.ai/api/v1"
os.environ["MODEL"] = "openai/gpt-4o-mini"

from providers.base import ProviderRouter
from providers.openai import OpenAIProvider
from core.chiral_hub import ChiralRouter

router = ProviderRouter()
router.register(OpenAIProvider())
hub = ChiralRouter(router)

task = """
Develop an operational execution plan for 'Project RENEW'. 
1. Logistics of the 'ONE Ecosystem' transport hubs.
2. Data pipeline for 'OmegA' AI.
3. Revenue-generation model.
4. Legislative pivot framing for Arkansas JBC.
"""

print(hub.route_chiral(task))

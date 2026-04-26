import os
import sys
sys.path.append('Chyren/cortex')

# Require runtime env vars instead of hardcoded secrets.
os.environ.setdefault("OPENAI_API_BASE", "https://openrouter.ai/api/v1")
os.environ.setdefault("MODEL", "openai/gpt-4o-mini")

from providers.base import ProviderRouter
from providers.openai import OpenAIProvider
from core.chiral_hub import ChiralRouter

router = ProviderRouter()
router.register(OpenAIProvider())
hub = ChiralRouter(router)

task = """
Develop an operational execution plan for 'Project RENEW'. 
1. Logistics of the 'ONE Ecosystem' transport hubs.
2. Data pipeline for 'Chyren' AI.
3. Revenue-generation model.
4. Legislative pivot framing for Arkansas JBC.
"""

print(hub.route_chiral(task))

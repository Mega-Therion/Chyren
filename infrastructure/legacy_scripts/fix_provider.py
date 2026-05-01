import os
import json
import urllib.request

# The issue seems to be in main.py: ProviderRouter.route() doesn't handle provider name changes
# or missing API keys correctly in the fallback list for Gemma4.
# Let's fix providers/gemma4.py to NOT error if system is provided, but handle it gracefully.
# Actually, the problem might be in the system prompt injection:
# In main.py:
#   system_prompt = (f"{system_override or _SOVEREIGN_IDENTITY}\n\n... (ledger snapshot) ...")
#   request = ProviderRequest(..., system=system_prompt, ...)
# In providers/gemma4.py:
#   if request.system:
#       user_content = f"{request.system}\n\n{request.prompt}"
# The combined string is huge. Maybe that causes a timeout or a malformed JSON?

# Let's try calling Gemma4Provider directly from main, skipping the router if needed.
# Wait, main.py prints [CHYREN] Active providers: gemma4
# So it *should* work.

# I suspect the state snapshot is massive and causes an empty response or timeout for gemma4.

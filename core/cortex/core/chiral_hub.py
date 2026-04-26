from dataclasses import dataclass
from providers.base import ProviderRequest, ProviderResponse, ProviderStatus

@dataclass
class ChiralState:
    intuition_temp: float = 0.95
    logic_temp: float = 0.15

class ChiralRouter:
    def __init__(self, router):
        self.router = router
        self.state = ChiralState()

    def route_chiral(self, prompt: str) -> str:
        # 1. Logical Hemisphere (The Anchor)
        logic_req = ProviderRequest(
            prompt=f"Audit this for logical consistency, factual errors, and hidden assumptions: {prompt}",
            system="You are the Logical Hemisphere. Your goal is grounding. Be cold, analytical, and prioritize truth."
        )
        logic_res = self.router.route(logic_req, preferred="openai")
        print(f"DEBUG: Logic res status={logic_res.status}, error={logic_res.error_message}")
        
        # 2. Intuitive Hemisphere (The Vision)
        intuition_req = ProviderRequest(
            prompt=f"Given this logical audit: {logic_res.text}. Expand on the visionary potential and creative possibilities of this prompt: {prompt}",
            system="You are the Intuitive Hemisphere. Your goal is vision. Be creative, non-linear, and bold."
        )
        intuition_res = self.router.route(intuition_req, preferred="openai")
        
        return f"LOGICAL AUDIT:\n{logic_res.text}\n\nINTUITIVE VISION:\n{intuition_res.text}"

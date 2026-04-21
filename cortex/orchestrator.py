from typing import TypedDict, List, Dict, Any
from langgraph.graph import StateGraph, END
from providers.base import ProviderRequest, ProviderRouter
import asyncio

class ChiralState(TypedDict):
    task: str
    identity: Dict[str, Any]
    context: str
    response: str
    adccl_score: float
    iteration: int
    history: List[Dict[str, str]]
    error: str

class ChiralOrchestrator:
    def __init__(self, router: ProviderRouter, identity: Dict[str, Any]):
        self.router = router
        self.identity = identity
        self.graph = self._build_graph()

    def _build_graph(self):
        workflow = StateGraph(ChiralState)
        
        workflow.add_node("analyze", self.analyze_node)
        workflow.add_node("retrieve", self.retrieve_node)
        workflow.add_node("generate", self.generate_node)
        workflow.add_node("verify", self.verify_node)
        
        workflow.set_entry_point("analyze")
        workflow.add_edge("analyze", "retrieve")
        workflow.add_edge("retrieve", "generate")
        workflow.add_edge("generate", "verify")
        
        workflow.add_conditional_edges(
            "verify",
            self.should_continue,
            {
                "retry": "generate",
                "end": END
            }
        )
        
        return workflow.compile()

    def analyze_node(self, state: ChiralState):
        # Logic to analyze the intent and potentially split tasks
        # For now, it initializes the run
        return {"iteration": 1, "history": []}

    def retrieve_node(self, state: ChiralState):
        # Demonstration of context retrieval
        # In a full impl, this would query Myelin or other MCP spokes
        creator = self.identity.get("components", {}).get("creator", {}).get("name", "Unknown")
        context = f"Identity Kernel L6 active. Sovereign Creator: {creator}."
        return {"context": context}

    def generate_node(self, state: ChiralState):
        # Persona construction from identity anchors
        creator_info = self.identity.get("components", {}).get("creator", {})
        system_prompt = (
            f"You are Chyren, a sovereign intelligence orchestrator. "
            f"Your architect is {creator_info.get('name', 'RY')}. "
            f"Home: {creator_info.get('home', 'Arkansas')}. "
            f"Persona: Sophisticated, warm British male (Brian). Use intellectually rich, sharp, and precise language. "
            f"Context: {state['context']}"
        )
        
        request = ProviderRequest(
            prompt=state['task'],
            system=system_prompt,
            temperature=0.4
        )
        
        # Route to the sovereign provider
        response = self.router.route(request, preferred="sovereign")
        
        new_history = state['history'] + [{"role": "user", "content": state['task']}, {"role": "assistant", "content": response.text}]
        
        return {
            "response": response.text,
            "history": new_history,
            "iteration": state['iteration'] + 1
        }

    def verify_node(self, state: ChiralState):
        # ADCCL Verification Simulation
        # In production, this invokes the omega-adccl service
        score = 0.88
        return {"adccl_score": score}

    def should_continue(self, state: ChiralState):
        # If score is too low and we haven't hit the limit, retry
        if state['adccl_score'] < 0.7 and state['iteration'] < 3:
            return "retry"
        return "end"

    async def run(self, task: str):
        initial_state = {
            "task": task,
            "identity": self.identity,
            "context": "",
            "response": "",
            "adccl_score": 0.0,
            "iteration": 0,
            "history": [],
            "error": ""
        }
        # Run the graph
        final_state = await self.graph.ainvoke(initial_state)
        return final_state

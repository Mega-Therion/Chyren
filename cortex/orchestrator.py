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
    def __init__(self, router: ProviderRouter, identity: Dict[str, Any], mcp_hub: Any = None):
        self.router = router
        self.identity = identity
        self.mcp_hub = mcp_hub
        self.graph = self._build_graph()

    def _build_graph(self):
        workflow = StateGraph(ChiralState)
        
        workflow.add_node("analyze", self.analyze_node)
        workflow.add_node("retrieve", self.retrieve_node)
        workflow.add_node("generate", self.generate_node)
        workflow.add_node("verify", self.verify_node)
        workflow.add_node("record_failure", self.record_failure_node)
        workflow.add_node("execute_tools", self.tool_executor_node)
        
        workflow.set_entry_point("analyze")
        workflow.add_edge("analyze", "retrieve")
        workflow.add_edge("retrieve", "generate")
        
        workflow.add_conditional_edges(
            "generate",
            self.check_for_tools,
            {
                "call": "execute_tools",
                "done": "verify"
            }
        )
        
        workflow.add_edge("execute_tools", "generate")
        
        workflow.add_conditional_edges(
            "verify",
            self.should_continue,
            {
                "retry": "generate",
                "end": END,
                "record": "record_failure"
            }
        )
        
        workflow.add_edge("record_failure", END)
        
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

    async def verify_node(self, state: ChiralState):
        # Live ADCCL Verification call to the Medulla API
        import httpx
        try:
            async with httpx.AsyncClient() as client:
                resp = await client.post(
                    "http://localhost:8080/api/verify",
                    json={"task": state['task'], "response": state['response']},
                    timeout=30.0
                )
                if resp.status_code == 200:
                    data = resp.json()
                    return {"adccl_score": data["score"]}
        except Exception as e:
            print(f"[VERIFY ERROR] {e}")
        
        # Fallback if API is offline
        return {"adccl_score": 0.0}

    def should_continue(self, state: ChiralState):
        # If score is too low and we haven't hit the limit, retry
        if state['adccl_score'] < 0.7:
            if state['iteration'] < 3:
                return "retry"
            else:
                return "record"
        return "end"

    async def record_failure_node(self, state: ChiralState):
        # Notify Medulla about the persistent failure so it can derive lessons
        import httpx
        try:
            async with httpx.AsyncClient() as client:
                await client.post(
                    "http://localhost:8080/api/dream/record",
                    json={
                        "task": state['task'],
                        "response": state['response'],
                        "score": state['adccl_score'],
                        "flags": ["CHIRAL_DRIFT_LIMIT_REACHED"]
                    },
                    timeout=5.0
                )
        except Exception as e:
            print(f"[DREAM ERROR] {e}")
        return {}

    def check_for_tools(self, state: ChiralState):
        if "<tool_call>" in state['response']:
            return "call"
        return "done"

    async def tool_executor_node(self, state: ChiralState):
        if not self.mcp_hub:
            return {"context": state['context'] + "\n[ERROR] MCP Hub not available."}
            
        import json
        import re
        
        response = state['response']
        match = re.search(r"<tool_call>(.*?)</tool_call>", response, re.DOTALL)
        if not match:
            return {}
            
        try:
            call_data = json.loads(match.group(1))
            tool_name = call_data.get("tool")
            arguments = call_data.get("input", {})
            
            # Simple heuristic for now: assume 'memory' server for all tools
            # In a real scenario, we'd map tool names to server names
            result = await self.mcp_hub.call_tool("memory", tool_name, arguments)
            
            tool_output = f"\n<tool_result>{result}</tool_result>\n"
            return {
                "context": state['context'] + tool_output,
                "response": "" # Reset response to trigger fresh generation
            }
        except Exception as e:
            return {"context": state['context'] + f"\n<tool_error>{e}</tool_error>"}

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

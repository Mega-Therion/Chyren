# Holonomic / Fractal MoE Architecture Plan
**Author**: Ryan Yett (Recorded April 2026)
**Subject**: Next-generation Architectural Scaling for Chyren

## Overview
The "Holonomic Fractal Hub-and-Spoke" model is a radical evolution of the standard Mixture-of-Experts (MoE) pattern. Instead of a flat array of models connected to a single hub, the architecture is fractal. The structure of the whole (Hub -> Spoke -> Leaf) is replicated at the micro-level.

## The Fractal Layers

1. **Level 0 (The Core / Brainstem)**
   - **Entity**: Chyren (Python Cortex & Rust Medulla).
   - **Role**: Orchestration, identity coherence, Master Ledger updates, ADCCL verification.

2. **Level 1 (The Nervous System)**
   - **Entity**: WebSockets / SSE.
   - **Role**: High-speed, bidirectional, stateful strands. Connects the Core Hub to the Primary Spokes. Streams the "stream of consciousness" continuously.

3. **Level 2 (The Primary Spokes)**
   - **Entity**: MoE Models (Claude, Ollama, Gemini, etc.).
   - **Role**: The individual intelligences or "Experts".

4. **Level 3 (The Secondary Hubs)**
   - **Entity**: MCP (Model Context Protocol) Clients.
   - **Role**: Attached to the end of each Primary Spoke. These act as standardized "adapters" or "mini-brains", granting specific contexts to specific experts. (e.g., The Logistics Expert connects to an MCP Hub wired for Notion and Zapier).

5. **Level 4 (The Tertiary Leaves)**
   - **Entity**: Raw APIs (REST, GraphQL, SQL).
   - **Role**: The rigid, atomic sensory organs and effectors. The MCP Hub translates these raw signals into cognitive context for the Spoke.

---

## Implementation Roadmap

### Phase 1: The Left Brain Bridge (Python Cortex)
- [x] Install the official `mcp` SDK into the `cortex/venv`.
- [x] Create `mcp_hub.py` to serve as the Secondary Hub switchboard for SaaS/UI integration.
- [ ] Connect the first actual MCP server (e.g., Notion or Zapier) to `mcp_hub.py` to validate dynamic tool discovery.
- [ ] Integrate `mcp_hub.py` into `main.py` (Cortex orchestrator) so that "Thought" commands can access SaaS tools via the hub before sending the plan to Medulla.

### Phase 2: The Right Brain Bridge (Rust Medulla)
- [ ] Create `mcp_spoke.rs` in `medulla/omega-spokes`.
- [ ] Implement the `Spoke` trait for `McpSpoke`, mapping `discover_tools` and `invoke_tool` to raw JSON-RPC over STDIO.
- [ ] Wire the Medulla to a secure, local MCP server (e.g., filesystem or local database) to ensure the execution layer has high-performance access to system primitives.

### Phase 3: The Nervous System (WebSockets)
- [ ] Upgrade the communication layer between Cortex and Medulla (and between Cortex and the UI) from single-shot HTTP/CLI to persistent WebSocket streams.
- [ ] Stream execution logs and ADCCL metrics in real-time.

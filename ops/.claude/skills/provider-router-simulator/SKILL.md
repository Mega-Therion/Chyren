---
name: provider-router-simulator
description: Stress-test routing, latency, quotas, and failover policy across local and cloud providers. Use this skill when asked to "simulate routing", "test failover", or "optimize model selection costs".
---

# Provider Router Simulator

## Purpose
This skill allows OmegA to optimize its substrate selection. It simulates real-world conditions (latency spikes, quota exhaustion, provider outages) to test how the routing logic (e.g., "use Haiku for summaries, use GPT-4 for code") performs under pressure.

## Core Capabilities
1.  **Quota Simulation**: Set token/request limits for each provider and observe the router's behavior when they are hit.
2.  **Latency Benchmarking**: Model the response time distribution for various providers (local Ollama vs. OpenAI vs. Anthropic).
3.  **Failover Logic Testing**: Verify that the system correctly falls back to a secondary provider when the primary fails.
4.  **Cost Optimization**: Calculate the projected monthly cost of a specific routing strategy.

## Workflows

### 1. Quota Exhaustion Simulation
**Trigger**: "What happens if we hit OpenAI rate limits?", "Test quota failover"

1.  **Define Providers**:
    - P1: OpenAI (Limit: 10 RPM)
    - P2: Anthropic (Limit: 50 RPM)
    - P3: Local Ollama (Unlimited)
2.  **Define Strategy**: "Route to P1 first, then P2, then P3."
3.  **Simulate Traffic**: Run a loop of 100 requests.
4.  **Report**: "Request #11 failed on P1, successfully routed to P2."

### 2. Latency-Aware Routing
**Trigger**: "Optimize for speed", "Compare latency between X and Y"

1.  **Gather Stats**: Input average TTFT (Time To First Token) for targets.
2.  **Define Constraints**: "Must respond in < 1s."
3.  **Simulate Selection**: Given a task, which provider would be picked?

## Output Format
Markdown summary of the simulation run.

---

**Resources**
-   `references/provider_metadata.md`: Latency and cost baselines for major providers.

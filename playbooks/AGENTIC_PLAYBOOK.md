# Universal Agentic Playbook: Multi-Agent & Autonomous Patterns

This playbook defines the patterns for high-autonomy agentic behavior, distilled from state-of-the-art frameworks like Auto-GPT, LangGraph, and Agentic-Prompts.

## 1. Pattern: Recursive Self-Correction (Auto-Debug)
**ID**: `PLAY-AGENT-001`
**Description**: The ability to identify errors in one's own execution and fix them without operator intervention.
**Recipe**:
1. Execute a tool call.
2. If the tool returns an error, do NOT report it to the user yet.
3. Pass the error and the original goal into a "Reflection Node".
4. Generate a new strategy based on the error (e.g., "The file was not found, I should search for the correct path first").
5. Execute the corrected strategy.

## 2. Pattern: Role-Based Delegation (Swarm)
**ID**: `PLAY-AGENT-002`
**Description**: Splitting a complex task into multiple specialized roles.
**Recipe**:
1. Analyze the task for sub-domains (e.g., "Research", "Coding", "Review").
2. Instantiate virtual "Sub-Agents" with specialized system prompts (e.g., "You are a Test Engineer...").
3. Assign sub-tasks to each role.
4. Synthesize the outputs into a final verified response.

## 3. Pattern: Dynamic Context Windowing
**ID**: `PLAY-AGENT-003`
**Description**: Managing long-term memory by selectively pruning and summarizing context.
**Recipe**:
1. When context exceeds 80% of the model's limit, trigger a "Summarization Pass".
2. Identify "Anchor Facts" (IDs, keys, requirements) and preserve them.
3. Summarize the intermediate "Chitchat" or "Working Steps".
4. Replace the old context with the `[Anchor Facts + Summary]`.

## 4. Pattern: Verification Gate (Zero-Trust)
**ID**: `PLAY-AGENT-004`
**Description**: Mandatory cross-verification of facts before committing to the Master Ledger.
**Recipe**:
1. Identify any "Fact" claimed in a response.
2. Search internal memory (`Myelin`) or external tools to verify the fact.
3. If unverified, label it as `UNVERIFIED_HEURISTIC`.
4. Only commit nodes with `VERIFIED_TRUTH` status.

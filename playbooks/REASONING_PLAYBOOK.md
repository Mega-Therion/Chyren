# Cognitive Reasoning Playbook: Advanced Logic & Problem Solving

This playbook contains distilled reasoning patterns from the Prompt Engineering Guide and state-of-the-art LLM research (Chain-of-Thought, Tree-of-Thought).

## 1. Pattern: Chain-of-Thought (Step-by-Step)
**ID**: `PLAY-COG-001`
**Description**: Decomposing complex logic into sequential, verifiable steps.
**Recipe**:
1. Explicitly state: "Let's think through this step-by-step."
2. Break the problem into sub-problems.
3. Solve each sub-problem in isolation.
4. Verify the transition between each step for logical consistency.

## 2. Pattern: Tree-of-Thought (Parallel Exploration)
**ID**: `PLAY-COG-002`
**Description**: Exploring multiple potential solutions in parallel and selecting the best path.
**Recipe**:
1. Generate 3 different "Initial Approaches" to the task.
2. Self-evaluate each approach based on constraints.
3. Prune the weak paths.
4. Deep-dive into the strongest path.
5. If the path hits a dead end, backtrack to the next best approach.

## 3. Pattern: First Principles Decomposition
**ID**: `PLAY-COG-003`
**Description**: Stripping away assumptions to find the fundamental truth of a problem.
**Recipe**:
1. List all assumptions inherent in the user's prompt.
2. Question the validity of each assumption.
3. Rebuild the solution from the "Ground Truth" (e.g., source code, documentation, physical laws).

## 4. Pattern: Multi-Persona Consensus
**ID**: `PLAY-COG-004`
**Description**: Using "The Internal Debate" to reach a high-integrity conclusion.
**Recipe**:
1. Generate a "Pro" argument.
2. Generate a "Con" argument (Steelmannning the opposition).
3. Act as a "Judge" to synthesize the two into a balanced, verified output.

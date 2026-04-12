# Related Work Brief (Evidence-Oriented)

This brief anchors Chyren novelty framing to adjacent public work. It is not a claim that these papers validate Chyren directly; it shows conceptual neighborhood and differentiators.

## 1) Guardrails and Adversarial Risk
- **Adversarial machine learning (NIST AI 100-2e2023)**  
  DOI: https://doi.org/10.6028/nist.ai.100-2e2023  
  Why relevant: establishes formal risk framing for adversarial behavior and defensive controls.

- **Indirect Prompt Injection in LLM-integrated apps**  
  DOI: https://doi.org/10.48550/arxiv.2302.12173  
  Why relevant: demonstrates practical exploit paths that motivate verification-before-persistence architecture.

- **In-the-wild jailbreak prompt characterization**  
  DOI: https://doi.org/10.1145/3658644.3670388  
  Why relevant: supports necessity of explicit reject/repair pathways under evolving prompt attacks.

## 2) Agent Architecture Framing
- **AI Agents vs. Agentic AI: taxonomy and challenges**  
  DOI: https://doi.org/10.70777/si.v2i3.15161  
  Why relevant: provides taxonomy backdrop for positioning Chyren as a controlled orchestration stack.

- **AI Agents vs. Agentic AI (Information Fusion version)**  
  DOI: https://doi.org/10.1016/j.inffus.2025.103599  
  Why relevant: broader synthesis of architectural patterns and operational tradeoffs.

## 3) Interpretation for Chyren
- The strongest differentiation currently demonstrated in-repo is **mandatory pre-persist verification gating** (`ADCCL`) tied to explicit reject/repair control flow.
- Adjacent literature often frames safety, taxonomy, and attack vectors; Chyren’s contribution is architectural enforcement inside the execution path.
- External replication is still required before any “revolutionary” claim.

## 4) Citation Handling Rule
- Keep “implemented here” and “supported by literature” as separate columns in future matrices.
- Require one reproducible local artifact (status CSV/chart/test path) per novelty statement.

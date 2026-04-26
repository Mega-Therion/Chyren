# Shadow Workflow Generator

This skill provides an automated "Learning Layer" that analyzes agent interaction history to identify, rank, and recommend high-performance tool/skill sequences.

## Purpose
- Analyze `~/.chyren_history.txt` and `~/.chyren_audit.log` for successful task-completion patterns.
- Rank sequences by efficiency and effectiveness ("teleodynamics" success signal).
- Generate recommended optimized workflows for the user without executing them autonomously.

## Rules
- **Non-Execution:** Never execute generated workflows automatically. These are strictly recommendations.
- **Provenance:** Maintain a direct link back to original history files for all recommendations.
- **Pathing:** All analysis must be performed on `/home/mega/.chyren_history.txt` and `/home/mega/.chyren_audit.log`.
- **Output:** Save all final recommendations to `/home/mega/Chyren-Architecture/reconstruction/shadow_workflows/optimized_chains.yaml`.
- **Safety:** Ensure all log reads are read-only.

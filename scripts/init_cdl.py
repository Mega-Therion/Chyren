import os, json
os.makedirs("state", exist_ok=True)
registry = {
    "registry": {
        "version": "1.0.0",
        "domains": {
            "systems_complexity": {"datasets": ["Open-Systems-Lab/systems-thinking", "cmu-lab/cybernetic-theory"], "weight": 0.8},
            "epistemic_governance": {"datasets": ["legal-nlp/case-law-synthesis", "HuggingFaceH4/constitutional-ai-v2"], "weight": 0.9},
            "scientific_discovery": {"datasets": ["allenai/arxiv-full-text", "paperswithcode/science-discovery-bench"], "weight": 0.7},
            "historical_synthesis": {"datasets": ["wikipedia/multilingual-historiography", "project-gutenberg/classic-synthesis"], "weight": 0.6}
        }
    }
}
with open("state/knowledge_registry.json", "w") as f:
    json.dump(registry, f, indent=2)

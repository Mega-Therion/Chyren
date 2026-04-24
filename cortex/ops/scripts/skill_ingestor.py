#!/usr/bin/env python3
import os
import sys
import json
import hashlib
import argparse
import re
from datetime import datetime
from qdrant_client import QdrantClient
from qdrant_client.models import PointStruct

# ─── Config ──────────────────────────────────────────────────────────────────
COLLECTION = "knowledge_matrix"
PLAYBOOKS_DIR = "/home/mega/Chyren/playbooks"

def get_qdrant():
    url = os.environ.get("QDRANT_URL", "http://localhost:6333")
    return QdrantClient(url=url)

def embed_text(text):
    # Reuse the same embedding logic as hf_pipeline_worker
    # For now, we'll use a simple mock or call the existing embed_text function if imported
    # (In a real system, we'd have a central embedding service)
    from hf_pipeline_worker import embed_text as real_embed
    return real_embed(text)

def stable_int_id(slug):
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def ingest_playbook(file_path):
    print(f"Ingesting playbook: {file_path}")
    with open(file_path, "r") as f:
        content = f.read()

    # Robust parsing: find sections starting with ## followed by Skill or Pattern
    skills = re.split(r'\n## (?:[\d\.]+\s+)?(?:Skill|Pattern):\s*', content)
    
    q_client = get_qdrant()
    points = []
    
    for i, skill_block in enumerate(skills):
        if i == 0: continue # Header
        
        lines = skill_block.strip().split("\n")
        title = lines[0].strip()
        body = "\n".join(lines[1:])
        
        # Determine Kind from original text if possible
        kind = "skill_recipe"
        if "Pattern:" in content.split(skill_block)[0].split("\n")[-1]:
            kind = "orchestration_pattern"
        
        # Extract ID
        skill_id_match = re.search(r'\*\*ID\*\*: `(.*?)`', body)
        skill_id = skill_id_match.group(1) if skill_id_match else f"SKILL-{i}"
        
        print(f"  -> Distilling {kind}: {title} ({skill_id})")
        
        vector = embed_text(f"Skill: {title}\n{body}")
        if not vector: continue
        
        entity = {
            "id": skill_id,
            "name": title,
            "description": body[:500],
            "realm": "internal",
            "kind": "skill_recipe",
            "provenance": {
                "createdAt": datetime.now().isoformat(),
                "createdBy": "R.W.Ϝ.Y.",
                "version": "v1",
                "source": os.path.basename(file_path)
            }
        }
        
        points.append(PointStruct(id=stable_int_id(skill_id), vector=vector, payload=entity))

    if points:
        q_client.upsert(COLLECTION, points=points)
        print(f"✓ Successfully ingested {len(points)} skill recipes.")

if __name__ == "__main__":
    # Add hf_pipeline_worker path to sys.path
    sys.path.append(os.path.dirname(os.path.abspath(__file__)))
    
    for filename in os.listdir(PLAYBOOKS_DIR):
        if filename.endswith(".md"):
            ingest_playbook(os.path.join(PLAYBOOKS_DIR, filename))

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

    # Check for YAML-like frontmatter (Superpowers style)
    frontmatter_match = re.search(r'^---\n(.*?)\n---\n', content, re.DOTALL)
    if frontmatter_match:
        fm_text = frontmatter_match.group(1)
        name_match = re.search(r'name:\s*(.*)', fm_text)
        desc_match = re.search(r'description:\s*(.*)', fm_text)
        if name_match and desc_match:
            name = name_match.group(1).strip()
            desc = desc_match.group(1).strip().strip('"')
            ingest_single_skill(name, desc, content, "external_skill", file_path)
            return

    # Fallback to section-based parsing (Chyren style)
    skills = re.split(r'\n## (?:[\d\.]+\s+)?(?:Skill|Pattern):\s*', content)
    for i, skill_block in enumerate(skills):
        if i == 0: continue # Header
        lines = skill_block.strip().split("\n")
        title = lines[0].strip()
        body = "\n".join(lines[1:])
        kind = "skill_recipe"
        if "Pattern:" in content.split(skill_block)[0].split("\n")[-1]:
            kind = "orchestration_pattern"
        ingest_single_skill(title, body[:500], body, kind, file_path)

def ingest_single_skill(name, description, full_content, kind, file_path):
    q_client = get_qdrant()
    # Extract ID from body if exists
    skill_id_match = re.search(r'\*\*ID\*\*: `(.*?)`', full_content)
    skill_id = skill_id_match.group(1) if skill_id_match else name.lower().replace(" ", "-")
    
    print(f"  -> Distilling {kind}: {name} ({skill_id})")
    vector = embed_text(f"Skill: {name}\n{description}\n{full_content[:1000]}")
    if not vector: return
    
    entity = {
        "id": skill_id,
        "name": name,
        "description": description[:1000],
        "realm": "external" if kind == "external_skill" else "internal",
        "kind": kind,
        "provenance": {
            "createdAt": datetime.now().isoformat(),
            "createdBy": "R.W.Ϝ.Y.",
            "source": os.path.basename(file_path)
        }
    }
    q_client.upsert(COLLECTION, points=[PointStruct(id=stable_int_id(skill_id), vector=vector, payload=entity)])

if __name__ == "__main__":
    # Add hf_pipeline_worker path to sys.path
    sys.path.append(os.path.dirname(os.path.abspath(__file__)))
    
    for root, dirs, files in os.walk(PLAYBOOKS_DIR):
        for filename in files:
            if filename.endswith(".md"):
                ingest_playbook(os.path.join(root, filename))

#!/usr/bin/env python3
import os
import sys
import json
import hashlib
import re
from datetime import datetime
from qdrant_client import QdrantClient
from qdrant_client.models import PointStruct

# Add hf_pipeline_worker path to sys.path
sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "cortex", "ops", "scripts"))
# Try to import embed_text, fallback to mock if needed
try:
    from hf_pipeline_worker import embed_text
except ImportError:
    def embed_text(text):
        print("Warning: hf_pipeline_worker not found, using zero vector mock")
        return [0.0] * 1536 # Assuming 1536 dim (OpenAI)

COLLECTION = "knowledge_matrix"
THEORIES_DIR = "/home/mega/Chyren/knowledge_injection/theories"

def get_qdrant():
    url = os.environ.get("QDRANT_URL", "http://localhost:6333")
    return QdrantClient(url=url)

def stable_int_id(slug):
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def calculate_chiral_resonance(text):
    # Mirroring the Rust implementation to ensure we tag it correctly
    resonance = 0.0
    symbols = ["∀", "∃", "→", "⇒", "⊢", "⊨", "λ", "∫", "∑", "∏", "∆", "∇", "∂", "χ", "Φ", "Ψ", "Ω", "Yettragrammaton"]
    for sym in symbols:
        if sym in text: resonance += 0.08
    if "$" in text or "\\begin{" in text: resonance += 0.2
    chiral_terms = ["chiral invariance", "Jacobian", "Hilbert space", "Stiefel manifold", "homotopy", "Pontryagin", "orthogonal projection", "topological invariant"]
    for term in chiral_terms:
        if term in text.lower(): resonance += 0.12
    if "```" in text: resonance += 0.15
    return min(1.0, resonance)

def ingest_theory(file_path):
    print(f"Ingesting theory: {file_path}")
    with open(file_path, "r") as f:
        content = f.read()

    name_match = re.search(r'^# (.*)', content)
    name = name_match.group(1).strip() if name_match else os.path.basename(file_path)
    
    slug = re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')
    
    resonance = calculate_chiral_resonance(content)
    print(f"  -> Calculated Chiral Resonance: {resonance:.2f}")
    
    vector = embed_text(f"Theory: {name}\n{content[:2000]}")
    if not vector: return

    q_client = get_qdrant()
    
    entity = {
        "id": slug,
        "name": name,
        "description": content[:500] + "...",
        "realm": "theoretical",
        "kind": "theoretical_proposition",
        "chiral_resonance": resonance,
        "provenance": {
            "createdAt": datetime.now().isoformat(),
            "createdBy": "R.W.Ϝ.Y.",
            "source": os.path.basename(file_path),
            "status": "unverified_theory",
            "verification_gate": "ADCCL_PENDING_LEAN4"
        },
        "content": content
    }
    
    q_client.upsert(
        COLLECTION, 
        points=[PointStruct(id=stable_int_id(slug), vector=vector, payload=entity)]
    )
    print(f"  ✓ Theory '{name}' injected into Myelin.")

if __name__ == "__main__":
    if not os.path.exists(THEORIES_DIR):
        os.makedirs(THEORIES_DIR)
        
    files = [f for f in os.listdir(THEORIES_DIR) if f.endswith(".md")]
    if not files:
        print(f"No theory files found in {THEORIES_DIR}")
        sys.exit(0)
        
    for filename in files:
        ingest_theory(os.path.join(THEORIES_DIR, filename))

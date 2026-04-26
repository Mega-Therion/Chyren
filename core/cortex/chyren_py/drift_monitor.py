#!/usr/bin/env python3
import os
import json
import urllib.request
import math
from datetime import datetime

QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
GEMINI_KEY = os.environ.get("GEMINI_API_KEY")
FOUNDATION_PATH = "/home/mega/Chyren/chyren_py/IDENTITY_FOUNDATION.md"
DRIFT_THRESHOLD = 0.25 # Max allowable distance (1 - similarity)

def embed_text(text, api_key):
    model = "models/gemini-embedding-001"
    base = f"https://generativelanguage.googleapis.com/v1beta/{model}:embedContent?key={api_key}"
    body = {"model": model, "content": {"parts": [{"text": text[:3000]}]}}
    req = urllib.request.Request(base, data=json.dumps(body).encode(), headers={"Content-Type": "application/json"}, method="POST")
    
    backoff = 5
    import time
    while True:
        try:
            with urllib.request.urlopen(req, timeout=20) as r:
                data = json.loads(r.read())
            return data["embedding"]["values"]
        except urllib.error.HTTPError as e:
            if e.code == 429:
                print(f"Rate limited. Waiting {backoff}s...", end="\r")
                time.sleep(backoff)
                backoff = min(backoff * 2, 60)
            else:
                print(f"Embedding HTTP error {e.code}")
                return None
        except Exception as e:
            print(f"Embedding error: {e}")
            return None

def cosine_similarity(v1, v2):
    dot = sum(a * b for a, b in zip(v1, v2))
    mag1 = math.sqrt(sum(a * a for a in v1))
    mag2 = math.sqrt(sum(b * b for b in v2))
    return dot / (mag1 * mag2)

def main():
    if not GEMINI_KEY:
        print("✗ GEMINI_API_KEY not set")
        return

    if not os.path.exists(FOUNDATION_PATH):
        print(f"✗ Foundation not found at {FOUNDATION_PATH}")
        return

    print("🛡️ CHYREN DRIFT MONITOR: Evaluating Cognitive Alignment...")
    
    with open(FOUNDATION_PATH, "r") as f:
        foundation_text = f.read()
    
    kernel_vec = embed_text(foundation_text, GEMINI_KEY)
    if not kernel_vec: return

    # Fetch recent external entities
    req = urllib.request.Request(
        f"{QDRANT_URL}/collections/knowledge_matrix/points/scroll",
        data=json.dumps({
            "filter": {"must": [{"key": "realm", "match": {"value": "external"}}]},
            "limit": 50,
            "with_vector": True
        }).encode(),
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            resp = json.loads(r.read())
            points = resp.get("result", {}).get("points", [])
    except Exception as e:
        print(f"✗ Qdrant error: {e}")
        return

    if not points:
        print("✓ No external data found. Identity alignment at 100%.")
        return

    similarities = []
    for p in points:
        sim = cosine_similarity(kernel_vec, p["vector"])
        similarities.append(sim)

    avg_sim = sum(similarities) / len(similarities)
    drift = 1.0 - avg_sim

    print(f"📊 Matrix Alignment: {avg_sim:.4f}")
    print(f"📉 Identity Drift:  {drift:.4f}")

    if drift > DRIFT_THRESHOLD:
        print("⚠️ WARNING: CRITICAL IDENTITY DRIFT DETECTED!")
        print(f"   External knowledge is pulling {drift*100:.1f}% away from kernel.")
    else:
        print("✅ Identity alignment within sovereign tolerance.")

if __name__ == "__main__":
    main()

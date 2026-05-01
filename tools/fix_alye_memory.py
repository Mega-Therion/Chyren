#!/usr/bin/env python3
import os
import json
import hashlib
import urllib.request

# Config
QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
COLLECTION = "knowledge_matrix"
GEMINI_KEY = os.environ.get("GEMINI_API_KEY")
TARGET_SLUG = "alye-lauren-muldoon"

def stable_int_id(slug):
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def embed_gemini(text, api_key):
    model = "models/gemini-embedding-001"
    base = f"https://generativelanguage.googleapis.com/v1beta/{model}:embedContent?key={api_key}"
    body = {"model": model, "content": {"parts": [{"text": text}]}}
    req = urllib.request.Request(base, data=json.dumps(body).encode(), headers={"Content-Type": "application/json"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=20) as r:
            data = json.loads(r.read())
        return data["embedding"]["values"]
    except Exception as e:
        print(f"Embedding error: {e}")
        return None

def main():
    if not GEMINI_KEY:
        print("ERROR: GEMINI_API_KEY not set")
        return

    print(f"Fixing memory for {TARGET_SLUG}...")
    
    content = "Alye Lauren Muldoon is a Travel Nurse (specializing in Labor & Delivery) based in Arkansas."
    vector = embed_gemini(content, GEMINI_KEY)
    
    if not vector:
        print("Failed to get embedding.")
        return

    payload = {
        "id": TARGET_SLUG,
        "name": "Alye Lauren Muldoon",
        "description": content,
        "realm": "people",
        "kind": "person",
        "provenance": {
            "createdAt": "2026-04-20T00:00:00Z",
            "createdBy": "R.W.Ϝ.Y.",
            "version": "v1"
        }
    }

    point = {
        "points": [{
            "id": stable_int_id(TARGET_SLUG),
            "vector": vector,
            "payload": payload
        }]
    }

    req = urllib.request.Request(
        f"{QDRANT_URL}/collections/{COLLECTION}/points",
        data=json.dumps(point).encode(),
        headers={"Content-Type": "application/json"},
        method="PUT"
    )
    
    try:
        with urllib.request.urlopen(req, timeout=10) as r:
            print("✓ Upserted to Qdrant successfully.")
    except Exception as e:
        print(f"Qdrant error: {e}")

if __name__ == "__main__":
    main()

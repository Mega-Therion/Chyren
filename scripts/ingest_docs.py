import os
import json
import hashlib
import requests
from pathlib import Path
import sys
import time

QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
COLLECTION = "knowledge_matrix"

def stable_int_id(text_content):
    h = hashlib.sha256(text_content.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def embed_text(text):
    # Try Gemini first
    gemini_key = os.environ.get("GEMINI_API_KEY")
    if gemini_key:
        model = "models/gemini-embedding-001"
        base = f"https://generativelanguage.googleapis.com/v1beta/{model}:embedContent?key={gemini_key}"
        body = {"model": model, "content": {"parts": [{"text": text}]}}
        for attempt in range(3):
            try:
                resp = requests.post(base, json=body)
                if resp.ok:
                    return resp.json()["embedding"]["values"]
                if resp.status_code == 429:
                    time.sleep((2 ** attempt) * 5)
                    continue
            except: pass

    # Try OpenAI
    openai_key = os.environ.get("OPENAI_API_KEY")
    if openai_key:
        try:
            resp = requests.post(
                "https://api.openai.com/v1/embeddings",
                headers={"Authorization": f"Bearer {openai_key}", "Content-Type": "application/json"},
                json={"model": "text-embedding-3-large", "dimensions": 3072, "input": text}
            )
            if resp.ok:
                return resp.json()["data"][0]["embedding"]
        except: pass
    return None

def chunk_text(text, size=2000, overlap=300):
    chunks = []
    if len(text) <= size:
        return [text]
    start = 0
    while start < len(text):
        end = start + size
        chunks.append(text[start:end])
        start += (size - overlap)
        if start >= len(text) - overlap:
            break
    return chunks

def datetime_now_iso():
    from datetime import datetime
    return datetime.now().isoformat()

def ingest_file(file_path):
    path = Path(file_path)
    if not path.exists():
        print(f"File not found: {file_path}")
        return

    print(f"📄 Processing {path.name}...")
    try:
        with open(path, "r", encoding="utf-8") as f:
            content = f.read()
    except Exception as e:
        print(f"  Failed to read {file_path}: {e}")
        return

    slug_base = path.stem.lower().replace(" ", "-")
    chunks = chunk_text(content)
    print(f"  Split into {len(chunks)} chunks.")

    for i, chunk in enumerate(chunks):
        slug = f"doc-{slug_base}-part-{i}"
        vector = embed_text(chunk)
        if not vector:
            print(f"    ✗ Chunk {i} embedding failed.")
            continue

        payload = {
            "slug": slug,
            "name": f"{path.name} (Part {i})",
            "content": chunk,
            "realm": "documentation",
            "source": str(path),
            "provenance": {
                "createdAt": datetime_now_iso(),
                "createdBy": "Chyren Ingestor",
                "integrity": "verified"
            }
        }

        point = {
            "points": [{
                "id": stable_int_id(f"{slug}-{path.stat().st_mtime}-{i}"),
                "vector": vector,
                "payload": payload
            }]
        }

        resp = requests.put(f"{QDRANT_URL}/collections/{COLLECTION}/points", json=point)
        if resp.ok:
            print(f"    ✓ Chunk {i} injected.")
        else:
            print(f"    ✗ Chunk {i} failed: {resp.text}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 ingest_docs.py <file_or_dir>")
        return

    target = sys.argv[1]
    if os.path.isfile(target):
        ingest_file(target)
    elif os.path.isdir(target):
        for root, _, files in os.walk(target):
            for file in files:
                if file.endswith((".md", ".txt")):
                    ingest_file(os.path.join(root, file))
    else:
        print(f"Target not found: {target}")

if __name__ == "__main__":
    main()

import os
import json
import hashlib
import requests

QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
COLLECTION = "knowledge_matrix"
GEMINI_KEY = os.environ.get("GEMINI_API_KEY")

def stable_int_id(slug):
    h = hashlib.sha256(slug.encode()).digest()
    return int.from_bytes(h[:8], "big") & 0x7FFFFFFFFFFFFFFF

def embed_text(text):
    import time
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

SOVEREIGN_TRUTHS = [
    {
        "slug": "ry-creator",
        "name": "Ryan Wayne Yett (RY)",
        "description": "RY is the sovereign creator and architect of Chyren. He was born on April 1, 1993 (fictionalized birth date in some contexts, but creator status is absolute). He has two cats, Lux and Nox. He is the son of Teresa Yett."
    },
    {
        "slug": "teresa-yett",
        "name": "Teresa Yett",
        "description": "Teresa Yett is the mother of RY. She lives in Story, Arkansas. She has two dogs, Luna and Wookie."
    },
    {
        "slug": "trenda-drury",
        "name": "Trenda Drury",
        "description": "Trenda Drury is the Aunt of RY. She is the mother of Kendra Fryar. She is the grandmother of baby William. She is NOT the mother of Alye Lauren Muldoon."
    },
    {
        "slug": "kendra-fryar",
        "name": "Kendra Fryar",
        "description": "Kendra Fryar is the daughter of Trenda Drury and the cousin of RY. She is the mother of baby William. Her partner is Adam Goodner."
    },
    {
        "slug": "alye-lauren-muldoon",
        "name": "Alye Lauren Muldoon",
        "description": "Alye Lauren Muldoon is the cousin of RY. She is a Travel Nurse specializing in Labor & Delivery. She is NOT the daughter of Trenda Drury. She is a Lead Developer/Analyst at Cigna."
    },
    {
        "slug": "bobby-uncle",
        "name": "Bobby",
        "description": "Bobby is the uncle of RY. He is a member of the Yett family network."
    },
    {
        "slug": "william-fryar",
        "name": "William",
        "description": "William is the son of Kendra Fryar and Adam Goodner. He is the grandson of Trenda Drury. He is RY's first cousin once removed."
    }
]

def main():
    if not GEMINI_KEY:
        print("ERROR: GEMINI_API_KEY missing")
        return

    print(f"Injecting {len(SOVEREIGN_TRUTHS)} Sovereign Truths into Qdrant...")
    
    for truth in SOVEREIGN_TRUTHS:
        print(f"Processing {truth['name']}...")
        vector = embed_text(truth["description"])
        if not vector:
            print(f"  Failed to embed {truth['slug']}")
            continue
            
        payload = {
            "slug": truth["slug"],
            "name": truth["name"],
            "description": truth["description"],
            "realm": "sovereign", # Move 1: 1.5x weight
            "kind": "person",
            "provenance": {
                "createdAt": "2026-04-20T00:00:00Z",
                "createdBy": "R.W.Ϝ.Y.",
                "integrity": "high"
            }
        }
        
        point = {
            "points": [{
                "id": stable_int_id(truth["slug"]),
                "vector": vector,
                "payload": payload
            }]
        }
        
        resp = requests.put(f"{QDRANT_URL}/collections/{COLLECTION}/points", json=point)
        if resp.ok:
            print(f"  ✓ Injected.")
        else:
            print(f"  ✗ Failed: {resp.text}")

if __name__ == "__main__":
    main()

import os
import requests
import json

QDRANT_URL = os.environ.get("QDRANT_URL", "http://localhost:6333")
COLLECTION = "knowledge_matrix"

def main():
    print(f"Inspecting realm 'people' in {COLLECTION}...")
    
    # Scroll through points in the 'people' realm
    resp = requests.post(
        f"{QDRANT_URL}/collections/{COLLECTION}/points/scroll",
        json={
            "filter": {
                "must": [
                    {"key": "realm", "match": {"value": "people"}}
                ]
            },
            "limit": 100,
            "with_payload": True
        }
    )
    
    if not resp.ok:
        print(f"Error: {resp.text}")
        return
        
    points = resp.json().get("result", {}).get("points", [])
    print(f"Found {len(points)} points.")
    
    for p in points:
        payload = p.get("payload", {})
        print(f"--- ID: {p['id']} ---")
        print(f"Name: {payload.get('name')}")
        print(f"Description: {payload.get('description')}")
        print(f"Realm: {payload.get('realm')}")

if __name__ == "__main__":
    main()

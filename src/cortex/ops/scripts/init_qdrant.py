#!/usr/bin/env python3
import os
from qdrant_client import QdrantClient
from qdrant_client.models import VectorParams, Distance

def init_qdrant():
    url = os.environ.get("QDRANT_URL", "http://localhost:6333")
    client = QdrantClient(url=url)
    
    collection_name = "knowledge_matrix"
    vector_size = 3072 # Gemini embedding-001 size
    
    try:
        client.get_collection(collection_name)
        print(f"Collection '{collection_name}' already exists.")
    except Exception:
        print(f"Creating collection '{collection_name}'...")
        client.create_collection(
            collection_name=collection_name,
            vectors_config=VectorParams(size=vector_size, distance=Distance.COSINE),
        )
        print(f"✓ Collection '{collection_name}' created.")

if __name__ == "__main__":
    init_qdrant()

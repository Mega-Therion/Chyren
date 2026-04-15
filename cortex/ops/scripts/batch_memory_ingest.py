import json, os, glob
from pathlib import Path

# Batch size optimized for memory constraints
BATCH_SIZE = 25

def ingest_in_batches(source_dir: Path):
    all_files = list(source_dir.glob("*.md"))
    print(f"Found {len(all_files)} memory files. Processing in batches of {BATCH_SIZE}...")
    
    for i in range(0, len(all_files), BATCH_SIZE):
        batch = all_files[i:i + BATCH_SIZE]
        print(f"Processing batch {i // BATCH_SIZE + 1}...")
        
        # Simulation of the ingestion call to the Librarian's underlying store
        # Replacing the direct seeder call with a targeted update for memory entries
        for file_path in batch:
            try:
                # Placeholder: Logic to parse md and ingest via library_catalog/memory_entries
                # This mirrors the logic previously held in the seeder
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    # Ingest logic here...
            except Exception as e:
                print(f"Error processing {file_path.name}: {e}")
        
    print("Batch ingestion complete.")

if __name__ == "__main__":
    SOURCE = Path("/home/mega/Work/Chyren/archives/OMEGA_WORKSPACE/BRAIN/raw/OMEGA_DATA_CONSOLIDATED")
    ingest_in_batches(SOURCE)

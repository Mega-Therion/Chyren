import json
import sys
import os
from datetime import datetime

def parse_rollout(file_path):
    entries = []
    with open(file_path, 'r') as f:
        for line in f:
            try:
                data = json.loads(line)
                if data.get('type') == 'response_item':
                    payload = data.get('payload', {})
                    if payload.get('type') == 'message':
                        role = payload.get('role')
                        content_list = payload.get('content', [])
                        text = ""
                        for c in content_list:
                            if c.get('type') == 'output_text':
                                text += c.get('text', '')
                        
                        if text:
                            entries.append({
                                "id": f"rollout_{datetime.now().timestamp()}_{len(entries)}",
                                "task": text[:1000], # Task summary
                                "content": text,
                                "source": os.path.basename(file_path),
                                "adccl_score": 0.8, # Assume high quality for rollouts
                                "created_at": data.get('timestamp', datetime.now().isoformat())
                            })
            except: continue
    return entries

def main():
    list_file = '/home/mega/Chyren/data/ledger/priority_ingestion_list.txt'
    all_memory = []
    
    with open(list_file, 'r') as f:
        for line in f:
            path = line.split('\t')[-1].strip()
            if os.path.exists(path):
                print(f"Ingesting: {path}")
                all_memory.extend(parse_rollout(path))
    
    output_path = '/home/mega/Chyren/data/ledger/synthesized_memories.json'
    with open(output_path, 'w') as f:
        json.dump(all_memory, f, indent=2)
    
    print(f"SUCCESS: Synthesized {len(all_memory)} entries into {output_path}")

if __name__ == "__main__":
    main()

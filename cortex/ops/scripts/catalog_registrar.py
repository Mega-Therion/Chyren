import psycopg2
import json
import os

# Load the active shard (The librarian shard)
POOL_PATH = "/home/mega/Chyren/hub/ops/db_pool.json"
with open(POOL_PATH, 'r') as f:
    pool = json.load(f)
    DATABASE_URL = pool["active_primary"]

def register_index_cards():
    print(f"--- REGISTERING LIBRARY INDEX CARDS ---")
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    cards = [
        {
            "shard_id": "primary_sweet_glade",
            "shelf_table": "family_profiles",
            "subject_domain": "identity",
            "subject_domain": "Original family profile data containing biographical facts for RY, Teresa, Jada, Travis, and Alye.",
            "keywords": ["family", "pottsville", "story", "pottsville", "machine_shop"]
        },
        {
            "shard_id": "overflow_little_moon",
            "shelf_table": "family_profiles",
            "subject_domain": "identity_hardened",
            "subject_domain": "Hardened family records containing specific pet names (Lux, Nox, Luna, Wookie) and exotic animal lore (Daisy the deer).",
            "keywords": ["pets", "animals", "horses", "deer", "veterinary"]
        },
        {
            "shard_id": "overflow_little_moon",
            "shelf_table": "omega_memory_entries",
            "subject_domain": "canonical_history",
            "subject_domain": "High-importance memory entries regarding creator academic path (dropout lore) and family history.",
            "keywords": ["college_history", "academic", "sovereign_learning"]
        }
    ]

    for card in cards:
        # Check if card for this shard/table/domain already exists
        cur.execute("""
            SELECT card_id FROM neocortex_library 
            WHERE shard_id = %s AND shelf_table = %s AND subject_domain = %s
        """, (card["shard_id"], card["shelf_table"], card["subject_domain"]))
        
        if not cur.fetchone():
            cur.execute("""
                INSERT INTO neocortex_library (shard_id, shelf_table, subject_domain, keywords)
                VALUES (%s, %s, %s, %s, %s)
            """, (card["shard_id"], card["shelf_table"], card["subject_domain"], card["subject_domain"], json.dumps(card["keywords"])))
            print(f"  Inserted Index Card: {card['subject_domain']} on {card['shard_id']}")

    conn.commit()
    cur.close()
    conn.close()
    print("--- REGISTRATION COMPLETE ---")

if __name__ == "__main__":
    register_index_cards()

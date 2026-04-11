import psycopg2
import json
import os
from datetime import datetime
import uuid

# Load DB URL from the pool registry
POOL_PATH = "/home/mega/Chyren/hub/ops/db_pool.json"
if os.path.exists(POOL_PATH):
    with open(POOL_PATH, 'r') as f:
        pool_data = json.load(f)
        DATABASE_URL = pool_data.get("active_primary")
else:
    # Fallback/Safety
    DATABASE_URL = "postgresql://neondb_owner:npg_HbW1Zlkjd7NI@ep-sweet-glade-anvm0pwn-pooler.c-6.us-east-1.aws.neon.tech/neondb?sslmode=require"

def update_facts(cur, name, last_name, new_facts):
    cur.execute("SELECT fun_facts FROM family_profiles WHERE name = %s AND last_name = %s", (name, last_name))
    res = cur.fetchone()
    facts = []
    if res and res[0]:
        facts = json.loads(res[0])
    
    # Add only if not already present
    for nf in new_facts:
        if nf not in facts:
            facts.append(nf)
    
    cur.execute("UPDATE family_profiles SET fun_facts = %s WHERE name = %s AND last_name = %s", (json.dumps(facts), name, last_name))

def harden_pets():
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    print("--- HARDENING PETS & BIOGRAPHICAL DETAILS ---")

    # 1. Teresa Yett
    print("Updating Teresa's pets...")
    update_facts(cur, 'Teresa', 'Yett', [
        "Has two dogs: Luna (girl dog) and Wookie (boy dog)"
    ])

    # 2. Ryan Wayne Yett (RY)
    print("Updating RY's pets...")
    update_facts(cur, 'Ryan Wayne Yett (RY)', 'Yett', [
        "Has two cats: Lux (white boy cat) and Nox (black girl cat)"
    ])

    # 3. Trenda Drury
    print("Updating Trenda's pets...")
    update_facts(cur, 'Trenda', 'Drury', [
        "Has two dogs: Scooby Doo (boy dog) and Lilly (girl dog)"
    ])

    # 4. Kendra Fryar & Adam Goodner
    print("Updating Kendra & Adam's pets...")
    kendra_adam_pets = ["Has a boy cat named Maxwell and a girl dog named Stella"]
    update_facts(cur, 'Kendra', 'Fryar', kendra_adam_pets)
    update_facts(cur, 'Adam', 'Goodner', kendra_adam_pets)

    # 5. Jada Beechy
    print("Updating Jada's professional and pet details...")
    jada_facts = [
        "Works as a Vet Assistant, often bringing her work home",
        "Home in Story is a 'herd' of animals: horses, inside/outside dogs, wildlife, and livestock",
        "Has cared for everything from feral squirrels to domesticated raccoons and miniature farm animals",
        "Was visited by a wild female deer named Daisy who would come to the house to be pet and fed, and even brought her baby around"
    ]
    update_facts(cur, 'Jada', 'Beechy', jada_facts)
    cur.execute("UPDATE family_profiles SET occupation = 'Professional Realtor & Veterinary Assistant' WHERE name = 'Jada' AND last_name = 'Beechy'")

    # 6. Travis Beechy
    print("Updating Travis's outdoorsman details...")
    travis_facts = [
        "Master of the art of calling animals; has turkey calling down to an exact science",
        "Expert hunter who effectively whispers animals in, explaining the many exotic wildlife pets they've had",
        "Handled and befriended 'Daisy' the wild deer at their home"
    ]
    update_facts(cur, 'Travis', 'Beechy', travis_facts)
    cur.execute("UPDATE family_profiles SET occupation = 'Carpenter, Handyman & Expert Outdoorsman (Master Animal Caller)' WHERE name = 'Travis' AND last_name = 'Beechy'")

    # 7. Inject into omega_memory_entries
    print("Injecting new memories...")
    memories = [
        ("Teresa Yett's dogs are Luna (female) and Wookie (male).", "pet_hardening", 0.95, "canonical"),
        ("RY has two cats: Lux (white male) and Nox (black female).", "pet_hardening", 1.0, "canonical"),
        ("Trenda's dogs are Scooby Doo (male) and Lilly (female).", "pet_hardening", 0.9, "canonical"),
        ("Kendra and Adam have a cat named Maxwell and a dog named Stella.", "pet_hardening", 0.9, "canonical"),
        ("Jada and Travis Beechy live in Story with a massive assortment of animals, including horses, dogs, squirrels, raccoons, and miniature farm animals. Jada is a vet assistant.", "pet_hardening", 0.95, "canonical"),
        ("Travis Beechy is a master at calling animals, especially turkeys. He can call almost any animal in, which is how they ended up with exotic pets like Daisy the wild deer.", "pet_hardening", 0.95, "canonical"),
        ("Daisy was a wild female deer who visited Jada and Travis in Story; she was tame enough to be fed and pet, and eventually brought her own baby deer around.", "pet_hardening", 1.0, "canonical")
    ]

    for content, source, importance, namespace in memories:
        cur.execute("SELECT id FROM omega_memory_entries WHERE content = %s", (content,))
        if not cur.fetchone():
            entry_id = str(uuid.uuid4())
            cur.execute("""
                INSERT INTO omega_memory_entries (id, content, source, importance, namespace, confidence, domain, version, created_at)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)
            """, (entry_id, content, source, importance, namespace, 1.0, "identity", 1, datetime.now().isoformat()))

    conn.commit()
    cur.close()
    conn.close()
    print("--- PET HARDENING COMPLETE ---")

if __name__ == "__main__":
    harden_pets()

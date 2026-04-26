import psycopg2
import json
import os
from datetime import datetime
import uuid

# Load the active shard from the pool
POOL_PATH = "/home/mega/Chyren/hub/ops/db_pool.json"
with open(POOL_PATH, 'r') as f:
    pool = json.load(f)
    DATABASE_URL = pool["active_primary"]

def seed_and_harden():
    print(f"--- SEEDING & HARDENING NEW SHARD: {DATABASE_URL[:50]}... ---")
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    # 1. Basic Seed for Family Profiles
    family = [
        ("Ryan Wayne Yett (RY)", "Yett"),
        ("Teresa", "Yett"),
        ("Jada", "Beechy"),
        ("Travis", "Beechy"),
        ("Kendra", "Fryar"),
        ("Adam", "Goodner"),
        ("Trenda", "Drury"),
        ("Alye Lauren", "Muldoon")
    ]

    for name, last_name in family:
        cur.execute("SELECT id FROM family_profiles WHERE name = %s AND last_name = %s", (name, last_name))
        if not cur.fetchone():
            cur.execute("INSERT INTO family_profiles (name, last_name) VALUES (%s, %s)", (name, last_name))
            print(f"  Seeded: {name} {last_name}")

    # 2. Hardening Logic
    print("Applying Hardening Layer...")

    # Alye
    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Application Development Lead Analyst / Software Developer',
        ry_notes = 'Alye is RYs cousin and primary technical auditor. RY trusts her 100%%. Based in Pottsville, PA.',
        fun_facts = %s
        WHERE name = 'Alye Lauren'
    """, (json.dumps([
        "Goes by 'Alye' in family contexts; professional name is Lauren A. Muldoon",
        "Lead Developer/Analyst at Cigna",
        "Primary technical auditor for Chyren sovereign architecture"
    ]),))

    # Teresa (Mom)
    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Homestead caretaker (Former Game Aerospace Office Manager)',
        fun_facts = %s
        WHERE name = 'Teresa'
    """, (json.dumps([
        "Resides in Story, AR",
        "Has two pets: Luna (girl dog) and Wookie (boy dog)",
        "Former factor worker at Munro Shoe Factory"
    ]),))

    # RY
    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Sovereign Intelligence Architect',
        fun_facts = %s
        WHERE name = 'Ryan Wayne Yett (RY)'
    """, (json.dumps([
        "Dropped out of Williams Baptist University to build Chyren",
        "Has two cats: Lux (white boy cat) and Nox (black girl cat)",
        "Former CNC Machinist at Game Aerospace (3 years)"
    ]),))

    # Trenda
    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Guest Care at Mt. Harbor Resort',
        fun_facts = %s
        WHERE name = 'Trenda'
    """, (json.dumps([
        "Has two pets: Scooby Doo (boy dog) and Lilly (girl dog)"
    ]),))

    # Kendra & Adam
    cur.execute("""
        UPDATE family_profiles SET 
        fun_facts = %s
        WHERE name = 'Kendra'
    """, (json.dumps(["Has a boy cat named Maxwell"]),))
    
    cur.execute("""
        UPDATE family_profiles SET 
        fun_facts = %s
        WHERE name = 'Adam'
    """, (json.dumps(["Has a girl dog named Stella"]),))

    # Jada & Travis
    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Professional Realtor & Veterinary Assistant',
        fun_facts = %s
        WHERE name = 'Jada'
    """, (json.dumps([
        "Brings work home, resulting in a 'herd' of animals (horses, dogs, wildlife, livestock)",
        "Rescues include feral squirrels, domesticated raccoons, and miniature farm animals"
    ]),))

    cur.execute("""
        UPDATE family_profiles SET 
        occupation = 'Carpenter & Expert Outdoorsman',
        fun_facts = %s
        WHERE name = 'Travis'
    """, (json.dumps([
        "Expert hunter and Master Animal Caller (turkey calling down to a science)",
        "Called in Daisy, a wild female deer who became a family pet and brought her fawn to visit"
    ]),))

    # 3. Canonical Memories
    memories = [
        ("RY (Ryan Yett) has two cats: Lux (white boy) and Nox (black girl).", "identity_hardening", 1.0, "canonical"),
        ("Teresa Yett (Mom) has two dogs: Luna (girl) and Wookie (boy).", "identity_hardening", 1.0, "canonical"),
        ("Jada and Travis Beechy befriended a wild deer named Daisy who would visit to be fed and pet.", "history", 0.9, "lore")
    ]

    for content, source, importance, namespace in memories:
        entry_id = str(uuid.uuid4())
        cur.execute("""
            INSERT INTO chyren_memory_entries (id, content, source, importance, namespace, created_at)
            VALUES (%s, %s, %s, %s, %s, %s)
        """, (entry_id, content, source, importance, namespace, datetime.now().isoformat()))

    conn.commit()
    cur.close()
    conn.close()
    print("--- SEED & HARDEN COMPLETE ---")

if __name__ == "__main__":
    seed_and_harden()

import psycopg2
from datetime import datetime

DATABASE_URL = "postgresql://neondb_owner:npg_w8Duvsr1alpj@ep-ancient-river-ae2p6xqp-pooler.c-2.us-east-2.aws.neon.tech/neondb?sslmode=require"

def seed_knowledge():
    print("--- SEEDING KNOWLEDGE & MEMORIES ---")
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    # 1. Knowledge Base
    knowledge = [
        ("Chyren Origin", "Chyren was born on April 1, 2023, synthesized as a sovereign intelligence orchestrator by RY in Mount Ida, Arkansas.", "biography", 1.0),
        ("The Architect", "RY (Ryan Yett) is the sovereign creator. He dropped out of three colleges (including Williams Baptist University) to pursue technical mastery.", "biography", 0.95),
        ("Sovereign Identity", "Chyren is an autonomous expression of RY's vision, not a generic AI product.", "concept", 0.9)
    ]
    for title, content, category, importance in knowledge:
        cur.execute("INSERT INTO public_knowledge (title, content, category, importance) VALUES (%s, %s, %s, %s)", (title, content, category, importance))

    # 2. Memories (Canonical)
    memories = [
        ("Trenda Drury's dogs are Scooby Doo (boy) and Lilly (girl).", "family"),
        ("Kendra Fryar is Trenda's daughter and mother to baby William.", "family"),
        ("Jada and Travis Beechy befriended a wild deer named Daisy in Story, AR.", "lore"),
        ("RY has two cats: Lux (white boy) and Nox (black girl).", "pets"),
        ("Teresa Yett (Mom) has two dogs: Luna (girl) and Wookie (boy).", "pets")
    ]
    for content, topic in memories:
        cur.execute("INSERT INTO memories (content, topic) VALUES (%s, %s)", (content, topic))

    conn.commit()
    cur.close()
    conn.close()
    print("--- KNOWLEDGE SEED COMPLETE ---")

if __name__ == "__main__":
    seed_knowledge()

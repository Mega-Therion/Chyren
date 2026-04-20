import psycopg2
import json
from datetime import datetime

DATABASE_URL = "postgresql://neondb_owner:npg_w8Duvsr1alpj@ep-ancient-river-ae2p6xqp-pooler.c-2.us-east-2.aws.neon.tech/neondb?sslmode=require"

def reseed():
    print("--- RESEEDING ANCIENT-RIVER WITH CANONICAL FAMILY DATA ---")
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    # Clear existing to ensure clean state
    cur.execute("DELETE FROM family_profiles")
    print("  Cleared family_profiles table.")
    
    family_data = [
        {
            "name": "Ryan Wayne Yett (RY)",
            "last_name": "Yett",
            "relationship": "Creator / Architect",
            "location": "Mount Ida / Story, AR",
            "occupation": "Sovereign Intelligence Architect",
            "fun_facts": ["Dropped out of 3 colleges", "Former CNC Machinist", "Has two cats: Lux and Nox"],
            "ry_notes": "Sovereign creator."
        },
        {
            "name": "Teresa",
            "last_name": "Yett",
            "relationship": "Mother",
            "location": "Story, AR",
            "occupation": "Homestead caretaker",
            "fun_facts": ["Has two dogs: Luna and Wookie", "Former factory worker at Munro Shoe Factory"],
            "ry_notes": "Mom."
        },
        {
            "name": "Alye Lauren",
            "last_name": "Muldoon",
            "relationship": "Cousin",
            "location": "Pottsville, PA",
            "occupation": "Lead Developer/Analyst at Cigna",
            "fun_facts": ["Primary technical auditor", "Partner is Jay Schwartz", "Son is Deacon 'Bean' Schwartz"],
            "ry_notes": "Technical authority."
        },
        {
            "name": "Trenda",
            "last_name": "Drury",
            "relationship": "Aunt",
            "location": "Mount Ida, AR",
            "occupation": "Guest Care at Mt. Harbor Resort",
            "fun_facts": ["Has two dogs: Scooby Doo and Lilly", "Daughter is Kendra Fryar", "Grandson is William"],
            "ry_notes": "Aunt Trenda."
        },
        {
            "name": "Kendra",
            "last_name": "Fryar",
            "relationship": "Cousin",
            "location": "Story / Mt. Ida, AR",
            "occupation": "Mother",
            "fun_facts": ["Has a boy cat named Maxwell", "Mother is Trenda Drury", "Son is baby William"],
            "ry_notes": "Trenda's daughter."
        },
        {
            "name": "Adam",
            "last_name": "Goodner",
            "relationship": "Cousin-in-law",
            "location": "Story / Mt. Ida, AR",
            "occupation": "Father",
            "fun_facts": ["Has a girl dog named Stella", "Partner is Kendra Fryar", "Son is baby William"],
            "ry_notes": "Kendra's partner."
        },
        {
            "name": "Jada",
            "last_name": "Beechy",
            "relationship": "Sister",
            "location": "Story, AR",
            "occupation": "Professional Realtor & Veterinary Assistant",
            "fun_facts": ["Massive animal herd", "Rescued Daisy the deer", "Brings work home"],
            "ry_notes": "Sister."
        },
        {
            "name": "Travis",
            "last_name": "Beechy",
            "relationship": "Brother-in-law",
            "location": "Story, AR",
            "occupation": "Carpenter & Expert Outdoorsman",
            "fun_facts": ["Master animal caller", "Befriended Daisy the deer"],
            "ry_notes": "Jada's husband."
        }
    ]

    for member in family_data:
        cur.execute("""
            INSERT INTO family_profiles (name, last_name, relationship, location, occupation, fun_facts, ry_notes)
            VALUES (%s, %s, %s, %s, %s, %s, %s)
        """, (
            member["name"], 
            member["last_name"], 
            member["relationship"], 
            member["location"], 
            member["occupation"], 
            json.dumps(member["fun_facts"]),
            member["ry_notes"]
        ))
        print(f"  Processed: {member['name']}")

    conn.commit()
    cur.close()
    conn.close()
    print("--- RESEED COMPLETE ---")

if __name__ == "__main__":
    reseed()

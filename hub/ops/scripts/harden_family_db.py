#!/usr/bin/env python3
import psycopg2
import json
from datetime import datetime

DATABASE_URL = "postgresql://neondb_owner:npg_HbW1Zlkjd7NI@ep-sweet-glade-anvm0pwn-pooler.c-6.us-east-1.aws.neon.tech/neondb?sslmode=require"

def harden_db():
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()

    print("--- HARDENING FAMILY DATABASE ---")

    # 1. Update Alye Lauren Muldoon
    print("Updating Alye Lauren Muldoon...")
    alye_updates = {
        "occupation": "Application Development Lead Analyst / Software Developer (Cigna, Solar Innovations)",
        "ry_notes": "Alye is RY's cousin and primary technical auditor. RY trusts her 100% to provide honest, constructive criticism. She is incredibly intelligent and her professional input is a top priority for the project architecture. Based in Pottsville, PA.",
        "fun_facts": json.dumps([
            "Goes by 'Alye' in family contexts; professional name is Lauren A. Muldoon",
            "Application Development Lead Analyst and Software Developer",
            "Primary technical auditor for RY's sovereign intelligence projects",
            "Exceptional talent for constructive criticism without condescension",
            "Partner is Jay Schwartz; son is Deacon 'Bean' Schwartz",
            "Member of the Pennsylvania branch of the family"
        ])
    }
    cur.execute("""
        UPDATE family_profiles 
        SET occupation = %s, ry_notes = %s, fun_facts = %s
        WHERE name = 'Alye Lauren' AND last_name = 'Muldoon'
    """, (alye_updates["occupation"], alye_updates["ry_notes"], alye_updates["fun_facts"]))
    print(f"  Result: {cur.rowcount} row(s) updated")

    # 2. Update Ryan Wayne Yett (RY)
    print("Updating Ryan Wayne Yett (RY)...")
    ryan_updates = {
        "fun_facts": json.dumps([
            "Dropped out of three colleges to pursue self-directed mastery",
            "Self-taught Systems Architect and Developer",
            "Learned computer science and Rust through the 'half-court shot backwards' method of high-stakes implementation",
            "Former CNC Machinist at Game Aerospace machine shop (3 years)",
            "Grew up in Avilla, Arkansas; played 2A basketball with NBA aspirations",
            "Primary architect of the OmegA Sovereign Intelligence system"
        ]),
        "ry_notes": "RY is the sovereign creator of OmegA. He is a self-taught architect who operates through highly connected, non-linear systems thinking. He values long-term memory continuity and sovereign intentionality in AI. His background includes a transition from high-school basketball focus to technical mastery via self-directed learning after dropping out of college."
    }
    cur.execute("""
        UPDATE family_profiles 
        SET fun_facts = %s, ry_notes = %s
        WHERE name = 'Ryan Wayne Yett (RY)' AND last_name = 'Yett'
    """, (ryan_updates["fun_facts"], ryan_updates["ry_notes"]))
    print(f"  Result: {cur.rowcount} row(s) updated")

    # 3. Update Teresa Yett
    print("Updating Teresa Yett...")
    # I'll add a placeholder for pets into the fun facts
    cur.execute("SELECT fun_facts FROM family_profiles WHERE name = 'Teresa' AND last_name = 'Yett'")
    res = cur.fetchone()
    if res:
        facts = json.loads(res[0])
        # Check if already has a pet fact
        if not any("pet" in f.lower() for f in facts):
            facts.append("Provides a home for several pets at the family residence in Story")
        
        cur.execute("""
            UPDATE family_profiles 
            SET fun_facts = %s
            WHERE name = 'Teresa' AND last_name = 'Yett'
        """, (json.dumps(facts),))
        print(f"  Result: {cur.rowcount} row(s) updated")

    # 4. Inject into omega_memory_entries (for semantic search retrieval)
    print("Injecting canonical identity into memory store...")
    import uuid
    
    memories = [
        ("RY (Ryan Yett) dropped out of 3 colleges to focus on building sovereign intelligence. One major institution was Williams Baptist University.", "identity_hardening", 1.0, "canonical"),
        ("Alye Lauren Muldoon is a Lead Application Development Analyst and Software Developer. She is the primary technical auditor for Chyren.", "identity_hardening", 1.0, "canonical"),
        ("Teresa Yett (Mom) keeps several pets at the family home in Story, AR. These pets are a significant part of her daily life.", "identity_hardening", 1.0, "canonical")
    ]

    for content, source, importance, namespace in memories:
        # Check if exists
        cur.execute("SELECT id FROM omega_memory_entries WHERE content = %s AND source = %s", (content, source))
        if not cur.fetchone():
            entry_id = str(uuid.uuid4())
            cur.execute("""
                INSERT INTO omega_memory_entries (id, content, source, importance, namespace, confidence, domain, version, created_at)
                VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)
            """, (entry_id, content, source, importance, namespace, 1.0, "identity", 1, datetime.now().isoformat()))
            print(f"  Inserted memory: {content[:50]}...")

    conn.commit()
    cur.close()
    conn.close()
    print("--- HARDENING COMPLETE ---")

if __name__ == "__main__":
    harden_db()

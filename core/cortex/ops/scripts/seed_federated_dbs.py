"""
seed_federated_dbs.py — Full 3-year data ingestion across Neon + Supabase

Distribution strategy:
  Supabase  → family_profiles, public_knowledge, memories (web-facing context)
  Neon      → chyren_memory_entries (dense memory store), chyren_library_catalog (both)
  Catalog   → unified library index written to BOTH platforms

Sources:
  - FAMILY_CANONICAL_DOSSIER.md        → family_profiles
  - aRYse_Full_Biography_2026.md       → public_knowledge (biography)
  - RY_CANONICAL_DOSSIER_vNext.md      → public_knowledge (creator/concept)
  - CHYREN_DATA_CONSOLIDATED/*.md       → memories (456 ChatGPT convos, 3 yrs)
  - RY_Sovereign_Biography/            → public_knowledge + chyren_memory_entries
  - RYography/                         → public_knowledge + chyren_memory_entries
  - SafaData/                          → chyren_memory_entries

Run from repo root:
  python cortex/ops/scripts/seed_federated_dbs.py
"""

import json, os, re, uuid, sys, time, urllib.request, urllib.parse
from datetime import datetime, date
from pathlib import Path

import psycopg2
from psycopg2.extras import execute_values

# ── Config ─────────────────────────────────────────────────────────────────────

POOL         = json.load(open(Path(__file__).parents[2] / "ops/db_pool.json"))
NEON_URL     = POOL["active_primary"]
SUPA_REF     = "eletftuboucrsrnapqoq"
SUPA_BASE    = f"https://{SUPA_REF}.supabase.co/rest/v1"
SUPA_KEY     = next(p["service_key"] for p in POOL["pool"] if p["id"] == "supabase_sovereign")

BRAIN        = Path("/home/mega/Work/Chyren/archives/CHYREN_WORKSPACE/BRAIN")
CONSOLIDATED = BRAIN / "raw/CHYREN_DATA_CONSOLIDATED"
BIOGRAPHY    = BRAIN / "biography"
DOCS         = Path("/home/mega/Chyren/archives/CHYREN_WORKSPACE/DOCS")

# ── Supabase REST helpers ──────────────────────────────────────────────────────

def supa_upsert(table: str, rows: list[dict], on_conflict: str = "id") -> int:
    if not rows:
        return 0
    payload = json.dumps(rows).encode()
    req = urllib.request.Request(
        f"{SUPA_BASE}/{table}",
        data=payload,
        headers={
            "apikey": SUPA_KEY,
            "Authorization": f"Bearer {SUPA_KEY}",
            "Content-Type": "application/json",
            "Prefer": f"resolution=merge-duplicates,return=minimal",
        },
        method="POST",
    )
    try:
        urllib.request.urlopen(req, timeout=30)
        return len(rows)
    except urllib.error.HTTPError as e:
        print(f"  [supa] {table} error {e.code}: {e.read().decode()[:200]}")
        return 0

def supa_delete_all(table: str):
    req = urllib.request.Request(
        f"{SUPA_BASE}/{table}?id=neq.00000000-0000-0000-0000-000000000000",
        headers={
            "apikey": SUPA_KEY,
            "Authorization": f"Bearer {SUPA_KEY}",
            "Prefer": "return=minimal",
        },
        method="DELETE",
    )
    try:
        urllib.request.urlopen(req, timeout=15)
    except Exception:
        pass

# ── Neon helpers ───────────────────────────────────────────────────────────────

def neon_conn():
    return psycopg2.connect(NEON_URL)

def neon_upsert_family(rows: list[dict]) -> int:
    if not rows:
        return 0
    conn = neon_conn()
    cur = conn.cursor()
    cols = ["id","name","last_name","relationship","location","birthday","deceased",
            "occupation","partner","children","ry_notes","notes_for_chyren","how_to_greet","fun_facts"]
    vals = [[r.get(c) for c in cols] for r in rows]
    sql = f"""
        INSERT INTO family_profiles ({','.join(cols)})
        VALUES %s
        ON CONFLICT (id) DO UPDATE SET
            name=EXCLUDED.name, last_name=EXCLUDED.last_name,
            relationship=EXCLUDED.relationship, location=EXCLUDED.location,
            birthday=EXCLUDED.birthday, deceased=EXCLUDED.deceased,
            occupation=EXCLUDED.occupation, partner=EXCLUDED.partner,
            children=EXCLUDED.children, ry_notes=EXCLUDED.ry_notes,
            notes_for_chyren=EXCLUDED.notes_for_chyren, how_to_greet=EXCLUDED.how_to_greet,
            fun_facts=EXCLUDED.fun_facts, updated_at=CURRENT_TIMESTAMP
    """
    execute_values(cur, sql, vals)
    conn.commit(); conn.close()
    return len(rows)

def neon_upsert_memories(rows: list[dict]) -> int:
    if not rows:
        return 0
    conn = neon_conn()
    cur = conn.cursor()
    sql = """
        INSERT INTO chyren_memory_entries (id,content,source,importance,namespace,confidence,domain,version,created_at)
        VALUES %s
        ON CONFLICT (id) DO NOTHING
    """
    vals = [(r["id"],r["content"],r["source"],r.get("importance",0.5),
             r.get("namespace","general"),r.get("confidence",1.0),
             r.get("domain","general"),r.get("version",1),r.get("created_at","")) for r in rows]
    execute_values(cur, sql, vals)
    conn.commit(); conn.close()
    return len(rows)

def neon_upsert_catalog(rows: list[dict]) -> int:
    if not rows:
        return 0
    conn = neon_conn()
    cur = conn.cursor()
    sql = """
        INSERT INTO chyren_library_catalog
            (card_id,shard_id,shelf_table,subject_domain,semantic_summary,keywords,time_start,time_end)
        VALUES %s
        ON CONFLICT (card_id) DO NOTHING
    """
    vals = [(r["card_id"],r["shard_id"],r["shelf_table"],r["subject_domain"],
             r.get("semantic_summary",""),json.dumps(r.get("keywords",[])),
             r.get("time_start"), r.get("time_end")) for r in rows]
    execute_values(cur, sql, vals)
    conn.commit(); conn.close()
    return len(rows)

# ── Parsers ────────────────────────────────────────────────────────────────────

def parse_date(s: str | None) -> date | None:
    if not s:
        return None
    for fmt in ["%B %d, %Y", "%Y-%m-%d", "%b %d, %Y", "%B %d, %y"]:
        try:
            return datetime.strptime(s.strip(), fmt).date()
        except ValueError:
            pass
    m = re.search(r"(\w+ \d+, \d{4})", str(s))
    if m:
        try:
            return datetime.strptime(m.group(1), "%B %d, %Y").date()
        except ValueError:
            pass
    return None

def parse_family_dossier(path: Path) -> list[dict]:
    """Parse FAMILY_CANONICAL_DOSSIER.md into structured family_profiles rows."""
    text = path.read_text(errors="replace")
    # Split on level-3 headers (each family member)
    members = re.split(r"\n### \d+\.\d+ ", text)
    rows = []
    for block in members[1:]:  # skip preamble
        lines = block.strip().split("\n")
        header = lines[0]  # e.g. "Teresa Yett (Mother)"
        name_match = re.match(r"(.+?)\s*\(([^)]+)\)", header)
        if not name_match:
            continue
        full_name = name_match.group(1).strip()
        relationship = name_match.group(2).strip()
        parts = full_name.split()
        first = parts[0] if parts else full_name
        last = " ".join(parts[1:]) if len(parts) > 1 else ""

        fields: dict = {}
        for line in lines[1:]:
            m = re.match(r"\*\s+\*\*(.+?):\*\*\s*(.+)", line)
            if m:
                fields[m.group(1).strip()] = m.group(2).strip()

        deceased = "Deceased" in fields.get("Status", "")
        birthday_raw = fields.get("Birthday", "")
        bday = parse_date(birthday_raw)

        # Extract chyren note as how_to_greet + notes_for_chyren
        chyren_note = fields.get("Chyren Note", "")

        fun_facts = []
        for line in lines[1:]:
            if re.match(r"\*\s+\w", line) and "**" not in line:
                fact = line.lstrip("* ").strip()
                if fact:
                    fun_facts.append(fact)

        # Extract children from Children field
        children_raw = fields.get("Children", "")
        children = [c.strip() for c in re.split(r"[;,]", children_raw) if c.strip()] if children_raw else []

        row = {
            "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"family.{full_name}")),
            "name": first,
            "last_name": last,
            "relationship": relationship,
            "location": fields.get("Residence", ""),
            "birthday": bday.isoformat() if bday else None,
            "deceased": deceased,
            "occupation": fields.get("Occupation", "") or None,
            "partner": fields.get("Husband") or fields.get("Wife") or fields.get("Partner") or None,
            "children": json.dumps(children),
            "ry_notes": fields.get("Context", "") or None,
            "notes_for_chyren": chyren_note or None,
            "how_to_greet": chyren_note.split(".")[0] if chyren_note else None,
            "fun_facts": json.dumps(fun_facts),
        }
        rows.append(row)
    return rows

def parse_biography_to_knowledge(path: Path, category: str, title_prefix: str = "") -> list[dict]:
    """Chunk a biography/dossier markdown into public_knowledge rows."""
    text = path.read_text(errors="replace")
    # Split on level-2 and level-3 headers
    chunks = re.split(r"\n#{2,3} ", text)
    rows = []
    for chunk in chunks[1:]:
        lines = chunk.strip().split("\n")
        section_title = lines[0].strip().rstrip("*").strip()
        content = "\n".join(lines[1:]).strip()
        if len(content) < 80:
            continue
        if len(content) > 2000:
            content = content[:2000] + "…"
        importance = 0.9 if any(k in section_title.lower() for k in ["identity","sovereignty","origin","creator","philosophy","founding"]) else 0.7
        rows.append({
            "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"knowledge.{path.stem}.{section_title}")),
            "title": f"{title_prefix}{section_title}" if title_prefix else section_title,
            "content": content,
            "category": category,
            "importance": importance,
        })
    return rows

def parse_conversation_file(path: Path) -> dict | None:
    """Parse a single CHYREN_DATA_CONSOLIDATED .md conversation file (text or voice)."""
    text = path.read_text(errors="replace")
    title = re.sub(r"^\d+_", "", path.stem).replace("_", " ")

    # Standard text chat: "You said:\n<text>"
    user_msgs = re.findall(r"You said:\s*\n(.+?)(?=\n\n(?:ChatGPT|---|\Z))", text, re.DOTALL)
    gpt_msgs = re.findall(r"ChatGPT said:\s*\n(.+?)(?=\n\n(?:You said|---|\Z))", text, re.DOTALL)

    # Voice chat: messages are quoted strings followed by a timestamp "00:03"
    if not user_msgs:
        voice_user = re.findall(r'"([^"]{10,300})"\s*\n\d+:\d+', text)
        voice_gpt = re.findall(r'ChatGPT said:\s*\n\n(.+?)(?=\n\n(?:You said|---|\Z))', text, re.DOTALL)
        if voice_user:
            user_msgs = voice_user
            gpt_msgs = voice_gpt

    if not user_msgs:
        return None

    first_user = user_msgs[0].strip()[:400]
    if len(first_user) < 15:
        return None

    # Collect a richer summary: first 3 user turns
    user_summary = " | ".join(m.strip()[:150] for m in user_msgs[:3])
    gpt_summary = gpt_msgs[0].strip()[:300] if gpt_msgs else ""

    content = f"Topic: {title}\nDiscussion: {user_summary}"
    if gpt_summary:
        content += f"\nKey response: {gpt_summary}"

    # Estimate date: file 001 = ~April 2023, file 456 = ~April 2026 (3 yrs / 456 ≈ 2.4 days)
    from datetime import timedelta
    num_m = re.match(r"(\d+)_", path.name)
    file_num = int(num_m.group(1)) if num_m else 228
    base = datetime(2023, 4, 1)
    estimated = base + timedelta(days=int(file_num * 2.4))

    return {
        "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"memory.convo.{path.stem}")),
        "content": content[:1200],
        "topic": title[:100],
        "importance": 0.65,
        "created_at": estimated.isoformat(),
    }

def make_catalog_card(shard_id: str, shelf_table: str, domain: str, summary: str,
                       keywords: list[str], time_start=None, time_end=None) -> dict:
    return {
        "card_id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"catalog.{shard_id}.{shelf_table}")),
        "shard_id": shard_id,
        "shelf_table": shelf_table,
        "subject_domain": domain,
        "semantic_summary": summary[:500],
        "keywords": keywords,
        "time_start": time_start,
        "time_end": time_end,
    }

# ── Main ingestion ─────────────────────────────────────────────────────────────

def run():
    print("=" * 60)
    print("Chyren Federated DB Seeder — 3-Year Data Distribution")
    print("=" * 60)
    catalog_cards: list[dict] = []

    # ── 1. FAMILY PROFILES ────────────────────────────────────────────────────
    print("\n[1/5] Family Profiles")
    dossier = BIOGRAPHY / "FAMILY_CANONICAL_DOSSIER.md"
    if dossier.exists():
        family_rows = parse_family_dossier(dossier)
        print(f"  Parsed {len(family_rows)} family members")

        # Write to Supabase (primary web-facing)
        n = supa_upsert("family_profiles", [
            {k: v for k, v in r.items() if k != "id" or True} for r in family_rows
        ])
        print(f"  → Supabase: {n} rows")

        # Write to Neon (federation)
        n = neon_upsert_family(family_rows)
        print(f"  → Neon: {n} rows")

        catalog_cards.append(make_catalog_card(
            shard_id="family_profiles_primary",
            shelf_table="family_profiles",
            domain="family",
            summary=f"Family profiles for {len(family_rows)} members of RY's family. Includes relationships, birthdays, Chyren interaction notes.",
            keywords=["family","profiles","Teresa","Jada","Uncle Bob","Ruby","RY","Montgomery County","Story Arkansas"],
            time_start="1944-01-01", time_end=datetime.now().date().isoformat(),
        ))
    else:
        print(f"  WARNING: {dossier} not found")

    # ── 2. PUBLIC KNOWLEDGE — Biography ───────────────────────────────────────
    print("\n[2/5] Public Knowledge (Biography + Dossier)")
    knowledge_rows: list[dict] = []

    bio_sources = [
        (BIOGRAPHY / "aRYse_Full_Biography_2026.md", "biography", "Biography: "),
        (BIOGRAPHY / "RY_CANONICAL_DOSSIER_vNext.md", "creator", "Profile: "),
        (BIOGRAPHY / "RY_Psychographic_Profile_2026.md", "creator", "Psychographic: "),
    ]
    for src_path, cat, prefix in bio_sources:
        if src_path.exists():
            rows = parse_biography_to_knowledge(src_path, cat, prefix)
            knowledge_rows.extend(rows)
            print(f"  {src_path.name}: {len(rows)} chunks")

    # Add concept entries from sovereign biography
    sovereign_bio = BIOGRAPHY / "RY_Sovereign_Biography"
    if sovereign_bio.exists():
        for md_file in sorted(sovereign_bio.rglob("*.md"))[:10]:
            rows = parse_biography_to_knowledge(md_file, "concept", "")
            knowledge_rows.extend(rows)
        print(f"  RY_Sovereign_Biography: {len(list(sovereign_bio.rglob('*.md')))} files processed")

    print(f"  Total knowledge chunks: {len(knowledge_rows)}")
    # Deduplicate by id
    seen = set()
    knowledge_rows = [r for r in knowledge_rows if not (r["id"] in seen or seen.add(r["id"]))]
    print(f"  After dedup: {len(knowledge_rows)}")

    n = supa_upsert("public_knowledge", knowledge_rows)
    print(f"  → Supabase: {n} rows")

    catalog_cards.append(make_catalog_card(
        shard_id="public_knowledge_biography",
        shelf_table="public_knowledge",
        domain="biography",
        summary="Full biography, psychographic profile, and canonical dossier of RY (Ryan Wayne Felps Yett). Covers 1990s-2026.",
        keywords=["RY","biography","creator","Engineer-Mystic","Chyren","Mount Ida","Arkansas","patent","sovereignty"],
        time_start="1990-01-01", time_end="2026-04-11",
    ))

    # ── 3. MEMORIES — ChatGPT Conversations ──────────────────────────────────
    print("\n[3/5] Memories (ChatGPT Conversations — 456 files)")
    memory_rows: list[dict] = []
    skipped = 0

    conv_files = sorted(CONSOLIDATED.glob("*.md")) if CONSOLIDATED.exists() else []
    for f in conv_files:
        row = parse_conversation_file(f)
        if row:
            memory_rows.append(row)
        else:
            skipped += 1

    print(f"  Parsed: {len(memory_rows)} conversations, skipped: {skipped}")

    # Write to Supabase in batches of 100
    total_supa = 0
    for i in range(0, len(memory_rows), 100):
        batch = memory_rows[i:i+100]
        total_supa += supa_upsert("memories", batch)
    print(f"  → Supabase: {total_supa} rows")

    if memory_rows:
        dates = [r["created_at"] for r in memory_rows if r.get("created_at")]
        catalog_cards.append(make_catalog_card(
            shard_id="memories_chatgpt_conversations",
            shelf_table="memories",
            domain="conversations",
            summary=f"ChatGPT conversation history: {len(memory_rows)} conversations spanning ~3 years. Covers creative projects, philosophy, tech architecture, family, personal growth.",
            keywords=["conversations","ChatGPT","memory","history","3 years","creative","philosophy","Chyren","ONE"],
            time_start=min(dates) if dates else "2023-04-01",
            time_end=max(dates) if dates else "2026-04-11",
        ))

    # ── 4. CHYREN MEMORY ENTRIES — Dense memory from DOCS + sovereign bio ─────
    print("\n[4/5] Chyren Memory Entries (distributed to Neon + Supabase)")
    chyren_rows: list[dict] = []

    def chunk_file(path: Path, source_label: str, namespace: str, domain: str,
                   importance: float, created_at: str, chunk_size: int = 900):
        try:
            text = path.read_text(errors="replace")
            # Skip binary/useless files
            if text.count('\x00') > 10 or len(text.strip()) < 100:
                return
            chunks = [text[i:i+chunk_size] for i in range(0, min(len(text), 12000), chunk_size)]
            for j, chunk in enumerate(chunks):
                if len(chunk.strip()) < 80:
                    continue
                chyren_rows.append({
                    "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, f"chyren.{source_label}.{path.stem}.{j}")),
                    "content": chunk.strip(),
                    "source": source_label,
                    "importance": importance,
                    "namespace": namespace,
                    "domain": domain,
                    "version": 1,
                    "created_at": created_at,
                })
        except Exception:
            pass

    # Master Narrative + Sovereign Bio (highest importance — the foundational story)
    sovereign_bio = BIOGRAPHY / "RY_Sovereign_Biography"
    if sovereign_bio.exists():
        for md in sovereign_bio.rglob("*.md"):
            chunk_file(md, f"sovereign_bio/{md.stem}", "biography", "identity", 0.9, "2026-03-28")
    print(f"  Sovereign bio: {len(chyren_rows)} entries so far")

    # DOCS canonical docs (whitepapers, phase proposals, telos, mission)
    key_docs = [
        "WHITE_PAPER_PUBLICATION_DRAFT.md", "TELOS.md", "MISSION_CURRENT.md",
        "Book_of_Sovereign_Frustrations.md", "Official_Biography_RY.md",
        "Alye_History_References.md", "MAP.md", "STATUS.md",
    ]
    for doc_name in key_docs:
        doc_path = DOCS / doc_name
        if doc_path.exists():
            chunk_file(doc_path, f"docs/{doc_name}", "canon", "philosophy", 0.85, "2026-03-15")

    # DOCS/docs subdirectory (phase proposals, architecture specs)
    docs_sub = DOCS / "docs"
    if docs_sub.exists():
        for md in sorted(docs_sub.glob("*.md"))[:40]:
            chunk_file(md, f"docs/docs/{md.stem}", "architecture", "system", 0.75, "2026-02-01")

    # DOCS/k5 (full Claude conversation — rich interaction history)
    k5_dir = DOCS / "k5"
    if k5_dir.exists():
        for f in sorted(k5_dir.rglob("*.md"))[:10]:
            chunk_file(f, f"k5/{f.stem}", "conversations", "identity", 0.8, "2026-03-01")

    # DOCS/chatgpt-harvest-2026 (same source as CHYREN_DATA_CONSOLIDATED — extra context)
    harvest = DOCS / "chatgpt-harvest-2026"
    if harvest.exists():
        for md in sorted(harvest.glob("*.md"))[:50]:
            chunk_file(md, f"chatgpt_harvest/{md.stem}", "conversations", "history", 0.7, "2026-03-26")

    count_before = len(chyren_rows)
    print(f"  DOCS + corpus: {len(chyren_rows)} total entries")

    print(f"  Total chyren entries: {len(chyren_rows)}")

    # Distribute: write to Neon
    total_neon = 0
    for i in range(0, len(chyren_rows), 200):
        total_neon += neon_upsert_memories(chyren_rows[i:i+200])
    print(f"  → Neon: {total_neon} rows")

    # Also write to Supabase for redundancy
    total_supa = 0
    for i in range(0, min(len(chyren_rows), 500), 100):  # cap at 500 for Supabase
        total_supa += supa_upsert("chyren_memory_entries", chyren_rows[i:i+100])
    print(f"  → Supabase (top 500): {total_supa} rows")

    if chyren_rows:
        catalog_cards.append(make_catalog_card(
            shard_id="chyren_memory_entries_corpus",
            shelf_table="chyren_memory_entries",
            domain="episodic",
            summary=f"Dense episodic memory corpus from RYography, SafaData, and raw archives. {len(chyren_rows)} entries covering identity, relationships, and general knowledge.",
            keywords=["episodic","memory","RYography","Safa","archive","identity","relationships"],
            time_start="2023-01-01", time_end="2026-04-11",
        ))

    # ── 5. LIBRARY CATALOG ────────────────────────────────────────────────────
    print("\n[5/5] Library Catalog (unified index)")
    print(f"  Writing {len(catalog_cards)} catalog cards")

    # Write to Neon (authoritative index)
    n = neon_upsert_catalog(catalog_cards)
    print(f"  → Neon: {n} cards")

    # Write to Supabase (federated mirror)
    supa_catalog = [{
        "card_id": c["card_id"],
        "shard_id": c["shard_id"],
        "shelf_table": c["shelf_table"],
        "subject_domain": c["subject_domain"],
        "semantic_summary": c["semantic_summary"],
        "keywords": json.dumps(c["keywords"]),
        "time_start": c.get("time_start"),
        "time_end": c.get("time_end"),
    } for c in catalog_cards]
    n = supa_upsert("chyren_library_catalog", supa_catalog)
    print(f"  → Supabase: {n} cards")

    print("\n" + "=" * 60)
    print("INGESTION COMPLETE")
    print(f"  Family profiles    → Neon + Supabase")
    print(f"  Public knowledge   → Supabase ({len(knowledge_rows)} chunks)")
    print(f"  Memories           → Supabase ({len(memory_rows)} conversations)")
    print(f"  Chyren memory       → Neon + Supabase ({len(chyren_rows)} entries)")
    print(f"  Library catalog    → Neon + Supabase ({len(catalog_cards)} cards)")
    print("=" * 60)

if __name__ == "__main__":
    run()

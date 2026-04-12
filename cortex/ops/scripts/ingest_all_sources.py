"""
ingest_all_sources.py — Ingest ALL remaining conversation sources into Chyren's memory

Sources handled:
  1. SafaData (15 unique ChatGPT convos not in main chatgptdata)
  2. Claude DOCS/claudedata (1 unique Claude convo)
  3. DeepSeek (17 convos, Dec 2025 – Jan 2026)
  4. Claude processed_2026 (61 markdown exports, Jan–Mar 2026)
  5. Gemini CLI sessions (14 sessions, Apr 2026)

Run from repo root:
  python cortex/ops/scripts/ingest_all_sources.py
"""

import json, uuid, re, glob, urllib.request
from datetime import datetime
from pathlib import Path
import psycopg2
from psycopg2.extras import execute_values

# ── Config ────────────────────────────────────────────────────────────────────
POOL     = json.load(open(Path(__file__).parents[2] / "ops/db_pool.json"))
NEON_URL = POOL["active_primary"]
SUPA_BASE = "https://eletftuboucrsrnapqoq.supabase.co/rest/v1"
SUPA_KEY  = next(p["service_key"] for p in POOL["pool"] if p["id"] == "supabase_sovereign")

BASE      = Path("/home/mega/Work/Chyren/archives/OMEGA_WORKSPACE")
BRAIN     = BASE / "BRAIN"
DOCS      = BASE / "DOCS"
SOV_BIO   = BRAIN / "biography/RY_Sovereign_Biography"

# ── Helpers ───────────────────────────────────────────────────────────────────

def supa_upsert(rows: list[dict]) -> int:
    if not rows:
        return 0
    payload = json.dumps(rows, default=str).encode()
    req = urllib.request.Request(
        f"{SUPA_BASE}/memories",
        data=payload,
        headers={
            "apikey": SUPA_KEY,
            "Authorization": f"Bearer {SUPA_KEY}",
            "Content-Type": "application/json",
            "Prefer": "resolution=merge-duplicates,return=minimal",
        },
        method="POST",
    )
    try:
        urllib.request.urlopen(req, timeout=30)
        return len(rows)
    except urllib.error.HTTPError as e:
        print(f"  [supa] error {e.code}: {e.read().decode()[:150]}")
        return 0


def neon_upsert(rows: list[dict]) -> int:
    if not rows:
        return 0
    conn = psycopg2.connect(NEON_URL)
    cur = conn.cursor()
    execute_values(
        cur,
        "INSERT INTO memories (id,content,topic,importance,created_at) VALUES %s ON CONFLICT (id) DO NOTHING",
        [(r["id"], r["content"], r["topic"], r["importance"], r["created_at"]) for r in rows],
    )
    conn.commit()
    conn.close()
    return len(rows)


def flush(rows: list[dict], label: str):
    n = neon_upsert(rows)
    s = supa_upsert(rows)
    print(f"  → Neon: {n}  Supabase: {s}  [{label}]")


def make_row(uid: str, content: str, topic: str, importance: float, created_at: str) -> dict:
    return {
        "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, uid)),
        "content": content[:1200],
        "topic": topic[:100],
        "importance": importance,
        "created_at": created_at,
    }


def extract_chatgpt_human(c: dict) -> list[str]:
    msgs = []
    for node in c.get("mapping", {}).values():
        msg = node.get("message")
        if not msg:
            continue
        if msg.get("author", {}).get("role") != "user":
            continue
        parts = msg.get("content", {}).get("parts", [])
        text = " ".join(str(p) for p in parts if isinstance(p, str)).strip()
        if len(text) > 20:
            msgs.append(text[:300])
    return msgs


# ── 1. SafaData unique convos ─────────────────────────────────────────────────
def ingest_safadata():
    print("\n[1] SafaData (unique ChatGPT convos)")
    chatgpt_ids = set()
    for f in (BRAIN / "conversations/chatgptdata").glob("conversations-*.json"):
        for c in json.load(open(f)):
            chatgpt_ids.add(c.get("id", "") or c.get("conversation_id", ""))

    safa_path = BRAIN / "conversations/SafaData/conversations.json"
    if not safa_path.exists():
        print("  SKIP: file not found")
        return

    safa = json.load(open(safa_path))
    new_convos = [c for c in safa if (c.get("id", "") or c.get("conversation_id", "")) not in chatgpt_ids]
    print(f"  Found {len(new_convos)} unique convos")

    rows = []
    for c in new_convos:
        created = c.get("create_time", 0)
        try:
            dt = datetime.fromtimestamp(created).isoformat()
        except:
            continue
        title = c.get("title", "Untitled")[:100]
        human = extract_chatgpt_human(c)
        if not human:
            continue
        summary = " | ".join(human[:3])
        rows.append(make_row(
            f"memory.chatgpt.safa.{c.get('id',title)}",
            f"Topic: {title}\nDiscussion: {summary}",
            title, 0.65, dt,
        ))

    flush(rows, "safadata")


# ── 2. Claude DOCS/claudedata ─────────────────────────────────────────────────
def ingest_claude_docsdata():
    print("\n[2] Claude DOCS/claudedata")
    path = DOCS / "claudedata/conversations.json"
    if not path.exists():
        print("  SKIP: file not found")
        return

    existing_uuids = set()
    sov_path = SOV_BIO / "exports/claude/conversations.json"
    if sov_path.exists():
        existing_uuids = {c["uuid"] for c in json.load(open(sov_path))}

    convos = [c for c in json.load(open(path)) if c["uuid"] not in existing_uuids]
    print(f"  Found {len(convos)} unique convos")

    rows = []
    for c in convos:
        dt_str = c.get("created_at", "").replace("Z", "+00:00")
        try:
            dt = datetime.fromisoformat(dt_str).isoformat()
        except:
            continue
        title = c.get("name") or "Claude conversation"
        human_msgs = [m.get("text", "") for m in c.get("chat_messages", [])
                      if m.get("sender") == "human" and len(m.get("text", "")) > 20]
        if not human_msgs:
            continue
        summary = " | ".join(m[:250] for m in human_msgs[:3])
        rows.append(make_row(
            f"memory.claude.docs.{c['uuid']}",
            f"Topic: {title}\nDiscussion: {summary}",
            title, 0.7, dt,
        ))

    flush(rows, "claude_docsdata")


# ── 3. DeepSeek convos ────────────────────────────────────────────────────────
def ingest_deepseek():
    print("\n[3] DeepSeek conversations")
    path = SOV_BIO / "exports/hidden_finds/deepseek_discovery/conversations.json"
    if not path.exists():
        print("  SKIP: file not found")
        return

    convos = json.load(open(path))
    print(f"  Found {len(convos)} convos")

    rows = []
    for c in convos:
        dt_str = c.get("inserted_at", "").replace("Z", "+00:00")
        try:
            dt = datetime.fromisoformat(dt_str).isoformat()
        except:
            dt = "2025-12-15T00:00:00"

        title = c.get("title", "DeepSeek conversation")[:100]

        # DeepSeek uses same mapping structure as ChatGPT
        human_msgs = []
        for node in c.get("mapping", {}).values():
            msg = node.get("message") if isinstance(node, dict) else None
            if not msg:
                continue
            role = msg.get("role", "") or (msg.get("author", {}).get("role", "") if isinstance(msg.get("author"), dict) else "")
            if role != "user":
                continue
            content = msg.get("content", "")
            if isinstance(content, str) and len(content) > 20:
                human_msgs.append(content[:300])
            elif isinstance(content, dict):
                parts = content.get("parts", [])
                text = " ".join(str(p) for p in parts if isinstance(p, str)).strip()
                if len(text) > 20:
                    human_msgs.append(text[:300])

        if not human_msgs:
            # Try flat message list
            for msg in c.get("messages", []):
                if msg.get("role") == "user":
                    text = msg.get("content", "")
                    if isinstance(text, str) and len(text) > 20:
                        human_msgs.append(text[:300])

        if not human_msgs:
            continue

        summary = " | ".join(human_msgs[:3])
        rows.append(make_row(
            f"memory.deepseek.{c.get('id', title)}",
            f"Topic: {title}\nDiscussion: {summary}",
            title, 0.7, dt,
        ))

    flush(rows, "deepseek")


# ── 4. Claude processed_2026 markdowns ────────────────────────────────────────
def ingest_processed_2026():
    print("\n[4] Claude processed_2026 markdown exports")
    proc_dir = SOV_BIO / "exports/claude/processed_2026"
    if not proc_dir.exists():
        print("  SKIP: directory not found")
        return

    md_files = sorted(proc_dir.glob("*.md"))
    print(f"  Found {len(md_files)} markdown files")

    rows = []
    for md in md_files:
        # Filename: 2026-01-07_Mobile_App_for_OMEGAI.md
        stem = md.stem
        date_match = re.match(r"(\d{4}-\d{2}-\d{2})", stem)
        dt = date_match.group(1) + "T00:00:00" if date_match else "2026-01-01T00:00:00"
        title = re.sub(r"^\d{4}-\d{2}-\d{2}_", "", stem).replace("_", " ")[:100]

        try:
            text = md.read_text(errors="replace")
            if len(text.strip()) < 50:
                continue
        except:
            continue

        # Extract human/user lines - look for common markdown chat patterns
        human_lines = []
        # Pattern: **User:** or **Human:** or lines after "---" headers with content
        for line in text.split("\n"):
            line = line.strip()
            if re.match(r"\*\*(User|Human|You|RY)\*\*:?\s*(.+)", line, re.I):
                m = re.match(r"\*\*(User|Human|You|RY)\*\*:?\s*(.+)", line, re.I)
                if m and len(m.group(2)) > 15:
                    human_lines.append(m.group(2)[:250])
            elif re.match(r"^> (.{20,})", line):
                # Blockquotes are often user messages in exported chats
                m = re.match(r"^> (.+)", line)
                if m:
                    human_lines.append(m.group(1)[:250])

        if human_lines:
            summary = " | ".join(human_lines[:3])
        else:
            # Fall back to first meaningful paragraph
            paras = [p.strip() for p in text.split("\n\n") if len(p.strip()) > 50]
            summary = paras[0][:400] if paras else text[:400]

        rows.append(make_row(
            f"memory.claude.proc2026.{md.stem}",
            f"Topic: {title}\nDiscussion: {summary}",
            title, 0.7, dt,
        ))

    flush(rows, "processed_2026")


# ── 5. Gemini CLI sessions ────────────────────────────────────────────────────
def ingest_gemini_cli():
    print("\n[5] Gemini CLI sessions")
    sessions = list(Path("/home/mega/.gemini/tmp").rglob("session-*.json"))
    print(f"  Found {len(sessions)} sessions")

    rows = []
    for s in sessions:
        try:
            d = json.load(open(s))
        except:
            continue

        dt_str = d.get("startTime", "").replace("Z", "+00:00")
        try:
            dt = datetime.fromisoformat(dt_str).isoformat()
        except:
            dt = "2026-04-07T00:00:00"

        session_id = d.get("sessionId", s.stem)
        msgs = d.get("messages", [])
        human_msgs = [m["content"][0]["text"] for m in msgs
                      if m.get("type") == "user"
                      and m.get("content")
                      and isinstance(m["content"], list)
                      and m["content"][0].get("text", "")
                      and len(m["content"][0]["text"]) > 20]

        if not human_msgs:
            continue

        # Group into one memory per session
        summary = " | ".join(human_msgs[:5])
        title = f"Gemini CLI session {dt[:10]}"
        rows.append(make_row(
            f"memory.gemini.cli.{session_id}",
            f"Topic: {title}\nDiscussion: {summary}",
            title, 0.6, dt,
        ))

    flush(rows, "gemini_cli")


# ── Main ──────────────────────────────────────────────────────────────────────
if __name__ == "__main__":
    print("=" * 55)
    print("Chyren — Full Source Ingestion")
    print("=" * 55)

    ingest_safadata()
    ingest_claude_docsdata()
    ingest_deepseek()
    ingest_processed_2026()
    ingest_gemini_cli()

    # Final counts
    conn = psycopg2.connect(NEON_URL)
    cur = conn.cursor()
    cur.execute("SELECT COUNT(*) FROM memories")
    total = cur.fetchone()[0]
    conn.close()
    print(f"\n{'='*55}")
    print(f"Neon memories total: {total}")
    print("Done.")

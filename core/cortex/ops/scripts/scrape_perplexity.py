"""
scrape_perplexity.py — Scrape last 10 Perplexity threads using your Chrome profile.

Opens a visible Chrome window with your existing login session so no credentials needed.
Saves threads to /tmp/perplexity_threads/ as JSON + ingest into Chyren memories.

Run:
  python3 cortex/ops/scripts/scrape_perplexity.py
"""

import json, uuid, time, urllib.request
from pathlib import Path
from datetime import datetime
import psycopg2
from psycopg2.extras import execute_values
from playwright.sync_api import sync_playwright

CHROME_PROFILE = Path.home() / ".config/google-chrome"
OUT_DIR = Path("/tmp/perplexity_threads")
OUT_DIR.mkdir(exist_ok=True)

POOL     = json.load(open(Path(__file__).parents[2] / "ops/db_pool.json"))
NEON_URL = POOL["active_primary"]
SUPA_BASE = "https://eletftuboucrsrnapqoq.supabase.co/rest/v1"
SUPA_KEY  = next(p["service_key"] for p in POOL["pool"] if p["id"] == "supabase_sovereign")


def supa_upsert(rows):
    if not rows: return 0
    payload = json.dumps(rows, default=str).encode()
    req = urllib.request.Request(
        f"{SUPA_BASE}/memories", data=payload,
        headers={"apikey": SUPA_KEY, "Authorization": f"Bearer {SUPA_KEY}",
                 "Content-Type": "application/json",
                 "Prefer": "resolution=merge-duplicates,return=minimal"},
        method="POST")
    try:
        urllib.request.urlopen(req, timeout=30)
        return len(rows)
    except urllib.error.HTTPError as e:
        print(f"  [supa] {e.code}: {e.read().decode()[:100]}")
        return 0


def neon_upsert(rows):
    if not rows: return 0
    conn = psycopg2.connect(NEON_URL)
    cur = conn.cursor()
    execute_values(cur,
        "INSERT INTO memories (id,content,topic,importance,created_at) VALUES %s ON CONFLICT (id) DO NOTHING",
        [(r["id"], r["content"], r["topic"], r["importance"], r["created_at"]) for r in rows])
    conn.commit(); conn.close()
    return len(rows)


def make_row(uid, content, topic, importance, created_at):
    return {
        "id": str(uuid.uuid5(uuid.NAMESPACE_DNS, uid)),
        "content": content[:1200],
        "topic": topic[:100],
        "importance": importance,
        "created_at": created_at,
    }


def scrape():
    threads = []

    with sync_playwright() as p:
        # Use Firefox session cookies (Perplexity is logged in there)
        import browser_cookie3, tempfile
        ff_cookies = list(browser_cookie3.firefox(domain_name='perplexity.ai'))
        print(f"  Loaded {len(ff_cookies)} Perplexity cookies from Firefox")

        tmp_profile = Path(tempfile.mkdtemp(prefix="pw_perp_"))
        print("Launching Playwright Chromium with Firefox cookies injected...")
        browser = p.chromium.launch_persistent_context(
            user_data_dir=str(tmp_profile),
            headless=False,
            args=["--no-sandbox"],
            ignore_https_errors=True,
        )
        page = browser.new_page()

        # Inject Firefox cookies into Playwright context
        pw_cookies = []
        for c in ff_cookies:
            cookie = {
                "name": c.name,
                "value": c.value,
                "domain": c.domain if c.domain.startswith(".") else f".{c.domain}",
                "path": c.path or "/",
                "secure": bool(c.secure),
                "httpOnly": False,
                "sameSite": "Lax",
            }
            exp = int(c.expires) if c.expires else 0
            # Firefox stores expiry in ms; Playwright wants seconds
            if exp > 1_000_000_000_000:
                exp = exp // 1000
            cookie["expires"] = exp if exp > 0 else -1
            pw_cookies.append(cookie)
        browser.add_cookies(pw_cookies)
        print(f"  Injected {len(pw_cookies)} cookies")

        print("Navigating to Perplexity library...")
        page.goto("https://www.perplexity.ai/library", wait_until="domcontentloaded", timeout=30000)
        time.sleep(3)

        print(f"Page title: {page.title()}")
        page.screenshot(path="/tmp/perplexity_library.png")
        print("Screenshot saved: /tmp/perplexity_library.png")

        # Find thread links — Perplexity library lists threads as anchor tags
        page.wait_for_selector("a[href*='/search/']", timeout=15000)
        links = page.query_selector_all("a[href*='/search/']")
        thread_urls = []
        seen = set()
        for link in links:
            href = link.get_attribute("href")
            if href and href not in seen:
                seen.add(href)
                full = f"https://www.perplexity.ai{href}" if href.startswith("/") else href
                title_el = link.inner_text().strip()[:120]
                thread_urls.append((full, title_el))
            if len(thread_urls) >= 10:
                break

        print(f"Found {len(thread_urls)} thread links")
        for url, title in thread_urls:
            print(f"  - {title[:60]} | {url}")

        # Visit each thread and scrape content
        for i, (url, title) in enumerate(thread_urls):
            print(f"\n[{i+1}/{len(thread_urls)}] Scraping: {title[:60]}")
            try:
                page.goto(url, wait_until="domcontentloaded", timeout=20000)
                time.sleep(2)

                # Extract user queries (the questions you asked)
                user_msgs = []
                for sel in ["[data-testid='user-message']", ".user-message", "h1", "h2"]:
                    els = page.query_selector_all(sel)
                    for el in els:
                        txt = el.inner_text().strip()
                        if len(txt) > 15:
                            user_msgs.append(txt[:400])
                    if user_msgs:
                        break

                # Extract AI answers too (for context)
                ai_texts = []
                for sel in ["[data-testid='answer']", ".prose", ".answer-content"]:
                    els = page.query_selector_all(sel)
                    for el in els[:3]:
                        txt = el.inner_text().strip()
                        if len(txt) > 50:
                            ai_texts.append(txt[:600])
                    if ai_texts:
                        break

                thread_data = {
                    "url": url,
                    "title": title,
                    "user_messages": user_msgs,
                    "ai_snippets": ai_texts,
                    "scraped_at": datetime.now().isoformat(),
                }
                threads.append(thread_data)

                # Save raw JSON
                safe_title = "".join(c if c.isalnum() or c in "- _" else "_" for c in title[:40])
                out_path = OUT_DIR / f"{i+1:02d}_{safe_title}.json"
                out_path.write_text(json.dumps(thread_data, indent=2))
                print(f"  Saved → {out_path}")
                print(f"  User msgs: {len(user_msgs)}  AI snippets: {len(ai_texts)}")

            except Exception as e:
                print(f"  ERROR: {e}")

        browser.close()

    return threads


def ingest(threads):
    rows = []
    for t in threads:
        title = t.get("title", "Perplexity thread")[:100]
        user_msgs = t.get("user_messages", [])
        if not user_msgs:
            continue
        summary = " | ".join(user_msgs[:3])
        dt = t.get("scraped_at", datetime.now().isoformat())[:19]
        uid = f"memory.perplexity.{t['url']}"
        rows.append(make_row(uid, f"Topic: {title}\nDiscussion: {summary}", title, 0.7, dt))

    if rows:
        n = neon_upsert(rows)
        s = supa_upsert(rows)
        print(f"\nIngested → Neon: {n}  Supabase: {s}")
    else:
        print("\nNo rows to ingest (no user messages found)")


if __name__ == "__main__":
    print("=" * 55)
    print("Perplexity Thread Scraper")
    print("=" * 55)
    threads = scrape()
    print(f"\nScraped {len(threads)} threads total")
    ingest(threads)
    print("Done. Raw files in /tmp/perplexity_threads/")

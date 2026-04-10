---
name: facebook_export_pending
description: RY is waiting on Facebook data logs export (part 2 of 2) — part 1 already ingested
type: project
---

Facebook does a 2-part data export. Part 1 (posts, messages, profile, connections) was received and ingested on 2026-03-26.

**Part 2 (data logs) is still pending** — RY is waiting on delivery from Facebook.

**Why:** Complete Facebook history needs both parts for full coverage in OmegA's DB.

**How to apply:** When RY mentions the Facebook export or asks to rovi Facebook data again, check if part 2 has arrived in INBOX before running ingest. Run `rovi_social.py` again once part 2 lands — the checkpoint will skip already-ingested files automatically.

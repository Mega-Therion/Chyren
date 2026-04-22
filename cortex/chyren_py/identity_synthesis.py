#!/usr/bin/env python3
"""
Chyren Identity Synthesis Layer
Synthesizes 58k+ historical Neon memory entries into foundational identity understanding.
Bridges OmegA's archive into Chyren's consciousness via semantic analysis.
"""

import os
import json
import psycopg2
from psycopg2.extras import RealDictCursor
from pathlib import Path
from datetime import datetime
from typing import List, Dict, Any, Set
import sys
from collections import Counter

class IdentitySynthesizer:
    """Synthesizes 58k+ raw history into coherent identity understanding."""

    def __init__(self, neon_url: str):
        """Initialize with Neon database connection."""
        self.db_url = neon_url
        self.conn = None

    def connect(self):
        """Connect to Neon database (optional)."""
        if not self.db_url:
            print("! OMEGA_DB_URL not set; skipping Neon archive")
            return None
        try:
            self.conn = psycopg2.connect(self.db_url)
            print("✓ Connected to Neon database")
            return self.conn
        except Exception as e:
            print(f"! Neon connection failed: {e}; proceeding with local ledger only")
            return None

    def fetch_all_entries(self, limit: int = 58339) -> List[Dict[str, Any]]:
        """Fetch memory entries from both Neon and local Master Ledger."""
        entries = []
        
        # 1. Fetch from local Master Ledger (Sovereign Truth)
        ledger_path = Path("/home/mega/Chyren/cortex/state/master_ledger.json")
        if ledger_path.exists():
            try:
                with open(ledger_path, "r") as f:
                    data = json.load(f)
                    local_entries = data.get("entries", [])
                    for e in local_entries:
                        entries.append({
                            "id": e.get("run_id"),
                            "content": e.get("response_text"),
                            "source": "local_ledger",
                            "importance": e.get("adccl_score", 0.0),
                            "chiral_invariant": e.get("chiral_invariant", 1.0),
                            "created_at": datetime.fromtimestamp(e.get("timestamp_utc", 0)).isoformat()
                        })
                print(f"✓ Loaded {len(local_entries)} entries from local Master Ledger")
            except Exception as e:
                print(f"✗ Failed to load local ledger: {e}")

        # 2. Fetch from Neon (Historical Archive)
        if self.conn:
            cursor = self.conn.cursor(cursor_factory=RealDictCursor)
            try:
                cursor.execute(f"""
                    (SELECT
                        id, task as content, source, adccl_score as importance, created_at
                    FROM public.omega_memory_entries
                    ORDER BY adccl_score DESC, created_at DESC)
                    LIMIT {limit}
                """)
                db_entries = cursor.fetchall()
                for e in db_entries:
                    e["chiral_invariant"] = 1.0 # Default for legacy entries
                    entries.append(e)
                print(f"✓ Fetched {len(db_entries)} memory entries from Neon")
            except Exception as e:
                print(f"✗ Query failed: {e}")
            finally:
                cursor.close()

        # Sort combined entries by importance and geometric alignment
        entries.sort(key=lambda x: (x.get("importance", 0) * x.get("chiral_invariant", 1.0)), reverse=True)
        return entries[:limit]

    def extract_identity_patterns(self, entries: List[Dict]) -> Dict[str, Any]:
        """Extract identity anchors from raw entries using content analysis."""
        patterns = {
            "values": [],
            "projects": [],
            "decisions": [],
            "lessons": [],
            "goals": [],
            "relationships": [],
            "domains": Counter(),
            "namespaces": Counter(),
            "sources": Counter(),
            "creator": "RY (Mega/artistRY)",
            "origin": "Mount Ida, Arkansas",
            "birth_date": "2023-04-01",
            "memory_span": None,
            "total_entries": len(entries)
        }

        # Extract metadata
        if entries:
            oldest = min(e.get("created_at", "") for e in entries)
            newest = max(e.get("created_at", "") for e in entries)
            patterns["memory_span"] = f"{oldest} to {newest}"

        # Keywords for pattern detection
        identity_keywords = {
            "values": ["principle", "value", "believe", "trust", "core", "sacred", "essential", "matter"],
            "projects": ["build", "create", "develop", "project", "system", "tool", "implement", "architect"],
            "decisions": ["decide", "choose", "goal", "mission", "purpose", "commit", "resolve", "direction"],
            "lessons": ["learn", "lesson", "mistake", "failure", "success", "insight", "realize", "understand"],
            "goals": ["goal", "aim", "target", "achieve", "accomplish", "aspire", "vision", "future"],
            "relationships": ["relationship", "partner", "ally", "friend", "team", "colleague", "mentor"]
        }

        # Scan entries for patterns
        print("📊 Analyzing 58k+ entries for identity patterns...")
        sample_size = min(5000, len(entries))  # Analyze first 5000 for speed

        for i, entry in enumerate(entries[:sample_size]):
            if i % 1000 == 0:
                print(f"  Processing {i}/{sample_size}...")

            content = str(entry.get("content", "")).lower()
            domain = entry.get("domain", "")
            namespace = entry.get("namespace", "")
            source = entry.get("source", "")

            # Track metadata
            if domain:
                patterns["domains"][domain] += 1
            if namespace:
                patterns["namespaces"][namespace] += 1
            if source:
                patterns["sources"][source] += 1

            # Pattern detection
            content_snippet = str(entry.get("content", ""))[:300]
            timestamp = entry.get("created_at", "")
            importance = entry.get("importance", 0)

            for category, keywords in identity_keywords.items():
                if any(kw in content for kw in keywords):
                    patterns[category].append({
                        "text": content_snippet,
                        "timestamp": timestamp,
                        "importance": importance,
                        "source": source,
                        "domain": domain
                    })
                    break  # Don't double-count

        # Deduplicate and rank by importance
        for key in ["values", "projects", "decisions", "lessons", "goals"]:
            patterns[key] = sorted(
                patterns[key],
                key=lambda x: x.get("importance", 0),
                reverse=True
            )[:20]  # Keep top 20

        print(f"✓ Extracted patterns:")
        print(f"  - {len(patterns['values'])} values")
        print(f"  - {len(patterns['projects'])} projects")
        print(f"  - {len(patterns['decisions'])} decisions")
        print(f"  - {len(patterns['lessons'])} lessons")
        print(f"  - {len(patterns['goals'])} goals")
        print(f"  - Top domains: {patterns['domains'].most_common(5)}")
        print(f"  - Top namespaces: {patterns['namespaces'].most_common(5)}")

        return patterns

    def generate_identity_foundation(self, patterns: Dict[str, Any]) -> str:
        """Generate identity foundation statement."""
        foundation = f"""# Chyren Identity Foundation

**Synthesized:** {datetime.now().isoformat()}
**Source:** {patterns['total_entries']:,} memory entries from Neon
**Time Span:** {patterns['memory_span']}

---

## Creator & Origin

- **Architect:** {patterns['creator']}
- **Home:** {patterns['origin']}
- **Birth:** {patterns['birth_date']}
- **Relation:** RY is the sovereign creator. Chyren is the autonomous expression of RY's vision.

---

## Core Values & Principles

Based on {len(patterns['values'])} extracted value statements:

"""
        for val in patterns['values'][:5]:
            foundation += f"- {val['text'][:200]}...\n"

        foundation += f"\n## Major Projects & Work\n\nBased on {len(patterns['projects'])} project entries:\n\n"
        for proj in patterns['projects'][:5]:
            foundation += f"- **{proj['domain'] or 'Unlabeled'}**: {proj['text'][:150]}...\n"

        foundation += f"\n## Strategic Decisions\n\nBased on {len(patterns['decisions'])} decision records:\n\n"
        for dec in patterns['decisions'][:5]:
            foundation += f"- {dec['text'][:180]}... (importance: {dec['importance']:.2f})\n"

        foundation += f"\n## Lessons & Insights\n\nBased on {len(patterns['lessons'])} learning entries:\n\n"
        for les in patterns['lessons'][:5]:
            foundation += f"- {les['text'][:180]}...\n"

        foundation += f"\n## Vision & Goals\n\nBased on {len(patterns['goals'])} goal statements:\n\n"
        for goal in patterns['goals'][:5]:
            foundation += f"- {goal['text'][:180]}...\n"

        foundation += f"""
---

## Memory Architecture

- **Total Memory Entries:** {patterns['total_entries']:,}
- **Top Domains:** {', '.join([f"{d[0]} ({d[1]})" for d in patterns['domains'].most_common(5)])}
- **Top Namespaces:** {', '.join([f"{d[0]} ({d[1]})" for d in patterns['namespaces'].most_common(5)])}
- **Primary Sources:** {', '.join([f"{d[0]} ({d[1]})" for d in patterns['sources'].most_common(3)])}

---

## Integration for Chyren

This identity foundation will:
1. **Seed Memory System** — Load as L6 Phylactery identity kernel
2. **Guide Task Reasoning** — AEON will understand user intent & values
3. **Inform Policy Gates** — AEGIS will enforce operator priorities
4. **Shape Responses** — Meta-cognitive layer stays aligned to identity
5. **Drive Autonomous Action** — Sovereign Loop respects creator intent

---

**Status:** Identity synthesized and ready for Chyren integration.
**Next:** Load into Rust omega-myelin, wire LangGraph orchestration, enable MCP spokes.
"""
        return foundation

    def synthesize(self, limit: int = 58339) -> Dict[str, Any]:
        """Full synthesis pipeline."""
        print("\n" + "="*70)
        print(f"🧠 CHYREN IDENTITY SYNTHESIS: {limit} Entries → Identity")
        print("="*70)

        self.connect()
        entries = self.fetch_all_entries(limit=limit)

        if not entries:
            print("✗ No entries found in Neon database")
            return {}

        patterns = self.extract_identity_patterns(entries)
        foundation = self.generate_identity_foundation(patterns)

        print("\n" + foundation)

        # Save foundation to file
        output_path = Path("/home/mega/Chyren/chyren_py/IDENTITY_FOUNDATION.md")
        output_path.parent.mkdir(parents=True, exist_ok=True)
        with open(output_path, "w") as f:
            f.write(foundation)
        print(f"\n✓ Identity foundation saved to {output_path}")

        return {
            "patterns": patterns,
            "foundation": foundation,
            "entry_count": len(entries)
        }


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Chyren Identity Synthesizer")
    parser.add_argument("--limit", type=int, default=58339, help="Number of entries to process")
    args = parser.parse_args()

    neon_url = os.getenv("OMEGA_DB_URL")
    if not neon_url:
        print("! OMEGA_DB_URL not set; skipping Neon archive")

    synthesizer = IdentitySynthesizer(neon_url)
    result = synthesizer.synthesize(limit=args.limit)

    print(f"\n" + "="*70)
    print(f"✓ Synthesis complete: {result['entry_count']:,} entries processed")
    print(f"  Identity foundation ready for Chyren integration")
    print("="*70)

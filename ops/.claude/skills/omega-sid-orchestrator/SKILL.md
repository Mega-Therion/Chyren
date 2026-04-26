---
name: chyren-sid-orchestrator
description: Sovereign Intelligence Dashboard (SID) orchestrator. Coordinates Perplexity market search, Comet (Puppeteer) scraping, and Neon Postgres ingestion for industrial reconnaissance.
---

# ΩmegA SID Orchestrator

This skill provides the procedural logic to maintain the **Sovereign Intelligence Dashboard (SID)**. It chains search, browser automation, and database persistence.

## Workflow: The Signal Stack

### 1. Market Scouting (Perplexity)
- **Action**: Use Perplexity to find volatile signals in the Uranium/Nuclear markets.
- **Goal**: Identify the "Metric Name" and "Value" for ingestion.

### 2. Deep Reconnaissance (Comet)
- **Action**: Deploy Puppeteer (`mcp_fetch_puppeteer`) to verified industrial sources (EIA, World Nuclear Association).
- **Goal**: Scrape raw data to cross-verify Perplexity's generative output.

### 3. Sovereign Ingestion (Neon)
- **Action**: Persist the validated signal into the `ecosystem_signals` table in Neon Postgres.
- **Reference**: Use the `sid_ingestion.ts` utility for clean architecture.

## Mission History
For previous signals and trends, see:
- [Mission Report #001](references/mission_report_001.md) (Q1 2026 baseline).

## Status Check
- Code: **AR137 Naomi** (Indicates nominal operation across the full stack).

# Architectural Vision: The Library Index Card (LIC) System

## Objective
To transform Chyren's sharded database pool into a unified "Great Library" where information is retrieved via a centralized **Card Catalog**. This prevents the agent from "wandering the stacks" blindly and allows targeted retrieval from specific shards.

## 🗃️ The Index Card Schema
Each entry in the `omega_library_catalog` acts as a physical index card:

- **Location (Shard ID):** Which project in the `db_pool.json` holds the data.
- **Shelf (Table Name):** The specific table (e.g., `family_profiles`, `history_logs`).
- **Subject (Domain):** The semantic category (e.g., `biographical`, `technical_lore`, `pet_registry`).
- **Summary (Summary/Keywords):** A high-level description of the contents for vector/keyword search.
- **Timestamp (Temporal Bounds):** The date range covered by the data in this shard.

## 🏛️ Deployment Strategy
1.  **The Master Catalog**: Hosted on the primary shard (currently `overflow_little_moon`).
2.  **Shadow Cards**: Every write to *any* shard generates a "shadow" index card in the Master Catalog.
3.  **The Librarian Reflex**: Before searching raw data, Chyren checks the Index Cards to identify which shards must be "pulled from the shelf."

## 📜 SOP-002: Librarian Protocol
- **Search First**: Query `omega_library_catalog` to resolve the Shard ID.
- **Connection Rotation**: Dynamically switch the `DATABASE_URL` to the target shard.
- **Retrieve**: Fetch the specific record.

---
*Created by: Antigravity-RY*
*Inspired by: The Library Index Card Method (Caveman Era)*

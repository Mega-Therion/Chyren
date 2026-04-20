# Canonical Data Layer (CDL) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Unify Chyren's disparate database schemas (Postgres, Qdrant, JSON) into a single, JSON-standardized entity pool using the existing `web/lib/schema/entity.ts` interface as the source of truth.

**Architecture:**
1. **Extraction Layer:** Medulla-based ETL workers extract raw records from Postgres/Qdrant.
2. **Standardization Engine:** Structured Extractor (or a lightweight Zod/TypeScript mapper) validates raw data against `Entity` interface.
3. **Consolidation Layer:** Standardized entities are stored in a new unified Postgres table `cdl_registry` with `JSONB` support and an associated Qdrant collection for semantic retrieval.

**Tech Stack:**
- **TypeScript/Node.js:** For data orchestration and validation.
- **PostgreSQL (Neon):** For persistent storage and JSONB indexing.
- **Qdrant:** For semantic search over unified entities.

---

### Task 1: Initialize Unified Schema Migrations
**Files:**
- Modify: `medulla/neocortex_schema.sql`
- Create: `medulla/migrations/2026-04-20-create-cdl-registry.sql`

- [ ] **Step 1: Define `cdl_registry` table in migration**
```sql
CREATE TABLE IF NOT EXISTS cdl_registry (
    id TEXT PRIMARY KEY,
    entity_data JSONB NOT NULL,
    domain TEXT NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_cdl_entity_data ON cdl_registry USING GIN (entity_data);
```
- [ ] **Step 2: Commit**
```bash
git add medulla/migrations/2026-04-20-create-cdl-registry.sql
git commit -m "feat(db): add cdl_registry table"
```

### Task 2: Standardizer Utility Implementation
**Files:**
- Create: `web/lib/schema/standardizer.ts`
- Create: `tests/standardizer.test.ts`

- [ ] **Step 1: Write failing test for standardizer**
```typescript
import { standardize } from '../web/lib/schema/standardizer';
test('standardizes raw postgres row to Entity', () => {
  const raw = { program_id: '...', domain: '...' };
  const entity = standardize(raw);
  expect(entity.id).toBeDefined();
});
```
- [ ] **Step 2: Implement minimal standardizer**
```typescript
import { Entity } from './entity';
export function standardize(raw: any): Entity {
  // Mapping logic
  return { ... } as Entity;
}
```
- [ ] **Step 3: Commit**
```bash
git add web/lib/schema/standardizer.ts tests/standardizer.test.ts
git commit -m "feat(schema): add standardization logic"
```

### Task 3: Ingestion Worker (ETL)
**Files:**
- Create: `scripts/ingest_to_cdl.ts`

- [ ] **Step 1: Implement ETL logic to move records to `cdl_registry`**
- [ ] **Step 2: Run ingestion and verify counts**
- [ ] **Step 3: Commit**
```bash
git add scripts/ingest_to_cdl.ts
git commit -m "feat(etl): implement CDL ingestion worker"
```

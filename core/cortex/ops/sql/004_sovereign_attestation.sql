-- 004_sovereign_attestation.sql
--
-- Adds first-class columns for the Sovereign Intelligence attestation tier:
--
--   * tee_attestation     — HMAC-SHA256 attestation signature emitted by
--                           chyren-tee-driver when an `@secure` substep
--                           routes through the hardware enclave. Format:
--                           `<measurement_hex>:<signature_hex>`.
--   * handover_signature  — SHA-256 hex of the artifact for entries marked
--                           HUMAN_ATTRIBUTION_REQUIRED (Authorial Proxy
--                           drafts awaiting Origin-Authority attestation).
--   * policy_manifest_root — Merkle root of the policy manifest in force at
--                           commit time; ties every ledger row back to a
--                           specific signed policy version.
--   * attested            — Boolean flipped by the Origin Authority once the
--                           draft is counter-signed. Drafts default false.
--
-- All four are nullable; existing rows are unaffected. The `flags JSON` column
-- continues to carry the same labels for backward compatibility — these
-- columns add an indexable surface for compliance queries (e.g. "all drafts
-- pending attestation older than 30 days").

ALTER TABLE ledger_entries
    ADD COLUMN IF NOT EXISTS tee_attestation     TEXT,
    ADD COLUMN IF NOT EXISTS handover_signature  TEXT,
    ADD COLUMN IF NOT EXISTS policy_manifest_root TEXT,
    ADD COLUMN IF NOT EXISTS attested            BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS ledger_entries_pending_attestation_idx
    ON ledger_entries (attested, run_id)
    WHERE handover_signature IS NOT NULL AND attested = FALSE;

CREATE INDEX IF NOT EXISTS ledger_entries_policy_root_idx
    ON ledger_entries (policy_manifest_root);

-- ── policy_manifests ────────────────────────────────────────────────────────
-- Append-only manifest store. Each row is a signed Merkle policy manifest
-- emitted by core/cortex/merkle_policy/merkle_service.py. `parent_root` forms
-- a hash-chain so the full policy history is reconstructable from any root.

CREATE TABLE IF NOT EXISTS policy_manifests (
    root         TEXT PRIMARY KEY,
    version      INTEGER NOT NULL,
    parent_root  TEXT REFERENCES policy_manifests(root),
    issuer       TEXT NOT NULL,
    signature    TEXT NOT NULL,
    clauses      JSONB NOT NULL,
    metadata     JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_utc  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS policy_manifests_version_idx
    ON policy_manifests (version DESC);

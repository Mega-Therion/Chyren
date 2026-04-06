#!/usr/bin/env node
/**
 * generate-context.mjs — Build-time Neon context fetcher
 *
 * Runs as `npm run prebuild`. Fetches family profiles, knowledge, and
 * personal memories from Neon and writes them to lib/generated-context.ts.
 *
 * If OMEGA_DB_URL is not set or Neon is unavailable, writes an empty
 * context so the build still succeeds.
 */

import { writeFileSync, existsSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dir = dirname(fileURLToPath(import.meta.url))
const OUT = join(__dir, '..', 'lib', 'generated-context.ts')

// ─── Neon HTTP SQL ────────────────────────────────────────────────────────────

async function neonQuery(query, host, auth) {
  const resp = await fetch(`https://${host}/sql`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'Neon-Connection-String': auth },
    body: JSON.stringify({ query, params: [] }),
  })
  if (!resp.ok) throw new Error(`Neon HTTP ${resp.status}`)
  const data = await resp.json()
  return data.rows ?? []
}

function parseConnectionString(connStr) {
  const match = connStr.match(/^postgresql:\/\/[^@]+@([^/?]+)/)
  if (!match) return null
  return { host: match[1], auth: connStr }
}

async function fetchContext() {
  const url = process.env.OMEGA_DB_URL
  if (!url) { console.warn('[gen-context] OMEGA_DB_URL not set — skipping'); return '' }

  const conn = parseConnectionString(url)
  if (!conn) { console.warn('[gen-context] invalid connection string'); return '' }

  const { host, auth } = conn
  console.log('[gen-context] fetching from Neon...')

  const [familyRows, knowledgeRows, memoryRows] = await Promise.all([
    neonQuery(
      `SELECT name, last_name, relationship, location, birthday, deceased,
              occupation, partner, children,
              ry_notes, notes_for_omega, how_to_greet
       FROM family_profiles ORDER BY id`,
      host, auth,
    ),
    neonQuery(
      `SELECT title, content, category, importance
       FROM public_knowledge
       WHERE category IN ('biography', 'creator', 'concept', 'quote')
       ORDER BY importance DESC NULLS LAST
       LIMIT 15`,
      host, auth,
    ),
    neonQuery(
      `SELECT content, importance, domain
       FROM omega_memory_entries
       WHERE namespace = 'personal'
         AND (superseded_by IS NULL OR superseded_by = '')
         AND content IS NOT NULL
         AND LENGTH(content) > 20
       ORDER BY importance DESC NULLS LAST
       LIMIT 10`,
      host, auth,
    ),
  ])

  const sections = []

  if (familyRows.length > 0) {
    const familyText = familyRows.map(p => {
      const fullName = [p.name, p.last_name].filter(Boolean).join(' ')
      const parts = [`${fullName} (${p.relationship})`]
      if (p.location) parts.push(`Location: ${p.location}`)
      if (p.birthday) parts.push(`Birthday: ${p.birthday}`)
      if (p.deceased) parts.push('Status: Deceased')
      if (p.occupation) parts.push(`Occupation: ${p.occupation}`)
      if (p.partner) parts.push(`Partner: ${p.partner}`)
      if (p.children) parts.push(`Children: ${p.children}`)
      if (p.how_to_greet) parts.push(`How to greet: ${p.how_to_greet}`)
      if (p.ry_notes) parts.push(`Notes from RY: ${String(p.ry_notes).slice(0, 250)}`)
      if (p.notes_for_omega) parts.push(`Instructions: ${String(p.notes_for_omega).slice(0, 250)}`)
      return parts.join('\n  ')
    }).join('\n\n')
    sections.push(`FAMILY PROFILES (${familyRows.length} members):\n${familyText}`)
  }

  const bioRows = knowledgeRows.filter(r => ['biography', 'creator', 'concept'].includes(String(r.category)))
  if (bioRows.length > 0) {
    const bioText = bioRows
      .map(r => `[${String(r.category).toUpperCase()}] ${r.title}: ${String(r.content).slice(0, 250)}`)
      .join('\n\n')
    sections.push(`KNOWLEDGE ABOUT RY & OMEGA:\n${bioText}`)
  }

  const quotes = knowledgeRows.filter(r => r.category === 'quote')
  if (quotes.length > 0) {
    const quoteText = quotes.slice(0, 6).map(r => `"${r.content}" — ${r.title}`).join('\n')
    sections.push(`RY CANONICAL QUOTES:\n${quoteText}`)
  }

  if (memoryRows.length > 0) {
    const memText = memoryRows.map(r => String(r.content).slice(0, 250)).join('\n')
    sections.push(`PERSONAL MEMORIES (high-importance):\n${memText}`)
  }

  if (sections.length === 0) return ''

  return `\n\n--- LIVE KNOWLEDGE CONTEXT (from Neon sovereign database) ---\n${sections.join('\n\n---\n')}\n--- END KNOWLEDGE CONTEXT ---`
}

// ─── Main ─────────────────────────────────────────────────────────────────────

try {
  const context = await fetchContext()

  const ts = `// AUTO-GENERATED at build time by scripts/generate-context.mjs
// Do not edit manually — regenerated on every \`npm run build\`.

export const GENERATED_RY_CONTEXT: string = ${JSON.stringify(context)}
`

  writeFileSync(OUT, ts, 'utf8')
  console.log(`[gen-context] wrote ${context.length} chars to lib/generated-context.ts`)
} catch (err) {
  console.error('[gen-context] error:', err.message)
  // Write empty context so build still succeeds
  writeFileSync(OUT, `// Auto-generated — context unavailable at build time.\nexport const GENERATED_RY_CONTEXT: string = ''\n`, 'utf8')
  console.log('[gen-context] wrote empty context (build will proceed without RAG)')
}

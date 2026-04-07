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

import { writeFileSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dir = dirname(fileURLToPath(import.meta.url))
const OUT = join(__dir, '..', 'lib', 'generated-context.ts')

// —— Neon HTTP SQL ————————————————————————————————————————
/** Timeout in ms for each Neon HTTP request (build-time fetch) */
const NEON_FETCH_TIMEOUT_MS = 8_000

async function neonQuery(query, host, auth) {
  const controller = new AbortController()
  const timer = setTimeout(() => controller.abort(), NEON_FETCH_TIMEOUT_MS)
  try {
    const resp = await fetch(`https://${host}/sql`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', 'Neon-Connection-String': auth },
      body: JSON.stringify({ query, params: [] }),
      signal: controller.signal,
    })
    if (!resp.ok) throw new Error(`Neon HTTP ${resp.status}`)
    const data = await resp.json()
    return data.rows ?? []
  } finally {
    clearTimeout(timer)
  }
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
      `SELECT content, memory_type, importance, created_at
       FROM personal_memories
       ORDER BY importance DESC, created_at DESC
       LIMIT 20`,
      host, auth,
    ),
  ])

  const parts = []

  if (familyRows.length > 0) {
    parts.push('FAMILY PROFILES:')
    for (const r of familyRows) {
      const name = [r.name, r.last_name].filter(Boolean).join(' ')
      const details = [
        r.relationship && `relationship: ${r.relationship}`,
        r.location && `location: ${r.location}`,
        r.birthday && `birthday: ${r.birthday}`,
        r.occupation && `occupation: ${r.occupation}`,
        r.partner && `partner: ${r.partner}`,
        r.children && `children: ${r.children}`,
        r.how_to_greet && `greet: ${r.how_to_greet}`,
        r.ry_notes && `notes: ${r.ry_notes}`,
        r.notes_for_omega && `operator-notes: ${r.notes_for_omega}`,
      ].filter(Boolean).join(', ')
      parts.push(`- ${name} (${details})`)
    }
  }

  if (knowledgeRows.length > 0) {
    parts.push('\nKNOWLEDGE BASE:')
    for (const r of knowledgeRows) {
      parts.push(`[${r.category}] ${r.title}: ${r.content}`)
    }
  }

  if (memoryRows.length > 0) {
    parts.push('\nPERSONAL MEMORIES:')
    for (const r of memoryRows) {
      parts.push(`- [${r.memory_type}] ${r.content}`)
    }
  }

  return parts.join('\n')
}

async function main() {
  let ctx = ''
  try {
    ctx = await fetchContext()
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err)
    if (msg.includes('aborted') || msg.toLowerCase().includes('timeout')) {
      console.warn(`[gen-context] Neon request timed out after ${NEON_FETCH_TIMEOUT_MS}ms — using empty context`)
    } else {
      console.warn('[gen-context] Neon fetch failed:', msg)
    }
    ctx = ''
  }

  const ts = `// AUTO-GENERATED — do not edit. Regenerated on each production deploy.\nexport const GENERATED_RY_CONTEXT = ${JSON.stringify(ctx)}\n`
  writeFileSync(OUT, ts, 'utf8')
  console.log(`[gen-context] wrote ${ctx.length} chars to lib/generated-context.ts`)
}

main()

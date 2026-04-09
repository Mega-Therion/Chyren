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
import { neon } from '@neondatabase/serverless'

const __dir = dirname(fileURLToPath(import.meta.url))
const OUT = join(__dir, '..', 'lib', 'generated-context.ts')

// —— Neon SQL ———————————————————————————————————————————————

async function fetchContext() {
  const url = process.env.OMEGA_DB_URL
  if (!url) { console.warn('[gen-context] OMEGA_DB_URL not set — skipping'); return '' }
  console.log('[gen-context] fetching from Neon...')
  const sql = neon(url)

  async function safeQuery(label, query) {
    try {
      return await query()
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err)
      if (/relation .* does not exist/i.test(msg)) {
        console.warn(`[gen-context] ${label} unavailable: ${msg}`)
        return []
      }
      throw err
    }
  }

  const [familyRows, knowledgeRows, memoryRows] = await Promise.all([
    safeQuery('family_profiles', () => sql`SELECT name, last_name, relationship, location, birthday, deceased,
               occupation, partner, children,
               ry_notes, notes_for_omega, how_to_greet
        FROM family_profiles ORDER BY id`),
    safeQuery('public_knowledge', () => sql`SELECT title, content, category, importance
        FROM public_knowledge
        WHERE category IN ('biography', 'creator', 'concept', 'quote')
        ORDER BY importance DESC NULLS LAST
        LIMIT 15`),
    safeQuery('personal_memories', () => sql`SELECT content, memory_type, importance, created_at
        FROM personal_memories
        ORDER BY importance DESC, created_at DESC
        LIMIT 20`),
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
    console.warn('[gen-context] Neon fetch failed:', msg)
    ctx = ''
  }

  const ts = `// AUTO-GENERATED — do not edit. Regenerated on each production deploy.\nexport const GENERATED_RY_CONTEXT = ${JSON.stringify(ctx)}\n`
  writeFileSync(OUT, ts, 'utf8')
  console.log(`[gen-context] wrote ${ctx.length} chars to lib/generated-context.ts`)
}

main()

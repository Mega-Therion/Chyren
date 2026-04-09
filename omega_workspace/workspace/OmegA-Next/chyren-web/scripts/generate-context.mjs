import { writeFileSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'
import { neon } from '@neondatabase/serverless'

const __dir = dirname(fileURLToPath(import.meta.url))
const OUT = join(__dir, '..', 'lib', 'generated-context.ts')

async function fetchContext() {
  if (!process.env.OMEGA_DB_URL) {
    console.warn('[gen-context] OMEGA_DB_URL not set — skipping')
    return ''
  }
  const sql = neon(process.env.OMEGA_DB_URL)

  const [familyRows, knowledgeRows, memoryRows] = await Promise.all([
    sql`SELECT name, last_name, relationship, location, birthday, deceased, occupation, partner, children, ry_notes, notes_for_omega, how_to_greet FROM family_profiles ORDER BY id`,
    sql`SELECT title, content, category, importance FROM public_knowledge WHERE category IN ('biography', 'creator', 'concept', 'quote') ORDER BY importance DESC NULLS LAST LIMIT 15`,
    sql`SELECT content, topic, created_at FROM memories ORDER BY created_at DESC LIMIT 20`,
  ])

  const parts = []

  if (familyRows.length > 0) {
    parts.push('FAMILY PROFILES:')
    for (const r of familyRows) {
      const name = [r.name, r.last_name].filter(Boolean).join(' ')
      const details = [`rel: ${r.relationship}`, `loc: ${r.location}`].filter(Boolean).join(', ')
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
      parts.push(`- [${r.topic}] ${r.content}`)
    }
  }

  return parts.join('\n')
}

async function main() {
  let ctx = ''
  try {
    ctx = await fetchContext()
  } catch (err) {
    console.warn('[gen-context] Neon fetch failed:', err)
  }
  const ts = `// AUTO-GENERATED\nexport const GENERATED_RY_CONTEXT = ${JSON.stringify(ctx)}\n`
  writeFileSync(OUT, ts, 'utf8')
}

main()

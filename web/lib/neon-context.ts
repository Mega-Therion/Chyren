/**
 * neon-context.ts — RY context with build-time bake + runtime live-fetch fallback
 *
 * Primary path: context is baked at build time by scripts/generate-context.mjs.
 * Fallback path: if the baked context is empty (e.g. Neon quota exceeded at build),
 * a live fetch is attempted from OMEGA_DB_URL on the first request and cached
 * in-process for CACHE_TTL_MS to avoid per-request DB round-trips.
 */

import { GENERATED_RY_CONTEXT } from './generated-context'

const MAX_WEB_CONTEXT_CHARS = 6000
const CACHE_TTL_MS = 5 * 60 * 1000 // 5 minutes

let _runtimeCache: string | null = null
let _cacheExpiry = 0
let _fetchInFlight: Promise<string> | null = null

async function fetchLiveContext(): Promise<string> {
  const url = process.env.OMEGA_DB_URL
  if (!url) return ''

  try {
    const { neon } = await import('@neondatabase/serverless')
    const sql = neon(url)

    const [familyRows, knowledgeRows, memoryRows] = await Promise.all([
      sql`SELECT name, last_name, relationship, location, birthday, deceased,
               occupation, partner, children, ry_notes, notes_for_omega, how_to_greet, fun_facts
          FROM family_profiles ORDER BY id`,
      sql`SELECT title, content, category, importance
          FROM public_knowledge
          WHERE category IN ('biography','creator','concept','quote')
          ORDER BY importance DESC NULLS LAST LIMIT 15`,
      sql`SELECT content, topic, created_at
          FROM memories ORDER BY created_at DESC LIMIT 20`,
    ])

    const parts: string[] = []

    if (familyRows.length > 0) {
      parts.push('FAMILY PROFILES:')
      for (const r of familyRows) {
        const name = [r.name, r.last_name].filter(Boolean).join(' ')
        const details = [
          `rel: ${r.relationship}`,
          `loc: ${r.location}`,
          r.occupation ? `occ: ${r.occupation}` : null,
        ]
          .filter(Boolean)
          .join(', ')
        parts.push(`- ${name} (${details})`)
        if (r.fun_facts) {
          try {
            const facts = JSON.parse(r.fun_facts as string)
            if (Array.isArray(facts)) facts.forEach((f: string) => parts.push(`  * ${f}`))
          } catch {
            parts.push(`  * ${r.fun_facts}`)
          }
        }
        if (r.ry_notes) parts.push(`  RY notes: ${r.ry_notes}`)
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
  } catch (err) {
    console.warn('[neon-context] live fetch failed:', (err as Error)?.message)
    return ''
  }
}

/**
 * Returns the RY context string.
 * Synchronous if baked context is present; async (cached) if live fetch is needed.
 */
export function getRYContext(): string {
  const baked = GENERATED_RY_CONTEXT.trim()

  if (baked) {
    return baked.length <= MAX_WEB_CONTEXT_CHARS
      ? baked
      : baked.slice(0, MAX_WEB_CONTEXT_CHARS) + '\n\n[Context truncated for web runtime reliability.]'
  }

  // Return cached runtime context if fresh
  if (_runtimeCache !== null && Date.now() < _cacheExpiry) {
    return _runtimeCache
  }

  // Kick off background refresh (fire-and-forget on this request)
  if (!_fetchInFlight) {
    _fetchInFlight = fetchLiveContext().then((ctx) => {
      _runtimeCache = ctx
      _cacheExpiry = Date.now() + CACHE_TTL_MS
      _fetchInFlight = null
      return ctx
    })
  }

  // Return stale cache or empty while fetch is in flight
  return _runtimeCache ?? ''
}

/**
 * Async version — awaits a live fetch if baked context is unavailable.
 * Use this in API routes where latency is acceptable on cache miss.
 */
export async function getRYContextAsync(): Promise<string> {
  const baked = GENERATED_RY_CONTEXT.trim()

  if (baked) {
    return baked.length <= MAX_WEB_CONTEXT_CHARS
      ? baked
      : baked.slice(0, MAX_WEB_CONTEXT_CHARS) + '\n\n[Context truncated for web runtime reliability.]'
  }

  if (_runtimeCache !== null && Date.now() < _cacheExpiry) {
    return _runtimeCache
  }

  if (!_fetchInFlight) {
    _fetchInFlight = fetchLiveContext().then((ctx) => {
      _runtimeCache = ctx
      _cacheExpiry = Date.now() + CACHE_TTL_MS
      _fetchInFlight = null
      return ctx
    })
  }

  return _fetchInFlight
}

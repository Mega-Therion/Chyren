/**
 * neon-context.ts — RY context with build-time bake + runtime live-fetch fallback
 *
 * Primary path: context is baked at build time by scripts/generate-context.mjs.
 * Fallback 1: live fetch from OMEGA_DB_URL (Neon) on first request.
 * Fallback 2: live fetch from SUPABASE_URL (Supabase REST) if Neon is over quota.
 * Results cached in-process for CACHE_TTL_MS to avoid per-request DB round-trips.
 */

import { GENERATED_RY_CONTEXT } from './generated-context'

const MAX_WEB_CONTEXT_CHARS = 6000
const CACHE_TTL_MS = 5 * 60 * 1000 // 5 minutes

let _runtimeCache: string | null = null
let _cacheExpiry = 0
let _fetchInFlight: Promise<string> | null = null

async function fetchFromSupabase(): Promise<string> {
  const supabaseUrl = process.env.SUPABASE_URL
  const supabaseKey = process.env.SUPABASE_SERVICE_KEY
  if (!supabaseUrl || !supabaseKey) return ''

  try {
    const base = supabaseUrl.replace(/\/$/, '')
    const headers = {
      apikey: supabaseKey,
      Authorization: `Bearer ${supabaseKey}`,
    }

    const [familyResp, knowledgeResp, memoryResp] = await Promise.all([
      fetch(`${base}/rest/v1/family_profiles?select=name,last_name,relationship,location,occupation,ry_notes,notes_for_omega,how_to_greet,fun_facts&order=id`, { headers }),
      fetch(`${base}/rest/v1/public_knowledge?select=title,content,category,importance&category=in.(biography,creator,concept,quote)&order=importance.desc.nullslast&limit=15`, { headers }),
      fetch(`${base}/rest/v1/memories?select=content,topic,created_at&order=created_at.desc&limit=20`, { headers }),
    ])

    const familyRows = familyResp.ok ? await familyResp.json() : []
    const knowledgeRows = knowledgeResp.ok ? await knowledgeResp.json() : []
    const memoryRows = memoryResp.ok ? await memoryResp.json() : []

    return buildContextString(familyRows, knowledgeRows, memoryRows)
  } catch (err) {
    console.warn('[neon-context] Supabase fetch failed:', (err as Error)?.message)
    return ''
  }
}

function buildContextString(
  familyRows: Record<string, unknown>[],
  knowledgeRows: Record<string, unknown>[],
  memoryRows: Record<string, unknown>[]
): string {
  const parts: string[] = []

  if (familyRows.length > 0) {
    parts.push('FAMILY PROFILES:')
    for (const r of familyRows) {
      const name = [r.name, r.last_name].filter(Boolean).join(' ')
      const details = [`rel: ${r.relationship}`, `loc: ${r.location}`, r.occupation ? `occ: ${r.occupation}` : null].filter(Boolean).join(', ')
      parts.push(`- ${name} (${details})`)
      if (r.fun_facts) {
        try {
          const facts = typeof r.fun_facts === 'string' ? JSON.parse(r.fun_facts) : r.fun_facts
          if (Array.isArray(facts)) facts.forEach((f: string) => parts.push(`  * ${f}`))
        } catch { parts.push(`  * ${r.fun_facts}`) }
      }
      if (r.ry_notes) parts.push(`  RY notes: ${r.ry_notes}`)
    }
  }
  if (knowledgeRows.length > 0) {
    parts.push('\nKNOWLEDGE BASE:')
    for (const r of knowledgeRows) parts.push(`[${r.category}] ${r.title}: ${r.content}`)
  }
  if (memoryRows.length > 0) {
    parts.push('\nPERSONAL MEMORIES:')
    for (const r of memoryRows) parts.push(`- [${r.topic}] ${r.content}`)
  }
  return parts.join('\n')
}

async function fetchLiveContext(): Promise<string> {
  const url = process.env.OMEGA_DB_URL
  if (!url) return fetchFromSupabase()

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

    return buildContextString(
      familyRows as Record<string, unknown>[],
      knowledgeRows as Record<string, unknown>[],
      memoryRows as Record<string, unknown>[]
    )
  } catch (err) {
    const msg = (err as Error)?.message ?? ''
    console.warn('[neon-context] Neon fetch failed:', msg)
    // Quota exceeded or connection error → fall back to Supabase
    if (msg.includes('quota') || msg.includes('transfer') || msg.includes('ECONNREFUSED') || msg.includes('connection')) {
      console.warn('[neon-context] Falling back to Supabase...')
      return fetchFromSupabase()
    }
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

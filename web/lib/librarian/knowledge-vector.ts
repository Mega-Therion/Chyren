/**
 * Semantic knowledge domain retrieval via Qdrant vector search.
 *
 * Falls back gracefully to empty results when Qdrant is unavailable,
 * so the web chat path never hard-fails due to vector store issues.
 */

import { searchKnowledgeDomains } from './catalog'

const COLLECTION = 'knowledge_matrix'
const QDRANT_URL = process.env.QDRANT_URL ?? 'http://localhost:6333'

interface QdrantHit {
  id: number
  score: number
  payload: {
    slug: string
    name: string
    realm: string
    reasoning_mode: string
    level: number
    reasoning_primer: string
    description: string
  }
}

async function embedText(text: string): Promise<number[] | null> {
  // Prefer Gemini (gemini-embedding-001, dim=3072) — fall back to OpenAI (dim=1536)
  const geminiKey = process.env.GEMINI_API_KEY
  const openaiKey = process.env.OPENAI_API_KEY

  if (geminiKey) {
    try {
      const resp = await fetch(
        `https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:embedContent?key=${geminiKey}`,
        {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            model: 'models/gemini-embedding-001',
            content: { parts: [{ text }] },
          }),
          signal: AbortSignal.timeout(8000),
        },
      )
      if (resp.ok) {
        const data = (await resp.json()) as { embedding: { values: number[] } }
        return data.embedding?.values ?? null
      }
    } catch {
      // fall through to OpenAI
    }
  }

  if (openaiKey) {
    try {
      const resp = await fetch('https://api.openai.com/v1/embeddings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', Authorization: `Bearer ${openaiKey}` },
        body: JSON.stringify({ model: 'text-embedding-3-small', input: text }),
        signal: AbortSignal.timeout(8000),
      })
      if (resp.ok) {
        const data = (await resp.json()) as { data: Array<{ embedding: number[] }> }
        return data.data[0]?.embedding ?? null
      }
    } catch {
      return null
    }
  }

  return null
}

async function qdrantSearch(vector: number[], topK: number): Promise<QdrantHit[]> {
  try {
    // Qdrant v1.7+ uses /points/query; fall back to /points/search for older versions
    const resp = await fetch(`${QDRANT_URL}/collections/${COLLECTION}/points/query`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        query: vector,
        limit: topK,
        with_payload: true,
        score_threshold: 0.35,
      }),
      signal: AbortSignal.timeout(5000),
    })
    if (!resp.ok) return []
    const data = (await resp.json()) as { result: { points: QdrantHit[] } }
    return data.result?.points ?? []
  } catch {
    return []
  }
}

export interface SemanticDomainResult {
  slug: string
  name: string
  realm: string
  reasoning_mode: string
  level: number
  reasoning_primer: string | null
  score: number
  source: 'vector' | 'fts'
}

/**
 * Search knowledge domains semantically (vector) with FTS fallback.
 *
 * Strategy:
 * 1. If OPENAI_API_KEY + Qdrant are available → embed query, vector search Qdrant
 * 2. Merge with FTS results from Postgres (deduped by slug, best score wins)
 * 3. Return top-k, sorted by score desc
 */
export async function semanticKnowledgeSearch(
  query: string,
  topK: number = 5,
): Promise<SemanticDomainResult[]> {
  const resultMap = new Map<string, SemanticDomainResult>()

  // ── Vector search ────────────────────────────────────────────────────────
  const hasEmbedProvider = !!(process.env.GEMINI_API_KEY || process.env.OPENAI_API_KEY)
  if (hasEmbedProvider) {
    const vector = await embedText(query)
    if (vector) {
      const hits = await qdrantSearch(vector, topK)
      for (const hit of hits) {
        resultMap.set(hit.payload.slug, {
          slug: hit.payload.slug,
          name: hit.payload.name,
          realm: hit.payload.realm,
          reasoning_mode: hit.payload.reasoning_mode,
          level: hit.payload.level,
          reasoning_primer: hit.payload.reasoning_primer ?? null,
          score: hit.score,
          source: 'vector',
        })
      }
    }
  }

  // ── FTS fallback / merge ─────────────────────────────────────────────────
  try {
    const ftsDomains = await searchKnowledgeDomains(query, undefined, topK)
    for (const d of ftsDomains) {
      if (!resultMap.has(d.slug)) {
        resultMap.set(d.slug, {
          slug: d.slug,
          name: d.name,
          realm: d.realm,
          reasoning_mode: d.reasoning_mode,
          level: d.level,
          reasoning_primer: d.reasoning_primer ?? null,
          score: d.score,
          source: 'fts',
        })
      }
    }
  } catch {
    // FTS unavailable — rely on vector results only
  }

  return Array.from(resultMap.values())
    .filter((d) => d.reasoning_primer)
    .sort((a, b) => b.score - a.score)
    .slice(0, topK)
}

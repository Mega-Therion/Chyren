import { neon, type NeonQueryFunction } from '@neondatabase/serverless'

let _sql: NeonQueryFunction<false, false> | null = null

function sql(): NeonQueryFunction<false, false> {
  if (_sql) return _sql
  const url = process.env.OMEGA_CATALOG_DB_URL
  if (!url) {
    throw new Error(
      'OMEGA_CATALOG_DB_URL is not set. The Library Index Card master catalog cannot be queried.',
    )
  }
  _sql = neon(url)
  return _sql
}

export interface IndexCard {
  card_id: string
  shard_id: string
  platform: 'neon' | 'supabase' | 'firebase' | 'cloudflare'
  shelf_table: string
  subject_domain: string
  summary: string
  keywords: string[]
  row_count_estimate: number | null
  last_indexed_at: string
}

export interface SearchResult {
  card_id: string
  shard_id: string
  platform: string
  shelf_table: string
  subject_domain: string
  summary: string
  score: number
}

export async function searchCatalog(
  query: string,
  domainHint?: string,
  maxRows: number = 10,
): Promise<SearchResult[]> {
  const rows = await sql()`
    SELECT card_id::text AS card_id, shard_id, platform, shelf_table,
           subject_domain, summary, score
    FROM omega_lic_search(${query}, ${domainHint ?? null}, ${maxRows})
  `
  return rows as unknown as SearchResult[]
}

export async function listShards(): Promise<
  Array<{ shard_id: string; platform: string; tables: number }>
> {
  const rows = await sql()`
    SELECT shard_id, platform, COUNT(*)::int AS tables
    FROM omega_library_catalog
    GROUP BY shard_id, platform
    ORDER BY shard_id
  `
  return rows as unknown as Array<{ shard_id: string; platform: string; tables: number }>
}

export async function getShardCards(shardId: string): Promise<IndexCard[]> {
  const rows = await sql()`
    SELECT card_id::text AS card_id, shard_id, platform, shelf_table,
           subject_domain, summary, keywords, row_count_estimate,
           last_indexed_at::text AS last_indexed_at
    FROM omega_library_catalog
    WHERE shard_id = ${shardId}
    ORDER BY shelf_table
  `
  return rows as unknown as IndexCard[]
}

export async function totalCards(): Promise<number> {
  const rows = await sql()`SELECT COUNT(*)::int AS n FROM omega_library_catalog`
  return (rows as unknown as Array<{ n: number }>)[0]?.n ?? 0
}

// ─── Knowledge Matrix ─────────────────────────────────────────────────────────

export interface KnowledgeDomain {
  domain_id: string
  slug: string
  name: string
  parent_slug: string | null
  level: number
  realm: string
  reasoning_mode: string
  description: string | null
  reasoning_primer: string | null
  score: number
}

export interface MatrixProgram {
  domain_id: string
  slug: string
  name: string
  parent_slug: string | null
  level: number
  realm: string
  reasoning_mode: string
  description: string | null
  purpose: string | null
  core_axioms: string[]
  key_methods: string[]
  key_figures: string[]
  sister_slugs: string[]
  query_patterns: string[]
  reasoning_primer: string | null
  updated_at: string
}

export async function searchKnowledgeDomains(
  query: string,
  realmHint?: string,
  maxRows: number = 8,
): Promise<KnowledgeDomain[]> {
  const rows = await sql()`
    SELECT domain_id::text AS domain_id, slug, name, parent_slug, level, realm,
           reasoning_mode, description, reasoning_primer, score
    FROM omega_knowledge_search(${query}, ${realmHint ?? null}, ${maxRows})
  `
  return rows as unknown as KnowledgeDomain[]
}

export async function getMatrixProgram(slug: string): Promise<MatrixProgram | null> {
  const rows = await sql()`
    SELECT domain_id::text AS domain_id, slug, name, parent_slug, level, realm,
           reasoning_mode, description, purpose,
           COALESCE(core_axioms, '[]'::jsonb) AS core_axioms,
           COALESCE(key_methods, '[]'::jsonb) AS key_methods,
           COALESCE(key_figures, '[]'::jsonb) AS key_figures,
           COALESCE(sister_slugs, '[]'::jsonb) AS sister_slugs,
           COALESCE(query_patterns, '[]'::jsonb) AS query_patterns,
           reasoning_primer,
           updated_at::text AS updated_at
    FROM omega_knowledge_domains
    WHERE slug = ${slug}
    LIMIT 1
  `
  const r = rows as unknown as MatrixProgram[]
  return r[0] ?? null
}

export async function getDomainsByRealm(
  realm: string,
  maxRows: number = 20,
): Promise<KnowledgeDomain[]> {
  const rows = await sql()`
    SELECT domain_id::text AS domain_id, slug, name, parent_slug, level, realm,
           reasoning_mode, description, reasoning_primer,
           0.0::float AS score
    FROM omega_knowledge_domains
    WHERE realm = ${realm}
    ORDER BY level ASC, sort_order ASC
    LIMIT ${maxRows}
  `
  return rows as unknown as KnowledgeDomain[]
}

export interface DreamDomain extends MatrixProgram {
  status: string
  formal_anchor: string | null
  millennium_target: boolean
}

export async function getSealedDomains(): Promise<DreamDomain[]> {
  const rows = await sql()`
    SELECT domain_id::text AS domain_id, slug, name, parent_slug, level, realm,
           reasoning_mode, description, purpose,
           COALESCE(core_axioms, '[]'::jsonb)   AS core_axioms,
           COALESCE(key_methods, '[]'::jsonb)   AS key_methods,
           COALESCE(key_figures, '[]'::jsonb)   AS key_figures,
           COALESCE(sister_slugs, '[]'::jsonb)  AS sister_slugs,
           COALESCE(query_patterns, '[]'::jsonb) AS query_patterns,
           reasoning_primer,
           updated_at::text AS updated_at,
           status,
           formal_anchor,
           millennium_target
    FROM omega_knowledge_domains
    WHERE status = 'sealed'
    ORDER BY level ASC, sort_order ASC
  `
  return rows as unknown as DreamDomain[]
}

export async function getMillenniumTargets(): Promise<DreamDomain[]> {
  const rows = await sql()`
    SELECT domain_id::text AS domain_id, slug, name, parent_slug, level, realm,
           reasoning_mode, description, purpose,
           COALESCE(core_axioms, '[]'::jsonb)   AS core_axioms,
           COALESCE(key_methods, '[]'::jsonb)   AS key_methods,
           COALESCE(key_figures, '[]'::jsonb)   AS key_figures,
           COALESCE(sister_slugs, '[]'::jsonb)  AS sister_slugs,
           COALESCE(query_patterns, '[]'::jsonb) AS query_patterns,
           reasoning_primer,
           updated_at::text AS updated_at,
           status,
           formal_anchor,
           millennium_target
    FROM omega_knowledge_domains
    WHERE millennium_target = true
    ORDER BY slug
  `
  return rows as unknown as DreamDomain[]
}

export async function updateDomainStatus(slug: string, status: string): Promise<void> {
  await sql()`
    UPDATE omega_knowledge_domains
    SET status = ${status}, updated_at = now()
    WHERE slug = ${slug}
  `
}

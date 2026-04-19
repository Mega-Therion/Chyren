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

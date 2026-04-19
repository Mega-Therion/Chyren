import { searchCatalog, listShards, getShardCards } from './catalog'

export interface MCPToolDef {
  name: string
  description: string
  inputSchema: Record<string, unknown>
}

export const LIBRARIAN_TOOLS: MCPToolDef[] = [
  {
    name: 'lic_search',
    description:
      'Search the Library Index Card catalog. Returns ranked candidate shards/tables for a free-text query. ' +
      'Use this BEFORE attempting to query any database — it tells you which shard holds the data you need.',
    inputSchema: {
      type: 'object',
      properties: {
        query: { type: 'string', description: 'Free-text search query (e.g., "family pet names", "memory entries since April")' },
        domain_hint: {
          type: 'string',
          description:
            'Optional subject_domain filter (e.g., "biographical", "memory", "operational_logs", "identity")',
        },
        max_rows: { type: 'integer', minimum: 1, maximum: 50, default: 10 },
      },
      required: ['query'],
    },
  },
  {
    name: 'lic_list_shards',
    description:
      'List every shard known to the catalog with table counts. Use this to get an overview of the data landscape.',
    inputSchema: { type: 'object', properties: {} },
  },
  {
    name: 'lic_get_shard',
    description:
      'Get every index card for a specific shard. Use this after lic_search to inspect a shard\'s full table list.',
    inputSchema: {
      type: 'object',
      properties: {
        shard_id: { type: 'string', description: 'Shard ID (e.g., "neon_technical", "supabase_chyren_sovereign")' },
      },
      required: ['shard_id'],
    },
  },
]

type ToolHandler = (args: Record<string, unknown>) => Promise<unknown>

export const LIBRARIAN_HANDLERS: Record<string, ToolHandler> = {
  lic_search: async (args) => {
    const query = String(args.query ?? '').trim()
    if (!query) throw new Error('lic_search: "query" is required')
    const domainHint = args.domain_hint ? String(args.domain_hint) : undefined
    const maxRows = typeof args.max_rows === 'number' ? args.max_rows : 10
    const results = await searchCatalog(query, domainHint, maxRows)
    return {
      query,
      domain_hint: domainHint ?? null,
      hits: results.length,
      results,
    }
  },
  lic_list_shards: async () => {
    const shards = await listShards()
    return { count: shards.length, shards }
  },
  lic_get_shard: async (args) => {
    const shardId = String(args.shard_id ?? '').trim()
    if (!shardId) throw new Error('lic_get_shard: "shard_id" is required')
    const cards = await getShardCards(shardId)
    return { shard_id: shardId, table_count: cards.length, cards }
  },
}

import { searchCatalog, listShards, getShardCards, searchKnowledgeDomains, getMatrixProgram, getDomainsByRealm } from './catalog'

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
  {
    name: 'knowledge_search',
    description:
      'Search Chyren\'s neocortex knowledge matrix for relevant domain matrix programs. ' +
      'Returns ranked domains with reasoning_primer — calibration instructions Chyren uses to think correctly in that domain. ' +
      'Use this when a topic requires domain-specific reasoning (math proofs, philosophy, ML, medicine, law, etc.).',
    inputSchema: {
      type: 'object',
      properties: {
        query: {
          type: 'string',
          description: 'Topic or question to find matching knowledge domains for (e.g., "Bayesian inference", "proof by induction", "constitutional law")',
        },
        realm_hint: {
          type: 'string',
          description: 'Optional realm filter: mathematics, logic, philosophy, computer_science, natural_science, social_science, humanities, applied, interdisciplinary, classical',
        },
        max_results: { type: 'integer', minimum: 1, maximum: 20, default: 5 },
      },
      required: ['query'],
    },
  },
  {
    name: 'get_matrix_program',
    description:
      'Fetch the full matrix program for a specific knowledge domain by its slug. ' +
      'Returns core_axioms, key_methods, key_figures, query_patterns, and the full reasoning_primer. ' +
      'Use this after knowledge_search identifies the right domain slug.',
    inputSchema: {
      type: 'object',
      properties: {
        slug: {
          type: 'string',
          description: 'Domain slug from knowledge_search results (e.g., "mathematical_proofs", "bayesian_statistics", "ethics")',
        },
      },
      required: ['slug'],
    },
  },
  {
    name: 'get_domain_by_realm',
    description:
      'List all knowledge domains within a specific realm. Useful for getting a full map of a field.',
    inputSchema: {
      type: 'object',
      properties: {
        realm: {
          type: 'string',
          description: 'Realm name: mathematics, logic, philosophy, computer_science, natural_science, social_science, humanities, applied, interdisciplinary, classical',
        },
        max_results: { type: 'integer', minimum: 1, maximum: 50, default: 20 },
      },
      required: ['realm'],
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
  knowledge_search: async (args) => {
    const query = String(args.query ?? '').trim()
    if (!query) throw new Error('knowledge_search: "query" is required')
    const realmHint = args.realm_hint ? String(args.realm_hint) : undefined
    const maxResults = typeof args.max_results === 'number' ? args.max_results : 5
    const domains = await searchKnowledgeDomains(query, realmHint, maxResults)
    return {
      query,
      realm_hint: realmHint ?? null,
      hits: domains.length,
      domains: domains.map((d) => ({
        slug: d.slug,
        name: d.name,
        realm: d.realm,
        reasoning_mode: d.reasoning_mode,
        level: d.level,
        reasoning_primer: d.reasoning_primer,
        score: d.score,
      })),
    }
  },
  get_matrix_program: async (args) => {
    const slug = String(args.slug ?? '').trim()
    if (!slug) throw new Error('get_matrix_program: "slug" is required')
    const program = await getMatrixProgram(slug)
    if (!program) return { found: false, slug }
    return { found: true, ...program }
  },
  get_domain_by_realm: async (args) => {
    const realm = String(args.realm ?? '').trim()
    if (!realm) throw new Error('get_domain_by_realm: "realm" is required')
    const maxResults = typeof args.max_results === 'number' ? args.max_results : 20
    const domains = await getDomainsByRealm(realm, maxResults)
    return { realm, count: domains.length, domains }
  },
}

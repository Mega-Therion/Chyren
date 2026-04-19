// MCP server registry: enumerates which MCP endpoints the chat route
// is allowed to call, and namespaces tool names so collisions across
// servers (e.g. two `search` tools) are unambiguous on the wire.

import { LIBRARIAN_TOOLS, LIBRARIAN_HANDLERS } from '@/lib/librarian/tools'
import { callTool, listTools, type MCPToolDef, type MCPToolResult } from './client'

export interface MCPServerEntry {
  id: string
  label: string
  // For internal servers we shortcut over HTTP and call handlers in-process.
  internal?: {
    tools: MCPToolDef[]
    handlers: Record<string, (args: Record<string, unknown>) => Promise<unknown>>
  }
  // For external servers we POST JSON-RPC to this URL.
  endpoint?: string
  headers?: Record<string, string>
}

const NAME_SEPARATOR = '__'

function buildRegistry(): MCPServerEntry[] {
  const entries: MCPServerEntry[] = [
    {
      id: 'librarian',
      label: 'Chyren Library Index Card catalog',
      internal: { tools: LIBRARIAN_TOOLS, handlers: LIBRARIAN_HANDLERS },
    },
  ]

  // Optional Supabase MCP — only enabled when a PAT is present.
  const supabasePat = process.env.SUPABASE_ACCESS_TOKEN
  const supabaseEndpoint = process.env.SUPABASE_MCP_URL
  if (supabasePat && supabaseEndpoint) {
    entries.push({
      id: 'supabase',
      label: 'Supabase project management',
      endpoint: supabaseEndpoint,
      headers: { Authorization: `Bearer ${supabasePat}` },
    })
  }

  return entries
}

let _cache: MCPServerEntry[] | null = null
function registry(): MCPServerEntry[] {
  if (!_cache) _cache = buildRegistry()
  return _cache
}

// Tool definition exposed to a model provider, with the server-id prefix baked in.
export interface RegisteredTool {
  serverId: string
  qualifiedName: string
  rawName: string
  description: string
  inputSchema: Record<string, unknown>
}

export async function listAllTools(): Promise<RegisteredTool[]> {
  const all: RegisteredTool[] = []
  for (const entry of registry()) {
    let tools: MCPToolDef[] = []
    if (entry.internal) {
      tools = entry.internal.tools
    } else if (entry.endpoint) {
      try {
        tools = await listTools(entry.endpoint, { headers: entry.headers })
      } catch {
        // External MCP unreachable — skip silently so the chat keeps working.
        continue
      }
    }
    for (const t of tools) {
      all.push({
        serverId: entry.id,
        qualifiedName: `${entry.id}${NAME_SEPARATOR}${t.name}`,
        rawName: t.name,
        description: t.description,
        inputSchema: t.inputSchema,
      })
    }
  }
  return all
}

export async function dispatchTool(
  qualifiedName: string,
  args: Record<string, unknown>,
): Promise<MCPToolResult> {
  const sepIdx = qualifiedName.indexOf(NAME_SEPARATOR)
  if (sepIdx === -1) {
    return errorResult(`Unknown tool: ${qualifiedName}`)
  }
  const serverId = qualifiedName.slice(0, sepIdx)
  const rawName = qualifiedName.slice(sepIdx + NAME_SEPARATOR.length)
  const entry = registry().find((e) => e.id === serverId)
  if (!entry) return errorResult(`Unknown MCP server: ${serverId}`)

  if (entry.internal) {
    const handler = entry.internal.handlers[rawName]
    if (!handler) return errorResult(`Tool not found on ${serverId}: ${rawName}`)
    try {
      const output = await handler(args)
      return {
        content: [{ type: 'text', text: JSON.stringify(output, null, 2) }],
        isError: false,
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err)
      return errorResult(msg)
    }
  }

  if (entry.endpoint) {
    try {
      return await callTool(entry.endpoint, rawName, args, { headers: entry.headers })
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err)
      return errorResult(msg)
    }
  }

  return errorResult(`MCP server ${serverId} has no transport configured`)
}

function errorResult(message: string): MCPToolResult {
  return { content: [{ type: 'text', text: `Error: ${message}` }], isError: true }
}

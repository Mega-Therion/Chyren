// Minimal MCP HTTP/JSON-RPC 2.0 client.
// Used by the chat route to call internal and external MCP servers
// (librarian, supabase, etc.) without spawning subprocesses.

export interface MCPToolDef {
  name: string
  description: string
  inputSchema: Record<string, unknown>
}

export interface MCPToolResult {
  content: Array<{ type: 'text'; text: string }>
  isError?: boolean
}

interface JsonRpcResponse<T = unknown> {
  jsonrpc: '2.0'
  id: string | number | null
  result?: T
  error?: { code: number; message: string; data?: unknown }
}

let _idCounter = 0
const nextId = () => ++_idCounter

async function rpc<T>(
  endpoint: string,
  method: string,
  params?: Record<string, unknown>,
  init?: RequestInit,
): Promise<T> {
  const resp = await fetch(endpoint, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...(init?.headers ?? {}) },
    body: JSON.stringify({ jsonrpc: '2.0', id: nextId(), method, params }),
    ...init,
  })
  if (!resp.ok) {
    throw new Error(`MCP ${endpoint} HTTP ${resp.status}: ${await resp.text().catch(() => '')}`)
  }
  const json = (await resp.json()) as JsonRpcResponse<T>
  if (json.error) {
    throw new Error(`MCP ${endpoint} ${method} error ${json.error.code}: ${json.error.message}`)
  }
  if (json.result === undefined) {
    throw new Error(`MCP ${endpoint} ${method} returned no result`)
  }
  return json.result
}

export async function listTools(endpoint: string, init?: RequestInit): Promise<MCPToolDef[]> {
  const result = await rpc<{ tools: MCPToolDef[] }>(endpoint, 'tools/list', undefined, init)
  return result.tools ?? []
}

export async function callTool(
  endpoint: string,
  name: string,
  args: Record<string, unknown>,
  init?: RequestInit,
): Promise<MCPToolResult> {
  return rpc<MCPToolResult>(endpoint, 'tools/call', { name, arguments: args }, init)
}

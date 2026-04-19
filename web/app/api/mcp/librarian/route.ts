import { type NextRequest, NextResponse } from 'next/server'
import { LIBRARIAN_TOOLS, LIBRARIAN_HANDLERS } from '@/lib/librarian/tools'

export const runtime = 'nodejs'
export const dynamic = 'force-dynamic'

const PROTOCOL_VERSION = '2024-11-05'
const SERVER_INFO = { name: 'chyren-librarian', version: '0.1.0' }

interface JsonRpcRequest {
  jsonrpc: '2.0'
  id?: string | number | null
  method: string
  params?: Record<string, unknown>
}

function rpcResult(id: string | number | null | undefined, result: unknown) {
  return NextResponse.json({ jsonrpc: '2.0', id: id ?? null, result })
}

function rpcError(id: string | number | null | undefined, code: number, message: string, data?: unknown) {
  return NextResponse.json(
    { jsonrpc: '2.0', id: id ?? null, error: { code, message, ...(data !== undefined ? { data } : {}) } },
    { status: code === -32600 ? 400 : 200 },
  )
}

export async function GET() {
  return NextResponse.json({
    server: SERVER_INFO,
    protocol: PROTOCOL_VERSION,
    transport: 'http',
    tools: LIBRARIAN_TOOLS.map((t) => t.name),
    docs: 'POST JSON-RPC 2.0 requests. Methods: initialize, tools/list, tools/call.',
  })
}

export async function POST(req: NextRequest) {
  let body: JsonRpcRequest
  try {
    body = (await req.json()) as JsonRpcRequest
  } catch {
    return rpcError(null, -32700, 'Parse error: invalid JSON')
  }

  if (body.jsonrpc !== '2.0' || typeof body.method !== 'string') {
    return rpcError(body.id, -32600, 'Invalid Request')
  }

  switch (body.method) {
    case 'initialize':
      return rpcResult(body.id, {
        protocolVersion: PROTOCOL_VERSION,
        serverInfo: SERVER_INFO,
        capabilities: { tools: {} },
      })

    case 'tools/list':
      return rpcResult(body.id, { tools: LIBRARIAN_TOOLS })

    case 'tools/call': {
      const params = body.params ?? {}
      const name = String(params.name ?? '')
      const args = (params.arguments ?? {}) as Record<string, unknown>
      const handler = LIBRARIAN_HANDLERS[name]
      if (!handler) {
        return rpcError(body.id, -32601, `Tool not found: ${name}`)
      }
      try {
        const output = await handler(args)
        return rpcResult(body.id, {
          content: [{ type: 'text', text: JSON.stringify(output, null, 2) }],
          isError: false,
        })
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err)
        return rpcResult(body.id, {
          content: [{ type: 'text', text: `Error: ${msg}` }],
          isError: true,
        })
      }
    }

    case 'ping':
      return rpcResult(body.id, {})

    default:
      return rpcError(body.id, -32601, `Method not found: ${body.method}`)
  }
}

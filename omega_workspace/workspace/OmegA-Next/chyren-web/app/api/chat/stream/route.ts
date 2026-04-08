import { type NextRequest } from 'next/server'
import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContext } from '@/lib/neon-context'

// Pre-compute system prompt once at module load (zero per-request overhead).
const _BASE_SYSTEM_PROMPT = (() => {
  const ctx = getRYContext()
  return ctx ? CHYREN_SYSTEM_PROMPT + ctx : CHYREN_SYSTEM_PROMPT
})()

export const runtime = 'nodejs'

// Model priority list: primary first, fallbacks after
const _MODEL_CHAIN = [
  process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile',
  'llama-3.1-8b-instant',
  'gemma2-9b-it',
]

function getRequiredEnv(name: string): string {
  const v = process.env[name]
  if (!v) throw new Error(`Missing required env var: ${name}`)
  return v
}

function _buildModel(modelId: string) {
  const apiKey = getRequiredEnv('GROQ_API_KEY')
  return apiKey ? modelId : modelId
}

/** Stream a plain-text error message back to the client so the UI shows it. */
function _errorStream(message: string): ReadableStream<Uint8Array> {
  const encoder = new TextEncoder()
  return new ReadableStream({
    start(controller) {
      // Vercel AI SDK data-stream format: "0:" prefix
      controller.enqueue(encoder.encode(`0:${JSON.stringify(message)}\n`))
      controller.close()
    },
  })
}

export async function POST(req: NextRequest) {
  const { message, messages } = await req.json().catch(() => ({}))
  const session = req.nextUrl.searchParams.get('session') ?? 'global'
  
  const content = messages?.length 
    ? messages[messages.length - 1].content 
    : message;

  if (!content) {
    return new Response(JSON.stringify({ error: 'Message is required' }), { status: 400 });
  }

  try {
    // Proxy to the Rust Sovereign Hub (Streaming Endpoint)
    const apiUrl = process.env.CHYREN_API_URL 
      ? `${process.env.CHYREN_API_URL}/api/chat/stream` 
      : 'http://chyren-api:8080/api/chat/stream';

    const resp = await fetch(apiUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ message: content, session_id: session })
    });

    if (!resp.ok) {
      const errorText = await resp.text();
      return new Response(JSON.stringify({ error: `Hub Error: ${errorText}` }), { status: resp.status });
    }

    // Direct proxy of the stream from Rust to Frontend
    return new Response(resp.body, {
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
      },
    });

  } catch (err: unknown) {
    console.error("[HUB PROXY] Offline:", err);
    return new Response(JSON.stringify({ error: "Sovereign Hub offline. Please ensure the 'chyren-api' service is running." }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' },
    });
  }
}

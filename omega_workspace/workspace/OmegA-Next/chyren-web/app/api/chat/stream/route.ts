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

type ChatMsg = { role: 'system' | 'user' | 'assistant'; content: string }

function toChatHistory(rawMessages: unknown, fallbackContent: string): ChatMsg[] {
  if (!Array.isArray(rawMessages) || rawMessages.length === 0) {
    return [{ role: 'user', content: fallbackContent }]
  }

  const normalized = rawMessages
    .map((m) => {
      const role = typeof m?.role === 'string' ? m.role : ''
      const content = typeof m?.content === 'string' ? m.content : ''
      if (!content || (role !== 'user' && role !== 'assistant')) return null
      return { role: role as 'user' | 'assistant', content }
    })
    .filter((m): m is { role: 'user' | 'assistant'; content: string } => Boolean(m))

  return normalized.length > 0 ? normalized : [{ role: 'user', content: fallbackContent }]
}

async function fetchGroqStream(history: ChatMsg[]): Promise<Response> {
  const apiKey = getRequiredEnv('GROQ_API_KEY')
  const endpoint = 'https://api.groq.com/openai/v1/chat/completions'
  const messages: ChatMsg[] = [{ role: 'system', content: _BASE_SYSTEM_PROMPT }, ...history]
  let lastError = 'unknown'

  for (const model of _MODEL_CHAIN) {
    const resp = await fetch(endpoint, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${apiKey}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        model,
        messages,
        stream: true,
        temperature: 0.5,
      }),
    })

    if (resp.ok && resp.body) return resp
    lastError = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
  }

  throw new Error(`Groq fallback failed: ${lastError}`)
}

function sseHeaders() {
  return {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    Connection: 'keep-alive',
  }
}

async function fetchHubStream(content: string, session: string): Promise<Response> {
  const base = process.env.CHYREN_API_URL
  if (!base) {
    throw new Error('Missing CHYREN_API_URL')
  }

  const apiUrl = `${base}/api/chat/stream`
  return fetch(apiUrl, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ message: content, session_id: session }),
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

  const history = toChatHistory(messages, content)
  let hubFailure: string | null = null

  try {
    // Prefer Rust sovereign hub when configured.
    if (process.env.CHYREN_API_URL) {
      const hubResp = await fetchHubStream(content, session)
      if (hubResp.ok && hubResp.body) {
        return new Response(hubResp.body, { headers: sseHeaders() })
      }

      hubFailure = await hubResp.text().catch(() => `Hub status ${hubResp.status}`)
      console.warn('[HUB PROXY] Hub returned non-OK, falling back to Groq:', hubFailure)
    }

    // Automatic failover path for Vercel if hub is unavailable.
    const groqResp = await fetchGroqStream(history)
    return new Response(groqResp.body, { headers: sseHeaders() })

  } catch (err: unknown) {
    const errMsg = err instanceof Error ? err.message : 'unknown error'
    console.error('[CHAT STREAM] Upstream failure:', { hubFailure, errMsg })
    return new Response(JSON.stringify({
      error: hubFailure
        ? `Hub unavailable and fallback failed: ${hubFailure}`
        : 'Sovereign Hub offline and fallback unavailable. Set CHYREN_API_URL and/or GROQ_API_KEY.',
    }), {
      status: 503,
      headers: { 'Content-Type': 'application/json' },
    })
  }
}

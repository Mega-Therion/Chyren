import { type NextRequest } from 'next/server'
import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContext } from '@/lib/neon-context'
import {
  getVerifiedMemberContext,
  processFamilyAuthMessage,
} from '@/lib/family-auth'

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
type ExpressionProfile = {
  styleId: string
  guidance: string
  temperature: number
}

const _EXPRESSION_PROFILES: ExpressionProfile[] = [
  {
    styleId: 'precise-formal',
    guidance:
      'Use precise, formal phrasing with concise structure. Keep claims explicit and avoid slang.',
    temperature: 0.45,
  },
  {
    styleId: 'warm-conversational',
    guidance:
      'Use warm, conversational phrasing with approachable language. Keep the same facts and constraints.',
    temperature: 0.62,
  },
  {
    styleId: 'strategic-executive',
    guidance:
      'Use strategic, executive-style phrasing focused on intent, outcomes, and operating principles.',
    temperature: 0.52,
  },
  {
    styleId: 'technical-direct',
    guidance:
      'Use technical, direct phrasing with compact wording. Prioritize clarity, system behavior, and rationale.',
    temperature: 0.48,
  },
]

function _hash32(input: string): number {
  let h = 2166136261
  for (let i = 0; i < input.length; i++) {
    h ^= input.charCodeAt(i)
    h = Math.imul(h, 16777619)
  }
  return h >>> 0
}

function pickExpressionProfile(sessionId: string): ExpressionProfile {
  const hash = _hash32(sessionId || 'global')
  return _EXPRESSION_PROFILES[hash % _EXPRESSION_PROFILES.length]
}

function buildSystemPrompt(
  sessionId: string,
  memberContext?: string | null,
): {
  prompt: string
  profile: ExpressionProfile
} {
  const profile = pickExpressionProfile(sessionId)
  let prompt =
    _BASE_SYSTEM_PROMPT +
    `\n\nResponse expression profile: ${profile.styleId}.\n` +
    `${profile.guidance}\n` +
    `Preserve semantic intent and policy constraints exactly; vary only phrasing and rhetorical framing.`
  if (memberContext) {
    prompt += `\n\n${memberContext}`
  }
  return { prompt, profile }
}

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

async function fetchGroqStream(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getRequiredEnv('GROQ_API_KEY')
  const endpoint = 'https://api.groq.com/openai/v1/chat/completions'
  const messages: ChatMsg[] = [{ role: 'system', content: systemPrompt }, ...history]
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
        temperature,
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

async function fetchHubStream(
  content: string,
  session: string,
  profile: ExpressionProfile,
  memberContext?: string | null,
): Promise<Response> {
  const base = process.env.CHYREN_API_URL
  if (!base) {
    throw new Error('Missing CHYREN_API_URL')
  }

  const apiUrl = `${base}/api/chat/stream`
  return fetch(apiUrl, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      message: content,
      session_id: session,
      response_style_hint: profile.styleId,
      response_style_guidance: profile.guidance,
      verified_member_context: memberContext ?? undefined,
    }),
  })
}

function createSingleSseTextResponse(text: string): Response {
  const body = `data: ${JSON.stringify({
    choices: [{ delta: { content: text } }],
  })}\n\n`
  return new Response(body, { headers: sseHeaders() })
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

  const familyAuth = await processFamilyAuthMessage(session, content)
  if (familyAuth.handled && familyAuth.reply) {
    return createSingleSseTextResponse(familyAuth.reply)
  }

  const memberContext = await getVerifiedMemberContext(session)
  const history = toChatHistory(messages, content)
  const { prompt: systemPrompt, profile } = buildSystemPrompt(session, memberContext)
  let hubFailure: string | null = null

  try {
    // Prefer Rust sovereign hub when configured.
    if (process.env.CHYREN_API_URL) {
      const hubResp = await fetchHubStream(content, session, profile, memberContext)
      if (hubResp.ok && hubResp.body) {
        return new Response(hubResp.body, { headers: sseHeaders() })
      }

      hubFailure = await hubResp.text().catch(() => `Hub status ${hubResp.status}`)
      console.warn('[HUB PROXY] Hub returned non-OK, falling back to Groq:', hubFailure)
    }

    // Automatic failover path for Vercel if hub is unavailable.
    const groqResp = await fetchGroqStream(history, systemPrompt, profile.temperature)
    return new Response(groqResp.body, { headers: sseHeaders() })

  } catch (err: unknown) {
    const errMsg = err instanceof Error ? err.message : 'unknown error'
    console.error('[CHAT STREAM] Upstream failure:', { hubFailure, errMsg })
    const offlineMessage = hubFailure
      ? 'Chyren is temporarily unavailable right now. Please try again in a moment.'
      : 'Chyren is not fully configured yet. Please try again in a moment.'

    // Return a valid SSE payload so the client can render a graceful offline message
    // instead of surfacing a hard transport error.
    return createSingleSseTextResponse(offlineMessage)
  }
}

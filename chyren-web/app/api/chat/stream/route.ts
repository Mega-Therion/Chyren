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

// Prefer the configured primary model, but keep lighter Groq models behind it so
// the chat does not hard-fail when the larger model hits a stricter quota bucket.
const _MODEL_CHAIN = Array.from(
  new Set([
    process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile',
    'llama-3.1-8b-instant',
    'meta-llama/llama-4-scout-17b-16e-instruct',
  ]),
)

function normalizeEnvValue(value: string | undefined): string | null {
  if (!value) return null

  const trimmed = value.trim().replace(/^['\"]|['\"]$/g, '')
  if (!trimmed) return null
  if (/^YOUR_(?:API_)?KEY$/i.test(trimmed)) return null
  if (/^YOUR_KEY$/i.test(trimmed)) return null
  if (/^REPLACE_ME$/i.test(trimmed)) return null

  return trimmed
}

function getOptionalEnv(name: string): string | null {
  return normalizeEnvValue(process.env[name])
}

function getRequiredEnv(name: string): string {
  const value = getOptionalEnv(name)
  if (!value) throw new Error(`Missing required env var: ${name}`)
  return value
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

const MAX_HISTORY_MESSAGES = 6
const MAX_HISTORY_CHARS = 2400
const MAX_MESSAGE_CHARS = 600

function truncateMessageContent(content: string): string {
  const trimmed = content.trim()
  if (trimmed.length <= MAX_MESSAGE_CHARS) return trimmed
  return `${trimmed.slice(0, MAX_MESSAGE_CHARS)}...`
}

function toChatHistory(rawMessages: unknown, fallbackContent: string): ChatMsg[] {
  if (!Array.isArray(rawMessages) || rawMessages.length === 0) {
    return [{ role: 'user', content: fallbackContent }]
  }

  const normalized = rawMessages
    .map((m) => {
      const role = typeof m?.role === 'string' ? m.role : ''
      const content = typeof m?.content === 'string' ? truncateMessageContent(m.content) : ''
      if (!content || (role !== 'user' && role !== 'assistant')) return null
      return { role: role as 'user' | 'assistant', content }
    })
    .filter((m): m is { role: 'user' | 'assistant'; content: string } => Boolean(m))

  if (normalized.length === 0) {
    return [{ role: 'user', content: fallbackContent }]
  }

  const recent: ChatMsg[] = []
  let totalChars = 0

  for (let i = normalized.length - 1; i >= 0; i -= 1) {
    const next = normalized[i]
    if (recent.length >= MAX_HISTORY_MESSAGES) break

    const nextSize = next.content.length
    if (recent.length > 0 && totalChars + nextSize > MAX_HISTORY_CHARS) break

    recent.push(next)
    totalChars += nextSize
  }

  return recent.reverse()
}

async function fetchGroqResponse(
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
        temperature,
      }),
    })

    if (resp.ok) {
      const payload = (await resp.json().catch(() => ({}))) as {
        choices?: Array<{ message?: { content?: string | Array<{ text?: string }> } }>
      }
      const message = payload.choices?.[0]?.message?.content
      const content =
        typeof message === 'string'
          ? message
          : Array.isArray(message)
            ? message
                .map((part) => (typeof part?.text === 'string' ? part.text : ''))
                .join('')
                .trim()
            : ''

      if (content) return createSingleSseTextResponse(content)
      lastError = `Groq model ${model} returned empty response`
      continue
    }

    lastError = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
  }

  throw new Error(`Groq fallback failed: ${lastError}`)
}

async function fetchOpenAIResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('OPENAI_API_KEY')
  if (!apiKey) throw new Error('Missing required env var: OPENAI_API_KEY')

  const resp = await fetch('https://api.openai.com/v1/chat/completions', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      model: getOptionalEnv('OPENAI_MODEL') ?? 'gpt-4.1-mini',
      messages: [{ role: 'system', content: systemPrompt }, ...history],
      temperature,
    }),
  })

  if (!resp.ok) {
    const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
    throw new Error(`OpenAI fallback failed: ${errorBody}`)
  }

  const payload = (await resp.json().catch(() => ({}))) as {
    choices?: Array<{ message?: { content?: string | Array<{ text?: string }> } }>
  }
  const message = payload.choices?.[0]?.message?.content
  const content =
    typeof message === 'string'
      ? message
      : Array.isArray(message)
        ? message
            .map((part) => (typeof part?.text === 'string' ? part.text : ''))
            .join('')
            .trim()
        : ''

  if (!content) throw new Error('OpenAI fallback failed: empty response')
  return createSingleSseTextResponse(content)
}

async function fetchAnthropicResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('ANTHROPIC_API_KEY')
  if (!apiKey) throw new Error('Missing required env var: ANTHROPIC_API_KEY')

  const resp = await fetch('https://api.anthropic.com/v1/messages', {
    method: 'POST',
    headers: {
      'x-api-key': apiKey,
      'anthropic-version': '2023-06-01',
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      model: getOptionalEnv('ANTHROPIC_MODEL') ?? 'claude-3-5-haiku-latest',
      system: systemPrompt,
      max_tokens: 1024,
      temperature,
      messages: history.map((entry) => ({ role: entry.role, content: entry.content })),
    }),
  })

  if (!resp.ok) {
    const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
    throw new Error(`Anthropic fallback failed: ${errorBody}`)
  }

  const payload = (await resp.json().catch(() => ({}))) as {
    content?: Array<{ text?: string }>
  }
  const content = payload.content?.map((part) => part.text ?? '').join('').trim() ?? ''
  if (!content) throw new Error('Anthropic fallback failed: empty response')

  return createSingleSseTextResponse(content)
}

async function fetchGeminiResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('GEMINI_API_KEY')
  if (!apiKey) throw new Error('Missing required env var: GEMINI_API_KEY')

  const model = getOptionalEnv('GEMINI_MODEL') ?? 'gemini-2.5-flash'
  const resp = await fetch(
    `https://generativelanguage.googleapis.com/v1beta/models/${model}:generateContent?key=${encodeURIComponent(apiKey)}`,
    {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        system_instruction: {
          parts: [{ text: systemPrompt }],
        },
        contents: history.map((entry) => ({
          role: entry.role === 'assistant' ? 'model' : 'user',
          parts: [{ text: entry.content }],
        })),
        generationConfig: {
          temperature,
        },
      }),
    },
  )

  if (!resp.ok) {
    const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
    throw new Error(`Gemini fallback failed: ${errorBody}`)
  }

  const payload = (await resp.json().catch(() => ({}))) as {
    candidates?: Array<{
      content?: {
        parts?: Array<{ text?: string }>
      }
    }>
  }
  const content =
    payload.candidates?.[0]?.content?.parts
      ?.map((part) => part.text ?? '')
      .join('')
      .trim() ?? ''

  if (!content) throw new Error('Gemini fallback failed: empty response')
  return createSingleSseTextResponse(content)
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
  const base = getOptionalEnv('CHYREN_API_URL')
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
    if (getOptionalEnv('CHYREN_API_URL')) {
      const hubResp = await fetchHubStream(content, session, profile, memberContext)
      if (hubResp.ok && hubResp.body) {
        return new Response(hubResp.body, { headers: sseHeaders() })
      }

      hubFailure = await hubResp.text().catch(() => `Hub status ${hubResp.status}`)
      console.warn('[HUB PROXY] Hub returned non-OK, falling back:', hubFailure)
    }

    const providers: Array<() => Promise<Response>> = [
      () => fetchGeminiResponse(history, systemPrompt, profile.temperature),
      () => fetchAnthropicResponse(history, systemPrompt, profile.temperature),
      () => fetchGroqResponse(history, systemPrompt, profile.temperature),
      () => fetchOpenAIResponse(history, systemPrompt, profile.temperature),
    ]

    let lastProviderError = 'No AI providers are configured.'
    for (const provider of providers) {
      try {
        const resp = await provider()
        if (resp.body) {
          return new Response(resp.body, { headers: sseHeaders() })
        }
        return resp
      } catch (error) {
        lastProviderError = error instanceof Error ? error.message : 'unknown provider failure'
        console.error('[CHAT STREAM] Provider attempt failed:', lastProviderError)
      }
    }

    throw new Error(lastProviderError)
  } catch (err: unknown) {
    const errMsg = err instanceof Error ? err.message : 'unknown error'
    console.error('[CHAT STREAM] Upstream failure:', { hubFailure, errMsg })

    const offlineMessage = hubFailure
      ? 'Chyren is temporarily unavailable right now. Please try again in a moment.'
      : 'Chyren is not fully configured yet. Please try again in a moment.'

    return createSingleSseTextResponse(offlineMessage)
  }
}

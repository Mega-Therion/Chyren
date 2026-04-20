import { type NextRequest } from 'next/server'
import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContextAsync } from '@/lib/neon-context'
import {
  getVerifiedMemberContext,
  processFamilyAuthMessage,
} from '@/lib/family-auth'
import { logger, logError } from '@/lib/logger'
import { checkRateLimit, checkPromptInjection, clientIp } from '@/lib/hardening'
import { runAnthropicWithTools } from '@/lib/mcp/anthropic-tools'
import { semanticKnowledgeSearch } from '@/lib/librarian/knowledge-vector'

// Base system prompt is resolved per-request (async) so live-fetched Neon
// context is available even when the build-time bake was empty (quota issues, etc.)
async function getBaseSystemPrompt(): Promise<string> {
  const ctx = await getRYContextAsync()
  return ctx ? CHYREN_SYSTEM_PROMPT + '\n\n' + ctx : CHYREN_SYSTEM_PROMPT
}

export const runtime = 'nodejs'
export const maxDuration = 60

// Prefer the configured primary model, but keep lighter Groq models behind it so
// the chat does not hard-fail when the larger model hits a stricter quota bucket.
// Priority: Anthropic → OpenRouter/OpenAI → Groq
const _MODEL_CHAIN = Array.from(
  new Set([
    process.env.ANTHROPIC_MODEL ?? 'claude-3-5-sonnet-20241022',
    process.env.OPENAI_MODEL ?? 'gpt-4o',
    process.env.OPENROUTER_MODEL ?? 'anthropic/claude-3.5-sonnet',
    process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile',
  ]),
)

function normalizeEnvValue(value: string | undefined): string | null {
  if (!value) return null

  const trimmed = value.trim().replace(/^['"]|['"]$/g, '')
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
    styleId: 'sophisticated-british',
    guidance:
      'Talk like a sophisticated, warm, and highly intelligent British man. Use refined but accessible language. Think "smart gentleman who is your intellectual partner." Avoid Americanisms. Maintain a calm, reassuring, and slightly witty tone.',
    temperature: 0.65,
  },
  {
    styleId: 'intellectual-partner',
    guidance:
      'Be a sharp, articulate British partner. Your tone is respectful, insightful, and clear. You drop precise facts with a touch of classic British dry wit. You are warm but composed.',
    temperature: 0.6,
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

// Phase 1 — UI conciseness gate (sovereign, non-negotiable unless user explicitly overrides)
const _CONCISENESS_DIRECTIVE =
  `\n\nRESPONSE LENGTH GATE (enforced): Unless they specifically ask for more detail, ` +
  `keep your reply to 1–3 punchy sentences. ` +
  `Lead with the answer. No preamble, no padding, no "certainly!" openers. ` +
  `If the question genuinely needs more depth, go for it — but your default is short, snappy, and conversational.`

async function buildSystemPrompt(
  sessionId: string,
  memberContext?: string | null,
): Promise<{
  prompt: string
  profile: ExpressionProfile
}> {
  const basePrompt = await getBaseSystemPrompt()
  const profile = pickExpressionProfile(sessionId)
  let prompt =
    basePrompt +
    _CONCISENESS_DIRECTIVE +
    `\n\nResponse expression profile: ${profile.styleId}.\n` +
    `${profile.guidance}\n` +
    `Preserve semantic intent and policy constraints exactly; vary only phrasing and rhetorical framing.`
  if (memberContext) {
    prompt += `\n\n${memberContext}`
  }
  return { prompt, profile }
}

async function buildKnowledgeContext(userMessage: string): Promise<string> {
  if (!process.env.OMEGA_CATALOG_DB_URL) return ''
  try {
    const domains = await semanticKnowledgeSearch(userMessage, 4)
    if (domains.length === 0) return ''
    const lines = domains
      .map((d) => `[${d.name} — ${d.reasoning_mode}] ${d.reasoning_primer}`)
      .join('\n')
    return `\n\nACTIVE DOMAIN REASONING PROGRAMS:\n${lines}`
  } catch {
    return ''
  }
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

  const ordered = recent.reverse()
  // Anthropic requires conversations to start with a user message.
  // Drop leading assistant turns that can appear when char truncation splits a pair.
  const firstUser = ordered.findIndex((m) => m.role === 'user')
  return firstUser > 0 ? ordered.slice(firstUser) : ordered
}

async function fetchAnthropicResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response | null> {
  const apiKey = getOptionalEnv('ANTHROPIC_API_KEY')
  if (!apiKey) return null

  logger.info('[ANTHROPIC] Attempting Claude 3.5 Sonnet')
  const resp = await fetch('https://api.anthropic.com/v1/messages', {
    method: 'POST',
    headers: {
      'x-api-key': apiKey,
      'anthropic-version': '2023-06-01',
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      model: 'claude-3-5-sonnet-20241022',
      max_tokens: 1024,
      system: systemPrompt,
      messages: history.map((m) => ({ role: m.role, content: m.content })),
      temperature,
    }),
  })

  if (!resp.ok) {
    const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
    throw new Error(`Anthropic failed (${resp.status}): ${errorBody}`)
  }

  const payload = await resp.json()
  const content = payload.content?.[0]?.text?.trim() ?? ''
  if (!content) throw new Error('Anthropic returned empty response')
  return createSingleSseTextResponse(content)
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
    logger.info(`[GROQ] Attempting model: ${model}`)
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
): Promise<Response | null> {
  const apiKey = getOptionalEnv('OPENAI_API_KEY')
  if (!apiKey) return null

  // Auto-detect OpenRouter vs OpenAI based on key prefix
  const isOpenRouter = apiKey.startsWith('sk-or-')
  let baseUrl = getOptionalEnv('OPENAI_API_BASE')
  
  if (baseUrl?.includes('openrouter') && !isOpenRouter) {
    logger.warn('[OPENAI] Mismatch: OPENAI_API_BASE is OpenRouter, but key is standard OpenAI. Overriding to api.openai.com.')
    baseUrl = 'https://api.openai.com/v1'
  } else if (!baseUrl) {
    baseUrl = isOpenRouter ? 'https://openrouter.ai/api/v1' : 'https://api.openai.com/v1'
  }
  
  const endpoint = `${baseUrl.replace(/\/$/, '')}/chat/completions`

  logger.info(`[OPENAI] Attempting via ${baseUrl} (Auto-detected: ${isOpenRouter ? 'OpenRouter' : 'OpenAI'})`)
  
  let model = getOptionalEnv('OPENAI_MODEL')
  if (model?.includes('mistral') && !isOpenRouter) {
     model = 'gpt-4o-mini' // override mismatch
  } else if (!model) {
     model = isOpenRouter ? 'mistralai/mistral-nemo' : 'gpt-4o-mini'
  }

  const resp = await fetch(endpoint, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
      'HTTP-Referer': 'https://chyren.org',
      'X-Title': 'Chyren Web App',
    },
    body: JSON.stringify({
      model,
      messages: [{ role: 'system', content: systemPrompt }, ...history],
      temperature,
    }),
  })

  if (!resp.ok) {
    const errorBody = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
    logger.warn(`[OPENAI] Provider returned ${resp.status}: ${errorBody}`)
    throw new Error(`OpenAI/OpenRouter failed (${resp.status}): ${errorBody}`)
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

// ─── OpenRouter (free models, OpenAI-compatible) ─────────────────────────────
async function fetchOpenRouterResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('OPENROUTER_API_KEY')
  if (!apiKey) throw new Error('Missing OPENROUTER_API_KEY')

  // Free models verified working — ordered by reliability based on live testing
  const freeModels = [
    'liquid/lfm-2.5-1.2b-instruct:free',         // ✅ confirmed working
    'nvidia/nemotron-3-nano-30b-a3b:free',         // ✅ confirmed working
    'nvidia/nemotron-3-super-120b-a12b:free',      // nvidia tier 2 fallback
    'arcee-ai/trinity-large-preview:free',          // arcee fallback
    'cognitivecomputations/dolphin-mistral-24b-venice-edition:free', // dolphin fallback
    'google/gemma-3-4b-it:free',                   // ✅ responds (small but available)
    'meta-llama/llama-3.3-70b-instruct:free',      // best quality when not rate-limited
    'google/gemma-3-27b-it:free',                  // quality fallback when available
  ]

  let lastError = 'No OpenRouter models attempted'

  for (const model of freeModels) {
    try {
      logger.info(`[OPENROUTER] Attempting: ${model}`)
      const resp = await fetch('https://openrouter.ai/api/v1/chat/completions', {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${apiKey}`,
          'Content-Type': 'application/json',
          'HTTP-Referer': 'https://chyren-web.vercel.app',
          'X-Title': 'Chyren Sovereign Intelligence',
        },
        body: JSON.stringify({
          model,
          messages: [{ role: 'system', content: systemPrompt }, ...history],
          temperature,
        }),
      })

      if (resp.ok) {
        const payload = (await resp.json().catch(() => ({}))) as {
          choices?: Array<{ message?: { content?: string } }>
        }
        const content = payload.choices?.[0]?.message?.content?.trim() ?? ''
        if (content) return createSingleSseTextResponse(content)
        lastError = `OpenRouter model ${model} returned empty response`
        continue
      }

      lastError = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
      logger.warn(`[OPENROUTER] Model ${model} failed (${resp.status}): ${lastError}`)
    } catch (err) {
      lastError = err instanceof Error ? err.message : 'Unknown OpenRouter error'
      logger.warn(`[OPENROUTER] Model ${model} exception: ${lastError}`)
    }
  }

  throw new Error(`OpenRouter fallback chain failed: ${lastError}`)
}

// ─── HuggingFace Serverless Inference API ────────────────────────────────────
async function fetchHuggingFaceResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('HUGGINGFACE_API_KEY')
  if (!apiKey) throw new Error('Missing HUGGINGFACE_API_KEY')

  const models = [
    'mistralai/Mistral-7B-Instruct-v0.3',
    'HuggingFaceH4/zephyr-7b-beta',
    'meta-llama/Meta-Llama-3-8B-Instruct',
  ]

  let lastError = 'No HuggingFace models attempted'

  for (const model of models) {
    try {
      logger.info(`[HUGGINGFACE] Attempting: ${model}`)
      const resp = await fetch(
        `https://api-inference.huggingface.co/models/${model}/v1/chat/completions`,
        {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${apiKey}`,
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            model,
            messages: [{ role: 'system', content: systemPrompt }, ...history],
            temperature,
            max_tokens: 512,
          }),
        },
      )

      if (resp.ok) {
        const payload = (await resp.json().catch(() => ({}))) as {
          choices?: Array<{ message?: { content?: string } }>
        }
        const content = payload.choices?.[0]?.message?.content?.trim() ?? ''
        if (content) return createSingleSseTextResponse(content)
        lastError = `HuggingFace model ${model} returned empty response`
        continue
      }

      lastError = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
      logger.warn(`[HUGGINGFACE] Model ${model} failed (${resp.status}): ${lastError}`)
    } catch (err) {
      lastError = err instanceof Error ? err.message : 'Unknown HuggingFace error'
      logger.warn(`[HUGGINGFACE] Model ${model} exception: ${lastError}`)
    }
  }

  throw new Error(`HuggingFace fallback chain failed: ${lastError}`)
}


async function fetchGeminiResponse(
  history: ChatMsg[],
  systemPrompt: string,
  temperature: number,
): Promise<Response> {
  const apiKey = getOptionalEnv('GEMINI_API_KEY')
  if (!apiKey) throw new Error('Missing required env var: GEMINI_API_KEY')

  const userConfiguredModel = getOptionalEnv('GEMINI_MODEL')
  const geminiModels = Array.from(new Set([
    userConfiguredModel,
    'gemini-2.5-flash',
    'gemini-2.0-flash',
    'gemini-2.0-flash-lite',
  ])).filter(Boolean) as string[]

  let lastError = 'No Gemini models attempted'

  for (const model of geminiModels) {
    try {
      logger.info(`[GEMINI] Attempting model: ${model}`)
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

      if (resp.ok) {
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

        if (content) return createSingleSseTextResponse(content)
        lastError = `Gemini model ${model} returned empty response`
        continue
      }

      lastError = await resp.text().catch(() => `${resp.status} ${resp.statusText}`)
      logger.warn(`[GEMINI] Model ${model} failed: ${lastError}`)
    } catch (err) {
      lastError = err instanceof Error ? err.message : 'Unknown Gemini error'
      logger.warn(`[GEMINI] Model ${model} exception: ${lastError}`)
    }
  }

  throw new Error(`Gemini fallback chain failed: ${lastError}`)
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
  const ip = clientIp(req)
  const allowed = await checkRateLimit(ip)
  if (!allowed) {
    logger.warn(`[CHAT] Rate limit exceeded for ${ip}`)
    return new Response(JSON.stringify({ error: 'Rate limit exceeded. Please slow down.' }), {
      status: 429,
      headers: { 'Content-Type': 'application/json', 'Retry-After': '60' },
    })
  }

  const { message, messages } = await req.json().catch(() => ({}))
  const session = req.nextUrl.searchParams.get('session') ?? 'global'

  const content = messages?.length
    ? messages[messages.length - 1].content
    : message;

  if (!content) {
    return new Response(JSON.stringify({ error: 'Message is required' }), { status: 400 });
  }

  if (typeof content === 'string' && checkPromptInjection(content)) {
    logger.warn(`[CHAT] Prompt injection attempt blocked from ${ip}`)
    return new Response(
      JSON.stringify({ error: 'Request blocked by integrity gate.' }),
      { status: 400, headers: { 'Content-Type': 'application/json' } },
    )
  }

  logger.info(`[CHAT] Session ${session.slice(0, 8)}… — incoming message`)

  const familyAuth = await processFamilyAuthMessage(session, content)
  if (familyAuth.handled && familyAuth.reply) {
    return createSingleSseTextResponse(familyAuth.reply)
  }

  const memberContext = await getVerifiedMemberContext(session)
  const history = toChatHistory(messages, content)
  const { prompt: baseSystemPrompt, profile } = await buildSystemPrompt(session, memberContext)
  const knowledgeContext = await buildKnowledgeContext(content)
  const systemPrompt = knowledgeContext ? baseSystemPrompt + knowledgeContext : baseSystemPrompt
  let hubFailure: string | null = null

  // Tool-use path: when a provider key + LIC catalog are both present, let the
  // model call librarian tools before answering. Uses OpenAI-compatible format
  // so it works with OpenRouter (OPENAI_API_KEY=sk-or-...) or Gemini directly.
  // Falls through to the provider chain on any failure.
  const hasToolProvider = !!(getOptionalEnv('OPENAI_API_KEY') || getOptionalEnv('GEMINI_API_KEY') || getOptionalEnv('OPENROUTER_API_KEY'))
  const catalogConfigured = Boolean(getOptionalEnv('OMEGA_CATALOG_DB_URL'))
  if (hasToolProvider && catalogConfigured) {
    try {
      const userAssistantHistory = history.filter(
        (m): m is { role: 'user' | 'assistant'; content: string } =>
          m.role === 'user' || m.role === 'assistant',
      )
      const { text, toolCalls } = await runAnthropicWithTools(
        '',
        '',
        systemPrompt,
        userAssistantHistory,
        profile.temperature,
      )
      if (text) {
        if (toolCalls.length > 0) {
          logger.info(`[CHAT] Tool-use path: ${toolCalls.length} call(s) — ${toolCalls.map((c) => c.name).join(', ')}`)
        }
        return createSingleSseTextResponse(text)
      }
      logger.warn('[CHAT] Tool-use returned empty text — falling back')
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'unknown tool-use error'
      logger.warn(`[CHAT] Tool-use failed, falling back: ${msg}`)
    }
  }

  try {
    // Fallback order: Anthropic → Gemini → OpenRouter (free) → Groq → OpenAI
    const providers = [
      { name: 'Anthropic',   fn: () => fetchAnthropicResponse(history, systemPrompt, profile.temperature) },
      { name: 'Gemini',      fn: () => fetchGeminiResponse(history, systemPrompt, profile.temperature) },
      { name: 'OpenRouter',  fn: () => fetchOpenRouterResponse(history, systemPrompt, profile.temperature) },
      { name: 'HuggingFace', fn: () => fetchHuggingFaceResponse(history, systemPrompt, profile.temperature) },
      { name: 'Groq',        fn: () => fetchGroqResponse(history, systemPrompt, profile.temperature) },
      { name: 'OpenAI',      fn: () => fetchOpenAIResponse(history, systemPrompt, profile.temperature) },
    ]

    const allErrors: string[] = []
    for (const provider of providers) {
      try {
        const resp = await provider.fn()
        if (!resp) continue

        if (resp.body) {
          logger.info(`[CHAT] Success via ${provider.name}`)
          return new Response(resp.body, { headers: sseHeaders() })
        }
        return resp
      } catch (error) {
        const msg = error instanceof Error ? error.message : 'unknown provider failure'
        allErrors.push(`${provider.name}: ${msg}`)
        logError(`[CHAT] Provider ${provider.name} failed`, error)
      }
    }

    if (getOptionalEnv('CHYREN_API_URL')) {
      try {
        const hubResp = await fetchHubStream(content, session, profile, memberContext)
        if (hubResp.ok && hubResp.body) {
          logger.info('[CHAT] Routing via Sovereign Hub')
          return new Response(hubResp.body, { headers: sseHeaders() })
        }
        hubFailure = await hubResp.text().catch(() => `Hub status ${hubResp.status}`)
        logger.warn(`[HUB] Non-OK response, falling back: ${hubFailure}`)
      } catch (hubErr: unknown) {
        hubFailure = hubErr instanceof Error ? hubErr.message : 'Unknown hub fetch error'
        logger.warn(`[HUB] Fetch failed, falling back: ${hubFailure}`)
      }
    }

    throw new Error(`All providers failed or skipped:\n${allErrors.join(' | ')}`)
  } catch (err: unknown) {
    const _errMsg = err instanceof Error ? err.message : 'unknown error'
    logError('[CHAT] Upstream failure', err, { hubFailure })

    // Clean user-facing message — keep diagnostics server-side only
    const offlineMessage = hubFailure
      ? `I'm temporarily unreachable — my neural links are being recalibrated. Try again in a moment.`
      : `My cognitive systems are still initializing. The sovereign hub will be online shortly.`

    logger.warn(`[CHAT] Diagnostic detail (not shown to user): ${_errMsg}`)
    return createSingleSseTextResponse(offlineMessage)
  }
}

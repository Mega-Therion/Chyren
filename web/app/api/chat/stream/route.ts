import { type NextRequest } from 'next/server'
import { streamText, type CoreMessage } from 'ai'
import { experimental_wrapLanguageModel as wrapLanguageModel } from 'ai'
import { fallback } from '@ai-sdk/provider'
import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContextAsync } from '@/lib/neon-context'
import {
  getVerifiedMemberContext,
  processFamilyAuthMessage,
} from '@/lib/family-auth'
import { logger, logError } from '@/lib/logger'
import { checkRateLimit, checkPromptInjection, clientIp } from '@/lib/hardening'
import { semanticKnowledgeSearch } from '@/lib/librarian/knowledge-vector'
import { ariGate, type AriGateResult } from '@/lib/ari-gate'
import { anthropic, google, groq, openai } from '@/lib/ai-gateway'
import { getSovereignTools } from '@/lib/ai-sdk-tools'

// Base system prompt is resolved per-request (async) so live-fetched Neon
// context is available even when the build-time bake was empty (quota issues, etc.)
async function getBaseSystemPrompt(): Promise<string> {
  const ctx = await getRYContextAsync()
  return ctx ? CHYREN_SYSTEM_PROMPT + '\n\n' + ctx : CHYREN_SYSTEM_PROMPT
}

export const runtime = 'nodejs'
export const maxDuration = 60



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

function createSingleSseTextResponse(text: string, ari?: AriGateResult): Response {
  const payload: Record<string, unknown> = {
    choices: [{ delta: { content: text } }],
  }
  if (ari) {
    payload.ari = {
      allowed:    ari.allowed,
      riskTier:  ari.riskTier,
      adcclScore: ari.adcclScore,
      ledgerHash: ari.ledgerHash.slice(0, 16),  // abbreviated for wire
      admittedAt: ari.admittedAt,
    }
  }
  const body = `data: ${JSON.stringify(payload)}\n\n`
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

  // ── ARI GATE — C.A.S. + I.A.F. + ADCCL pre-flight ──────────────────────
  const ariResult = await ariGate(typeof content === 'string' ? content : '')
  if (!ariResult.allowed) {
    logger.warn(`[ARI] Gate rejected intent — ${ariResult.rejectionReason}`)
    const rejection = ariResult.iafOk
      ? `My alignment layer has evaluated this request and cannot proceed. (ADCCL: ${ariResult.adcclScore.toFixed(2)}, Tier: ${ariResult.riskTier})`
      : `I.A.F. safety floor triggered — this request falls outside my sovereign alignment boundary.`
    return createSingleSseTextResponse(rejection, ariResult)
  }
  logger.info(`[ARI] Gate passed — tier=${ariResult.riskTier} adccl=${ariResult.adcclScore.toFixed(2)}`)
  // ─────────────────────────────────────────────────────────────────────────

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
  
  try {
    const sovereignTools = await getSovereignTools()
    
    // Convert history to AI SDK CoreMessage format
    const sdkMessages: CoreMessage[] = history.map(m => ({
      role: m.role,
      content: m.content
    }))

    const result = await streamText({
      model: fallback([
        anthropic('claude-3-5-sonnet-20241022'),
        google('gemini-2.0-flash'),
        groq('llama-3.3-70b-versatile'),
        openai('gpt-4o-mini')
      ]),
      system: systemPrompt,
      messages: sdkMessages,
      tools: sovereignTools,
      maxSteps: 5,
      temperature: profile.temperature,
      experimental_telemetry: {
        isEnabled: true,
        functionId: 'chyren-chat-v2'
      }
    })

    return result.toDataStreamResponse({
      headers: {
        'X-Chyren-Session': session,
        'X-Chyren-ARI': JSON.stringify({
          allowed: ariResult.allowed,
          riskTier: ariResult.riskTier,
          adcclScore: ariResult.adcclScore
        })
      }
    })

  } catch (err: unknown) {
    const _errMsg = err instanceof Error ? err.message : 'unknown error'
    logError('[CHAT] Upstream failure', err)

    // Final Sovereign Hub Fallback (direct fetch)
    if (getOptionalEnv('CHYREN_API_URL')) {
      try {
        const hubResp = await fetchHubStream(content, session, profile, memberContext)
        if (hubResp.ok && hubResp.body) {
          logger.info('[CHAT] Routing via Sovereign Hub')
          return new Response(hubResp.body, { headers: sseHeaders() })
        }
      } catch (hubErr) {
        logger.warn(`[HUB] Final fallback failed: ${hubErr}`)
      }
    }

    const offlineMessage = `My cognitive systems are still initializing. The sovereign hub will be online shortly.`
    return createSingleSseTextResponse(offlineMessage)
  }
}

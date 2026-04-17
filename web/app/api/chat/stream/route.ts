import { type NextRequest } from 'next/server'
import { streamText, type CoreMessage } from 'ai'
import { openai } from '@ai-sdk/openai'
import { anthropic } from '@ai-sdk/anthropic'
import { google } from '@ai-sdk/google'
import { groq } from '@ai-sdk/groq'

import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContextAsync } from '@/lib/neon-context'
import {
  getVerifiedMemberContext,
  processFamilyAuthMessage,
} from '@/lib/family-auth'
import { logger, logError } from '@/lib/logger'
import { env } from '@/lib/env'

export const runtime = 'nodejs'
export const maxDuration = 60 // Allow longer for slow providers

// Expression Profiles for rhetorical variation
const EXPRESSION_PROFILES = [
  { id: 'precise-formal', guidance: 'Use precise, formal phrasing. Concise structure. Lead with the answer.', temperature: 0.4 },
  { id: 'warm-conversational', guidance: 'Use warm, approachable language. Conversational but efficient.', temperature: 0.6 },
  { id: 'strategic-executive', guidance: 'Focus on intent, outcomes, and principles. High-level but actionable.', temperature: 0.5 },
  { id: 'technical-direct', guidance: 'Prioritize clarity, rationale, and system behavior. Compact wording.', temperature: 0.45 },
]

function getProfile(sessionId: string) {
  let hash = 0
  for (let i = 0; i < sessionId.length; i++) {
    hash = (hash << 5) - hash + sessionId.charCodeAt(i)
    hash |= 0
  }
  return EXPRESSION_PROFILES[Math.abs(hash) % EXPRESSION_PROFILES.length]
}

const CONCISENESS_GATE = `
RESPONSE LENGTH GATE (enforced): Unless explicitly asked for more, limit reply to 1-3 concise sentences.
Lead with the answer. Omit preamble/filler. Be direct.
`

async function getSystemPrompt(session: string, memberCtx: string | null) {
  const ryCtx = await getRYContextAsync()
  const profile = getProfile(session)
  
  return [
    CHYREN_SYSTEM_PROMPT,
    ryCtx ? `\n\nCORE CONTEXT:\n${ryCtx}` : '',
    memberCtx ? `\n\nVERIFIED MEMBER CONTEXT:\n${memberCtx}` : '',
    CONCISENESS_GATE,
    `\nEXPRESSION PROFILE: ${profile.id}\n${profile.guidance}`,
  ].join('\n')
}

export async function POST(req: NextRequest) {
  const { message, messages } = await req.json().catch(() => ({}))
  const session = req.nextUrl.searchParams.get('session') ?? 'global'
  
  const content = messages?.length 
    ? messages[messages.length - 1].content 
    : message

  if (!content) {
    return new Response('Message required', { status: 400 })
  }

  try {
    // 1. Family Auth Gate
    const familyAuth = await processFamilyAuthMessage(session, content)
    if (familyAuth.handled && familyAuth.reply) {
      return new Response(`data: ${JSON.stringify({ choices: [{ delta: { content: familyAuth.reply } }] })}\n\n`, {
        headers: { 'Content-Type': 'text/event-stream' }
      })
    }

    const memberContext = await getVerifiedMemberContext(session)
    const systemPrompt = await getSystemPrompt(session, memberContext)
    const profile = getProfile(session)

    // Convert history to CoreMessage format
    const history: CoreMessage[] = (messages || []).map((m: any) => ({
      role: m.role === 'user' ? 'user' : 'assistant',
      content: m.content
    }))

    // 2. Hub Proxy (Sovereign Primary)
    const hubUrl = process.env.CHYREN_API_URL
    if (hubUrl) {
      try {
        const hubResp = await fetch(`${hubUrl}/api/chat/stream`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            message: content,
            session_id: session,
            response_style_hint: profile.id,
            verified_member_context: memberContext ?? undefined,
          }),
        })
        if (hubResp.ok && hubResp.body) {
          logger.info({ session }, 'Routing to Sovereign Hub')
          return new Response(hubResp.body, {
            headers: {
              'Content-Type': 'text/event-stream',
              'Cache-Control': 'no-cache',
              'Connection': 'keep-alive',
            }
          })
        }
      } catch (err) {
        logError('Hub proxy failed, falling back to local providers', err)
      }
    }

    // 3. Fallback Provider Chain (Enterprise Resilience)
    const providers = [
      { name: 'openai', model: openai(process.env.OPENAI_MODEL || 'gpt-4o-mini') },
      { name: 'google', model: google(process.env.GEMINI_MODEL || 'gemini-2.0-flash-exp') },
      { name: 'anthropic', model: anthropic(process.env.ANTHROPIC_MODEL || 'claude-3-5-haiku-latest') },
      { name: 'groq', model: groq(process.env.GROQ_MODEL || 'llama-3.3-70b-versatile') },
    ].filter(p => {
      if (p.name === 'openai') return !!process.env.OPENAI_API_KEY
      if (p.name === 'google') return !!process.env.GEMINI_API_KEY
      if (p.name === 'anthropic') return !!process.env.ANTHROPIC_API_KEY
      if (p.name === 'groq') return !!process.env.GROQ_API_KEY
      return false
    })

    for (const p of providers) {
      try {
        logger.info({ session, provider: p.name }, 'Attempting AI provider')
        const result = streamText({
          model: p.model,
          system: systemPrompt,
          messages: history,
          temperature: profile.temperature,
          onFinish: ({ text }) => {
            logger.info({ session, provider: p.name, length: text.length }, 'Stream complete')
          }
        })

        return result.toDataStreamResponse({
          sendUsage: false,
          getErrorMessage: (err) => `Provider ${p.name} failed: ${err}`
        })
      } catch (err) {
        logError(`Provider ${p.name} failed`, err)
        continue
      }
    }

    throw new Error('All providers exhausted')

  } catch (err) {
    logError('Global chat stream failure', err)
    return new Response(`data: ${JSON.stringify({ choices: [{ delta: { content: "Chyren is temporarily out of range. Stability protocols initiated." } }] })}\n\n`, {
      headers: { 'Content-Type': 'text/event-stream' }
    })
  }
}

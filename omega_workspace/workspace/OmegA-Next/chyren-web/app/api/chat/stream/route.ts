import { createGroq } from '@ai-sdk/groq'
import { streamText, type ModelMessage } from 'ai'
import { NextRequest } from 'next/server'
import { checkRateLimit, checkPromptInjection } from '@/lib/hardening'
import { setBrainState } from '@/lib/brain-state-store'
import { ADCCL } from '@/lib/adccl'
import { CHYREN_SYSTEM_PROMPT } from '@/lib/phylactery'
import { getRYContext } from '@/lib/neon-context'

// Pre-compute system prompt once at module load (zero per-request overhead).
const BASE_SYSTEM_PROMPT = (() => {
  const ctx = getRYContext()
  return ctx ? CHYREN_SYSTEM_PROMPT + ctx : CHYREN_SYSTEM_PROMPT
})()

export const runtime = 'nodejs'

// Model priority list: primary first, fallbacks after
const MODEL_CHAIN = [
  process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile',
  'llama-3.1-8b-instant',
  'gemma2-9b-it',
]

function getRequiredEnv(name: string): string {
  const v = process.env[name]
  if (!v) throw new Error(`Missing required env var: ${name}`)
  return v
}

function buildModel(modelId: string) {
  const apiKey = getRequiredEnv('GROQ_API_KEY')
  const groq = createGroq({ apiKey })
  return groq(modelId)
}

const adccl = new ADCCL(0.7)

/** Stream a plain-text error message back to the client so the UI shows it. */
function errorStream(message: string): ReadableStream<Uint8Array> {
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
  const ip = req.headers.get('x-forwarded-for') ?? 'unknown'
  if (!await checkRateLimit(ip)) {
    return new Response('Too Many Requests', { status: 429 })
  }

  const session = req.nextUrl.searchParams.get('session') ?? 'global'
  const { messages = [], message } = await req.json().catch(() => ({}))

  const chatMessages: { role: string; content: string }[] = messages.length
    ? messages
    : [{ role: 'user', content: message ?? '' }]

  if (!chatMessages.length || !chatMessages[chatMessages.length - 1]?.content) {
    return new Response(JSON.stringify({ error: 'Message is required' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    })
  }

  const lastUserContent = chatMessages[chatMessages.length - 1]?.content ?? ''
  if (checkPromptInjection(lastUserContent)) {
    return new Response(JSON.stringify({ error: 'Request rejected by integrity filter' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    })
  }

  // System prompt is pre-computed at module load — zero per-request I/O.
  const systemPrompt = BASE_SYSTEM_PROMPT

  void setBrainState(session, { stage: 'provider_call', provider: 0.95, adccl: 0.2 })

  const resetToIdle = () => {
    setTimeout(() => {
      void setBrainState(session, { stage: 'idle', provider: 0.05, ledger: 0.02, adccl: 0.05 })
    }, 3000)
  }

  const typedMessages = chatMessages as ModelMessage[]

  // Try each model in chain until one succeeds (handles rate limits gracefully)
  for (const modelId of MODEL_CHAIN) {
    try {
      console.log(`[chat/stream] attempting provider=groq/${modelId}`)
      const model = buildModel(modelId)

      const result = streamText({
        model,
        system: systemPrompt,
        messages: typedMessages,
        onError: ({ error }) => {
          console.error(`[chat/stream] streamText error on ${modelId}:`, error)
        },
        onFinish: async ({ text }) => {
          const task = chatMessages[chatMessages.length - 1]?.content ?? ''
          const verification = adccl.verify(text, task)
          if (!verification.passed) {
            console.error(`[ADCCL] Integrity failure (score ${verification.score}): ${verification.flags?.join(', ')}`)
            await setBrainState(session, { stage: 'rejected', adccl: verification.score })
          } else {
            await setBrainState(session, { stage: 'ledger_commit', ledger: 0.95, adccl: verification.score })
          }
          resetToIdle()
        },
      })

      // Probe the stream: if first chunk throws a rate-limit error, fall through to next model
      const streamResponse = result.toTextStreamResponse()
      const reader = streamResponse.body?.getReader()
      if (!reader) {
        // No body — fall through
        continue
      }

      // Pass-through stream, intercepting rate-limit signals
      let isRateLimited = false
      const passThrough = new ReadableStream<Uint8Array>({
        async start(controller) {
          const decoder = new TextDecoder()
          try {
            while (true) {
              const { done, value } = await reader.read()
              if (done) break
              const text = decoder.decode(value, { stream: true })
              // Detect Groq rate-limit error propagated through stream
              if (text.includes('rate_limit_exceeded') || text.includes('Rate limit')) {
                isRateLimited = true
                break
              }
              controller.enqueue(value)
            }
          } catch (e) {
            const msg = (e as Error).message ?? ''
            if (msg.includes('rate') || msg.includes('Rate') || msg.includes('429')) {
              isRateLimited = true
            } else {
              controller.error(e)
              return
            }
          } finally {
            if (!isRateLimited) {
              controller.close()
            }
          }
        },
      })

      if (!isRateLimited) {
        return new Response(passThrough, {
          headers: streamResponse.headers,
        })
      }

      console.warn(`[chat/stream] rate-limited on ${modelId}, trying next model`)
      continue
    } catch (err) {
      const msg = (err as Error)?.message ?? String(err)
      const isRateLimit = msg.includes('rate') || msg.includes('Rate') || msg.includes('429') || msg.includes('quota')
      if (isRateLimit) {
        console.warn(`[chat/stream] rate-limit error on ${modelId}: ${msg}`)
        continue // try next model
      }
      // Non-rate-limit error: return immediately
      return new Response(JSON.stringify({ error: msg }), {
        status: 500,
        headers: { 'Content-Type': 'application/json' },
      })
    }
  }

  // All models exhausted — return a user-visible error through the stream
  console.error('[chat/stream] all models rate-limited or unavailable')
  void setBrainState(session, { stage: 'rejected', adccl: 0 })
  resetToIdle()

  return new Response(errorStream('Neural link saturated — all inference pathways are rate-limited. Please wait 60 seconds and retry.'), {
    status: 200,
    headers: { 'Content-Type': 'text/plain; charset=utf-8', 'X-Vercel-AI-Data-Stream': 'v1' },
  })
}

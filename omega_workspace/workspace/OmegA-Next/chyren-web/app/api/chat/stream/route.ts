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

function getRequiredEnv(name: string): string {
  const v = process.env[name]
  if (!v) throw new Error(`Missing required env var: ${name}`)
  return v
}

function getModel() {
  const apiKey = getRequiredEnv('GROQ_API_KEY')
  const groq = createGroq({ apiKey })
  return groq(process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile')
}

const adccl = new ADCCL(0.7)

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

  let model: ReturnType<ReturnType<typeof createGroq>>
  try {
    model = getModel()
  } catch (err) {
    const msg = (err as Error)?.message ?? String(err)
    return new Response(JSON.stringify({ error: msg }), {
      status: 500,
      headers: { 'Content-Type': 'application/json' },
    })
  }

  console.log(
    `[chat/stream] provider=groq/${process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile'}`,
  )

  // System prompt is pre-computed at module load — zero per-request I/O.
  const systemPrompt = BASE_SYSTEM_PROMPT

  void setBrainState(session, { stage: 'provider_call', provider: 0.95, adccl: 0.2 })

  const resetToIdle = () => {
    setTimeout(() => {
      void setBrainState(session, { stage: 'idle', provider: 0.05, ledger: 0.02, adccl: 0.05 })
    }, 3000)
  }

  const typedMessages = chatMessages as ModelMessage[]

  const result = streamText({
    model,
    system: systemPrompt,
    messages: typedMessages,
    onError: ({ error }) => console.error('[chat/stream] streamText error:', error),
    onFinish: async ({ text }) => {
      const task = chatMessages[chatMessages.length - 1]?.content ?? ''
      const verification = adccl.verify(text, task)
      if (!verification.passed) {
        console.error(`[ADCCL] Integrity failure (score ${verification.score}): ${verification.flags.join(', ')}`)
        await setBrainState(session, { stage: 'rejected', adccl: verification.score })
      } else {
        await setBrainState(session, { stage: 'ledger_commit', ledger: 0.95, adccl: verification.score })
      }
      resetToIdle()
    },
  })
  return result.toTextStreamResponse()
}

import { createGoogleGenerativeAI } from '@ai-sdk/google'
import { createGroq } from '@ai-sdk/groq'
import { streamText } from 'ai'
import { NextRequest } from 'next/server'
import { checkRateLimit, checkPromptInjection } from '@/lib/hardening'
import { setBrainState } from '@/lib/brain-state-store'

export const runtime = 'nodejs'

const SYSTEM_PROMPT = `You are Chyren — a sovereign intelligence orchestrator. \
You operate with precision, no stubs, and no hallucinations. \
You route tasks through verified AI providers with integrity checks. \
Be concise, direct, and authoritative.`

function getGroqModel() {
  const groq = createGroq({ apiKey: process.env.GROQ_API_KEY ?? '' })
  return groq(process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile')
}

function getGeminiModel() {
  const google = createGoogleGenerativeAI({ apiKey: process.env.GEMINI_API_KEY ?? '' })
  return google(process.env.GEMINI_MODEL ?? 'gemini-2.0-flash')
}

export async function POST(req: NextRequest) {
  // Rate limit
  const ip = req.headers.get('x-forwarded-for') ?? 'unknown'
  if (!checkRateLimit(ip)) {
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

  // Prompt injection guard
  const lastUserContent = chatMessages[chatMessages.length - 1]?.content ?? ''
  if (checkPromptInjection(lastUserContent)) {
    return new Response(JSON.stringify({ error: 'Request rejected by integrity filter' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    })
  }

  // Provider: Groq primary → Gemini fallback
  const useGroq = Boolean(process.env.GROQ_API_KEY)
  const model = useGroq ? getGroqModel() : getGeminiModel()
  const providerLabel = useGroq ? `groq/${process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile'}` : `gemini/${process.env.GEMINI_MODEL ?? 'gemini-2.0-flash'}`
  console.log(`[chat/stream] provider=${providerLabel}`)

  setBrainState(session, { stage: 'provider_call', provider: 0.95, adccl: 0.2 })

  const resetToIdle = () => {
    setTimeout(() => {
      setBrainState(session, { stage: 'idle', provider: 0.05, ledger: 0.02, adccl: 0.05 })
    }, 3000)
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const typedMessages = chatMessages as any

  const result = streamText({
    model,
    system: SYSTEM_PROMPT,
    messages: typedMessages,
    onError: ({ error }) => console.error('[chat/stream] streamText error:', error),
    onFinish: () => {
      setBrainState(session, { stage: 'ledger_commit', ledger: 0.95, adccl: 0.5 })
      resetToIdle()
    },
  })
  return result.toTextStreamResponse()
}

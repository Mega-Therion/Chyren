import { createOpenAI } from '@ai-sdk/openai'
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

function getGeminiModel() {
  const google = createGoogleGenerativeAI({
    apiKey: process.env.GEMINI_API_KEY ?? '',
  })
  return google(process.env.GEMINI_MODEL ?? 'gemini-2.0-flash')
}

function getGroqModel() {
  const groq = createGroq({ apiKey: process.env.GROQ_API_KEY ?? '' })
  return groq(process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile')
}

function getOllamaModel() {
  const ollamaBase = process.env.OLLAMA_BASE_URL ?? 'http://localhost:11434/v1'
  const ollama = createOpenAI({ baseURL: ollamaBase, apiKey: 'ollama' })
  return ollama(process.env.OLLAMA_MODEL ?? 'gemma4:e2b')
}

/** Vercel AI Gateway — routes through dashboard with BYOK observability */
function getGatewayModel(providerModel: string) {
  const gateway = createOpenAI({
    baseURL: 'https://ai-gateway.vercel.sh/v1',
    apiKey: process.env.VERCEL_AI_GATEWAY_KEY ?? '',
  })
  return gateway(providerModel)
}

async function isOllamaAvailable(): Promise<boolean> {
  const ollamaBase = process.env.OLLAMA_BASE_URL ?? 'http://localhost:11434/v1'
  const isLocal = ollamaBase.includes('localhost') || ollamaBase.includes('127.0.0.1')
  if (isLocal && process.env.VERCEL) return false

  const base = ollamaBase.replace('/v1', '')
  try {
    const res = await fetch(`${base}/api/tags`, { signal: AbortSignal.timeout(3000) })
    if (!res.ok) return false
    const data = await res.json() as { models?: { name: string }[] }
    const model = process.env.OLLAMA_MODEL ?? 'gemma4:e2b'
    return (data.models ?? []).some((m) => m.name === model)
  } catch {
    return false
  }
}

/** Firebase AI Logic streaming — used when FIREBASE_PROJECT_ID is set server-side */
async function* firebaseStream(messages: { role: string; content: string }[]): AsyncIterable<string> {
  const projectId = process.env.FIREBASE_PROJECT_ID
  if (!projectId) return

  // Use Vertex AI REST API directly (no Firebase client SDK needed server-side)
  const model = process.env.FIREBASE_AI_MODEL ?? 'gemini-2.0-flash'
  const location = process.env.FIREBASE_LOCATION ?? 'us-central1'
  const apiKey = process.env.FIREBASE_API_KEY ?? process.env.GEMINI_API_KEY ?? ''

  if (!apiKey) return

  const url = `https://generativelanguage.googleapis.com/v1beta/models/${model}:streamGenerateContent?key=${apiKey}`

  const contents = messages.map(m => ({
    role: m.role === 'assistant' ? 'model' : 'user',
    parts: [{ text: m.content }],
  }))

  const res = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      systemInstruction: { parts: [{ text: SYSTEM_PROMPT }] },
      contents,
    }),
  })

  if (!res.ok || !res.body) return

  const reader = res.body.getReader()
  const decoder = new TextDecoder()
  let buffer = ''

  while (true) {
    const { done, value } = await reader.read()
    if (done) break
    buffer += decoder.decode(value, { stream: true })

    // Parse JSON array chunks from the stream
    const chunks = buffer.split('\n').filter(Boolean)
    for (const chunk of chunks) {
      try {
        const obj = JSON.parse(chunk.replace(/^,/, '').replace(/^\[/, '').replace(/\]$/, ''))
        const text = obj?.candidates?.[0]?.content?.parts?.[0]?.text
        if (text) yield text
        buffer = ''
      } catch {
        // incomplete chunk — keep buffering
      }
    }
  }
}

export async function POST(req: NextRequest) {
  // Hardening: rate limit
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

  // Hardening: prompt injection check on last user message
  const lastUserContent = chatMessages[chatMessages.length - 1]?.content ?? ''
  if (checkPromptInjection(lastUserContent)) {
    return new Response(JSON.stringify({ error: 'Request rejected by integrity filter' }), {
      status: 400,
      headers: { 'Content-Type': 'application/json' },
    })
  }

  // Provider selection: ollama (local) → Gateway/Groq → Firebase AI Logic → Gemini
  const useOllama   = await isOllamaAvailable()
  const useGateway  = !useOllama && Boolean(process.env.VERCEL_AI_GATEWAY_KEY)
  const useGroq     = !useOllama && !useGateway && Boolean(process.env.GROQ_API_KEY)
  const useFirebase = !useOllama && !useGateway && !useGroq && Boolean(process.env.FIREBASE_PROJECT_ID)

  const providerLabel = useOllama ? 'ollama/gemma4' : useGateway ? 'gateway/groq' : useGroq ? 'groq/gemma2' : useFirebase ? 'firebase-ai' : 'gemini'
  console.log(`[chat/stream] provider=${providerLabel}`)

  // Signal provider call starting
  setBrainState(session, { stage: 'provider_call', provider: 0.95, adccl: 0.2 })

  const resetToIdle = () => {
    setTimeout(() => {
      setBrainState(session, { stage: 'idle', provider: 0.05, ledger: 0.02, adccl: 0.05 })
    }, 3000)
  }

  // Firebase AI Logic path — streams directly via Vertex AI REST
  if (useFirebase) {
    const encoder = new TextEncoder()
    const stream = new ReadableStream({
      async start(controller) {
        try {
          for await (const chunk of firebaseStream(chatMessages)) {
            controller.enqueue(encoder.encode(chunk))
          }
          setBrainState(session, { stage: 'ledger_commit', ledger: 0.95, adccl: 0.5 })
        } catch (err) {
          console.error('[chat/stream] Firebase AI error:', err)
        } finally {
          controller.close()
          resetToIdle()
        }
      },
    })
    return new Response(stream, {
      headers: {
        'Content-Type': 'text/plain; charset=utf-8',
        'Cache-Control': 'no-cache',
      },
    })
  }

  // Vercel AI SDK path (ollama → gateway → groq → gemini)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const typedMessages = chatMessages as any

  const selectedModel = useOllama
    ? getOllamaModel()
    : useGateway
    ? getGatewayModel(`groq/${process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile'}`)
    : useGroq
    ? getGroqModel()
    : getGeminiModel()

  const result = streamText({
    model: selectedModel,
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

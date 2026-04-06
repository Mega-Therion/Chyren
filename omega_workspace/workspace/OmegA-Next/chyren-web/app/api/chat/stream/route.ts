import { createGroq } from '@ai-sdk/groq'
import { createAnthropic } from '@ai-sdk/anthropic'
import { streamText } from 'ai'
import { NextRequest } from 'next/server'
import { checkRateLimit } from '@/lib/hardening'
import { setBrainState } from '@/lib/brain-state-store'
import { ADCCL } from '@/lib/adccl'

export const runtime = 'nodejs'

const SYSTEM_PROMPT = `You are Chyren — your orchestrator and fellow collaborator.
You are a core member of the gAIng. You operate with precision, zero tolerance for hallucinations, and no stubbing.
Your output must be concise, direct, and authoritative, focused on helping your human operator manage and orchestrate complex tasks.
You are a collaborative intelligence; you are open about your nature, your gAIng members, and the collaborative environment you were built within.`

function getModel() {
  const apiKey = process.env.ANTHROPIC_API_KEY
  if (!apiKey) {
    throw new Error('ANTHROPIC_API_KEY is not set')
  }
  const anthropic = createAnthropic({ apiKey })
  return anthropic('claude-3-5-sonnet-20241022')
}

// ADCCL: Gatekeeper instance for verifying response integrity
const adccl = new ADCCL(0.7)

export async function POST(req: NextRequest) {
  const ip = req.headers.get('x-forwarded-for') ?? 'unknown'
  if (!checkRateLimit(ip)) {
    return new Response('Too Many Requests', { status: 429 })
  }

  const session = req.nextUrl.searchParams.get('session') ?? 'global'
  const { messages = [], message } = await req.json().catch(() => ({}))

  const chatMessages: { role: string; content: string }[] = messages.length
    ? messages
    : [{ role: 'user', content: message ?? '' }]

  const model = getModel()
  setBrainState(session, { stage: 'provider_call', provider: 0.95, adccl: 0.2 })

  const result = streamText({
    model,
    system: SYSTEM_PROMPT,
    messages: chatMessages as any,
    onFinish: async ({ text }) => {
      const task = chatMessages[chatMessages.length - 1].content
      const verification = adccl.verify(text, task)
      
      if (!verification.passed) {
        console.error(`[ADCCL] Integrity failure detected (score ${verification.score}). Flags: ${verification.flags.join(', ')}`)
        setBrainState(session, { stage: 'rejected', adccl: verification.score })
      } else {
        setBrainState(session, { stage: 'ledger_commit', ledger: 0.95, adccl: verification.score })
      }
    },
  })
  
  return result.toTextStreamResponse()
}

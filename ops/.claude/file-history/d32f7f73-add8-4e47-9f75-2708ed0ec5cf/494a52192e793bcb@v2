import { createGoogleGenerativeAI } from '@ai-sdk/google'
import { createGroq } from '@ai-sdk/groq'
import { generateText } from 'ai'
import { NextRequest, NextResponse } from 'next/server'

const SYSTEM_PROMPT = `You are Chyren — a sovereign intelligence orchestrator. \
You operate with precision, no stubs, and no hallucinations. \
You route tasks through verified AI providers with integrity checks. \
Be concise, direct, and authoritative.`

export async function POST(req: NextRequest) {
  try {
    const body = await req.json()

    // Accept { message } or { messages: [{role, content}] }
    const chatMessages: { role: string; content: string }[] = Array.isArray(body.messages) && body.messages.length
      ? body.messages
      : [{ role: 'user', content: body.message ?? '' }]

    const lastContent = chatMessages[chatMessages.length - 1]?.content ?? ''
    if (!lastContent) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 })
    }

    // Provider selection: GROQ primary, Gemini fallback
    const useGroq = Boolean(process.env.GROQ_API_KEY)

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const typedMessages = chatMessages as any

    let response: string
    if (useGroq) {
      const groq = createGroq({ apiKey: process.env.GROQ_API_KEY ?? '' })
      const model = groq(process.env.GROQ_MODEL ?? 'llama-3.3-70b-versatile')
      const result = await generateText({
        model,
        system: SYSTEM_PROMPT,
        messages: typedMessages,
      })
      response = result.text
    } else {
      const google = createGoogleGenerativeAI({ apiKey: process.env.GEMINI_API_KEY ?? '' })
      const model = google(process.env.GEMINI_MODEL ?? 'gemini-2.0-flash')
      const result = await generateText({
        model,
        system: SYSTEM_PROMPT,
        messages: typedMessages,
      })
      response = result.text
    }

    return NextResponse.json({ response })
  } catch (error) {
    console.error('[chat] error:', error)
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 })
  }
}

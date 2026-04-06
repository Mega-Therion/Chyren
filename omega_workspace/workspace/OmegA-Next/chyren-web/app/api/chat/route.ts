import { createAnthropic } from '@ai-sdk/anthropic'
import { createGoogleGenerativeAI } from '@ai-sdk/google'
import { generateText } from 'ai'
import { NextRequest, NextResponse } from 'next/server'

const SYSTEM_PROMPT = `You are Chyren — a sovereign intelligence orchestrator. \
You operate with precision, no stubs, and no hallucinations. \
You route tasks through verified AI providers with integrity checks. \
Be concise, direct, and authoritative.`

export async function POST(req: NextRequest) {
  try {
    const body = await req.json()

    const chatMessages: { role: string; content: string }[] = Array.isArray(body.messages) && body.messages.length
      ? body.messages
      : [{ role: 'user', content: body.message ?? '' }]

    const lastContent = chatMessages[chatMessages.length - 1]?.content ?? ''
    if (!lastContent) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 })
    }

    const useAnthropic = Boolean(process.env.ANTHROPIC_API_KEY)

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const typedMessages = chatMessages as any

    let model
    if (useAnthropic) {
      const anthropic = createAnthropic({ apiKey: process.env.ANTHROPIC_API_KEY ?? '' })
      model = anthropic(process.env.ANTHROPIC_MODEL ?? 'claude-haiku-4-5-20251001')
    } else {
      const google = createGoogleGenerativeAI({ apiKey: process.env.GEMINI_API_KEY ?? '' })
      model = google(process.env.GEMINI_MODEL ?? 'gemini-2.0-flash')
    }

    const result = await generateText({ model, system: SYSTEM_PROMPT, messages: typedMessages })
    return NextResponse.json({ response: result.text })
  } catch (error) {
    console.error('[chat] error:', error)
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 })
  }
}

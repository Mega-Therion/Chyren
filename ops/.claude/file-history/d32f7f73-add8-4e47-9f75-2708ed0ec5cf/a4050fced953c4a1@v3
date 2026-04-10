import { NextResponse } from 'next/server'

// Temporary diagnostic endpoint — shows which env vars are set (NOT their values)
export async function GET() {
  const vars = ['ANTHROPIC_API_KEY', 'GROQ_API_KEY', 'GEMINI_API_KEY', 'GROQ_MODEL', 'GEMINI_MODEL', 'OPENAI_API_KEY']
  const status: Record<string, { set: boolean; length: number }> = {}
  for (const v of vars) {
    const val = process.env[v] ?? ''
    status[v] = { set: Boolean(val), length: val.length }
  }

  // Quick Anthropic probe
  let anthropicProbe = 'skipped'
  if (process.env.ANTHROPIC_API_KEY) {
    try {
      const res = await fetch('https://api.anthropic.com/v1/messages', {
        method: 'POST',
        headers: {
          'x-api-key': process.env.ANTHROPIC_API_KEY,
          'anthropic-version': '2023-06-01',
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ model: 'claude-haiku-4-5-20251001', max_tokens: 5, messages: [{ role: 'user', content: 'say OK' }] }),
        signal: AbortSignal.timeout(10000),
      })
      const data = await res.json() as { content?: { text: string }[]; error?: { message: string } }
      anthropicProbe = res.ok ? `ok: ${data.content?.[0]?.text}` : `http_${res.status}: ${data.error?.message ?? ''}`
    } catch (e) {
      anthropicProbe = `error: ${(e as Error).message}`
    }
  }

  return NextResponse.json({ env: status, anthropicProbe })
}

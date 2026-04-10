import { NextResponse } from 'next/server'

// Temporary diagnostic endpoint — shows which env vars are set (NOT their values)
export async function GET() {
  const vars = ['GROQ_API_KEY', 'GEMINI_API_KEY', 'GROQ_MODEL', 'GEMINI_MODEL', 'OPENAI_API_KEY']
  const status: Record<string, { set: boolean; length: number }> = {}
  for (const v of vars) {
    const val = process.env[v] ?? ''
    status[v] = { set: Boolean(val), length: val.length }
  }

  // Quick Groq probe
  let groqProbe = 'skipped'
  if (process.env.GROQ_API_KEY) {
    try {
      const res = await fetch('https://api.groq.com/openai/v1/models', {
        headers: { Authorization: `Bearer ${process.env.GROQ_API_KEY}` },
        signal: AbortSignal.timeout(5000),
      })
      groqProbe = res.ok ? 'ok' : `http_${res.status}`
    } catch (e) {
      groqProbe = `error: ${(e as Error).message}`
    }
  }

  return NextResponse.json({ env: status, groqProbe })
}

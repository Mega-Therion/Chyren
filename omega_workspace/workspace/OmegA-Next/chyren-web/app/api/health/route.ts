import { NextResponse } from 'next/server'

/**
 * Same-origin health check for the web deployment (Rust `chyren-api` uses `/health` on port 8080).
 */
export async function GET() {
  const hasCronSecret = Boolean(process.env.CRON_SECRET)
  const hasGroqKey = Boolean(process.env.GROQ_API_KEY)
  const ctxLen = (() => {
    try {
      return (process.env.OMEGA_DB_URL ? 'runtime' : 'build_or_empty')
    } catch {
      return 'unknown'
    }
  })()

  return NextResponse.json({
    status: 'operational',
    timestamp: Date.now() / 1000,
    layer: 'chyren-web',
    config: {
      hasCronSecret,
      hasGroqKey,
      contextMode: ctxLen,
    },
  })
}

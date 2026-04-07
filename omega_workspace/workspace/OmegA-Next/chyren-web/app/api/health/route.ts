import { NextResponse } from 'next/server'

/**
 * Same-origin health check for the web deployment (Rust `chyren-api` uses `/health` on port 8080).
 * Returns minimal status only — no secret enumeration.
 */
export async function GET() {
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
    contextMode: ctxLen,
  })
}

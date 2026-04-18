import { type NextRequest } from 'next/server'
import { getBrainState } from '@/lib/brain-state-store'
import { checkRateLimit } from '@/lib/hardening'

export const runtime = 'nodejs'

/** Validate session param: must be 1-64 word characters (alphanumeric + underscore) */
const SESSION_RE = /^[\w-]{1,64}$/

export async function GET(req: NextRequest) {
  // Rate-limit by IP to prevent SSE abuse
  const ip =
    req.headers.get('x-forwarded-for')?.split(',')[0].trim() ??
    req.headers.get('x-real-ip') ??
    '127.0.0.1'

  const allowed = await checkRateLimit(ip)
  if (!allowed) {
    return new Response(
      JSON.stringify({ error: 'Too many requests' }),
      { status: 429, headers: { 'Content-Type': 'application/json' } }
    )
  }

  const rawSession = req.nextUrl.searchParams.get('session') ?? 'global'

  // Reject sessions that don't match the expected format
  if (!SESSION_RE.test(rawSession)) {
    return new Response(
      JSON.stringify({ error: 'Invalid session identifier' }),
      { status: 400, headers: { 'Content-Type': 'application/json' } }
    )
  }

  const session = rawSession
  const encoder = new TextEncoder()

  const stream = new ReadableStream({
    async start(controller) {
      let lastJson = ''
      const maxTicks = 60
      let tick = 0

      while (tick < maxTicks) {
        const state = await getBrainState(session)
        const json = JSON.stringify(state)

        if (json !== lastJson) {
          controller.enqueue(encoder.encode(`data: ${json}\n\n`))
          lastJson = json
        } else {
          // keepalive comment
          controller.enqueue(encoder.encode(':\n\n'))
        }

        tick++
        await new Promise((r) => setTimeout(r, 500))
      }

      controller.close()
    },
  })

  return new Response(stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache',
      Connection: 'keep-alive',
    },
  })
}

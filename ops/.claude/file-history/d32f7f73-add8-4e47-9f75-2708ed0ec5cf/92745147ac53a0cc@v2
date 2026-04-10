import { NextRequest } from 'next/server'
import { getBrainState } from '@/lib/brain-state-store'

export const runtime = 'nodejs'

export async function GET(req: NextRequest) {
  const session = req.nextUrl.searchParams.get('session') ?? 'global'
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

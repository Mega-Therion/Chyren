/**
 * /api/cron/warm-context — Neon context warmer
 */
import { getRYContext } from '@/lib/neon-context'

export const runtime = 'nodejs'
export const maxDuration = 30

export async function GET() {
  return runWarm();
}

export async function POST() {
  return runWarm();
}

async function runWarm() {
  const start = Date.now()
  try {
    const context = await getRYContext()
    const ms = Date.now() - start
    return Response.json({ ok: true, length: context.length, ms })
  } catch (err) {
    const ms = Date.now() - start
    console.error('[warm-context] error:', (err as Error)?.message)
    return Response.json({ ok: false, error: (err as Error)?.message, ms }, { status: 500 })
  }
}

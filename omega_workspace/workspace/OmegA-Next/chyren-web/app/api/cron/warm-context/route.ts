/**
 * /api/cron/warm-context — Neon context warmer
 */
import { getRYContext } from '@/lib/neon-context'
import type { NextRequest } from 'next/server'

export const runtime = 'nodejs'
export const maxDuration = 30

function isAuthorized(req: NextRequest): boolean {
  const secret = process.env.CRON_SECRET
  if (!secret) return false
  const auth = req.headers.get('authorization') ?? ''
  return auth === `Bearer ${secret}`
}

export async function GET(req: NextRequest) {
  if (!isAuthorized(req)) return new Response('Unauthorized', { status: 401 })
  return runWarm()
}

export async function POST(req: NextRequest) {
  if (!isAuthorized(req)) return new Response('Unauthorized', { status: 401 })
  return runWarm()
}

async function runWarm() {
  const start = Date.now()
  try {
    const context = getRYContext()
    const ms = Date.now() - start
    return Response.json({ ok: true, length: context.length, ms })
  } catch (err) {
    const ms = Date.now() - start
    console.error('[warm-context] error:', (err as Error)?.message)
    return Response.json({ ok: false, error: (err as Error)?.message, ms }, { status: 500 })
  }
}

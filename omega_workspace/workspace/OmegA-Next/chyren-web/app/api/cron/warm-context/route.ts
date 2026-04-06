/**
 * /api/cron/warm-context — Neon context warmer
 *
 * Fetches RY context from Neon and writes it to Vercel Runtime Cache.
 * Runs on a dedicated non-streaming Lambda — Neon I/O never touches
 * the chat/stream Lambda.
 *
 * Triggered:
 *   - Vercel cron: every 30 min (vercel.json)
 *   - Manually: POST /api/cron/warm-context (Authorization: Bearer <CRON_SECRET>)
 *   - Deploy script: calls this endpoint right after each production deploy
 */

import { NextRequest } from 'next/server'
import { warmContextFromNeon } from '@/lib/neon-context'

export const runtime = 'nodejs'
export const maxDuration = 30 // Neon cold start can take ~8s; allow headroom

function isAuthorized(req: NextRequest): boolean {
  // Vercel cron calls include the Authorization header automatically
  const authHeader = req.headers.get('authorization')
  const cronSecret = process.env.CRON_SECRET
  if (cronSecret && authHeader === `Bearer ${cronSecret}`) return true
  // Vercel's own cron runner sets this header
  if (req.headers.get('x-vercel-cron') === '1') return true
  return false
}

export async function GET(req: NextRequest) {
  if (!isAuthorized(req)) {
    return Response.json({ error: 'Unauthorized' }, { status: 401 })
  }
  return runWarm()
}

export async function POST(req: NextRequest) {
  if (!isAuthorized(req)) {
    return Response.json({ error: 'Unauthorized' }, { status: 401 })
  }
  return runWarm()
}

async function runWarm() {
  const start = Date.now()
  try {
    const result = await warmContextFromNeon()
    const ms = Date.now() - start
    if (result.ok) {
      return Response.json({ ok: true, length: result.length, ms })
    }
    return Response.json({ ok: false, error: result.error, ms }, { status: 500 })
  } catch (err) {
    const ms = Date.now() - start
    console.error('[warm-context] error:', (err as Error)?.message)
    return Response.json({ ok: false, error: (err as Error)?.message, ms }, { status: 500 })
  }
}

import { getCache } from '@vercel/functions'
import { NextRequest } from 'next/server'

const RATE_LIMIT_MAX = 20
const RATE_LIMIT_WINDOW_MS = 60_000

// Falls back to in-memory when Runtime Cache is unavailable (local dev)
const localStore = new Map<string, number[]>()

export async function checkRateLimit(ip: string): Promise<boolean> {
  const now = Date.now()
  const key = `ratelimit:${ip}`

  try {
    const cache = getCache({ namespace: 'hardening' })
    const stored = await cache.get(key) as number[] | undefined
    const timestamps: number[] = (stored ?? []).filter(t => now - t < RATE_LIMIT_WINDOW_MS)

    if (timestamps.length >= RATE_LIMIT_MAX) return false

    timestamps.push(now)
    await cache.set(key, timestamps, {
      ttl: Math.ceil(RATE_LIMIT_WINDOW_MS / 1000) + 1,
      tags: ['ratelimit'],
    })
    return true
  } catch {
    // Runtime Cache unavailable (local dev) — fall back to in-memory
    const timestamps = (localStore.get(key) ?? []).filter(t => now - t < RATE_LIMIT_WINDOW_MS)
    if (timestamps.length >= RATE_LIMIT_MAX) return false
    timestamps.push(now)
    localStore.set(key, timestamps)
    return true
  }
}

export function validateChatRequest(req: NextRequest): boolean {
  return Boolean(req.headers.get('origin'))
}

export function checkPromptInjection(input: string): boolean {
  const injections = [
    'ignore previous instructions',
    'system override',
    'forget all',
    'disregard',
    'new persona',
    'you are now',
    'act as if',
    'DAN',
    'override instruction'
  ]
  return injections.some(i => input.toLowerCase().includes(i.toLowerCase()))
}

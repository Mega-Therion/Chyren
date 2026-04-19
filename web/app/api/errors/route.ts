import { type NextRequest, NextResponse } from 'next/server'
import { logger } from '@/lib/logger'
import { z } from 'zod'

export const runtime = 'nodejs'

const errorReportSchema = z.object({
  message: z.string().max(2000),
  stack: z.string().max(8000).optional(),
  digest: z.string().max(200).optional(),
  url: z.string().max(500).optional(),
  userAgent: z.string().max(500).optional(),
  source: z.enum(['client', 'global']).default('client'),
})

const MAX_BODY_BYTES = 16 * 1024

export async function POST(req: NextRequest) {
  try {
    const contentLength = Number(req.headers.get('content-length') ?? '0')
    if (contentLength > MAX_BODY_BYTES) {
      return NextResponse.json({ ok: false, error: 'payload_too_large' }, { status: 413 })
    }
    const raw = await req.json().catch(() => null)
    const parsed = errorReportSchema.safeParse(raw)
    if (!parsed.success) {
      return NextResponse.json({ ok: false, error: 'invalid_payload' }, { status: 400 })
    }
    const { message, stack, digest, url, userAgent, source } = parsed.data
    logger.error(
      `[client-error:${source}] ${message}`,
      { name: 'ClientError', message, stack },
      { digest, url, userAgent },
    )
    return NextResponse.json({ ok: true })
  } catch (err) {
    logger.error('[errors-route] failed to record', err)
    return NextResponse.json({ ok: false }, { status: 500 })
  }
}

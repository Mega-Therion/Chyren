/**
 * /api/tts — Text-to-Speech route
 *
 * Local-first TTS pipeline:
 *   1. Forward text to local Piper / Sherpa-ONNX HTTP server (port 5030 by default)
 *   2. Fallback: Google Cloud TTS (if API key available)
 *   3. Returns audio/wav stream (or audio/mp3)
 *
 * Query params:
 *   ?text=Hello+world    — text to synthesize
 *   ?voice=en_US-amy     — optional voice ID (default: server default)
 *   ?rate=1.0            — optional speed (0.5–2.0)
 *
 * POST body (alternative):
 *   { "text": "Hello world", "voice": "en_US-amy", "rate": 1.0 }
 */

import { type NextRequest, NextResponse } from 'next/server'
import { checkRateLimit, clientIp } from '@/lib/hardening'

export const runtime = 'nodejs'

function getOptionalEnv(name: string): string | null {
  const val = process.env[name]?.trim().replace(/^['"]|['"]$/g, '')
  if (!val || /^(YOUR_|REPLACE)/i.test(val)) return null
  return val
}

// ─── Local Piper / Sherpa-ONNX TTS server ────────────────────────────────────

async function tryLocalTts(
  text: string,
  voice?: string,
  rate?: number,
): Promise<Response | null> {
  const base = getOptionalEnv('PIPER_API_URL') ?? 'http://127.0.0.1:5030'

  try {
    // Piper HTTP server typically accepts GET with query params
    // or POST with JSON body. Try POST first (more robust).
    const body: Record<string, unknown> = { text }
    if (voice) body.voice = voice
    if (rate) body.speed = rate

    const res = await fetch(`${base}/api/tts`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(10_000),
    })

    if (!res.ok || !res.body) return null

    // Piper returns audio/wav directly
    return res
  } catch {
    return null
  }
}

// ─── Sherpa-ONNX HTTP server (alternative local engine) ──────────────────────

async function trySherpaOnnx(
  text: string,
  _voice?: string,
  rate?: number,
): Promise<Response | null> {
  const base = getOptionalEnv('SHERPA_API_URL')
  if (!base) return null

  try {
    const params = new URLSearchParams({ text })
    if (rate) params.set('speed', String(rate))

    const res = await fetch(`${base}/api/tts?${params}`, {
      method: 'GET',
      signal: AbortSignal.timeout(10_000),
    })

    if (!res.ok || !res.body) return null
    return res
  } catch {
    return null
  }
}

// ─── ElevenLabs TTS (free tier: 10k chars/month, natural quality) ────────────

async function tryElevenLabsTts(
  text: string,
  rate?: number,
): Promise<Response | null> {
  const apiKey = getOptionalEnv('ELEVENLABS_API_KEY')
  if (!apiKey) return null

  // Use a natural, casual American male voice by default
  // Adam (pNInz6obpgDQGcFmaJgB) = natural casual male
  const voiceId = getOptionalEnv('ELEVENLABS_VOICE_ID') ?? 'pNInz6obpgDQGcFmaJgB'

  try {
    const res = await fetch(
      `https://api.elevenlabs.io/v1/text-to-speech/${voiceId}`,
      {
        method: 'POST',
        headers: {
          'xi-api-key': apiKey,
          'Content-Type': 'application/json',
          Accept: 'audio/mpeg',
        },
        body: JSON.stringify({
          text,
          model_id: 'eleven_turbo_v2_5',
          voice_settings: {
            stability: 0.45,
            similarity_boost: 0.75,
            style: 0.35,
            use_speaker_boost: true,
            speed: rate ?? 1.05,
          },
        }),
        signal: AbortSignal.timeout(12_000),
      },
    )

    if (!res.ok || !res.body) {
      console.warn('[TTS] ElevenLabs failed:', res.status)
      return null
    }

    return new Response(res.body, {
      headers: {
        'Content-Type': 'audio/mpeg',
        'Cache-Control': 'public, max-age=3600',
      },
    })
  } catch (err) {
    console.error('[TTS] ElevenLabs error:', err)
    return null
  }
}

// ─── Google Cloud TTS fallback ───────────────────────────────────────────────

async function tryGoogleTts(
  text: string,
  voice?: string,
  rate?: number,
): Promise<Response | null> {
  const apiKey = getOptionalEnv('GOOGLE_TTS_API_KEY') ?? getOptionalEnv('GEMINI_API_KEY')
  if (!apiKey) return null

  try {
    const res = await fetch(
      `https://texttospeech.googleapis.com/v1/text:synthesize?key=${encodeURIComponent(apiKey)}`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          input: { text },
          voice: {
            languageCode: 'en-US',
            name: voice ?? 'en-US-Neural2-D',
            ssmlGender: 'MALE',
          },
          audioConfig: {
            audioEncoding: 'MP3',
            speakingRate: rate ?? 1.0,
            pitch: -1.0,
          },
        }),
        signal: AbortSignal.timeout(8_000),
      },
    )

    if (!res.ok) {
      const errText = await res.text().catch(() => `${res.status}`)
      console.warn('[TTS] Google Cloud TTS failed:', errText)
      return null
    }

    const data = await res.json().catch(() => null)
    const audioContent = data?.audioContent as string | undefined
    if (!audioContent) return null

    // audioContent is base64-encoded — decode and return as audio response
    const audioBuffer = Buffer.from(audioContent, 'base64')
    return new Response(audioBuffer, {
      headers: {
        'Content-Type': 'audio/mp3',
        'Content-Length': String(audioBuffer.length),
        'Cache-Control': 'public, max-age=86400',
      },
    })
  } catch (err) {
    console.error('[TTS] Google Cloud TTS error:', err)
    return null
  }
}

// ─── Route handler ───────────────────────────────────────────────────────────

export async function POST(req: NextRequest) {
  if (!(await checkRateLimit(clientIp(req)))) {
    return NextResponse.json({ error: 'Rate limit exceeded' }, { status: 429, headers: { 'Retry-After': '60' } })
  }
  try {
    const body = await req.json().catch(() => ({}))
    const text = (body.text ?? '').trim()
    const voice = body.voice
    const rate = body.rate ? Number(body.rate) : undefined

    if (!text) {
      return NextResponse.json({ error: 'Text is required' }, { status: 400 })
    }

    // Pipeline: ElevenLabs → local Piper → Sherpa-ONNX → Google Cloud TTS
    let ttsResponse = await tryElevenLabsTts(text, rate)

    if (!ttsResponse) {
      ttsResponse = await tryLocalTts(text, voice, rate)
    }

    if (!ttsResponse) {
      ttsResponse = await trySherpaOnnx(text, voice, rate)
    }

    if (!ttsResponse) {
      ttsResponse = await tryGoogleTts(text, voice, rate)
    }

    if (!ttsResponse) {
      return NextResponse.json(
        { error: 'No TTS engine available', fallback: 'browser' },
        { status: 503 },
      )
    }

    // Stream the audio response through
    const contentType = ttsResponse.headers.get('Content-Type') ?? 'audio/wav'
    return new Response(ttsResponse.body, {
      headers: {
        'Content-Type': contentType,
        'Cache-Control': 'public, max-age=3600',
      },
    })
  } catch (err) {
    console.error('[TTS] Unhandled error:', err)
    return NextResponse.json({ error: 'TTS processing failed' }, { status: 500 })
  }
}

// Also support GET for simple integration
export async function GET(req: NextRequest) {
  if (!(await checkRateLimit(clientIp(req)))) {
    return NextResponse.json({ error: 'Rate limit exceeded' }, { status: 429, headers: { 'Retry-After': '60' } })
  }
  const text = req.nextUrl.searchParams.get('text') ?? ''
  const voice = req.nextUrl.searchParams.get('voice') ?? undefined
  const rate = req.nextUrl.searchParams.get('rate')
    ? Number(req.nextUrl.searchParams.get('rate'))
    : undefined

  if (!text.trim()) {
    return NextResponse.json({ error: 'Text is required' }, { status: 400 })
  }

  // Reuse pipeline
  let ttsResponse = await tryElevenLabsTts(text, rate)
  if (!ttsResponse) ttsResponse = await tryLocalTts(text, voice, rate)
  if (!ttsResponse) ttsResponse = await trySherpaOnnx(text, voice, rate)
  if (!ttsResponse) ttsResponse = await tryGoogleTts(text, voice, rate)

  if (!ttsResponse) {
    return NextResponse.json(
      { error: 'No TTS engine available', fallback: 'browser' },
      { status: 503 },
    )
  }

  const contentType = ttsResponse.headers.get('Content-Type') ?? 'audio/wav'
  return new Response(ttsResponse.body, {
    headers: { 'Content-Type': contentType, 'Cache-Control': 'public, max-age=3600' },
  })
}

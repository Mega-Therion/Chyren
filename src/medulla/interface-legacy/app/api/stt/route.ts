/**
 * /api/stt — Speech-to-Text route
 *
 * Local-first STT pipeline:
 *   1. Forward audio to local whisper.cpp server (port 8178 by default)
 *   2. Fallback: Groq whisper-large-v3-turbo API
 *   3. Returns { transcription: string }
 *
 * Accepts multipart/form-data with an "audio" field (audio/webm blob).
 */

import { type NextRequest, NextResponse } from 'next/server'

export const runtime = 'nodejs'

function getOptionalEnv(name: string): string | null {
  const val = process.env[name]?.trim().replace(/^['"]|['"]$/g, '')
  if (!val || /^(YOUR_|REPLACE)/i.test(val)) return null
  return val
}

// ─── Local whisper.cpp server ────────────────────────────────────────────────

async function tryLocalWhisper(audioBlob: Blob): Promise<string | null> {
  const base = getOptionalEnv('WHISPER_API_URL') ?? 'http://127.0.0.1:8178'

  try {
    const form = new FormData()
    form.append('file', audioBlob, 'audio.webm')
    form.append('response_format', 'json')

    const res = await fetch(`${base}/inference`, {
      method: 'POST',
      body: form,
      signal: AbortSignal.timeout(15_000),
    })

    if (!res.ok) return null

    const data = await res.json().catch(() => null)
    // whisper.cpp server returns { text: "..." }
    const text = data?.text?.trim()
    return text || null
  } catch {
    return null
  }
}

// ─── Groq Whisper API fallback ───────────────────────────────────────────────

async function tryGroqWhisper(audioBlob: Blob): Promise<string | null> {
  const apiKey = getOptionalEnv('GROQ_API_KEY')
  if (!apiKey) return null

  try {
    const form = new FormData()
    form.append('file', audioBlob, 'audio.webm')
    form.append('model', 'whisper-large-v3-turbo')
    form.append('response_format', 'json')
    form.append('language', 'en')

    const res = await fetch('https://api.groq.com/openai/v1/audio/transcriptions', {
      method: 'POST',
      headers: { Authorization: `Bearer ${apiKey}` },
      body: form,
      signal: AbortSignal.timeout(20_000),
    })

    if (!res.ok) {
      const errText = await res.text().catch(() => `${res.status}`)
      console.warn('[STT] Groq whisper failed:', errText)
      return null
    }

    const data = await res.json().catch(() => null)
    return data?.text?.trim() || null
  } catch (err) {
    console.error('[STT] Groq whisper error:', err)
    return null
  }
}

// ─── Route handler ───────────────────────────────────────────────────────────

export async function POST(req: NextRequest) {
  try {
    const formData = await req.formData()
    const audioFile = formData.get('audio')

    if (!audioFile || !(audioFile instanceof Blob)) {
      return NextResponse.json({ error: 'No audio file provided' }, { status: 400 })
    }

    // Pipeline: local whisper.cpp → Groq whisper → empty response
    let transcription = await tryLocalWhisper(audioFile)

    if (!transcription) {
      transcription = await tryGroqWhisper(audioFile)
    }

    if (!transcription) {
      return NextResponse.json(
        { transcription: '', error: 'All STT providers unavailable' },
        { status: 200 }, // 200 so the client doesn't crash — empty transcription is safe
      )
    }

    return NextResponse.json({ transcription })
  } catch (err) {
    console.error('[STT] Unhandled error:', err)
    return NextResponse.json({ error: 'STT processing failed' }, { status: 500 })
  }
}

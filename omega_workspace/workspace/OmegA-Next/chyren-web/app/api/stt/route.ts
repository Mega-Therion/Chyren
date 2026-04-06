import { NextRequest, NextResponse } from 'next/server'

export const runtime = 'nodejs'

export async function POST(req: NextRequest) {
  const apiKey = process.env.OPENAI_API_KEY
  if (!apiKey) {
    return NextResponse.json({ error: 'OPENAI_API_KEY is not configured' }, { status: 500 })
  }

  const formData = await req.formData()
  const file = formData.get('audio') as File
  if (!file) return NextResponse.json({ error: 'No audio' }, { status: 400 })

  const upstream = new FormData()
  upstream.append('file', file, file.name || 'voice-input.webm')
  upstream.append('model', 'gpt-4o-mini-transcribe')

  const res = await fetch('https://api.openai.com/v1/audio/transcriptions', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${apiKey}`,
    },
    body: upstream,
  })

  if (!res.ok) {
    const body = await res.text().catch(() => '')
    return NextResponse.json(
      { error: `Transcription failed: ${body.slice(0, 500)}` },
      { status: res.status },
    )
  }

  const data = (await res.json()) as { text?: string }
  const transcription = data.text?.trim()
  if (!transcription) {
    return NextResponse.json({ error: 'No transcription returned from provider' }, { status: 502 })
  }

  return NextResponse.json({ transcription })
}

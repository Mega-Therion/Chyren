import { NextRequest, NextResponse } from 'next/server'

export const runtime = 'nodejs'

export async function POST(req: NextRequest) {
  const apiKey = process.env.OPENAI_API_KEY
  if (!apiKey) {
    return NextResponse.json({ error: 'OPENAI_API_KEY is not configured' }, { status: 500 })
  }

  const { text, voice = 'alloy' } = await req.json()
  if (!text) return NextResponse.json({ error: 'No text provided' }, { status: 400 })

  const res = await fetch('https://api.openai.com/v1/audio/speech', {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      model: 'gpt-4o-mini-tts',
      voice,
      input: text,
      format: 'mp3',
    }),
  })

  if (!res.ok) {
    const body = await res.text().catch(() => '')
    return NextResponse.json(
      { error: `Speech synthesis failed: ${body.slice(0, 500)}` },
      { status: res.status },
    )
  }

  const audioBuffer = Buffer.from(await res.arrayBuffer())

  return new NextResponse(audioBuffer, {
    headers: {
      'Content-Type': 'audio/mpeg',
      'Content-Length': audioBuffer.length.toString(),
    },
  })
}

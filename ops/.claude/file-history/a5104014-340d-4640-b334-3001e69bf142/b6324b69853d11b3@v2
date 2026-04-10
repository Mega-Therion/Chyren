import { NextRequest, NextResponse } from 'next/server'

export async function POST(req: NextRequest) {
  const { text } = await req.json()
  if (!text) return NextResponse.json({ error: 'No text provided' }, { status: 400 })

  // TODO: wire ElevenLabs or Web Speech API backend
  const audioBuffer = Buffer.from('mock-audio-data')
  return new NextResponse(audioBuffer, {
    headers: {
      'Content-Type': 'audio/mpeg',
      'Content-Length': audioBuffer.length.toString(),
    },
  })
}

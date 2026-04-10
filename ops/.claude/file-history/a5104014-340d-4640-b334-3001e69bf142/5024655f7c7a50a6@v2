import { NextRequest, NextResponse } from 'next/server'

export async function POST(req: NextRequest) {
  const formData = await req.formData()
  const file = formData.get('audio') as File
  if (!file) return NextResponse.json({ error: 'No audio' }, { status: 400 })

  // TODO: wire Whisper/STT
  return NextResponse.json({ transcription: 'System voice input verified.' })
}

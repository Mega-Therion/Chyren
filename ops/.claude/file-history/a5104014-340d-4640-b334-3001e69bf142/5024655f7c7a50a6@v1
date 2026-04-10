import { NextRequest, NextResponse } from 'next/server';

export async function POST(req: NextRequest) {
    // 1. Receive audio blob
    const formData = await req.formData();
    const file = formData.get('audio') as File;
    if (!file) return NextResponse.json({ error: 'No audio' }, { status: 400 });

    // 2. Placeholder for Whisper/STT integration
    // 3. Normalize transcription
    return NextResponse.json({ transcription: "System voice input verified." });
}

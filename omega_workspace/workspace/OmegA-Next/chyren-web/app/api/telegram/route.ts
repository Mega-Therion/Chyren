import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();
  const { message } = body;
  
  if (!message) {
    return NextResponse.json({ error: 'No message provided' }, { status: 400 });
  }

  // TODO: Integrate with OMEGA orchestrator (e.g., call local bridge/service)
  console.log('Received message from Telegram:', message);

  return NextResponse.json({ status: 'success', received: message });
}

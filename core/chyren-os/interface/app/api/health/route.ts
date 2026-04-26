import { NextResponse } from 'next/server'

export async function GET() {
  return NextResponse.json(
    {
      status: 'healthy',
      system: 'Chyren Sovereign Web Hub',
      timestamp: new Date().toISOString(),
      version: '1.0.0-enterprise',
      env: process.env.NODE_ENV,
    },
    { status: 200 }
  )
}

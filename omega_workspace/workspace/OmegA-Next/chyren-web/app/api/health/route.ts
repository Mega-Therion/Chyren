import { NextResponse } from 'next/server'

/**
 * Same-origin health check for the web deployment (Rust `chyren-api` uses `/health` on port 8080).
 */
export async function GET() {
  return NextResponse.json({
    status: 'operational',
    timestamp: Date.now() / 1000,
    layer: 'chyren-web',
  })
}

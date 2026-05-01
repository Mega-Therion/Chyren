import { NextResponse } from 'next/server'
import { dataAccess } from '@/lib/dal'

export async function POST(request: Request) {
  try {
    const body = await request.json()
    const { text, media = [], tags = [], platforms = [] } = body

    if (!text) {
      return NextResponse.json({ error: 'Text is required' }, { status: 400 })
    }

    // 1. Persist to Sovereign Ledger (Database)
    const status = await dataAccess.createStatus({ text, media, tags })
    const statusData = status[0]

    // 2. Fan-out to Discord if requested or by default
    if (platforms.includes('discord') || platforms.length === 0) {
      const discordWebhook = process.env.DISCORD_STATUS_WEBHOOK
      if (discordWebhook) {
        try {
          await fetch(discordWebhook, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              content: text,
              embeds: media.length > 0 ? media.map((url: string) => ({ image: { url } })) : undefined
            })
          })
        } catch (error) {
          console.error('Discord fan-out failed:', error)
        }
      }
    }

    // Note: Other platforms (X, Threads, etc.) can be added here or handled via n8n

    return NextResponse.json({
      success: true,
      status: statusData,
      message: 'Status published and fanned out'
    })
  } catch (error) {
    console.error('Status publication failed:', error)
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 })
  }
}

export async function GET() {
  try {
    const statuses = await dataAccess.getStatuses()
    return NextResponse.json(statuses)
  } catch (error) {
    return NextResponse.json({ error: 'Failed to fetch statuses' }, { status: 500 })
  }
}

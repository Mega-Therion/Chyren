import { NextRequest, NextResponse } from 'next/server'

export async function POST(req: NextRequest) {
  try {
    const body = await req.json()
    const { message } = body

    if (!message) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 })
    }

    const encoder = new TextEncoder()

    const lowerMessage = (message as string).toLowerCase()
    let responseText: string
    if (lowerMessage.includes('hello') || lowerMessage.includes('hi')) {
      responseText = "Hello! I'm Chyren, the Sovereign Intelligence Orchestrator. I'm ready to route your tasks through verified AI providers with integrity checks."
    } else if (lowerMessage.includes('help') || lowerMessage.includes('what can')) {
      responseText = 'I can help you with:\n\n• **Task Routing**: Send tasks to multiple AI providers (Anthropic, OpenAI, DeepSeek, Gemini)\n• **Integrity Verification**: Every response is checked for accuracy and drift\n• **Multi-provider**: Get the best response by comparing across providers\n• **Threat Detection**: Built-in safety checks and policy enforcement\n\nWhat would you like to do?'
    } else {
      responseText = `I received your message: "${message}". In production, this would be routed through the Sovereign Hub with verified provider execution. For now, I'm in demo mode. Try asking me about my capabilities or how I work!`
    }

    const words = responseText.split(' ')
    const stream = new ReadableStream({
      async start(controller) {
        for (let i = 0; i < words.length; i++) {
          controller.enqueue(encoder.encode((i === 0 ? '' : ' ') + words[i]))
          await new Promise((resolve) => setTimeout(resolve, 40))
        }
        controller.close()
      },
    })

    return new NextResponse(stream, {
      headers: {
        'Content-Type': 'text/plain; charset=utf-8',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
      },
    })
  } catch (error) {
    console.error('Chat stream error:', error)
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 })
  }
}

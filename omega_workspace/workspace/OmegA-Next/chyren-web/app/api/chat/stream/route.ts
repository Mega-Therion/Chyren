import { NextRequest, NextResponse } from 'next/server'

export async function POST(req: NextRequest) {
  try {
    const body = await req.json()
    const { message, context } = body

    if (!message) {
      return NextResponse.json({ error: 'Message is required' }, { status: 400 })
    }

    // Create a readable stream for the response
    const encoder = new TextEncoder()
    let responseText = ''

    // Simulate streaming response from Chyren backend
    // In production, this would call your actual Rust/Python backend
    const mockResponses: { [key: string]: string } = {
      hello: 'Hello! I\'m Chyren, the Sovereign Intelligence Orchestrator. I\'m ready to route your tasks through verified AI providers with integrity checks.',
      help: 'I can help you with:\n\n• **Task Routing**: Send tasks to multiple AI providers (Anthropic, OpenAI, DeepSeek, Gemini)\n• **Integrity Verification**: Every response is checked for accuracy and drift\n• **Multi-provider**: Get the best response by comparing across providers\n• **Threat Detection**: Built-in safety checks and policy enforcement\n\nWhat would you like to do?',
      default: `I received your message: "${message}". In production, this would be routed through the Sovereign Hub with verified provider execution. For now, I'm in demo mode. Try asking me about my capabilities or how I work!`,
    }

    const lowerMessage = message.toLowerCase()
    if (lowerMessage.includes('hello') || lowerMessage.includes('hi')) {
      responseText = mockResponses.hello
    } else if (lowerMessage.includes('help') || lowerMessage.includes('what can')) {
      responseText = mockResponses.help
    } else {
      responseText = mockResponses.default
    }

    // Stream the response character by character
    const stream = new ReadableStream({
      async start(controller) {
        for (const char of responseText) {
          controller.enqueue(encoder.encode(char))
          await new Promise((resolve) => setTimeout(resolve, 10))
        }
        controller.close()
      },
    })

    return new NextResponse(stream, {
      headers: {
        'Content-Type': 'text/event-stream',
        'Cache-Control': 'no-cache',
        'Connection': 'keep-alive',
      },
    })
  } catch (error) {
    console.error('Chat stream error:', error)
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 })
  }
}

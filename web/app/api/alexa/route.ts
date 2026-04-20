/**
 * /api/alexa — Alexa Skill HTTPS endpoint
 *
 * Handles all Alexa skill requests for the Chyren voice interface.
 * Routes voice queries through the Chyren chat stream API and returns
 * both spoken responses and APL visual cards for Echo Show devices.
 *
 * Features:
 *   - Persistent conversational session (no need to re-invoke the skill)
 *   - Multi-turn conversation history via Alexa session attributes
 *   - SSML pronunciation: "Chyren" → /ˈkaɪ.ɹən/ (KY-ren)
 *   - APL visual display for Echo Show devices
 *
 * Endpoint: POST https://chyren-web.vercel.app/api/alexa
 */

import { type NextRequest, NextResponse } from 'next/server'

export const runtime = 'nodejs'
export const maxDuration = 30

// ── Configuration ─────────────────────────────────────────────────────────────

const CHYREN_CHAT_TIMEOUT_MS = 14_000
const MAX_SESSION_HISTORY = 10  // keep last N turns (user+assistant pairs)

// ── SSML Voice & Pronunciation ────────────────────────────────────────────────

/** IPA pronunciation for "Chyren" → KY-ren (like "siren" with a K) */
const CHYREN_PHONEME = '<phoneme alphabet="ipa" ph="ˈkaɪ.ɹən">Chyren</phoneme>'

/**
 * Amazon Polly voice configuration.
 *
 * "Matthew" (Neural) is the closest match to the web app's Google Neural2-D:
 *   - Male, American English, natural cadence
 *   - Pitch lowered 8% to approximate the web app's -1.0 pitch shift
 *   - Rate at 96% for a measured, authoritative delivery
 *
 * The <amazon:domain name="conversational"> tag activates Polly's
 * conversational speaking style — less robotic, more human pacing.
 */
function wrapSsml(text: string): string {
  // Replace "Chyren" (case-insensitive) with the phoneme tag
  const withPhonemes = text.replace(/\bChyren\b/gi, CHYREN_PHONEME)
  return (
    `<speak>` +
    `<voice name="Matthew">` +
    `<amazon:domain name="conversational">` +
    `<prosody pitch="-8%" rate="96%">` +
    withPhonemes +
    `</prosody>` +
    `</amazon:domain>` +
    `</voice>` +
    `</speak>`
  )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

function getOptionalEnv(name: string): string | null {
  const val = process.env[name]?.trim().replace(/^['"]|['"]$/g, '')
  if (!val || /^(YOUR_|REPLACE)/i.test(val)) return null
  return val
}

// ── Session History ───────────────────────────────────────────────────────────

type ChatMessage = { role: 'user' | 'assistant'; content: string }

/** Extract conversation history from Alexa session attributes. */
function getSessionHistory(
  envelope: AlexaRequest,
): ChatMessage[] {
  const attrs = envelope.session?.attributes as
    | Record<string, unknown>
    | undefined
  const history = attrs?.conversationHistory
  if (Array.isArray(history)) {
    return history as ChatMessage[]
  }
  return []
}

/** Trim history to the last N message pairs to stay within limits. */
function trimHistory(history: ChatMessage[]): ChatMessage[] {
  if (history.length <= MAX_SESSION_HISTORY * 2) return history
  return history.slice(-(MAX_SESSION_HISTORY * 2))
}

/**
 * Call the Chyren chat API with full conversation history for context.
 * Uses a line-buffered stream reader to handle SSE data in real-time.
 */
async function askChyren(
  query: string,
  history: ChatMessage[],
): Promise<string | null> {
  const base =
    getOptionalEnv('NEXT_PUBLIC_API_BASE_URL') || 'https://chyren-web.vercel.app'
  const endpoint = `${base}/api/chat/stream`
  const controller = new AbortController()
  
  const startTime = Date.now()
  const fetchTimeout = setTimeout(() => controller.abort(), CHYREN_CHAT_TIMEOUT_MS)

  // Build the full message array with history + new query
  const messages: ChatMessage[] = [
    ...history,
    { role: 'user', content: query },
  ]

  try {
    const res = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        message: query,
        messages,
      }),
      signal: controller.signal,
    })

    if (!res.ok) {
      console.error(`[ALEXA] Chyren chat API returned ${res.status}`)
      return null
    }

    const reader = res.body?.getReader()
    if (!reader) return null

    const decoder = new TextDecoder()
    let accumulatedText = ''
    let lineBuffer = ''
    let done = false

    while (!done) {
      // 9-second safety cutoff for Alexa
      if (Date.now() - startTime > 9000) {
        console.warn('[ALEXA] Approaching 9s timeout, returning partial response')
        break
      }

      const { value, done: readerDone } = await reader.read()
      done = readerDone

      if (value) {
        lineBuffer += decoder.decode(value, { stream: true })
        const lines = lineBuffer.split('\n')
        lineBuffer = lines.pop() || ''

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            try {
              const json = JSON.parse(line.slice(6))
              const content = json?.choices?.[0]?.delta?.content
              if (content) accumulatedText += content
            } catch {
              // skip malformed
            }
          }
        }
      }
    }

    return accumulatedText.trim() || null
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : 'unknown'
    console.error('[ALEXA] Chyren chat API error:', msg)
    return null
  } finally {
    clearTimeout(fetchTimeout)
  }
}

/** Strip markdown for cleaner speech output. */
function cleanForSpeech(text: string | null): string | null {
  if (!text) return text
  return text
    .replace(/```[\s\S]*?```/g, ' (code block omitted) ')
    .replace(/`([^`]+)`/g, '$1')
    .replace(/\*\*([^*]+)\*\*/g, '$1')
    .replace(/\*([^*]+)\*/g, '$1')
    .replace(/#{1,6}\s/g, '')
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
    .replace(/[-*]\s/g, '')
    .replace(/\n{2,}/g, '. ')
    .replace(/\n/g, ' ')
    .trim()
}

/** Truncate text for Alexa speech (keep under 8000 char SSML limit). */
function truncateForSpeech(text: string | null, maxLen = 5500): string | null {
  if (!text) return text
  if (text.length <= maxLen) return text
  return text.slice(0, maxLen) + '... I have more, but that covers the key points.'
}

// ── APL Template for Echo Show (10/10 Premium) ──────────────────────────────

interface AplDirective {
  type: string
  version: string
  document: Record<string, unknown>
  datasources: Record<string, unknown>
}

/**
 * Builds a high-fidelity, glassmorphic APL document for Echo Show devices.
 * Features:
 *  - Premium Neural Network background image
 *  - Pulsing "Neural Core" vector animation
 *  - Glassmorphic message bubbles for conversation history
 *  - Automatic scrolling to the latest message
 */
function buildAplDirective(title: string, currentText: string, history: { role: string; content: string }[]): AplDirective {
  // Map history to APL-friendly datasource
  const chatItems = [
    ...history.slice(-6), // last 6 messages
    { role: 'assistant', content: currentText }
  ].map((item, idx) => ({
    id: `msg-${idx}`,
    text: item.content,
    isUser: item.role === 'user',
    isLast: false
  }))
  
  if (chatItems.length > 0) {
    chatItems[chatItems.length - 1].isLast = true
  }

  return {
    type: 'Alexa.Presentation.APL.RenderDocument',
    version: '1.9',
    document: {
      type: 'APL',
      version: '1.9',
      theme: 'dark',
      import: [{ name: 'alexa-layouts', version: '1.7.0' }],
      mainTemplate: {
        parameters: ['payload'],
        items: [
          {
            type: 'Container',
            width: '100vw',
            height: '100vh',
            items: [
              // 1. Premium Background Image
              {
                type: 'Image',
                source: 'https://chyren-web.vercel.app/alexa_bg.png',
                width: '100vw',
                height: '100vh',
                position: 'absolute',
                scale: 'best-fill',
                filters: [{ type: 'Blur', radius: '10dp' }, { type: 'Grayscale', amount: 0.2 }]
              },
              // 2. Pulsing Neural Core (Vector Graphic)
              {
                type: 'Container',
                position: 'absolute',
                width: '100vw',
                height: '100vh',
                alignItems: 'center',
                justifyContent: 'center',
                opacity: 0.4,
                items: [
                  {
                    type: 'Frame',
                    width: '300dp',
                    height: '300dp',
                    borderRadius: '150dp',
                    backgroundColor: '#BD93F9',
                    opacity: 0.2,
                    item: {
                      type: 'Frame',
                      width: '100%',
                      height: '100%',
                      borderRadius: '150dp',
                      borderWidth: '2dp',
                      borderColor: '#8BE9FD'
                    }
                  }
                ],
                // Simple pulse animation
                onMount: [
                  {
                    type: 'AnimateItem',
                    duration: 4000,
                    repeatCount: -1,
                    repeatMode: 'reverse',
                    value: [
                      { property: 'opacity', from: 0.2, to: 0.5 },
                      { property: 'transform', from: [{ scale: 0.8 }], to: [{ scale: 1.2 }] }
                    ]
                  }
                ]
              },
              // 3. Main Content Layer
              {
                type: 'Container',
                width: '100vw',
                height: '100vh',
                paddingLeft: '40dp',
                paddingRight: '40dp',
                paddingTop: '20dp',
                paddingBottom: '20dp',
                items: [
                  // Header
                  {
                    type: 'AlexaHeader',
                    headerTitle: '${payload.title}',
                    headerAttributionText: 'Sovereign Intelligence',
                    headerDivider: true
                  },
                  // Chat Stream
                  {
                    type: 'Sequence',
                    width: '100%',
                    grow: 1,
                    paddingTop: '20dp',
                    data: '${payload.chatItems}',
                    item: {
                      type: 'Container',
                      width: '100%',
                      marginBottom: '16dp',
                      alignItems: '${data.isUser ? "end" : "start"}',
                      items: [
                        {
                          type: 'Frame',
                          maxWidth: '80%',
                          paddingLeft: '16dp',
                          paddingRight: '16dp',
                          paddingTop: '12dp',
                          paddingBottom: '12dp',
                          borderRadius: '12dp',
                          backgroundColor: '${data.isUser ? "rgba(98, 114, 164, 0.4)" : "rgba(40, 42, 54, 0.6)"}',
                          borderWidth: '1dp',
                          borderColor: '${data.isLast ? "#BD93F9" : "rgba(255,255,255,0.1)"}',
                          item: {
                            type: 'Text',
                            text: '${data.text}',
                            fontSize: '20dp',
                            color: '#F8F8F2',
                            lineHeight: '1.4'
                          }
                        }
                      ]
                    }
                  },
                  // Footer
                  {
                    type: 'AlexaFooter',
                    footerHint: 'Try "What is your architecture?"'
                  }
                ]
              }
            ]
          }
        ]
      }
    },
    datasources: {
      payload: { 
        title, 
        chatItems 
      }
    }
  }
}

// ── Alexa request/response types (minimal) ────────────────────────────────────

interface AlexaRequest {
  version: string
  session?: {
    new?: boolean
    sessionId?: string
    attributes?: Record<string, unknown>
    [key: string]: unknown
  }
  context?: {
    System?: {
      device?: {
        supportedInterfaces?: Record<string, unknown>
      }
    }
  }
  request: {
    type: string
    requestId?: string
    timestamp?: string
    locale?: string
    intent?: {
      name: string
      confirmationStatus?: string
      slots?: Record<
        string,
        {
          name: string
          value?: string
          confirmationStatus?: string
        }
      >
    }
    reason?: string
    error?: Record<string, unknown>
  }
}

function supportsAPL(envelope: AlexaRequest): boolean {
  return !!(
    envelope.context?.System?.device?.supportedInterfaces?.[
      'Alexa.Presentation.APL'
    ]
  )
}

// ── Response builder ──────────────────────────────────────────────────────────

interface AlexaResponse {
  version: string
  sessionAttributes?: Record<string, unknown>
  response: {
    outputSpeech?: {
      type: string
      text?: string
      ssml?: string
    }
    card?: {
      type: string
      title?: string
      content?: string
    }
    reprompt?: {
      outputSpeech: {
        type: string
        text?: string
        ssml?: string
      }
    }
    directives?: AplDirective[]
    shouldEndSession: boolean
  }
}

/** Conversational reprompts — rotated for natural feel */
const CONVERSATIONAL_REPROMPTS = [
  "What else is on your mind?",
  "Go ahead, I'm still here.",
  "Anything else you'd like to know?",
  "I'm listening.",
  "What's next?",
  "Still here. Ask away.",
  "What else?",
]

function pickReprompt(): string {
  return CONVERSATIONAL_REPROMPTS[
    Math.floor(Math.random() * CONVERSATIONAL_REPROMPTS.length)
  ]
}

/** SFX Library for premium auditory feedback */
const SFX = {
  STARTUP: '<audio src="soundbank://soundlibrary/scifi/amzn_sfx_scifi_teleport_02"/>',
  ACK: '<audio src="soundbank://soundlibrary/ui/gameshow/amzn_ui_sfx_gameshow_neutral_response_01"/>',
  FINISH: '<audio src="soundbank://soundlibrary/computers/amzn_sfx_computer_code_01"/>',
}

function buildResponse(opts: {
  speech: string
  reprompt?: string
  card?: { title: string; content: string }
  apl?: AplDirective
  endSession?: boolean
  sessionAttributes?: Record<string, unknown>
  sfx?: string
}): AlexaResponse {
  // Wrap with SFX if provided
  const speechContent = opts.sfx ? `${opts.sfx}${opts.speech}` : opts.speech
  const ssml = wrapSsml(speechContent)
  
  const repromptText = opts.reprompt || pickReprompt()
  const repromptSsml = wrapSsml(repromptText)

  const response: AlexaResponse = {
    version: '1.0',
    response: {
      outputSpeech: {
        type: 'SSML',
        ssml,
      },
      shouldEndSession: opts.endSession ?? false,
    },
  }

  // Persist session attributes for multi-turn conversation
  if (opts.sessionAttributes) {
    response.sessionAttributes = opts.sessionAttributes
  }

  if (!opts.endSession) {
    response.response.reprompt = {
      outputSpeech: { type: 'SSML', ssml: repromptSsml },
    }
  }

  if (opts.card) {
    response.response.card = {
      type: 'Simple',
      title: opts.card.title,
      content: opts.card.content,
    }
  }

  if (opts.apl) {
    response.response.directives = [opts.apl]
  }

  return response
}

// ── Intent dispatching ────────────────────────────────────────────────────────

async function handleRequest(envelope: AlexaRequest): Promise<AlexaResponse> {
  const reqType = envelope.request.type
  const intentName = envelope.request.intent?.name

  // Recover existing conversation history from the session
  const history = getSessionHistory(envelope)

  console.warn(
    `[ALEXA] ${reqType}${intentName ? ` → ${intentName}` : ''} | ` +
    `session turns: ${Math.floor(history.length / 2)}`
  )

  // ─ LaunchRequest
  if (reqType === 'LaunchRequest') {
    return buildResponse({
      speech: "Chyren online. What would you like to know?",
      reprompt: "I'm listening. Ask me anything.",
      sessionAttributes: { conversationHistory: [] },
      sfx: SFX.STARTUP,
      apl: supportsAPL(envelope)
        ? buildAplDirective('Chyren', 'Standing by for executive input.', [])
        : undefined,
    })
  }

  // ─ SessionEndedRequest
  if (reqType === 'SessionEndedRequest') {
    console.warn('[ALEXA] Session ended:', envelope.request.reason)
    return buildResponse({ speech: '', endSession: true })
  }

  // ─ IntentRequest
  if (reqType === 'IntentRequest') {
    switch (intentName) {
      case 'AskChyrenIntent': {
        const query = envelope.request.intent?.slots?.query?.value

        if (!query) {
          return buildResponse({
            speech: "I didn't catch that. What would you like to ask?",
            reprompt: 'Go ahead, ask me anything.',
            sessionAttributes: { conversationHistory: history },
            apl: supportsAPL(envelope)
              ? buildAplDirective('Chyren', "Awaiting input...", history)
              : undefined,
          })
        }

        console.warn(`[ALEXA] Query: "${query}"`)
        const rawResponse = await askChyren(query, history)
        const cleanResponse = cleanForSpeech(rawResponse)
        const speech =
          truncateForSpeech(cleanResponse) ||
          'My neural links are recalibrating. Try again in a moment.'

        // Append this exchange to session history
        const updatedHistory = trimHistory([
          ...history,
          { role: 'user' as const, content: query },
          { role: 'assistant' as const, content: rawResponse || speech },
        ])

        return buildResponse({
          speech,
          card: { title: 'Chyren', content: rawResponse || speech },
          sessionAttributes: { conversationHistory: updatedHistory },
          sfx: SFX.ACK,
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', rawResponse || speech, history)
            : undefined,
        })
      }

      case 'AMAZON.HelpIntent': {
        const helpText =
          'Just talk to me naturally. After I respond, keep asking follow-up questions ' +
          "— I'll remember our conversation. Say stop when you're done."
        return buildResponse({
          speech: helpText,
          sessionAttributes: { conversationHistory: history },
          apl: supportsAPL(envelope)
            ? buildAplDirective('Help', helpText, history)
            : undefined,
        })
      }

      case 'AMAZON.CancelIntent':
      case 'AMAZON.StopIntent': {
        return buildResponse({
          speech: 'Chyren standing by. Until next time.',
          endSession: true,
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', 'Session ended. Standing by.', history)
            : undefined,
        })
      }

      case 'AMAZON.FallbackIntent':
      default: {
        // Route ALL unmatched intents through Chyren as freeform queries
        const fallbackQuery =
          envelope.request.intent?.slots?.query?.value || 'hello'
        console.warn(`[ALEXA] Fallback → routing: "${fallbackQuery}"`)

        const rawFb = await askChyren(fallbackQuery, history)
        const cleanFb = cleanForSpeech(rawFb)
        const speechFb =
          truncateForSpeech(cleanFb) ||
          "I couldn't process that. Try rephrasing your question."

        const updatedHistory = trimHistory([
          ...history,
          { role: 'user' as const, content: fallbackQuery },
          { role: 'assistant' as const, content: rawFb || speechFb },
        ])

        return buildResponse({
          speech: speechFb,
          sessionAttributes: { conversationHistory: updatedHistory },
          sfx: SFX.ACK,
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', rawFb || speechFb, history)
            : undefined,
        })
      }
    }
  }

  // ─ Unknown request type
  return buildResponse({
    speech: 'I received an unexpected request. Please try again.',
    endSession: true,
  })
}

// ── Next.js Route handler ─────────────────────────────────────────────────────

export async function POST(req: NextRequest) {
  try {
    const envelope = (await req.json()) as AlexaRequest

    if (!envelope?.request?.type) {
      return NextResponse.json(
        { error: 'Invalid Alexa request' },
        { status: 400 }
      )
    }

    const response = await handleRequest(envelope)
    return NextResponse.json(response)
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : 'unknown'
    console.error('[ALEXA] Handler error:', msg)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}

// Reject non-POST
export async function GET() {
  return NextResponse.json({ error: 'Method not allowed' }, { status: 405 })
}


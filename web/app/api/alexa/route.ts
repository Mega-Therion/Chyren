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

// ── SSML Pronunciation ────────────────────────────────────────────────────────

/** IPA pronunciation for "Chyren" → KY-ren (like "siren" with a K) */
const CHYREN_PHONEME = '<phoneme alphabet="ipa" ph="ˈkaɪ.ɹən">Chyren</phoneme>'

/**
 * Replace occurrences of "Chyren" in speech text with the SSML phoneme.
 * Returns SSML-wrapped speech.
 */
function wrapSsml(text: string): string {
  // Replace "Chyren" (case-insensitive) with the phoneme tag
  const withPhonemes = text.replace(/\bChyren\b/gi, CHYREN_PHONEME)
  return `<speak>${withPhonemes}</speak>`
}

// ── Helpers ───────────────────────────────────────────────────────────────────

function getOptionalEnv(name: string): string | null {
  const val = process.env[name]?.trim().replace(/^['"]|['"]$/g, '')
  if (!val || /^(YOUR_|REPLACE)/i.test(val)) return null
  return val
}

/** Parse SSE-formatted text from the Chyren chat stream. */
function parseSseText(raw: string): string | null {
  const parts: string[] = []
  for (const line of raw.split('\n')) {
    if (line.startsWith('data: ')) {
      try {
        const json = JSON.parse(line.slice(6))
        const content = json?.choices?.[0]?.delta?.content
        if (content) parts.push(content)
      } catch {
        // skip malformed lines
      }
    }
  }
  return parts.join('') || null
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

// ── Chyren API ────────────────────────────────────────────────────────────────

/**
 * Call the Chyren chat API with full conversation history for context.
 */
async function askChyren(
  query: string,
  history: ChatMessage[],
): Promise<string | null> {
  const base =
    getOptionalEnv('NEXT_PUBLIC_API_BASE_URL') || 'https://chyren-web.vercel.app'
  const endpoint = `${base}/api/chat/stream`
  const controller = new AbortController()
  const timeout = setTimeout(() => controller.abort(), CHYREN_CHAT_TIMEOUT_MS)

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

    const raw = await res.text()
    return parseSseText(raw)
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : 'unknown'
    console.error('[ALEXA] Chyren chat API error:', msg)
    return null
  } finally {
    clearTimeout(timeout)
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

// ── APL Template for Echo Show ────────────────────────────────────────────────

interface AplDirective {
  type: string
  version: string
  document: Record<string, unknown>
  datasources: Record<string, unknown>
}

function buildAplDirective(title: string, bodyText: string): AplDirective {
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
              // Background
              {
                type: 'Frame',
                width: '100vw',
                height: '100vh',
                position: 'absolute',
                backgroundColor: '#0a0a14',
              },
              // Accent bar
              {
                type: 'Frame',
                width: '100vw',
                height: '4dp',
                position: 'absolute',
                top: '0',
                background: {
                  type: 'linear',
                  colorRange: ['#BD93F9', '#50FA7B', '#8BE9FD'],
                  inputRange: [0, 0.5, 1],
                  angle: 90,
                },
              },
              // Content
              {
                type: 'Container',
                width: '100vw',
                height: '100vh',
                paddingLeft: '40dp',
                paddingRight: '40dp',
                paddingTop: '32dp',
                paddingBottom: '24dp',
                justifyContent: 'spaceBetween',
                items: [
                  // Header
                  {
                    type: 'Container',
                    direction: 'row',
                    alignItems: 'center',
                    items: [
                      {
                        type: 'Text',
                        text: '◈',
                        fontSize: '36dp',
                        color: '#BD93F9',
                      },
                      {
                        type: 'Text',
                        text: '${payload.title}',
                        fontSize: '28dp',
                        fontWeight: '700',
                        color: '#F8F8F2',
                        paddingLeft: '12dp',
                      },
                    ],
                  },
                  // Divider
                  {
                    type: 'Frame',
                    width: '100%',
                    height: '1dp',
                    backgroundColor: '#44475A',
                    marginTop: '8dp',
                    marginBottom: '12dp',
                  },
                  // Body
                  {
                    type: 'ScrollView',
                    grow: 1,
                    shrink: 1,
                    items: [
                      {
                        type: 'Text',
                        text: '${payload.body}',
                        fontSize: '22dp',
                        color: '#E0E0E0',
                        lineHeight: '1.4',
                      },
                    ],
                  },
                  // Footer
                  {
                    type: 'Text',
                    text: 'Chyren — Sovereign Intelligence',
                    fontSize: '14dp',
                    color: '#6272A4',
                    textAlign: 'right',
                    marginTop: '8dp',
                  },
                ],
              },
            ],
          },
        ],
      },
    },
    datasources: {
      payload: { title, body: bodyText },
    },
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

function buildResponse(opts: {
  speech: string
  reprompt?: string
  card?: { title: string; content: string }
  apl?: AplDirective
  endSession?: boolean
  sessionAttributes?: Record<string, unknown>
}): AlexaResponse {
  const ssml = wrapSsml(opts.speech)
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
      apl: supportsAPL(envelope)
        ? buildAplDirective('Chyren', 'Sovereign Intelligence — Awaiting your command.')
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
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', rawResponse || speech)
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
            ? buildAplDirective('Help', helpText)
            : undefined,
        })
      }

      case 'AMAZON.CancelIntent':
      case 'AMAZON.StopIntent': {
        return buildResponse({
          speech: 'Chyren standing by. Until next time.',
          endSession: true,
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', 'Session ended. Standing by.')
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
          apl: supportsAPL(envelope)
            ? buildAplDirective('Chyren', rawFb || speechFb)
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

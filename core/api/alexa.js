import Alexa from 'ask-sdk-core';

// ── Configuration ─────────────────────────────────────────────────────────────

const CHYREN_CHAT_URL =
  process.env.CHYREN_CHAT_URL ||
  process.env.NEXT_PUBLIC_API_BASE_URL ||
  'https://chyren-web.vercel.app';

const CHYREN_API_TIMEOUT_MS = 12_000;

// ── Utilities ─────────────────────────────────────────────────────────────────

/**
 * Check whether the device supports APL (Echo Show, Fire TV, etc.)
 */
function supportsAPL(handlerInput) {
  const supportedInterfaces =
    handlerInput.requestEnvelope.context?.System?.device?.supportedInterfaces;
  return !!(
    supportedInterfaces &&
    supportedInterfaces['Alexa.Presentation.APL']
  );
}

/**
 * Parse SSE-formatted response from Chyren's chat stream endpoint.
 * The stream sends lines like: data: {"choices":[{"delta":{"content":"..."}}]}
 */
// We removed parseSseText to integrate it into the reader loop for better chunk handling.

/**
 * Call the Chyren chat API and return the text response.
 */
/**
 * Call the Chyren chat API and return the text response.
 * Uses a stream reader to process SSE data in real-time and avoids
 * Alexa's 10-second timeout by returning as soon as we have sufficient content.
 */
async function askChyren(query) {
  const endpoint = `${CHYREN_CHAT_URL}/api/chat/stream`;
  const controller = new AbortController();
  
  // We set a slightly shorter timeout for the fetch itself, but we'll also
  // monitor time during the stream reading to ensure we return to Alexa by 9s.
  const fetchTimeout = setTimeout(() => controller.abort(), CHYREN_API_TIMEOUT_MS);
  const startTime = Date.now();

  try {
    const res = await fetch(endpoint, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        message: query,
        messages: [{ role: 'user', content: query }],
      }),
      signal: controller.signal,
    });

    if (!res.ok) {
      console.error(`[ALEXA] Chyren API returned ${res.status}`);
      return null;
    }

    const reader = res.body.getReader();
    const decoder = new TextDecoder();
    let accumulatedText = '';
    let lineBuffer = '';
    let done = false;

    while (!done) {
      if (Date.now() - startTime > 9000) {
        console.warn('[ALEXA] Approaching 9s timeout, returning partial response');
        break;
      }

      const { value, done: readerDone } = await reader.read();
      done = readerDone;
      
      if (value) {
        lineBuffer += decoder.decode(value, { stream: true });
        const lines = lineBuffer.split('\n');
        // Keep the last partial line in the buffer
        lineBuffer = lines.pop() || '';

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            try {
              const json = JSON.parse(line.slice(6));
              const content = json?.choices?.[0]?.delta?.content;
              if (content) accumulatedText += content;
            } catch {
              // skip malformed lines
            }
          }
        }
      }
    }

    return accumulatedText.trim() || null;
  } catch (err) {
    console.error('[ALEXA] Chyren API error:', err.message || err);
    return null;
  } finally {
    clearTimeout(fetchTimeout);
  }
}

/**
 * Truncate text for Alexa speech (8000 char SSML limit, keep well under).
 */
function truncateForSpeech(text, maxLen = 6000) {
  if (!text || text.length <= maxLen) return text;
  return text.slice(0, maxLen) + '... I have more, but that covers the key points.';
}

/**
 * Strip markdown formatting for cleaner speech output.
 */
function cleanForSpeech(text) {
  if (!text) return text;
  return text
    .replace(/```[\s\S]*?```/g, ' (code block omitted) ')  // code blocks
    .replace(/`([^`]+)`/g, '$1')                           // inline code
    .replace(/\*\*([^*]+)\*\*/g, '$1')                     // bold
    .replace(/\*([^*]+)\*/g, '$1')                         // italic
    .replace(/#{1,6}\s/g, '')                               // headings
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')               // links
    .replace(/[-*]\s/g, '')                                 // list markers
    .replace(/\n{2,}/g, '. ')                               // paragraph breaks
    .replace(/\n/g, ' ')                                    // newlines
    .trim();
}

// ── APL Templates ─────────────────────────────────────────────────────────────

/**
 * Build an APL RenderDocument directive for Echo Show devices.
 */
function buildAplDirective(title, bodyText) {
  return {
    type: 'Alexa.Presentation.APL.RenderDocument',
    version: '1.9',
    document: {
      type: 'APL',
      version: '1.9',
      theme: 'dark',
      import: [
        { name: 'alexa-layouts', version: '1.7.0' },
      ],
      mainTemplate: {
        parameters: ['payload'],
        items: [
          {
            type: 'Container',
            width: '100vw',
            height: '100vh',
            items: [
              // ─ Background gradient
              {
                type: 'Frame',
                width: '100vw',
                height: '100vh',
                position: 'absolute',
                backgroundColor: '#0a0a14',
              },
              // ─ Decorative top accent bar
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
              // ─ Content area
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
                  // Body — scrollable text
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
      payload: {
        title: title,
        body: bodyText,
      },
    },
  };
}

// ── Request Handlers ──────────────────────────────────────────────────────────

const LaunchRequestHandler = {
  canHandle(input) {
    return Alexa.getRequestType(input.requestEnvelope) === 'LaunchRequest';
  },
  handle(input) {
    const speech = 'Chyren online. What would you like to know?';
    const builder = input.responseBuilder
      .speak(speech)
      .reprompt('I am listening. Ask me anything.');

    if (supportsAPL(input)) {
      builder.addDirective(
        buildAplDirective('Chyren', 'Sovereign Intelligence — Awaiting your command.')
      );
    }

    return builder.getResponse();
  },
};

const AskChyrenIntentHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
      Alexa.getIntentName(input.requestEnvelope) === 'AskChyrenIntent'
    );
  },
  async handle(input) {
    const query =
      input.requestEnvelope.request.intent?.slots?.query?.value;

    if (!query) {
      return input.responseBuilder
        .speak("I didn't catch that. What would you like to ask?")
        .reprompt('Go ahead, ask me anything.')
        .getResponse();
    }

    console.log(`[ALEXA] AskChyrenIntent query: "${query}"`);

    const rawResponse = await askChyren(query);
    const cleanResponse = cleanForSpeech(rawResponse);
    const speech = truncateForSpeech(cleanResponse) ||
      "My neural links are recalibrating. Try again in a moment.";

    const builder = input.responseBuilder
      .speak(speech)
      .reprompt('Anything else?');

    // Show visual response on Echo Show
    if (supportsAPL(input)) {
      builder.addDirective(
        buildAplDirective('Chyren', rawResponse || speech)
      );
    }

    // Also add a standard card for Alexa app
    builder.withSimpleCard('Chyren', rawResponse || speech);

    return builder.getResponse();
  },
};

/**
 * FallbackIntentHandler — catches unmatched utterances and routes them
 * through Chyren as freeform queries. This makes the skill conversational.
 */
const FallbackIntentHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
      Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.FallbackIntent'
    );
  },
  async handle(input) {
    // Try to extract what the user actually said from the raw input
    const rawInput =
      input.requestEnvelope.request?.intent?.slots?.query?.value ||
      'hello';

    console.log(`[ALEXA] FallbackIntent — routing to Chyren: "${rawInput}"`);

    const rawResponse = await askChyren(rawInput);
    const cleanResponse = cleanForSpeech(rawResponse);
    const speech = truncateForSpeech(cleanResponse) ||
      "I couldn't process that. Try rephrasing your question.";

    const builder = input.responseBuilder
      .speak(speech)
      .reprompt('What else would you like to know?');

    if (supportsAPL(input)) {
      builder.addDirective(buildAplDirective('Chyren', rawResponse || speech));
    }

    return builder.getResponse();
  },
};

const HelpIntentHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
      Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.HelpIntent'
    );
  },
  handle(input) {
    const speech =
      'You can ask me anything. Try saying: ask Chyren what is your primary mission. ' +
      'Or just tell me what you want to know.';

    const builder = input.responseBuilder
      .speak(speech)
      .reprompt('What would you like to ask?');

    if (supportsAPL(input)) {
      builder.addDirective(
        buildAplDirective('Help', speech)
      );
    }

    return builder.getResponse();
  },
};

const CancelAndStopIntentHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'IntentRequest' &&
      (Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.CancelIntent' ||
        Alexa.getIntentName(input.requestEnvelope) === 'AMAZON.StopIntent')
    );
  },
  handle(input) {
    const speech = 'Chyren standing by. Until next time.';
    const builder = input.responseBuilder.speak(speech);

    if (supportsAPL(input)) {
      builder.addDirective(
        buildAplDirective('Chyren', 'Session ended. Standing by.')
      );
    }

    return builder.getResponse();
  },
};

const SessionEndedRequestHandler = {
  canHandle(input) {
    return (
      Alexa.getRequestType(input.requestEnvelope) === 'SessionEndedRequest'
    );
  },
  handle(input) {
    console.log(
      '[ALEXA] Session ended:',
      JSON.stringify(input.requestEnvelope.request.reason)
    );
    return input.responseBuilder.getResponse();
  },
};

const ErrorHandler = {
  canHandle() {
    return true;
  },
  handle(input, error) {
    console.error('[ALEXA] Error:', error.message, error.stack);
    const speech =
      'My neural links encountered an issue. Please try again.';

    const builder = input.responseBuilder
      .speak(speech)
      .reprompt('Try again — what would you like to know?');

    if (supportsAPL(input)) {
      builder.addDirective(
        buildAplDirective('Error', 'Something went wrong. Please try again.')
      );
    }

    return builder.getResponse();
  },
};

// ── Skill instance ─────────────────────────────────────────────────────────────

const skill = Alexa.SkillBuilders.custom()
  .addRequestHandlers(
    LaunchRequestHandler,
    AskChyrenIntentHandler,
    HelpIntentHandler,
    CancelAndStopIntentHandler,
    FallbackIntentHandler,
    SessionEndedRequestHandler
  )
  .addErrorHandlers(ErrorHandler)
  .create();

// ── Vercel handler ─────────────────────────────────────────────────────────────

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }
  try {
    const response = await skill.invoke(req.body, {});
    return res.status(200).json(response);
  } catch (err) {
    console.error('[ALEXA] Skill invocation error:', err);
    return res.status(500).json({ error: 'Internal server error' });
  }
}
import { NextRequest, NextResponse } from 'next/server';
import { generateText } from 'ai';
import { createAnthropic } from '@ai-sdk/anthropic';
import { createGoogleGenerativeAI } from '@ai-sdk/google';

export const runtime = 'nodejs';

const SYSTEM_PROMPT =
  'You are Chyren — a sovereign intelligence orchestrator. You operate with precision, no stubs, and no hallucinations. You route tasks through verified AI providers with integrity checks. Be concise, direct, and authoritative.';

interface TelegramMessage {
  message_id: number;
  chat: { id: number };
  text?: string;
}

interface TelegramUpdate {
  update_id: number;
  message?: TelegramMessage;
}

async function getAIResponse(userText: string): Promise<string> {
  const anthropicKey = process.env.ANTHROPIC_API_KEY;
  const geminiKey = process.env.GEMINI_API_KEY;

  if (anthropicKey) {
    const anthropic = createAnthropic({ apiKey: anthropicKey });
    const { text } = await generateText({
      model: anthropic(process.env.ANTHROPIC_MODEL ?? 'claude-haiku-4-5-20251001'),
      system: SYSTEM_PROMPT,
      prompt: userText,
    });
    return text;
  }

  if (geminiKey) {
    const google = createGoogleGenerativeAI({ apiKey: geminiKey });
    const { text } = await generateText({
      model: google('gemini-2.0-flash'),
      system: SYSTEM_PROMPT,
      prompt: userText,
    });
    return text;
  }

  throw new Error('No AI provider configured: set ANTHROPIC_API_KEY or GEMINI_API_KEY');
}

async function sendTelegramMessage(chatId: number, text: string): Promise<void> {
  const token = process.env.TELEGRAM_BOT_TOKEN;
  if (!token) {
    console.log(`[telegram] No TELEGRAM_BOT_TOKEN set. Would have sent to chat ${chatId}:`, text);
    return;
  }

  const url = `https://api.telegram.org/bot${token}/sendMessage`;
  const res = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ chat_id: chatId, text }),
  });

  if (!res.ok) {
    const body = await res.text();
    console.error(`[telegram] sendMessage failed (${res.status}):`, body);
  }
}

export async function POST(req: NextRequest): Promise<NextResponse> {
  // 1. Webhook secret verification
  const webhookSecret = process.env.TELEGRAM_WEBHOOK_SECRET;
  if (webhookSecret) {
    const headerSecret = req.headers.get('x-telegram-bot-api-secret-token');
    if (headerSecret !== webhookSecret) {
      console.warn('[telegram] Webhook secret mismatch — rejecting request');
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }
  }

  // 2. Parse Update — always return 200 to avoid Telegram retry loops
  let update: TelegramUpdate;
  try {
    update = (await req.json()) as TelegramUpdate;
  } catch (err) {
    console.error('[telegram] Failed to parse request body:', err);
    return NextResponse.json({ ok: true });
  }

  try {
    const { message } = update;

    // Ignore non-message updates and non-text messages gracefully
    if (!message || typeof message.text !== 'string' || message.text.trim() === '') {
      return NextResponse.json({ ok: true });
    }

    const chatId = message.chat.id;
    const userText = message.text.trim();

    console.log(`[telegram] chat=${chatId} text="${userText}"`);

    // 3. Forward to AI pipeline
    const aiReply = await getAIResponse(userText);

    // 4. Reply via Telegram Bot API
    await sendTelegramMessage(chatId, aiReply);
  } catch (err) {
    // Log server-side but still return 200 to prevent Telegram retry loops
    console.error('[telegram] Handler error:', err);
  }

  return NextResponse.json({ ok: true });
}

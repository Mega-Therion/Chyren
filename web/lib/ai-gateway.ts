/**
 * AI Gateway Utility
 * 
 * Provides a unified way to proxy AI SDK providers through the Vercel AI Gateway.
 * This enables centralized logging, caching, and cross-provider fallbacks.
 */

import { createAnthropic } from '@ai-sdk/anthropic';
import { createOpenAI } from '@ai-sdk/openai';
import { createGoogleGenerativeAI as createGoogle } from '@ai-sdk/google';

const GATEWAY_ID = process.env.VERCEL_AI_GATEWAY_ID || 'chyren-gateway';
const PROJECT_ID = process.env.VERCEL_PROJECT_ID || 'chyren-web';

/**
 * Wraps a provider with Vercel AI Gateway proxying.
 * Pattern: https://gateway.ai.vercel.com/v1/projects/<project>/gateways/<gateway>/<provider>
 */
export function getGatewayHeaders(): Record<string, string> {
  const key = process.env.VERCEL_AI_GATEWAY_KEY;
  return key ? { 'Authorization': `Bearer ${key}` } : {};
}

export function getGatewayUrl(provider: string) {
  return `https://gateway.ai.vercel.com/v1/projects/${PROJECT_ID}/gateways/${GATEWAY_ID}/${provider}`;
}

export const anthropic = createAnthropic({
  baseURL: getGatewayUrl('anthropic'),
  apiKey: process.env.ANTHROPIC_API_KEY,
  headers: getGatewayHeaders(),
});

export const openai = createOpenAI({
  baseURL: getGatewayUrl('openai'),
  apiKey: process.env.OPENAI_API_KEY,
  headers: getGatewayHeaders(),
});

export const google = createGoogle({
  baseURL: getGatewayUrl('google'),
  apiKey: process.env.GEMINI_API_KEY,
  headers: getGatewayHeaders(),
});

// For Groq and OpenRouter, we use the OpenAI provider with custom base URLs
export const groq = createOpenAI({
  baseURL: getGatewayUrl('groq'),
  apiKey: process.env.GROQ_API_KEY,
  headers: getGatewayHeaders(),
});

export const openrouter = createOpenAI({
  baseURL: getGatewayUrl('openrouter'),
  apiKey: process.env.OPENROUTER_API_KEY,
  headers: getGatewayHeaders(),
});

export const ollama = createOpenAI({
  baseURL: process.env.OLLAMA_BASE_URL || 'http://localhost:11434/v1',
  apiKey: 'ollama',
});

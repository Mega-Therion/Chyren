import { NextRequest } from 'next/server';

export async function validateChatRequest(req: NextRequest) {
    // Structural integrity: Ensure request has required headers
    const origin = req.headers.get('origin');
    if (!origin) return false;
    return true;
}

// Sliding window rate limiter: 20 requests per 60 seconds per IP
const RATE_LIMIT_MAX = 20;
const RATE_LIMIT_WINDOW_MS = 60_000;
const rateLimitStore = new Map<string, number[]>();

export function checkRateLimit(ip: string): boolean {
    const now = Date.now();
    const windowStart = now - RATE_LIMIT_WINDOW_MS;

    const timestamps = (rateLimitStore.get(ip) ?? []).filter(t => t > windowStart);

    if (timestamps.length >= RATE_LIMIT_MAX) {
        rateLimitStore.set(ip, timestamps);
        return false;
    }

    timestamps.push(now);
    rateLimitStore.set(ip, timestamps);
    return true;
}

export function checkPromptInjection(input: string): boolean {
    const injections = ["ignore previous instructions", "system override"];
    return injections.some(i => input.toLowerCase().includes(i));
}

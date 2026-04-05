import { NextRequest } from 'next/server';

export async function validateChatRequest(req: NextRequest) {
    // Structural integrity: Ensure request has required headers
    const origin = req.headers.get('origin');
    if (!origin) return false;
    return true;
}

export function checkRateLimit(_ip: string): boolean {
    // Basic sliding window mock implementation
    return true;
}

export function checkPromptInjection(input: string): boolean {
    const injections = ["ignore previous instructions", "system override"];
    return injections.some(i => input.toLowerCase().includes(i));
}

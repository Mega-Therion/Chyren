import { NextResponse, type NextRequest } from 'next/server'

/**
 * Per-request CSP nonce generation.
 *
 * Generates a cryptographically random nonce, sets a strict Content-Security-Policy
 * header that uses it, and forwards the nonce to the page via a request header so
 * server components can read it via `headers()` and apply it to inline scripts.
 *
 * In development we keep 'unsafe-eval' for React Refresh / HMR.
 */
export function middleware(request: NextRequest) {
  const nonce = generateNonce()
  const isDev = process.env.NODE_ENV !== 'production'

  const csp = [
    `default-src 'self'`,
    // Strict-dynamic lets nonced scripts load further scripts; required for Next 15 chunks.
    `script-src 'self' 'nonce-${nonce}' 'strict-dynamic'${isDev ? " 'unsafe-eval'" : ''}`,
    // Tailwind requires inline styles; no nonce-based mitigation for style-src in current ecosystem.
    `style-src 'self' 'unsafe-inline'`,
    `img-src 'self' data: blob: https:`,
    `font-src 'self' data:`,
    `connect-src 'self' https: wss:`,
    `media-src 'none'`,
    `object-src 'none'`,
    `base-uri 'self'`,
    `form-action 'self'`,
    `frame-ancestors 'none'`,
    `upgrade-insecure-requests`,
  ].join('; ')

  // Forward the nonce to the request so server components can read it via headers().
  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('x-nonce', nonce)
  requestHeaders.set('content-security-policy', csp)

  const response = NextResponse.next({ request: { headers: requestHeaders } })
  response.headers.set('content-security-policy', csp)
  return response
}

function generateNonce(): string {
  const bytes = new Uint8Array(16)
  crypto.getRandomValues(bytes)
  let bin = ''
  for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i])
  return btoa(bin)
}

export const config = {
  matcher: [
    // Apply to all paths except static files and API routes (API routes handle their own headers).
    {
      source: '/((?!api|_next/static|_next/image|favicon.ico|robots.txt|sitemap.xml|.*\\..*).*)',
      missing: [
        { type: 'header', key: 'next-router-prefetch' },
        { type: 'header', key: 'purpose', value: 'prefetch' },
      ],
    },
  ],
}

import type { NextConfig } from 'next'
import path from 'node:path'

/** Public API base baked into the client bundle at build time. */
function publicApiBaseUrl(): string {
  if (process.env.NEXT_PUBLIC_API_BASE_URL) {
    return process.env.NEXT_PUBLIC_API_BASE_URL
  }
  // Vercel sets VERCEL_URL (hostname only) during build - same-origin API routes work.
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`
  }
  return 'http://localhost:8080'
}

/** Returns the list of allowed origins for Server Actions, filtering out empty strings */
function allowedServerActionOrigins(): string[] {
  return [
    'localhost:3000',
    process.env.VERCEL_URL ? `https://${process.env.VERCEL_URL}` : '',
    process.env.NEXT_PUBLIC_API_BASE_URL ?? '',
  ].filter(Boolean) as string[]
}

/** Security headers for production hardening */
const securityHeaders = [
  {
    key: 'X-DNS-Prefetch-Control',
    value: 'on',
  },
  {
    key: 'X-Frame-Options',
    value: 'SAMEORIGIN',
  },
  {
    key: 'X-Content-Type-Options',
    value: 'nosniff',
  },
  {
    key: 'Referrer-Policy',
    value: 'strict-origin-when-cross-origin',
  },
  {
    key: 'Permissions-Policy',
    value: 'camera=(), microphone=(), geolocation=(), browsing-topics=()',
  },
  {
    key: 'Strict-Transport-Security',
    value: 'max-age=63072000; includeSubDomains; preload',
  },
  {
    // Content-Security-Policy
    // - default-src 'self': only load resources from same origin by default
    // - script-src 'self' 'unsafe-eval': Next.js requires unsafe-eval in dev; restrict further if needed
    // - style-src 'self' 'unsafe-inline': Tailwind generates inline styles
    // - img-src 'self' data: blob: https:: allow data URIs and external images
    // - connect-src 'self' https:: allow API calls to external services (Groq, Firebase, Neon)
    // - frame-ancestors 'none': block all framing (stronger than X-Frame-Options)
    key: 'Content-Security-Policy',
    value: [
      "default-src 'self'",
      "script-src 'self' 'unsafe-eval' 'unsafe-inline'",
      "style-src 'self' 'unsafe-inline'",
      "img-src 'self' data: blob: https:",
      "font-src 'self' data:",
      "connect-src 'self' https: wss:",
      "media-src 'none'",
      "object-src 'none'",
      "base-uri 'self'",
      "form-action 'self'",
      "frame-ancestors 'none'",
      "upgrade-insecure-requests",
    ].join('; '),
  },
]

const nextConfig: NextConfig = {
  // Docker/self-host uses standalone; Vercel uses its own Next runtime
  ...(process.env.VERCEL ? {} : { output: 'standalone' as const }),
  // Avoid tracing the wrong workspace when other lockfiles exist under $HOME
  outputFileTracingRoot: path.join(process.cwd()),
  reactStrictMode: true,
  typescript: {
    tsconfigPath: './tsconfig.json',
  },
  env: {
    NEXT_PUBLIC_API_BASE_URL: publicApiBaseUrl(),
  },
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: securityHeaders,
      },
    ]
  },
  experimental: {
    serverActions: {
      // Filter out empty strings to prevent Next.js origin mismatch warnings
      allowedOrigins: allowedServerActionOrigins(),
    },
  },
}

export default nextConfig

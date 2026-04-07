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
  // Suppress known experimental warnings
  experimental: {
    serverActions: {
      allowedOrigins: ['localhost:3000', process.env.VERCEL_URL ?? ''],
    },
  },
}

export default nextConfig

import type { NextConfig } from 'next'
import path from 'node:path'

/** Public API base baked into the client bundle at build time. */
function publicApiBaseUrl(): string {
  if (process.env.NEXT_PUBLIC_API_BASE_URL) {
    return process.env.NEXT_PUBLIC_API_BASE_URL
  }
  // Vercel sets VERCEL_URL (hostname only) during build — same-origin API routes work.
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`
  }
  return 'http://localhost:8080'
}

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
        headers: [
          {
            key: 'Content-Security-Policy',
            value: "default-src 'self'; script-src 'self' 'unsafe-eval' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https: blob:; connect-src 'self' https: wss:; font-src 'self' data:;",
          },
          {
            key: 'X-Content-Type-Options',
            value: 'nosniff',
          },
          {
            key: 'X-Frame-Options',
            value: 'DENY',
          },
          {
            key: 'X-XSS-Protection',
            value: '1; mode=block',
          },
        ],
      },
    ]
  },
}

export default nextConfig

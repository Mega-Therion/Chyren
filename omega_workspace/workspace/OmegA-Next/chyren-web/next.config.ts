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
}

export default nextConfig

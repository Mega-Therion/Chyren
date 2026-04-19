import type { NextConfig } from 'next'
import path from 'node:path'
import bundleAnalyzer from '@next/bundle-analyzer'

const withBundleAnalyzer = bundleAnalyzer({
  enabled: process.env.ANALYZE === 'true',
  openAnalyzer: false,
})

/** Public API base baked into the client bundle at build time. */
function publicApiBaseUrl(): string {
  if (process.env.NEXT_PUBLIC_API_BASE_URL) {
    return process.env.NEXT_PUBLIC_API_BASE_URL
  }
  if (process.env.VERCEL_URL) {
    return `https://${process.env.VERCEL_URL}`
  }
  return 'http://localhost:8080'
}

/**
 * Static security headers applied to every response.
 * NOTE: Content-Security-Policy is set per-request in middleware.ts so it can
 * include a fresh nonce for inline scripts/styles. Do not duplicate it here.
 */
const securityHeaders = [
  { key: 'X-DNS-Prefetch-Control', value: 'on' },
  { key: 'X-Frame-Options', value: 'DENY' },
  { key: 'X-Content-Type-Options', value: 'nosniff' },
  { key: 'Referrer-Policy', value: 'strict-origin-when-cross-origin' },
  {
    key: 'Permissions-Policy',
    value: 'camera=(), microphone=(), geolocation=(), browsing-topics=()',
  },
  {
    key: 'Strict-Transport-Security',
    value: 'max-age=63072000; includeSubDomains; preload',
  },
  { key: 'Cross-Origin-Opener-Policy', value: 'same-origin' },
  { key: 'Cross-Origin-Resource-Policy', value: 'same-origin' },
]

const nextConfig: NextConfig = {
  ...(process.env.VERCEL ? {} : { output: 'standalone' as const }),
  outputFileTracingRoot: path.join(process.cwd()),
  reactStrictMode: true,
  poweredByHeader: false,
  compress: true,
  productionBrowserSourceMaps: false,
  typescript: {
    tsconfigPath: './tsconfig.json',
  },
  experimental: {
    optimizePackageImports: [
      'lucide-react',
      'framer-motion',
      '@ai-sdk/anthropic',
      '@ai-sdk/google',
      '@ai-sdk/groq',
      '@ai-sdk/openai',
      '@ai-sdk/react',
    ],
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
}

export default withBundleAnalyzer(nextConfig)

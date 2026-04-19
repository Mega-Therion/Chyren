import type { Metadata, Viewport } from 'next'
import { headers } from 'next/headers'
import { Analytics } from '@vercel/analytics/next'
import { SpeedInsights } from '@vercel/speed-insights/next'
import { Inter, JetBrains_Mono, Orbitron } from 'next/font/google'
import './globals.css'

const fontSans = Inter({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-sans',
  weight: ['300', '400', '500', '600'],
})

const fontDisplay = Orbitron({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-display',
  weight: ['500', '700', '900'],
})

const fontMono = JetBrains_Mono({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-mono',
  weight: ['400', '500'],
})

const SITE_URL =
  process.env.NEXT_PUBLIC_SITE_URL ??
  (process.env.VERCEL_URL ? `https://${process.env.VERCEL_URL}` : 'https://chyren.org')

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: {
    default: 'CHYREN // Sovereign Intelligence',
    template: '%s // CHYREN',
  },
  description:
    'Sovereign Intelligence Orchestrator — verified AI task routing with cryptographic integrity gates and append-only ledger.',
  applicationName: 'Chyren',
  keywords: [
    'sovereign intelligence',
    'AI orchestrator',
    'ADCCL',
    'verified AI',
    'cognitive shell',
    'Chyren',
  ],
  authors: [{ name: 'Chyren Sovereign Intelligence' }],
  creator: 'Chyren',
  publisher: 'Chyren',
  formatDetection: { email: false, address: false, telephone: false },
  alternates: { canonical: '/' },
  openGraph: {
    type: 'website',
    siteName: 'Chyren',
    title: 'CHYREN // Sovereign Intelligence',
    description:
      'Verified AI task routing with cryptographic integrity gates and append-only ledger.',
    url: SITE_URL,
    locale: 'en_US',
    images: [
      {
        url: '/banner.svg',
        width: 1200,
        height: 630,
        alt: 'Chyren Sovereign Intelligence',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'CHYREN // Sovereign Intelligence',
    description: 'Verified AI task routing with cryptographic integrity gates.',
    images: ['/banner.svg'],
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-snippet': -1,
      'max-image-preview': 'large',
      'max-video-preview': -1,
    },
  },
  icons: {
    icon: '/favicon.ico',
    shortcut: '/favicon.ico',
  },
  manifest: '/manifest.webmanifest',
}

export const viewport: Viewport = {
  themeColor: '#000000',
  colorScheme: 'dark',
  width: 'device-width',
  initialScale: 1,
  viewportFit: 'cover',
}

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  // Reading the header here ensures the layout is rendered per-request, which
  // is required so the middleware-issued nonce reaches the document via
  // Next's Script tag injection.
  await headers()
  return (
    <html lang="en" className={`${fontSans.variable} ${fontDisplay.variable} ${fontMono.variable}`}>
      <body>
        {children}
        <Analytics />
        <SpeedInsights />
      </body>
    </html>
  )
}

import type { MetadataRoute } from 'next'

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: 'Chyren Sovereign Intelligence',
    short_name: 'Chyren',
    description:
      'Verified AI task routing with cryptographic integrity gates and append-only ledger.',
    start_url: '/',
    display: 'standalone',
    background_color: '#000000',
    theme_color: '#000000',
    orientation: 'portrait',
    categories: ['productivity', 'utilities', 'developer'],
    icons: [
      {
        src: '/banner.svg',
        sizes: 'any',
        type: 'image/svg+xml',
        purpose: 'any',
      },
    ],
  }
}

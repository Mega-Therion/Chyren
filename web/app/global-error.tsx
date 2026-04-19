'use client'

import { useEffect } from 'react'

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string }
  reset: () => void
}) {
  useEffect(() => {
    console.error('Global Sovereign Hub Fault:', error)
    void reportGlobalError(error)
  }, [error])

  return (
    <html lang="en">
      <body
        style={{
          margin: 0,
          minHeight: '100vh',
          background: '#000',
          color: '#fff',
          fontFamily: 'system-ui, -apple-system, sans-serif',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          justifyContent: 'center',
          padding: '2rem',
          textAlign: 'center',
        }}
      >
        <h1 style={{ color: '#f43f5e', letterSpacing: '0.3em', fontSize: '1.25rem' }}>
          GLOBAL_FAULT
        </h1>
        <p style={{ opacity: 0.6, marginTop: '0.5rem' }}>
          The sovereign hub encountered a critical fault.
        </p>
        {error.digest && (
          <p style={{ opacity: 0.4, fontSize: '0.75rem', marginTop: '0.5rem' }}>
            ref: {error.digest}
          </p>
        )}
        <button
          type="button"
          onClick={() => reset()}
          style={{
            marginTop: '1.5rem',
            padding: '0.75rem 1.5rem',
            borderRadius: '999px',
            border: '1px solid rgba(244,63,94,0.4)',
            background: 'rgba(244,63,94,0.2)',
            color: '#f43f5e',
            cursor: 'pointer',
            fontFamily: 'inherit',
          }}
        >
          REBOOT_LINK
        </button>
      </body>
    </html>
  )
}

async function reportGlobalError(error: Error & { digest?: string }) {
  try {
    await fetch('/api/errors', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      keepalive: true,
      body: JSON.stringify({
        source: 'global',
        message: error.message ?? 'Unknown error',
        stack: error.stack?.slice(0, 8000),
        digest: error.digest,
        url: typeof window !== 'undefined' ? window.location.href : undefined,
        userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : undefined,
      }),
    })
  } catch {
    // Intentional no-op
  }
}

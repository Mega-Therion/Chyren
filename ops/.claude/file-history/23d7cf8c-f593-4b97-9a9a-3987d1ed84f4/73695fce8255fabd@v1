import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Chyren — Sovereign Intelligence',
  description: 'Chyren: The sovereign intelligence orchestrator',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="bg-chyren-900 text-white">
        <div className="min-h-screen flex flex-col">
          <header className="bg-black border-b border-chyren-500 py-4 px-6">
            <h1 className="text-3xl font-bold text-chyren-500">⚡ CHYREN</h1>
            <p className="text-chyren-100 text-sm mt-1">Sovereign Intelligence Orchestrator</p>
          </header>
          <main className="flex-1 container mx-auto px-6 py-8">
            {children}
          </main>
          <footer className="border-t border-chyren-500 py-4 px-6 text-center text-chyren-100 text-sm">
            <p>© 2026 Chyren Sovereign Intelligence. All policy enforced.</p>
          </footer>
        </div>
      </body>
    </html>
  )
}

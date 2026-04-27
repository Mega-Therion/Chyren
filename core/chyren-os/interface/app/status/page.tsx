import React from 'react'
import { dataAccess } from '@/lib/dal'
import { motion } from 'framer-motion'

export const dynamic = 'force-dynamic'

export default async function StatusPage() {
  const statuses = await dataAccess.getStatuses()

  return (
    <div className="min-h-screen bg-black text-white p-8 font-sans">
      <div className="max-w-2xl mx-auto space-y-12">
        <header className="space-y-2 border-b border-white/10 pb-8">
          <h1 className="text-4xl font-display font-bold tracking-tighter">CHYREN // STATUS</h1>
          <p className="text-white/40 text-sm font-mono uppercase tracking-widest">Sovereign Event Feed</p>
        </header>

        <div className="space-y-16">
          {statuses.map((status: any) => (
            <article key={status.id} className="group space-y-4">
              <div className="flex items-center gap-4 text-[10px] font-mono text-white/20 uppercase tracking-[0.3em]">
                <span>{new Date(status.created_at).toLocaleString()}</span>
                <span className="h-px flex-1 bg-white/5" />
                <span>ID: {status.id.slice(0, 8)}</span>
              </div>
              
              <p className="text-lg leading-relaxed text-white/80 group-hover:text-white transition-colors">
                {status.text}
              </p>

              {status.media && status.media.length > 0 && (
                <div className="grid gap-4 mt-4">
                  {status.media.map((url: string, i: number) => (
                    <img 
                      key={i} 
                      src={url} 
                      alt="Status media" 
                      className="rounded-lg border border-white/10 opacity-80 hover:opacity-100 transition-opacity max-w-full h-auto"
                    />
                  ))}
                </div>
              )}

              {status.tags && status.tags.length > 0 && (
                <div className="flex gap-2">
                  {status.tags.map((tag: string) => (
                    <span key={tag} className="text-[10px] font-mono text-cyan-500/50">
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </article>
          ))}

          {statuses.length === 0 && (
            <div className="py-20 text-center text-white/20 font-mono text-sm uppercase tracking-widest">
              No status events recorded in the ledger.
            </div>
          )}
        </div>

        <footer className="pt-20 pb-10 text-center">
          <a 
            href="/status/rss" 
            className="text-[10px] font-mono text-white/20 hover:text-cyan-500 transition-colors uppercase tracking-widest"
          >
            RSS Feed
          </a>
        </footer>
      </div>
    </div>
  )
}

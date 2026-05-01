'use client'

import React, { useEffect, useRef, useState } from 'react'

interface LedgerEntry {
  run_id: string
  task: string
  provider: string
  score: number
  status: 'verified' | 'rejected'
  timestamp: string
}

interface MetricsData {
  integrity_rate: number
  total_runs: number
  verified: number
  rejected: number
  recent: LedgerEntry[]
  source?: 'db' | 'mock'
}

const POLL_INTERVAL_MS = 10_000

function scoreColor(score: number): string {
  if (score >= 0.7) return '#39ff14'
  if (score >= 0.5) return '#ffd700'
  return '#ff4444'
}

function ScorePip({ score }: { score: number }) {
  const color = scoreColor(score)
  return (
    <span
      className="font-mono text-[10px] tabular-nums px-1.5 py-0.5 rounded"
      style={{
        color,
        background: `${color}14`,
        border: `1px solid ${color}36`,
      }}
    >
      {(score * 100).toFixed(0)}%
    </span>
  )
}

function IntegrityBar({ rate }: { rate: number }) {
  const pct = Math.round(rate * 100)
  const color = pct >= 90 ? '#39ff14' : pct >= 70 ? '#ffd700' : '#ff4444'
  return (
    <div className="flex items-center gap-2">
      <div className="flex-1 h-1 rounded-full" style={{ background: 'rgba(255,255,255,0.06)' }}>
        <div
          className="h-1 rounded-full transition-all duration-700"
          style={{ width: `${pct}%`, background: color, boxShadow: `0 0 6px ${color}88` }}
        />
      </div>
      <span className="font-mono text-[11px] tabular-nums" style={{ color }}>
        {pct}%
      </span>
    </div>
  )
}

function EntryRow({ entry }: { entry: LedgerEntry }) {
  const isVerified = entry.status === 'verified'
  const ts = new Date(entry.timestamp)
  const timeLabel = ts.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false })

  return (
    <div className="flex items-start gap-2 py-2 border-b" style={{ borderColor: 'rgba(255,255,255,0.04)' }}>
      {/* Status dot */}
      <span
        className="mt-0.5 w-1.5 h-1.5 rounded-full flex-shrink-0"
        style={{ background: isVerified ? '#39ff14' : '#ff4444', marginTop: '5px', boxShadow: isVerified ? '0 0 4px #39ff1466' : '0 0 4px #ff444466' }}
      />
      <div className="flex-1 min-w-0">
        <p className="text-[11px] text-slate-300 truncate leading-tight">{entry.task || '—'}</p>
        <div className="flex items-center gap-2 mt-0.5">
          <span className="font-mono text-[9px] text-slate-600 uppercase tracking-wider">{entry.provider}</span>
          <span className="text-[9px] text-slate-700">{timeLabel}</span>
        </div>
      </div>
      <ScorePip score={entry.score} />
    </div>
  )
}

export function LedgerFeed() {
  const [data, setData] = useState<MetricsData | null>(null)
  const [offline, setOffline] = useState(false)
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null)
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const fetchMetrics = async () => {
    try {
      const res = await fetch('/api/metrics', { cache: 'no-store' })
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const json = (await res.json()) as MetricsData
      setData(json)
      setOffline(false)
      setLastUpdated(new Date())
    } catch {
      setOffline(true)
    }
  }

  useEffect(() => {
    void fetchMetrics()
    timerRef.current = setInterval(() => { void fetchMetrics() }, POLL_INTERVAL_MS)
    return () => {
      if (timerRef.current) clearInterval(timerRef.current)
    }
  }, [])

  return (
    <div
      className="flex flex-col h-full"
      style={{
        background: 'rgba(0,0,0,0.72)',
        backdropFilter: 'blur(20px)',
        borderLeft: '1px solid rgba(255,255,255,0.06)',
      }}
    >
      {/* Header */}
      <div className="px-4 py-3 border-b" style={{ borderColor: 'rgba(255,255,255,0.06)' }}>
        <div className="flex items-center justify-between mb-1">
          <span className="font-mono text-[10px] tracking-[0.2em] uppercase text-slate-500">
            Ledger Integrity
          </span>
          {offline ? (
            <span className="font-mono text-[9px] text-red-500 tracking-wider">OFFLINE</span>
          ) : data?.source === 'mock' ? (
            <span className="font-mono text-[9px] text-amber-600 tracking-wider">MOCK</span>
          ) : (
            <span
              className="w-1.5 h-1.5 rounded-full"
              style={{ background: '#39ff14', boxShadow: '0 0 4px #39ff1466' }}
            />
          )}
        </div>
        {offline ? (
          <p className="text-[11px] text-red-400/70">Ledger offline — retrying…</p>
        ) : data ? (
          <IntegrityBar rate={data.integrity_rate} />
        ) : (
          <div className="h-1 rounded-full animate-pulse" style={{ background: 'rgba(255,255,255,0.06)' }} />
        )}
      </div>

      {/* Summary counts */}
      {!offline && data && (
        <div className="grid grid-cols-3 gap-px border-b" style={{ borderColor: 'rgba(255,255,255,0.06)', background: 'rgba(255,255,255,0.03)' }}>
          {[
            { label: 'RUNS', value: data.total_runs },
            { label: 'PASS', value: data.verified, color: '#39ff14' },
            { label: 'FAIL', value: data.rejected, color: '#ff4444' },
          ].map(({ label, value, color }) => (
            <div key={label} className="px-3 py-2 text-center" style={{ background: 'rgba(0,0,0,0.4)' }}>
              <div className="font-mono text-[14px] font-bold" style={{ color: color ?? '#fff' }}>
                {value}
              </div>
              <div className="font-mono text-[8px] tracking-wider text-slate-600 mt-0.5">{label}</div>
            </div>
          ))}
        </div>
      )}

      {/* Recent entries */}
      <div className="flex-1 overflow-y-auto px-4">
        {offline ? (
          <div className="flex flex-col items-center justify-center h-full gap-2 text-slate-600">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
              <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            <span className="text-[10px] font-mono uppercase tracking-wider">Ledger unreachable</span>
          </div>
        ) : !data || data.recent.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <span className="text-[10px] font-mono uppercase tracking-wider text-slate-700">No entries</span>
          </div>
        ) : (
          <div className="mt-1">
            {data.recent.map(entry => (
              <EntryRow key={entry.run_id} entry={entry} />
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      {lastUpdated && !offline && (
        <div className="px-4 py-2 border-t" style={{ borderColor: 'rgba(255,255,255,0.04)' }}>
          <span className="font-mono text-[9px] text-slate-700 tracking-wider">
            UPDATED {lastUpdated.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false })}
          </span>
        </div>
      )}
    </div>
  )
}

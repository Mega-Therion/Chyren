import { NextResponse } from 'next/server';

export const runtime = 'nodejs'

interface LedgerEntry {
  run_id: string
  task: string
  provider: string
  score: number
  status: 'verified' | 'rejected'
  timestamp: string
}

interface MetricsPayload {
  integrity_rate: number
  total_runs: number
  verified: number
  rejected: number
  recent: LedgerEntry[]
  source: 'db' | 'mock'
  // Legacy prometheus-style fields for MetricsDashboard compatibility
  metrics: {
    chyren_task_admitted_total: number
    chyren_active_runs: number
    chyren_adccl_score: number
  }
  timestamp: string
}

function mockPayload(): MetricsPayload {
  const now = new Date()
  const recent: LedgerEntry[] = Array.from({ length: 5 }, (_, i) => {
    const score = +(Math.random() * 0.4 + 0.6).toFixed(2)
    return {
      run_id: crypto.randomUUID().replace(/-/g, '').slice(0, 16),
      task: ['Explain the ADCCL gate', 'Summarize ledger state', 'Reason about memory', 'Diagnose drift', 'Verify phylactery'][i],
      provider: ['openai', 'anthropic', 'groq', 'gemini', 'openai'][i],
      score,
      status: score >= 0.7 ? 'verified' : 'rejected',
      timestamp: new Date(now.getTime() - i * 65_000).toISOString(),
    }
  })
  const verified = recent.filter(r => r.status === 'verified').length
  return {
    integrity_rate: +(verified / recent.length).toFixed(2),
    total_runs: 142,
    verified: 135,
    rejected: 7,
    recent,
    source: 'mock',
    metrics: {
      chyren_task_admitted_total: 142,
      chyren_active_runs: 0,
      chyren_adccl_score: 0.95,
    },
    timestamp: now.toISOString(),
  }
}

export async function GET() {
  const dbUrl = process.env.OMEGA_DB_URL

  if (dbUrl) {
    try {
      // Dynamic import so build doesn't fail when pg isn't installed
      const { Pool } = await import('pg')
      const pool = new Pool({ connectionString: dbUrl, ssl: { rejectUnauthorized: false } })

      const [totalRes, recentRes] = await Promise.all([
        pool.query<{ total: string; verified: string; rejected: string; avg_score: string }>(`
          SELECT
            COUNT(*)::text AS total,
            COUNT(*) FILTER (WHERE adccl_passed = true)::text AS verified,
            COUNT(*) FILTER (WHERE adccl_passed = false)::text AS rejected,
            COALESCE(AVG(adccl_score), 0)::text AS avg_score
          FROM ledger_entries
        `),
        pool.query<{ run_id: string; task: string; provider: string; adccl_score: number; adccl_passed: boolean; created_at: string }>(`
          SELECT run_id, task, provider, adccl_score, adccl_passed, created_at
          FROM ledger_entries
          ORDER BY created_at DESC
          LIMIT 5
        `),
      ])

      await pool.end()

      const totals = totalRes.rows[0]
      const total = parseInt(totals?.total ?? '0', 10)
      const verified = parseInt(totals?.verified ?? '0', 10)
      const rejected = parseInt(totals?.rejected ?? '0', 10)
      const avgScore = parseFloat(totals?.avg_score ?? '0')

      const recent: LedgerEntry[] = recentRes.rows.map(r => ({
        run_id: r.run_id ?? crypto.randomUUID(),
        task: (r.task ?? '').slice(0, 60),
        provider: r.provider ?? 'unknown',
        score: +(r.adccl_score ?? 0).toFixed(2),
        status: r.adccl_passed ? 'verified' : 'rejected',
        timestamp: r.created_at,
      }))

      const payload: MetricsPayload = {
        integrity_rate: total > 0 ? +(verified / total).toFixed(4) : 1,
        total_runs: total,
        verified,
        rejected,
        recent,
        source: 'db',
        metrics: {
          chyren_task_admitted_total: total,
          chyren_active_runs: 0,
          chyren_adccl_score: +avgScore.toFixed(4),
        },
        timestamp: new Date().toISOString(),
      }

      return NextResponse.json(payload)
    } catch (err) {
      console.error('[metrics] DB query failed, falling back to mock:', err)
    }
  }

  // Fallback: try Prometheus-style Medulla metrics endpoint (legacy path)
  const metricsUrl = process.env.MEDULLA_METRICS_URL
  if (metricsUrl) {
    try {
      const response = await fetch(metricsUrl, { cache: 'no-store' })
      if (response.ok) {
        const data = await response.text()
        const metrics: Record<string, number> = {}
        for (const line of data.split('\n')) {
          if (line && !line.startsWith('#')) {
            const [key, val] = line.split(' ')
            if (key && val) metrics[key] = parseFloat(val)
          }
        }
        const admitted = metrics['chyren_task_admitted_total'] ?? 0
        const adcclScore = metrics['chyren_adccl_score'] ?? 0

        const payload: MetricsPayload = {
          integrity_rate: adcclScore,
          total_runs: admitted,
          verified: Math.round(admitted * adcclScore),
          rejected: Math.round(admitted * (1 - adcclScore)),
          recent: [],
          source: 'mock',
          metrics: {
            chyren_task_admitted_total: admitted,
            chyren_active_runs: metrics['chyren_active_runs'] ?? 0,
            chyren_adccl_score: adcclScore,
          },
          timestamp: new Date().toISOString(),
        }
        return NextResponse.json(payload)
      }
    } catch {
      // fall through to mock
    }
  }

  return NextResponse.json(mockPayload())
}

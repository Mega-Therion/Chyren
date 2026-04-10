import { NextRequest } from 'next/server'

export const runtime = 'edge'

// Simulates Chyren's cognitive pipeline stages as brain activity levels.
// When real telemetry is wired from the Rust CLI, replace these with actual events.
const PIPELINE_STAGES = [
  // [stage-name, adccl, provider, threat, phylactery, ledger, alignment, duration-ms]
  { name: 'identity_load',   adccl: 0.1,  provider: 0.05, threat: 0.05, phylactery: 0.9,  ledger: 0.1,  alignment: 0.3,  ms: 400  },
  { name: 'alignment_check', adccl: 0.2,  provider: 0.1,  threat: 0.1,  phylactery: 0.4,  ledger: 0.1,  alignment: 0.95, ms: 500  },
  { name: 'threat_scan',     adccl: 0.3,  provider: 0.1,  threat: 0.9,  phylactery: 0.2,  ledger: 0.1,  alignment: 0.4,  ms: 400  },
  { name: 'provider_call',   adccl: 0.4,  provider: 0.95, threat: 0.1,  phylactery: 0.3,  ledger: 0.2,  alignment: 0.3,  ms: 1200 },
  { name: 'adccl_verify',    adccl: 0.95, provider: 0.3,  threat: 0.2,  phylactery: 0.3,  ledger: 0.3,  alignment: 0.6,  ms: 600  },
  { name: 'ledger_commit',   adccl: 0.5,  provider: 0.2,  threat: 0.05, phylactery: 0.5,  ledger: 0.95, alignment: 0.2,  ms: 300  },
  { name: 'idle',            adccl: 0.05, provider: 0.05, threat: 0.02, phylactery: 0.08, ledger: 0.02, alignment: 0.05, ms: 0    },
]

export async function GET(req: NextRequest) {
  const encoder = new TextEncoder()

  const stream = new ReadableStream({
    async start(controller) {
      const send = (data: object) => {
        controller.enqueue(encoder.encode(`data: ${JSON.stringify(data)}\n\n`))
      }

      for (const stage of PIPELINE_STAGES) {
        send({
          stage: stage.name,
          state: {
            adccl: stage.adccl,
            provider: stage.provider,
            threat: stage.threat,
            phylactery: stage.phylactery,
            ledger: stage.ledger,
            alignment: stage.alignment,
          },
        })
        if (stage.ms > 0) {
          await new Promise((r) => setTimeout(r, stage.ms))
        }
      }

      controller.close()
    },
  })

  return new Response(stream, {
    headers: {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache',
      Connection: 'keep-alive',
    },
  })
}

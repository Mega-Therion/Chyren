import { useState, useCallback, useRef } from 'react'
import type { BrainState } from '@/components/ChyrenBrain'

const IDLE: BrainState = {
  adccl: 0.05,
  provider: 0.05,
  threat: 0.02,
  phylactery: 0.08,
  ledger: 0.02,
  alignment: 0.05,
}

export function useBrainState() {
  const [brainState, setBrainState] = useState<BrainState>(IDLE)
  const [activeStage, setActiveStage] = useState<string>('idle')
  const esRef = useRef<EventSource | null>(null)

  const activate = useCallback(() => {
    if (esRef.current) {
      esRef.current.close()
    }

    const es = new EventSource('/api/brain-state')
    esRef.current = es

    es.onmessage = (e) => {
      try {
        const { stage, state } = JSON.parse(e.data)
        setActiveStage(stage)
        setBrainState(state)
      } catch {
        // ignore parse errors
      }
    }

    es.onerror = () => {
      es.close()
      esRef.current = null
      // Fade back to idle
      setTimeout(() => {
        setBrainState(IDLE)
        setActiveStage('idle')
      }, 800)
    }
  }, [])

  return { brainState, activeStage, activate }
}

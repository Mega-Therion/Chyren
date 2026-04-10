import { useState, useCallback, useRef, useEffect } from 'react'
import { type BrainState, IDLE_BRAIN } from '@/components/ChyrenBrain'

export function useBrainState() {
  const [brainState, setBrainState] = useState<BrainState>(IDLE_BRAIN)
  const [activeStage, setActiveStage] = useState<string>('idle')
  const esRef = useRef<EventSource | null>(null)

  useEffect(() => {
    return () => {
      esRef.current?.close()
    }
  }, [])

  const activate = useCallback(() => {
    esRef.current?.close()

    const es = new EventSource('/api/auth/brain-state')
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
      setTimeout(() => {
        setBrainState(IDLE_BRAIN)
        setActiveStage('idle')
      }, 800)
    }
  }, [])

  return { brainState, activeStage, activate }
}

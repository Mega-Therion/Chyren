import { getCache } from '@vercel/functions'

export interface BrainState {
  adccl: number
  provider: number
  threat: number
  phylactery: number
  ledger: number
  alignment: number
  stage: string
}

export const IDLE_STATE: BrainState = {
  adccl: 0.05,
  provider: 0.05,
  threat: 0.02,
  phylactery: 0.08,
  ledger: 0.02,
  alignment: 0.05,
  stage: 'idle',
}

// Local fallback for dev environments where Runtime Cache is unavailable
const localStore = new Map<string, BrainState>()

export async function setBrainState(session: string, state: Partial<BrainState>): Promise<void> {
  try {
    const cache = getCache({ namespace: 'brain' })
    const current = (await cache.get(session) as BrainState | undefined) ?? { ...IDLE_STATE }
    await cache.set(session, { ...current, ...state }, { ttl: 60, tags: ['brain-state'] })
  } catch {
    const current = localStore.get(session) ?? { ...IDLE_STATE }
    localStore.set(session, { ...current, ...state })
  }
}

export async function getBrainState(session: string): Promise<BrainState> {
  try {
    const cache = getCache({ namespace: 'brain' })
    return (await cache.get(session) as BrainState | undefined) ?? { ...IDLE_STATE }
  } catch {
    return localStore.get(session) ?? { ...IDLE_STATE }
  }
}

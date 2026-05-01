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

// Module-level shared state store keyed by session ID
const stateStore = new Map<string, BrainState>()

export function setBrainState(session: string, state: Partial<BrainState>): void {
  const current = stateStore.get(session) ?? { ...IDLE_STATE }
  stateStore.set(session, { ...current, ...state })
}

export function getBrainState(session: string): BrainState {
  return stateStore.get(session) ?? { ...IDLE_STATE }
}

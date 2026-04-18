/**
 * haptics-ry — Micro-vibration engine
 *
 * Provides precise haptic cues for the Chyren interface.
 * Falls back silently on unsupported platforms.
 * All patterns are deterministic and non-blocking.
 */

type HapticPattern = 'send' | 'heartbeat' | 'receive' | 'error' | 'longpress'

const PATTERNS: Record<HapticPattern, number[]> = {
  /** Sharp, crisp tap on message dispatch */
  send: [12],
  /** Soft rhythmic pulse while local model generates */
  heartbeat: [30, 120, 30, 120, 30],
  /** Light confirmation on message arrival */
  receive: [8],
  /** Double-tap for error states */
  error: [60, 80, 60],
  /** Distinct pulse for long-press activation */
  longpress: [20, 60, 20],
}

let _heartbeatIntervalId: ReturnType<typeof setInterval> | null = null

function canVibrate(): boolean {
  return typeof window !== 'undefined' && typeof navigator !== 'undefined' && 'vibrate' in navigator
}

export function haptic(pattern: HapticPattern): void {
  if (!canVibrate()) return
  try {
    navigator.vibrate(PATTERNS[pattern])
  } catch {
    // Silently ignore — some browsers block without user gesture
  }
}

/**
 * Start a continuous heartbeat pulse while the local model is under load.
 * Automatically stops when stopHeartbeat() is called.
 */
export function startHeartbeat(): void {
  if (_heartbeatIntervalId !== null) return
  haptic('heartbeat')
  _heartbeatIntervalId = setInterval(() => {
    haptic('heartbeat')
  }, 1200)
}

export function stopHeartbeat(): void {
  if (_heartbeatIntervalId !== null) {
    clearInterval(_heartbeatIntervalId)
    _heartbeatIntervalId = null
  }
}

/**
 * draft-ry — Draft message persistence
 *
 * Caches the user's in-progress input to localStorage so it survives
 * backgrounding, tab switches, and page refreshes.
 *
 * Operates with a per-session key so different conversation threads
 * don't collide. Writes are debounced to avoid spamming storage I/O.
 */

const STORAGE_PREFIX = 'chyren:draft:'
const DEBOUNCE_MS = 300

const _timers = new Map<string, ReturnType<typeof setTimeout>>()

function key(sessionId: string): string {
  return `${STORAGE_PREFIX}${sessionId}`
}

function isAvailable(): boolean {
  try {
    const test = '__chyren_ls_check__'
    localStorage.setItem(test, '1')
    localStorage.removeItem(test)
    return true
  } catch {
    return false
  }
}

/** Persist a draft, debounced. Pass empty string to clear. */
export function saveDraft(sessionId: string, text: string): void {
  const existing = _timers.get(sessionId)
  if (existing) clearTimeout(existing)

  const t = setTimeout(() => {
    if (!isAvailable()) return
    if (text.trim()) {
      localStorage.setItem(key(sessionId), text)
    } else {
      localStorage.removeItem(key(sessionId))
    }
    _timers.delete(sessionId)
  }, DEBOUNCE_MS)

  _timers.set(sessionId, t)
}

/** Load a cached draft for the session. Returns '' if none or unavailable. */
export function loadDraft(sessionId: string): string {
  if (!isAvailable()) return ''
  return localStorage.getItem(key(sessionId)) ?? ''
}

/** Immediately purge the draft for a session (call on successful send). */
export function clearDraft(sessionId: string): void {
  const existing = _timers.get(sessionId)
  if (existing) {
    clearTimeout(existing)
    _timers.delete(sessionId)
  }
  if (!isAvailable()) return
  localStorage.removeItem(key(sessionId))
}

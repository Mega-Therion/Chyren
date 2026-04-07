type HistoryItem = { content?: unknown }

export function DialogLoopDetector(history: unknown[]): boolean {
  // Detect repetition loops in conversation history
  if (history.length < 3) return false

  const last = history[history.length - 1] as HistoryItem
  const prev = history[history.length - 2] as HistoryItem

  if (typeof last?.content !== 'string' || typeof prev?.content !== 'string') return false
  return last.content === prev.content
}

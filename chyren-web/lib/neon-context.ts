/**
 * neon-context.ts — Build-time RAG context
 *
 * Context is generated at build time by scripts/generate-context.mjs
 * (runs as npm prebuild). Zero runtime I/O — the string is baked into
 * the bundle. Refreshed on every production deploy.
 */

import { GENERATED_RY_CONTEXT } from './generated-context'

const MAX_WEB_CONTEXT_CHARS = 6000

/**
 * Returns the RY context string. Synchronous, zero latency.
 * Returns '' if context was unavailable at build time.
 */
export function getRYContext(): string {
  if (GENERATED_RY_CONTEXT.length <= MAX_WEB_CONTEXT_CHARS) {
    return GENERATED_RY_CONTEXT
  }

  return (
    GENERATED_RY_CONTEXT.slice(0, MAX_WEB_CONTEXT_CHARS) +
    '\n\n[Context truncated for web runtime reliability.]'
  )
}

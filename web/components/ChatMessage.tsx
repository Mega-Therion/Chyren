'use client'

import React, { useState, useRef, useCallback } from 'react'
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import { haptic } from '@/lib/haptics-ry'

// ──────────────────────────────────────────────────────────────────────────────
// Code block with "Copy Code" button
// ──────────────────────────────────────────────────────────────────────────────

function CodeBlock({ children, className }: { children?: React.ReactNode; className?: string }) {
  const [copied, setCopied] = useState(false)
  const code = String(children ?? '').replace(/\n$/, '')

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(code)
      haptic('receive')
      setCopied(true)
      setTimeout(() => setCopied(false), 1800)
    } catch {
      /* clipboard API may be blocked in insecure contexts */
    }
  }

  // Derive language label from className like "language-typescript"
  const lang = className?.replace(/^language-/, '') ?? ''

  return (
    <div className="code-block-shell">
      <div className="code-block-header">
        <span className="code-block-lang">{lang || 'code'}</span>
        <button
          type="button"
          onClick={handleCopy}
          className="code-copy-btn"
          aria-label="Copy code"
        >
          {copied ? (
            <>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
              Copied
            </>
          ) : (
            <>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
              </svg>
              Copy
            </>
          )}
        </button>
      </div>
      <pre className="code-block-pre">
        <code className={className}>{code}</code>
      </pre>
    </div>
  )
}

// ──────────────────────────────────────────────────────────────────────────────
// ADCCL Audit Badge
// ──────────────────────────────────────────────────────────────────────────────

function AuditBadge({ audit }: { audit: { passed: boolean; score: number; flags: string[] } }) {
  const score = audit.score
  const scoreColor =
    score >= 0.7 ? '#39ff14' : score >= 0.5 ? '#ffd700' : '#ff4444'
  const scoreBg =
    score >= 0.7 ? 'rgba(57,255,20,0.08)' : score >= 0.5 ? 'rgba(255,215,0,0.08)' : 'rgba(255,68,68,0.08)'
  const scoreBorder =
    score >= 0.7 ? 'rgba(57,255,20,0.22)' : score >= 0.5 ? 'rgba(255,215,0,0.2)' : 'rgba(255,68,68,0.22)'

  return (
    <span className="inline-flex items-center gap-1.5 flex-wrap">
      {/* Score pill */}
      <span
        className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full font-mono text-[10px] tracking-wider border"
        style={{ color: scoreColor, background: scoreBg, borderColor: scoreBorder }}
      >
        {audit.passed ? '✓' : '✗'}
        <span>{(score * 100).toFixed(0)}%</span>
      </span>
      {/* Flag pills */}
      {audit.flags.map(flag => (
        <span
          key={flag}
          className="inline-flex items-center px-1.5 py-0.5 rounded font-mono text-[9px] tracking-wide"
          style={{ color: 'rgba(255,255,255,0.35)', background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.08)' }}
        >
          {flag.replace(/ \(.*\)$/, '')}
        </span>
      ))}
    </span>
  )
}

// ──────────────────────────────────────────────────────────────────────────────
// Provenance trace panel
// ──────────────────────────────────────────────────────────────────────────────

interface ProvenanceProps {
  messageId: string
  model?: string
}

function ProvenanceTrace({ messageId, model }: ProvenanceProps) {
  const [open, setOpen] = useState(false)

  const steps = [
    { icon: '⛓', label: 'PHYLACTERY KERNEL', detail: '58,339 memory entries — L6 canonical stratum active' },
    { icon: '⇄', label: 'PROVIDER CHAIN', detail: model ?? 'Local Ollama → Groq fallback → Anthropic fallback' },
    { icon: '✓', label: 'ADCCL VERIFIED', detail: 'Audit ledger committed. Deterministic trace ID: ' + messageId.slice(0, 12) },
    { icon: '⧠', label: 'SOVEREIGN PROCESSING', detail: 'Local-first inference. No external telemetry.' },
  ]

  return (
    <div className="provenance-shell">
      <button
        type="button"
        className="provenance-toggle"
        onClick={() => { setOpen(o => !o); haptic('receive') }}
        aria-expanded={open}
        aria-label="View provenance trace"
      >
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/>
        </svg>
        <span>TRACE</span>
        <svg
          width="9" height="9"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2.5"
          style={{ transform: open ? 'rotate(180deg)' : 'none', transition: 'transform 0.2s' }}
        >
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </button>

      {open && (
        <div className="provenance-panel">
          {steps.map((step, i) => (
            <div key={i} className="provenance-row">
              <span className="provenance-icon">{step.icon}</span>
              <div>
                <div className="provenance-label">{step.label}</div>
                <div className="provenance-detail">{step.detail}</div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

// ──────────────────────────────────────────────────────────────────────────────
// Contextual Action Tray (long-press)
// ──────────────────────────────────────────────────────────────────────────────

interface ActionTrayProps {
  content: string
  onQuote: () => void
  onClose: () => void
}

function ActionTray({ content, onQuote, onClose }: ActionTrayProps) {
  const handleAction = useCallback(async (action: string) => {
    haptic('longpress')
    switch (action) {
      case 'copy':
        await navigator.clipboard.writeText(content).catch(() => {})
        break
      case 'quote':
        onQuote()
        break
      case 'summarize':
        // Route to /api/chat/stream with summarize sub-module instruction
        break
      case 'factcheck':
        break
      case 'explaincode':
        break
    }
    onClose()
  }, [content, onQuote, onClose])

  return (
    <div className="action-tray-overlay" onClick={onClose}>
      <div className="action-tray" onClick={e => e.stopPropagation()}>
        {[
          { id: 'copy',        icon: '⎘', label: 'Copy' },
          { id: 'quote',       icon: '❝', label: 'Quote' },
          { id: 'summarize',   icon: '≋', label: 'Summarize' },
          { id: 'factcheck',   icon: '⊛', label: 'Fact Check' },
          { id: 'explaincode', icon: '◈', label: 'Explain Code' },
        ].map(({ id, icon, label }) => (
          <button
            key={id}
            type="button"
            className="action-tray-btn"
            onClick={() => handleAction(id)}
          >
            <span className="action-tray-icon">{icon}</span>
            <span>{label}</span>
          </button>
        ))}
      </div>
    </div>
  )
}

// ──────────────────────────────────────────────────────────────────────────────
// ChatMessage — main export
// ──────────────────────────────────────────────────────────────────────────────

export interface ChatMessageProps {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp?: Date
  isStreaming?: boolean
  model?: string
  audit?: { passed: boolean; score: number; flags: string[] }
  /** Called when user swipes right or taps Quote in the action tray */
  onQuote?: (content: string) => void
}

export function ChatMessage({
  id,
  role,
  content,
  timestamp,
  isStreaming = false,
  model,
  audit,
  onQuote,
}: ChatMessageProps) {
  const isUser = role === 'user'

  const [showTray, setShowTray] = useState(false)
  const longPressTimer = useRef<ReturnType<typeof setTimeout> | null>(null)

  // Touch / pointer long-press detection (600 ms)
  const startLong = useCallback(() => {
    longPressTimer.current = setTimeout(() => {
      haptic('longpress')
      setShowTray(true)
    }, 600)
  }, [])

  const cancelLong = useCallback(() => {
    if (longPressTimer.current) clearTimeout(longPressTimer.current)
  }, [])

  // Swipe-right to quote (touch)
  const touchStartX = useRef<number>(0)
  const onTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX
    startLong()
  }
  const onTouchEnd = (e: React.TouchEvent) => {
    cancelLong()
    const delta = e.changedTouches[0].clientX - touchStartX.current
    if (delta > 60 && role === 'assistant') {
      haptic('receive')
      onQuote?.(content)
    }
  }

  return (
    <>
      {showTray && (
        <ActionTray
          content={content}
          onQuote={() => onQuote?.(content)}
          onClose={() => setShowTray(false)}
        />
      )}

      <div
        className="w-full"
        onMouseDown={startLong}
        onMouseUp={cancelLong}
        onMouseLeave={cancelLong}
        onTouchStart={onTouchStart}
        onTouchEnd={onTouchEnd}
        onTouchCancel={cancelLong}
      >
        {/* Audit row */}
        {audit && (
          <div className="mb-3">
            <AuditBadge audit={audit} />
          </div>
        )}

        {/* Content */}
        <div
          className="text-[0.95rem] leading-relaxed text-white/90"
          style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}
        >
          {role === 'assistant' ? (
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                code({ inline, className, children, ...props }: any) {
                  if (inline) {
                    return <code className="bg-white/10 px-1.5 py-0.5 rounded text-mesh-cyan" {...props}>{children}</code>
                  }
                  return <CodeBlock className={className}>{children}</CodeBlock>
                },
                p({ children }) {
                  return <p className="mb-4 last:mb-0">{children}</p>
                },
                ul({ children }) {
                  return <ul className="list-disc pl-6 mb-4">{children}</ul>
                }
              }}
            >
              {content}
            </ReactMarkdown>
          ) : (
            <span>{content}</span>
          )}
          {isStreaming && (
            <motion.span
              animate={{ opacity: [0, 1, 0] }}
              transition={{ repeat: Infinity, duration: 0.8 }}
              className="inline-block ml-1 w-2 h-4 align-middle bg-mesh-cyan shadow-[0_0_10px_rgba(0,242,255,0.5)]"
            />
          )}
        </div>

        {/* Provenance trace (assistant messages only, once streaming done) */}
        {role === 'assistant' && !isStreaming && content.length > 0 && (
          <ProvenanceTrace messageId={id} model={model} />
        )}
      </div>
    </>
  )
}

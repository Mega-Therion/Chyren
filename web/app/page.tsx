'use client'

import React, { useCallback, useEffect, useRef, useState } from 'react'
import dynamic from 'next/dynamic'
import { AlertCircle, Loader2, Volume2, VolumeX } from 'lucide-react'
import { ChatMessage } from '@/components/ChatMessage'
import { ChatInput } from '@/components/ChatInput'
import { MetricsDashboard } from '@/components/MetricsDashboard';
import { startHeartbeat, stopHeartbeat } from '@/lib/haptics-ry'
import { clearDraft } from '@/lib/draft-ry'
import { createTtsEngine, type TtsEngine, playLatencyChime } from '@/lib/tts-ry'

const NeuralBrain = dynamic(() => import('@/components/NeuralBrain'), { ssr: false })

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
  model?: string
  audit?: {
    passed: boolean
    score: number
    flags: string[]
  }
}

function parseHubChunk(chunk: string): {
  content: string
  audit?: { passed: boolean; score: number; flags: string[] }
} {
  let content = ''
  let audit: { passed: boolean; score: number; flags: string[] } | undefined

  for (const line of chunk.split('\n')) {
    const trimmed = line.trim()
    if (!trimmed.startsWith('data: ')) continue
    try {
      const json = JSON.parse(trimmed.slice(6))
      const delta = json.choices?.[0]?.delta?.content
      if (delta) content += delta
      if (json.status === 'audited') audit = json.audit_report
    } catch {
      // Ignore malformed SSE frames
    }
  }
  return { content, audit }
}

function consumeSseBuffer(buffer: string): {
  events: Array<{ content: string; audit?: { passed: boolean; score: number; flags: string[] } }>
  remaining: string
} {
  const records = buffer.split('\n\n')
  const remaining = records.pop() ?? ''
  const events = records.map(parseHubChunk).filter(e => e.content || e.audit)
  return { events, remaining }
}

const EMPTY_PROMPTS = [
  'Summarize my current project status and next actions.',
  'Draft a deployment checklist for chyren-web on Vercel.',
  'Find UI/UX issues in this chat UI and propose fixes.',
  'Help me write a clear GitHub issue for the biggest bug here.',
]

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [streamingId, setStreamingId] = useState<string | null>(null)
  const [ttsEnabled, setTtsEnabled] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [sessionId] = useState<string>(() => crypto.randomUUID().replace(/-/g, ''))
  const [audioLevel, setAudioLevel] = useState(0) // 0–1 from mic
  const [quotedText, setQuotedText] = useState<string | undefined>()

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const chatWindowRef = useRef<HTMLDivElement>(null)
  const ttsRef = useRef<TtsEngine | null>(null)

  useEffect(() => {
    ttsRef.current = createTtsEngine()
    return () => {
      ttsRef.current?.halt()
    }
  }, [])

  const scrollToBottom = useCallback((behavior: ScrollBehavior = 'smooth') => {
    messagesEndRef.current?.scrollIntoView({ behavior, block: 'end' })
    requestAnimationFrame(() => {
      const el = chatWindowRef.current
      if (el) el.scrollTop = el.scrollHeight
    })
  }, [])

  useEffect(() => {
    scrollToBottom('smooth')
  }, [messages, scrollToBottom])

  useEffect(() => {
    if (isStreaming) startHeartbeat()
    else stopHeartbeat()
    return () => stopHeartbeat()
  }, [isStreaming])

  const handleBargeIn = useCallback(() => {
    ttsRef.current?.halt()
  }, [])

  const handleQuote = useCallback((content: string) => {
    setQuotedText(content)
  }, [])

  const sendMessage = useCallback(async (text: string) => {
    const trimmed = text.trim()
    if (!trimmed || isStreaming) return

    ttsRef.current?.halt()
    ttsRef.current?.reset()
    setError(null)
    clearDraft(sessionId)

    const userMsg: Message = {
      id: `u-${Date.now()}`,
      role: 'user',
      content: trimmed,
      timestamp: new Date(),
    }

    const requestMessages = [...messages, userMsg].map(({ role, content }) => ({ role, content }))
    setMessages(prev => [...prev, userMsg])
    setIsStreaming(true)

    const assistantId = `a-${Date.now()}`
    setStreamingId(assistantId)
    setMessages(prev => [...prev, { id: assistantId, role: 'assistant', content: '', timestamp: new Date() }])

    if (ttsEnabled) {
      playLatencyChime()
    }

    try {
      const res = await fetch(`/api/chat/stream?session=${sessionId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: trimmed, messages: requestMessages }),
      })
      if (!res.ok) throw new Error('Neural link failure')

      const reader = res.body?.getReader()
      const decoder = new TextDecoder()
      let accumulated = ''
      let sseBuffer = ''

      if (reader) {
        while (true) {
          const { done, value } = await reader.read()
          sseBuffer += decoder.decode(value ?? new Uint8Array(), { stream: !done })
          const { events, remaining } = consumeSseBuffer(sseBuffer)
          sseBuffer = remaining

          for (const { content: delta, audit } of events) {
            if (delta) {
              accumulated += delta
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, content: accumulated } : m)))
              if (ttsEnabled) ttsRef.current?.feedDelta(delta)
              scrollToBottom('auto')
            }
            if (audit) {
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, audit } : m)))
            }
          }

          if (done) break
        }
      }
    } catch {
      setError('Neural connection disrupted')
      setMessages(prev => prev.filter(m => m.id !== assistantId))
    } finally {
      setIsStreaming(false)
      setStreamingId(null)
      if (ttsEnabled) ttsRef.current?.finish()
      scrollToBottom('smooth')
    }
  }, [isStreaming, messages, scrollToBottom, sessionId, ttsEnabled])

  const headerLoadClass = isStreaming ? 'header-title--active' : ''

  return (
    <div className="omega-viewport">
      <div className="omega-bg-fx">
        <NeuralBrain isActive={isStreaming} audioLevel={audioLevel} />
      </div>

      <div className="omega-orb orb-1" />
      <div className="omega-orb orb-2" />

      <main className="phone-container">
        <div className="phone-notch" />

        <header className="phone-chrome">
          <h1 className={`phone-title ${headerLoadClass}`}>CHYREN</h1>
          <div className="sovereign-seal">R.W.&#x03DC;.Y. &mdash; SOVEREIGN INTELLIGENCE</div>
          {isStreaming && (
            <span className="header-inference-badge">
              <span className="header-pulse-dot" />
              PROCESSING
            </span>
          )}
        </header>

        {/* Pipeline status bar */}
        <div className="pipeline-bar" aria-label="Pipeline stages">
          {(['ALIGN', 'AEON', 'PROVIDER', 'ADCCL', 'LEDGER'] as const).map((stage, i, arr) => (
            <React.Fragment key={stage}>
              <span className={`pipeline-stage${isStreaming ? ' pipeline-stage--active' : ''}`}>
                <span className="pipeline-dot" />
                {stage}
              </span>
              {i < arr.length - 1 && <span className="pipeline-sep" aria-hidden="true" />}
            </React.Fragment>
          ))}
        </div>

        <MetricsDashboard />

        <section
          ref={chatWindowRef}
          className="chat-window"
          aria-label="Chat transcript"
          onClick={(e) => {
            const target = e.target as HTMLElement
            if (!target.closest('textarea') && !target.closest('button')) {
              ;(document.activeElement as HTMLElement | null)?.blur()
            }
          }}
        >
          {messages.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-inner">
                <div className="empty-state-sigil">Ω</div>
                <div className="empty-state-title">Awaiting Intent</div>
                <p className="empty-state-subtitle">
                  Start with a concrete task. You can paste an error log, request a change, or pick one of these.
                </p>
                <div className="empty-prompts" role="list">
                  {EMPTY_PROMPTS.map(p => (
                    <button
                      key={p}
                      type="button"
                      className="prompt-chip"
                      onClick={() => { void sendMessage(p) }}
                      role="listitem"
                    >
                      {p}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          ) : (
            messages.map(msg => (
              <ChatMessage
                key={msg.id}
                id={msg.id}
                role={msg.role}
                content={msg.content}
                timestamp={msg.timestamp}
                isStreaming={msg.id === streamingId}
                model={msg.model}
                audit={msg.audit}
                onQuote={handleQuote}
              />
            ))
          )}

          <div ref={messagesEndRef} style={{ height: 1 }} />
        </section>

        {error && (
          <div className="px-6 py-2 bg-rose-500/10 border-t border-rose-500/20 text-rose-400 text-xs flex items-center gap-2">
            <AlertCircle size={12} /> {error}
          </div>
        )}

        <footer className="input-dock">
          <div className="input-dock-controls">
            <button
              type="button"
              className="p-2 opacity-40 hover:opacity-100 transition-opacity"
              aria-label={ttsEnabled ? 'Disable text to speech' : 'Enable text to speech'}
              aria-pressed={ttsEnabled}
              onClick={() => setTtsEnabled(v => !v)}
            >
              {ttsEnabled ? <Volume2 size={16} className="text-cyan-400" /> : <VolumeX size={16} />}
            </button>
          </div>

          <ChatInput
            onSend={(t) => { void sendMessage(t) }}
            onBargeIn={handleBargeIn}
            quotedText={quotedText}
            onQuoteConsumed={() => setQuotedText(undefined)}
            onAudioLevel={setAudioLevel}
            disabled={false}
            isLoading={isStreaming}
            sessionId={sessionId}
          />

          {isStreaming && (
            <div className="streaming-indicator">
              <Loader2 size={11} className="animate-spin opacity-60" />
              <span>Generating…</span>
            </div>
          )}
        </footer>
      </main>
    </div>
  )
}


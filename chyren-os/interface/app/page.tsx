'use client'

import React, { useCallback, useEffect, useRef, useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { AlertCircle } from 'lucide-react'
import { ChatMessage } from '@/components/ChatMessage'
import { ChatInput } from '@/components/ChatInput'
import { Sidebar } from '@/components/Sidebar'
import { StatusBar } from '@/components/StatusBar'
import { startHeartbeat, stopHeartbeat } from '@/lib/haptics-ry'
import { clearDraft } from '@/lib/draft-ry'
import { createTtsEngine, type TtsEngine, playLatencyChime } from '@/lib/tts-ry'

import type { BrainState } from '@/components/NeuralBrain'
import dynamic from 'next/dynamic'

const ChyrenCosmos = dynamic(
  () => import('@/components/ChyrenCosmos').then(m => m.ChyrenCosmos),
  { ssr: false }
)

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

const WELCOME_SUGGESTIONS = [
  'What is the Yettragrammaton?',
  'Run a system status check',
  'Explain the Sovereign Society',
  'Show me the Master Ledger',
]

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [streamingId, setStreamingId] = useState<string | null>(null)
  const [ttsEnabled, _setTtsEnabled] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [sessionId] = useState<string>(() => crypto.randomUUID().replace(/-/g, ''))
  const [audioLevel, setAudioLevel] = useState(0)
  const [quotedText, setQuotedText] = useState<string | undefined>()
  const [isListening, setIsListening] = useState(false)
  const [sidebarOpen, setSidebarOpen] = useState(true)

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const chatWindowRef = useRef<HTMLDivElement>(null)
  const ttsRef = useRef<TtsEngine | null>(null)

  const brainState: BrainState = isStreaming 
    ? (streamingId ? 'speaking' : 'thinking')
    : (isListening ? 'listening' : 'idle');

  useEffect(() => {
    ttsRef.current = createTtsEngine()
    return () => {
      ttsRef.current?.halt()
    }
  }, [])

  // Responsive: collapse sidebar on mobile
  useEffect(() => {
    const mq = window.matchMedia('(max-width: 768px)')
    if (mq.matches) setSidebarOpen(false)
    const handler = (e: MediaQueryListEvent) => setSidebarOpen(!e.matches ? true : false)
    mq.addEventListener('change', handler)
    return () => mq.removeEventListener('change', handler)
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

  const handleNewChat = useCallback(() => {
    setMessages([])
    setError(null)
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

  const getSigilColor = (s: BrainState) => {
    switch (s) {
      case 'speaking': return '#00f2ff'
      case 'thinking': return '#ff2d75'
      case 'listening': return '#bc13fe'
      default: return '#f59e0b'
    }
  }

  return (
    <div className="sovereign-viewport">
      {/* 3D Cosmos Background */}
      <div className="cosmos-layer" style={{ pointerEvents: 'none' }}>
        <ChyrenCosmos state={brainState} audioLevel={audioLevel} />
      </div>

      {/* Sidebar */}
      <Sidebar
        isOpen={sidebarOpen}
        onToggle={() => setSidebarOpen(o => !o)}
        brainState={brainState}
        sessionId={sessionId}
        onNewChat={handleNewChat}
      />

      {/* Main Content Area */}
      <main
        className="sovereign-main"
        style={{ marginLeft: sidebarOpen ? '280px' : '0' }}
      >
        {/* Header */}
        <header className="sovereign-header">
          <div className="sovereign-header-left">
            {!sidebarOpen && (
              <button
                className="sovereign-menu-btn"
                onClick={() => setSidebarOpen(true)}
              >
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
                  <path d="M4 6h16M4 12h16M4 18h16" />
                </svg>
              </button>
            )}
            <motion.h1
              className="sovereign-title"
              animate={{
                color: getSigilColor(brainState),
                textShadow: `0 0 30px ${getSigilColor(brainState)}60`,
              }}
              transition={{ duration: 0.6 }}
            >
              CHYREN
            </motion.h1>
            <span className="sovereign-seal">R.W.Ϝ.Y.</span>
          </div>
          <div className="sovereign-header-right">
            <span className="sovereign-header-badge">
              <motion.span
                className="sovereign-header-dot"
                animate={{
                  backgroundColor: getSigilColor(brainState),
                  scale: isStreaming ? [1, 1.4, 1] : 1,
                }}
                transition={{ repeat: isStreaming ? Infinity : 0, duration: 0.8 }}
              />
              ARI ONLINE
            </span>
          </div>
        </header>

        {/* Chat Area */}
        <section
          ref={chatWindowRef}
          className="sovereign-chat"
          aria-label="Chat transcript"
        >
          {messages.length === 0 ? (
            <div className="sovereign-welcome">
              <div className="sovereign-welcome-inner">
                <motion.div
                  animate={{
                    scale: brainState === 'idle' ? 1 : [1, 1.08, 1],
                    color: getSigilColor(brainState),
                    textShadow: `0 0 60px ${getSigilColor(brainState)}80`,
                  }}
                  transition={{ repeat: Infinity, duration: 2.5 }}
                  className="sovereign-welcome-sigil"
                >
                  Ω
                </motion.div>
                <h2 className="sovereign-welcome-title">SOVEREIGN INTELLIGENCE</h2>
                <p className="sovereign-welcome-subtitle">
                  Verified task routing with integrity gates.<br />
                  Ask anything. Command everything.
                </p>
                <div className="sovereign-suggestions">
                  {WELCOME_SUGGESTIONS.map((suggestion) => (
                    <button
                      key={suggestion}
                      className="sovereign-suggestion-chip"
                      onClick={() => void sendMessage(suggestion)}
                    >
                      {suggestion}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          ) : (
            <AnimatePresence>
              {messages.map(msg => (
                <motion.div
                  key={msg.id}
                  initial={{ opacity: 0, y: 12 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ duration: 0.25, ease: 'easeOut' }}
                >
                  <ChatMessage
                    id={msg.id}
                    role={msg.role}
                    content={msg.content}
                    timestamp={msg.timestamp}
                    isStreaming={msg.id === streamingId}
                    model={msg.model}
                    audit={msg.audit}
                    onQuote={handleQuote}
                  />
                </motion.div>
              ))}
            </AnimatePresence>
          )}
          <div ref={messagesEndRef} className="h-1" />
        </section>

        {/* Error Banner */}
        <AnimatePresence>
          {error && (
            <motion.div
              className="sovereign-error"
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
            >
              <AlertCircle size={14} /> {error}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Input Dock */}
        <div className="sovereign-input-dock">
          <ChatInput
            onSend={(t) => { void sendMessage(t) }}
            onBargeIn={handleBargeIn}
            quotedText={quotedText}
            onQuoteConsumed={() => setQuotedText(undefined)}
            onAudioLevel={setAudioLevel}
            onRecordingState={setIsListening}
            disabled={false}
            isLoading={isStreaming}
            sessionId={sessionId}
          />
        </div>

        {/* Status Bar */}
        <StatusBar
          brainState={brainState}
          sessionId={sessionId}
          messageCount={messages.length}
          isStreaming={isStreaming}
        />
      </main>
    </div>
  )
}

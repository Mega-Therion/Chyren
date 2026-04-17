'use client'

import React, { useCallback, useEffect, useRef, useState } from 'react'
import { motion } from 'framer-motion'
import { AlertCircle } from 'lucide-react'
import { ChatMessage } from '@/components/ChatMessage'
import { ChatInput } from '@/components/ChatInput'
import { startHeartbeat, stopHeartbeat } from '@/lib/haptics-ry'
import { clearDraft } from '@/lib/draft-ry'
import { createTtsEngine, type TtsEngine, playLatencyChime } from '@/lib/tts-ry'

import { NeuralBrain, type BrainState } from '@/components/NeuralBrain'

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

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const chatWindowRef = useRef<HTMLDivElement>(null)
  const ttsRef = useRef<TtsEngine | null>(null)

  // Determine the overall state for colors
  const brainState: BrainState = isStreaming 
    ? (streamingId ? 'speaking' : 'thinking')
    : (isListening ? 'listening' : 'idle');

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

  // Get color for OmegA based on state
  const getSigilColor = (s: BrainState) => {
    switch (s) {
      case 'speaking': return '#00f2ff'; // Cyan
      case 'thinking': return '#ff2d75'; // Rose
      case 'listening': return '#bc13fe'; // Violet
      default: return '#f59e0b'; // Amber
    }
  };

  return (
    <div className="omega-viewport bg-black">
      <div className="omega-bg-fx">
        <NeuralBrain _isActive={brainState !== 'idle'} audioLevel={audioLevel} state={brainState} />
        <div className="omega-orb orb-1" />
        <div className="omega-orb orb-2" />
      </div>

      <main className="phone-container !bg-black/40 !border-white/5 !shadow-2xl">
        <header className="phone-chrome !border-b-0 !bg-transparent pt-12">
          <h1 className="phone-title !text-white opacity-80 !tracking-[0.5em]">CHYREN</h1>
        </header>

        <section
          ref={chatWindowRef}
          className="chat-window"
          aria-label="Chat transcript"
        >
          {messages.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-inner">
                <motion.div 
                  animate={{ 
                    scale: brainState === 'idle' ? 1 : [1, 1.1, 1],
                    color: getSigilColor(brainState),
                    textShadow: `0 0 40px ${getSigilColor(brainState)}`
                  }}
                  transition={{ repeat: Infinity, duration: 2 }}
                  className="empty-state-sigil"
                  style={{ color: getSigilColor(brainState) }}
                >Ω</motion.div>
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
          <div ref={messagesEndRef} className="h-0.5" />
        </section>

        {error && (
          <div className="px-6 py-2 bg-rose-500/10 border-t border-rose-500/20 text-rose-400 text-xs flex items-center gap-2">
            <AlertCircle size={12} /> {error}
          </div>
        )}

        <footer className="input-dock !bg-transparent">
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
        </footer>
      </main>
    </div>
  )
}

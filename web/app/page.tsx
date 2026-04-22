'use client'

import React, { useCallback, useEffect, useRef, useState } from 'react'
import dynamic from 'next/dynamic'
import { motion, AnimatePresence } from 'framer-motion'
import { AlertCircle, ShieldCheck, ShieldX } from 'lucide-react'
import { ChatMessage } from '@/components/ChatMessage'
import { ChatInput } from '@/components/ChatInput'
import { startHeartbeat, stopHeartbeat } from '@/lib/haptics-ry'
import { clearDraft } from '@/lib/draft-ry'
import { createTtsEngine, type TtsEngine, playLatencyChime } from '@/lib/tts-ry'
import { MetricsDashboard } from '@/components/MetricsDashboard'
import type { BrainState } from '@/components/NeuralBrain'

const NeuralBrain = dynamic(
  () => import('@/components/NeuralBrain').then(m => m.NeuralBrain),
  { ssr: false, loading: () => null },
)

// ─── ARI metadata type (mirrors lib/ari-gate.ts) ─────────────────────────────
interface AriMeta {
  allowed: boolean
  riskTier: 'Benign' | 'Elevated' | 'Critical'
  adcclScore: number
  ledgerHash: string
  admittedAt: string
}

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
  model?: string
  audit?: { passed: boolean; score: number; flags: string[] }
  ari?: AriMeta
}

// ─── Pipeline stages ──────────────────────────────────────────────────────────
type PipelineStage = 'idle' | 'vad' | 'stt' | 'cas' | 'llm' | 'tts' | 'done'
const PIPELINE_LABELS: Record<PipelineStage, string> = {
  idle: '', vad: 'VAD', stt: 'STT', cas: 'C.A.S.', llm: 'LLM', tts: 'TTS', done: 'DONE',
}

function parseHubChunk(chunk: string): {
  content: string
  audit?: { passed: boolean; score: number; flags: string[] }
  ari?: AriMeta
} {
  let content = ''
  let audit: { passed: boolean; score: number; flags: string[] } | undefined
  let ari: AriMeta | undefined
  for (const line of chunk.split('\n')) {
    const trimmed = line.trim()
    if (!trimmed.startsWith('data: ')) continue
    try {
      const json = JSON.parse(trimmed.slice(6))
      const delta = json.choices?.[0]?.delta?.content
      if (delta) content += delta
      if (json.status === 'audited') audit = json.audit_report
      if (json.ari) ari = json.ari as AriMeta
    } catch { /* ignore malformed frames */ }
  }
  return { content, audit, ari }
}

function consumeSseBuffer(buffer: string): {
  events: Array<{ content: string; audit?: { passed: boolean; score: number; flags: string[] }; ari?: AriMeta }>
  remaining: string
} {
  const records = buffer.split('\n\n')
  const remaining = records.pop() ?? ''
  const events = records.map(parseHubChunk).filter(e => e.content || e.audit || e.ari)
  return { events, remaining }
}

// ─── ARI verdict badge ────────────────────────────────────────────────────────
function AriVerdictBadge({ ari }: { ari: AriMeta }) {
  const tierStyle = {
    Benign:   { bg: 'rgba(57,255,20,0.08)',   border: 'rgba(57,255,20,0.3)',  color: '#39ff14' },
    Elevated: { bg: 'rgba(255,223,0,0.08)',   border: 'rgba(255,223,0,0.3)', color: '#ffdf00' },
    Critical: { bg: 'rgba(255,45,117,0.08)', border: 'rgba(255,45,117,0.3)', color: '#ff2d75' },
  }[ari.riskTier]
  const Icon = ari.allowed ? ShieldCheck : ShieldX
  return (
    <motion.div initial={{ opacity: 0, y: 4 }} animate={{ opacity: 1, y: 0 }}
      className="flex items-center gap-2 mt-2 flex-wrap pl-3">
      <span style={{ background: tierStyle.bg, border: `1px solid ${tierStyle.border}`, color: tierStyle.color }}
        className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[0.6rem] font-mono tracking-widest uppercase">
        <Icon size={9} /> C.A.S. · {ari.riskTier}
      </span>
      <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[0.6rem] font-mono tracking-widest uppercase"
        style={{ background: 'rgba(0,242,255,0.06)', border: '1px solid rgba(0,242,255,0.2)', color: 'rgba(0,242,255,0.85)' }}>
        ADCCL {(ari.adcclScore * 100).toFixed(0)}%
      </span>
      <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[0.6rem] font-mono tracking-widest uppercase"
        style={{ background: 'rgba(188,19,254,0.06)', border: '1px solid rgba(188,19,254,0.2)', color: 'rgba(188,19,254,0.75)' }}>
        R.W.Ϝ.Y. · {ari.ledgerHash.slice(0, 8)}…
      </span>
    </motion.div>
  )
}

// ─── Pipeline status bar ──────────────────────────────────────────────────────
const STAGE_ORDER: PipelineStage[] = ['vad', 'stt', 'cas', 'llm', 'tts']
function PipelineBar({ stage }: { stage: PipelineStage }) {
  if (stage === 'idle') return null
  return (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
      className="pipeline-bar" aria-label="ARI pipeline status">
      {STAGE_ORDER.map((s, i) => {
        const stageIdx = STAGE_ORDER.indexOf(stage)
        const isDone   = i < stageIdx
        const isActive = i === stageIdx
        return (
          <React.Fragment key={s}>
            <span className={`pipeline-stage ${isActive ? 'pipeline-stage--active' : ''} ${isDone ? 'pipeline-stage--done' : ''}`}>
              <span className="pipeline-dot" />
              {PIPELINE_LABELS[s]}
            </span>
            {i < STAGE_ORDER.length - 1 && <span className="pipeline-sep" />}
          </React.Fragment>
        )
      })}
    </motion.div>
  )
}

export default function ChatPage() {
  const [messages, setMessages]     = useState<Message[]>([])
  const [isStreaming, setIsStreaming] = useState(false)
  const [streamingId, setStreamingId] = useState<string | null>(null)
  const [ttsEnabled]                 = useState(true)
  const [error, setError]            = useState<string | null>(null)
  const [sessionId]                  = useState<string>(() => crypto.randomUUID().replace(/-/g, ''))
  const [audioLevel, setAudioLevel]  = useState(0)
  const [quotedText, setQuotedText]  = useState<string | undefined>()
  const [isListening, setIsListening] = useState(false)
  const [pipeline, setPipeline]      = useState<PipelineStage>('idle')

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const chatWindowRef  = useRef<HTMLDivElement>(null)
  const ttsRef         = useRef<TtsEngine | null>(null)

  const latestAri = messages.findLast(m => m.role === 'assistant' && m.ari)?.ari
  const brainState: BrainState = isStreaming
    ? (streamingId ? 'speaking' : 'thinking')
    : (isListening ? 'listening' : 'idle')
  const riskTier = latestAri?.riskTier ?? 'Benign'

  useEffect(() => {
    ttsRef.current = createTtsEngine()
    return () => { ttsRef.current?.halt() }
  }, [])

  const scrollToBottom = useCallback((behavior: ScrollBehavior = 'smooth') => {
    messagesEndRef.current?.scrollIntoView({ behavior, block: 'end' })
  }, [])

  useEffect(() => { scrollToBottom('smooth') }, [messages, scrollToBottom])
  useEffect(() => {
    if (isStreaming) startHeartbeat()
    else stopHeartbeat()
    return () => stopHeartbeat()
  }, [isStreaming])

  const handleBargeIn = useCallback(() => { ttsRef.current?.halt() }, [])
  const handleQuote   = useCallback((content: string) => { setQuotedText(content) }, [])

  const sendMessage = useCallback(async (text: string) => {
    const trimmed = text.trim()
    if (!trimmed || isStreaming) return

    ttsRef.current?.halt()
    ttsRef.current?.reset()
    setError(null)
    clearDraft(sessionId)

    const userMsg: Message = { id: `u-${Date.now()}`, role: 'user', content: trimmed, timestamp: new Date() }
    const requestMessages  = [...messages, userMsg].map(({ role, content }) => ({ role, content }))
    setMessages(prev => [...prev, userMsg])
    setIsStreaming(true)
    setPipeline('cas')

    const assistantId = `a-${Date.now()}`
    setStreamingId(assistantId)
    setMessages(prev => [...prev, { id: assistantId, role: 'assistant', content: '', timestamp: new Date() }])
    if (ttsEnabled) playLatencyChime()

    try {
      setPipeline('llm')
      const res = await fetch(`/api/chat/stream?session=${sessionId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: trimmed, messages: requestMessages }),
      })
      if (!res.ok) throw new Error('Neural link failure')

      const reader  = res.body?.getReader()
      const decoder = new TextDecoder()
      let accumulated = ''
      let sseBuffer   = ''
      let gotFirst    = false

      if (reader) {
        while (true) {
          const { done, value } = await reader.read()
          sseBuffer += decoder.decode(value ?? new Uint8Array(), { stream: !done })
          const { events, remaining } = consumeSseBuffer(sseBuffer)
          sseBuffer = remaining

          for (const { content: delta, audit, ari } of events) {
            if (delta) {
              if (!gotFirst) { gotFirst = true; setPipeline('tts') }
              accumulated += delta
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, content: accumulated } : m)))
              if (ttsEnabled) ttsRef.current?.feedDelta(delta)
              scrollToBottom('auto')
            }
            if (audit)
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, audit } : m)))
            if (ari)
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, ari } : m)))
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
      setPipeline('done')
      setTimeout(() => setPipeline('idle'), 1800)
      if (ttsEnabled) ttsRef.current?.finish()
      scrollToBottom('smooth')
    }
  }, [isStreaming, messages, scrollToBottom, sessionId, ttsEnabled])

  return (
    <div className="dashboard-root">
      <div className="sovereign-bg">
        <div className="energy-orb" style={{ top: '10%', left: '20%', width: '300px', height: '300px', background: 'var(--gold-glow)' }} />
        <div className="energy-orb" style={{ bottom: '20%', right: '10%', width: '400px', height: '400px', background: 'var(--mesh-violet)', opacity: 0.1 }} />
      </div>

      <div className="fixed inset-0 z-0 pointer-events-none">
        <NeuralBrain _isActive={brainState !== 'idle'} audioLevel={audioLevel} state={brainState} riskTier={riskTier} />
      </div>

      {/* Sovereign Hub Core - Chat & Interaction */}
      <main className="hub-core glass-panel overflow-hidden">
        <header className="hub-header">
          <div className="flex flex-col">
            <h1 className="hub-title">CHYREN</h1>
          </div>

          <div className="flex items-center gap-4">
            <AnimatePresence><PipelineBar stage={pipeline} /></AnimatePresence>
          </div>
        </header>

        <section ref={chatWindowRef} className="chat-window">
          {messages.length === 0 ? (
            <div className="flex-1 flex items-center justify-center p-12">
              <div className="max-w-xl text-center">
                <motion.div 
                  initial={{ scale: 0.9, opacity: 0 }}
                  animate={{ scale: 1, opacity: 1 }}
                  className="text-7xl mb-6 text-gold-core opacity-80"
                  style={{ textShadow: '0 0 40px var(--gold-glow)' }}
                >
                  Ω
                </motion.div>
                <h2 className="text-xl mb-4 text-white/80 tracking-[0.2em]">How can I help you today?</h2>
                <p className="text-white/40 font-light mb-10 leading-relaxed">
                  I am Chyren. Start a conversation below.
                </p>
                <div className="grid grid-cols-2 gap-4">
                  {['What are the Millennium Prize Problems?', 'Explain your ARI architecture', 'What is ADCCL?', 'Tell me about your memory system']
                    .map(p => (
                      <button 
                        key={p} 
                        className="p-4 rounded-xl bg-white/5 border border-white/10 text-white/60 text-sm hover:bg-white/10 hover:border-white/20 transition-all text-left"
                        onClick={() => void sendMessage(p)}
                      >
                        {p}
                      </button>
                    ))}
                </div>
              </div>
            </div>
          ) : (
            messages.map(msg => (
              <div key={msg.id} className={`message-bubble ${msg.role === 'assistant' ? 'message-assistant' : 'message-user'}`}>
                <ChatMessage
                  id={msg.id} role={msg.role} content={msg.content} timestamp={msg.timestamp}
                  isStreaming={msg.id === streamingId} model={msg.model} audit={msg.audit} onQuote={handleQuote}
                />
                {msg.role === 'assistant' && msg.ari && <AriVerdictBadge ari={msg.ari} />}
              </div>
            ))
          )}
          <div ref={messagesEndRef} className="h-4" />
        </section>

        {error && (
          <div className="mx-8 mb-4 p-3 rounded-xl bg-rose-500/10 border border-rose-500/20 text-rose-400 text-xs flex items-center gap-2">
            <AlertCircle size={14} /> {error}
          </div>
        )}

        <footer className="hub-footer">
          <div className="input-container">
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
        </footer>
      </main>
    </div>
  )
}

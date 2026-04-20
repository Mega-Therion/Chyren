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

  const brainState: BrainState = isStreaming
    ? (streamingId ? 'speaking' : 'thinking')
    : (isListening ? 'listening' : 'idle')

  useEffect(() => {
    ttsRef.current = createTtsEngine()
    return () => { ttsRef.current?.halt() }
  }, [])

  const scrollToBottom = useCallback((behavior: ScrollBehavior = 'smooth') => {
    messagesEndRef.current?.scrollIntoView({ behavior, block: 'end' })
    requestAnimationFrame(() => {
      const el = chatWindowRef.current
      if (el) el.scrollTop = el.scrollHeight
    })
  }, [])

  useEffect(() => { scrollToBottom('smooth') }, [messages, scrollToBottom])
  useEffect(() => {
    if (isStreaming) startHeartbeat()
    else stopHeartbeat()
    return () => stopHeartbeat()
  }, [isStreaming])

  const handleBargeIn = useCallback(() => { ttsRef.current?.halt() }, [])
  const handleQuote   = useCallback((content: string) => { setQuotedText(content) }, [])

  const getSigilColor = (s: BrainState) => {
    switch (s) {
      case 'speaking':  return '#00f2ff'
      case 'thinking':  return '#ff2d75'
      case 'listening': return '#bc13fe'
      default:          return '#f59e0b'
    }
  }

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
    <div className="omega-viewport bg-black flex w-full h-full relative">
      <div className="omega-bg-fx">
        <NeuralBrain _isActive={brainState !== 'idle'} audioLevel={audioLevel} state={brainState} />
        <div className="omega-orb orb-1" />
        <div className="omega-orb orb-2" />
      </div>

      {/* Holonomic telemetry panel – desktop only */}
      <div className="hidden xl:flex flex-col w-[450px] z-10 h-full overflow-hidden p-6 absolute left-0 top-0 bottom-0">
        <div className="mb-4">
          <h2 className="text-xl font-mono text-white/80 tracking-[0.2em] uppercase">Holonomic Core</h2>
          <p className="text-xs text-white/40 font-mono mt-1">Real-time MCP Spoke &amp; Cortex Telemetry</p>
        </div>
        <div className="flex-1 overflow-hidden bg-black/60 rounded-2xl border border-white/10 backdrop-blur-md shadow-2xl">
          <MetricsDashboard />
        </div>
      </div>

      <main className="phone-container !bg-black/40 !border-white/5 !shadow-2xl relative z-20 mx-auto">

        {/* ARI Header */}
        <header className="phone-chrome !border-b-0 !bg-transparent pt-10 pb-1 gap-0.5">
          <div className="phone-notch" />
          <motion.h1 className="phone-title !tracking-[0.5em]"
            animate={{
              textShadow: brainState === 'idle'
                ? '0 0 18px rgba(245,158,11,0.55)'
                : ['0 0 24px rgba(245,158,11,1)', '0 0 60px rgba(245,158,11,0.5)', '0 0 24px rgba(245,158,11,1)']
            }}
            transition={{ repeat: Infinity, duration: 1.6 }}>
            CHYREN
          </motion.h1>

          {/* Sovereign seal row */}
          <div className="flex items-center gap-2 mt-1">
            <span className="sovereign-seal tracking-[0.3em]">R.W.Ϝ.Y.</span>
            <span style={{ width: 1, height: 10, background: 'rgba(245,158,11,0.25)' }} />
            <span className="sovereign-seal">ARI GENESIS</span>
            <span style={{ width: 1, height: 10, background: 'rgba(245,158,11,0.25)' }} />
            <motion.span className="sovereign-seal" style={{ color: 'rgba(57,255,20,0.7)' }}
              animate={{ opacity: [0.5, 1, 0.5] }} transition={{ repeat: Infinity, duration: 2 }}>
              ADCCL ACTIVE
            </motion.span>
          </div>

          <AnimatePresence><PipelineBar stage={pipeline} /></AnimatePresence>
        </header>

        {/* Chat area */}
        <section ref={chatWindowRef} className="chat-window" aria-label="Chat transcript">
          {messages.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-inner">
                <motion.div
                  animate={{ scale: brainState === 'idle' ? 1 : [1, 1.1, 1], color: getSigilColor(brainState), textShadow: `0 0 40px ${getSigilColor(brainState)}` }}
                  transition={{ repeat: Infinity, duration: 2 }}
                  className="empty-state-sigil" style={{ color: getSigilColor(brainState) }}>Ω
                </motion.div>
                <p className="empty-state-title" style={{ color: 'rgba(245,158,11,0.6)' }}>ARI INSTANCE ONLINE</p>
                <p className="empty-state-subtitle">Sovereign intelligence active. C.A.S. gate armed. ADCCL threshold 0.7.</p>
                <div className="empty-prompts">
                  {['What are the Millennium Prize Problems?', 'Explain your ARI architecture', 'What is ADCCL?', 'Tell me about your memory system']
                    .map(p => <button key={p} className="prompt-chip" onClick={() => void sendMessage(p)}>{p}</button>)}
                </div>
              </div>
            </div>
          ) : (
            messages.map(msg => (
              <React.Fragment key={msg.id}>
                <ChatMessage
                  id={msg.id} role={msg.role} content={msg.content} timestamp={msg.timestamp}
                  isStreaming={msg.id === streamingId} model={msg.model} audit={msg.audit} onQuote={handleQuote}
                />
                {msg.role === 'assistant' && msg.ari && <AriVerdictBadge ari={msg.ari} />}
              </React.Fragment>
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

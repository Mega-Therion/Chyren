'use client'

import React, { useCallback, useEffect, useRef, useState } from 'react'
import { useChat } from '@ai-sdk/react'
import { motion } from 'framer-motion'
import { AlertCircle } from 'lucide-react'

import { ChatMessage } from '@/components/ChatMessage'
import { ChatInput } from '@/components/ChatInput'
import { startHeartbeat, stopHeartbeat } from '@/lib/haptics-ry'
import { clearDraft } from '@/lib/draft-ry'
import { createTtsEngine, type TtsEngine, playLatencyChime } from '@/lib/tts-ry'
import { NeuralBrain, type BrainState } from '@/components/NeuralBrain'

export default function ChatPage() {
  const [sessionId] = useState(() => crypto.randomUUID().replace(/-/g, ''))
  const [audioLevel, setAudioLevel] = useState(0)
  const [quotedText, setQuotedText] = useState<string | undefined>()
  const [isListening, setIsListening] = useState(false)
  const [error, _setError] = useState<string | null>(null)

  const messagesEndRef = useRef<HTMLDivElement>(null)
  const chatWindowRef = useRef<HTMLDivElement>(null)
  const ttsRef = useRef<TtsEngine | null>(null)

  const {
    messages,
    input,
    handleInputChange,
    handleSubmit,
    isLoading,
    data,
    append,
    reload,
    stop,
  } = useChat({
    api: `/api/chat/stream?session=${sessionId}`,
    initialMessages: [],
    onFinish: (message) => {
      ttsRef.current?.finish()
      stopHeartbeat()
    },
    onError: (err) => {
      console.error('Neural connection disrupted:', err)
      stopHeartbeat()
    },
    onResponse: () => {
      if (ttsRef.current) {
        ttsRef.current.reset()
        playLatencyChime()
      }
      startHeartbeat()
    }
  })

  // Brain state derived from AI SDK status
  const brainState: BrainState = isLoading
    ? (messages[messages.length - 1]?.role === 'assistant' ? 'speaking' : 'thinking')
    : (isListening ? 'listening' : 'idle')

  useEffect(() => {
    ttsRef.current = createTtsEngine()
    return () => {
      ttsRef.current?.halt()
    }
  }, [])

  // Sync TTS with incoming message chunks
  useEffect(() => {
    const lastMsg = messages[messages.length - 1]
    if (isLoading && lastMsg?.role === 'assistant' && lastMsg.content) {
      // We need a way to feed only the NEW content to TTS
      // For now, TTS engine should handle deduplication or we feed it carefully
      // Actually, standard TTS engines usually have a way to feed deltas.
      // Since useChat updates the whole content, we need to track what we've already fed.
    }
  }, [messages, isLoading])

  const scrollToBottom = useCallback((behavior: ScrollBehavior = 'smooth') => {
    messagesEndRef.current?.scrollIntoView({ behavior, block: 'end' })
    if (chatWindowRef.current) {
      chatWindowRef.current.scrollTop = chatWindowRef.current.scrollHeight
    }
  }, [])

  useEffect(() => {
    scrollToBottom(isLoading ? 'auto' : 'smooth')
  }, [messages, isLoading, scrollToBottom])

  const handleSend = (text: string) => {
    ttsRef.current?.halt()
    clearDraft(sessionId)
    append({ role: 'user', content: text })
  }

  const getSigilColor = (s: BrainState) => {
    switch (s) {
      case 'speaking': return '#00f2ff'
      case 'thinking': return '#ff2d75'
      case 'listening': return '#bc13fe'
      default: return '#f59e0b'
    }
  }

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

        <section ref={chatWindowRef} className="chat-window">
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
                >Ω</motion.div>
                <div className="empty-state-title">Chyren Sovereign Intelligence</div>
                <div className="empty-state-subtitle">Awaiting operator input. Neural link established.</div>
              </div>
            </div>
          ) : (
            messages.map((m, i) => (
              <ChatMessage
                key={m.id}
                id={m.id}
                role={m.role as 'user' | 'assistant'}
                content={m.content}
                timestamp={m.createdAt ? new Date(m.createdAt) : new Date()}
                isStreaming={isLoading && i === messages.length - 1 && m.role === 'assistant'}
                onQuote={setQuotedText}
                // Extract audit data if sent via AI SDK data channel
                audit={data?.find((d: any) => d.messageId === m.id)?.audit}
              />
            ))
          )}
          <div ref={messagesEndRef} className="h-0.5" />
        </section>

        {error && (
          <div className="px-6 py-2 bg-rose-500/10 border-t border-rose-500/20 text-rose-400 text-xs flex items-center gap-2">
            <AlertCircle size={12} /> {error.message}
          </div>
        )}

        <footer className="input-dock !bg-transparent">
          <ChatInput
            onSend={handleSend}
            onBargeIn={() => ttsRef.current?.halt()}
            quotedText={quotedText}
            onQuoteConsumed={() => setQuotedText(undefined)}
            onAudioLevel={setAudioLevel}
            onRecordingState={setIsListening}
            isLoading={isLoading}
            sessionId={sessionId}
          />
        </footer>
      </main>
    </div>
  )
}

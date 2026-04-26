'use client'

import React, { useRef, useState, useCallback, useEffect } from 'react'
import { useVoiceEngine } from '@/hooks/useVoiceEngine'
import { haptic } from '@/lib/haptics-ry'
import { saveDraft, loadDraft, clearDraft } from '@/lib/draft-ry'

interface ChatInputProps {
  onSend: (message: string) => void
  onBargeIn?: () => void
  /** Injected by parent when swipe-to-quote fires */
  quotedText?: string
  onQuoteConsumed?: () => void
  onAudioLevel?: (level: number) => void
  onRecordingState?: (isRecording: boolean) => void
  disabled?: boolean
  isLoading?: boolean
  sessionId?: string
}

export function ChatInput({
  onSend,
  onBargeIn,
  quotedText,
  onQuoteConsumed,
  onAudioLevel,
  onRecordingState,
  disabled = false,
  isLoading = false,
  sessionId = 'global',
}: ChatInputProps) {
  const [input, setInput] = useState('')
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const hasLoadedDraftRef = useRef(false)

  // --- Draft persistence ---
  useEffect(() => {
    // Load draft once on mount
    if (!hasLoadedDraftRef.current) {
      hasLoadedDraftRef.current = true
      const saved = loadDraft(sessionId)
      if (saved) setInput(saved)
    }
  }, [sessionId])

  // Persist draft on every change (debounced inside saveDraft)
  useEffect(() => {
    saveDraft(sessionId, input)
  }, [input, sessionId])

  // --- Swipe-to-quote injection ---
  useEffect(() => {
    if (quotedText) {
      const prefix = `> ${quotedText}\n\n`
      setInput(prev => prefix + prev)
      onQuoteConsumed?.()
      requestAnimationFrame(() => textareaRef.current?.focus())
    }
  }, [quotedText, onQuoteConsumed])

  // --- Auto-resize textarea ---
  useEffect(() => {
    const ta = textareaRef.current
    if (!ta) return
    ta.style.height = '22px'
    ta.style.height = Math.min(ta.scrollHeight, 120) + 'px'
  }, [input])

  const handleTranscript = useCallback((text: string) => {
    setInput(prev => (prev ? `${prev} ${text}` : text))
    requestAnimationFrame(() => textareaRef.current?.focus())
  }, [])

  const { isRecording, audioLevel, startRecording, stopRecording } = useVoiceEngine({
    onTranscript: handleTranscript,
    onLevel: (lvl) => onAudioLevel?.(lvl),
    onBargeIn: () => onBargeIn?.(),
  })

  // Drive parent with audio level even when not sending
  useEffect(() => {
    onRecordingState?.(isRecording)
  }, [isRecording, onRecordingState])

  useEffect(() => {
    if (isRecording) onAudioLevel?.(audioLevel)
  }, [audioLevel, isRecording, onAudioLevel])

  const handleSend = useCallback(() => {
    const trimmed = input.trim()
    if (!trimmed || disabled || isLoading) return
    haptic('send')
    onSend(trimmed)
    setInput('')
    clearDraft(sessionId)
    if (textareaRef.current) {
      textareaRef.current.style.height = '22px'
      textareaRef.current.blur()       // Dismiss keyboard (Phase 1)
    }
  }, [input, disabled, isLoading, onSend, sessionId])

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    if ((e.nativeEvent as any)?.isComposing) return
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  return (
    <div className="relative group">
      <div className="input-shell backdrop-blur-3xl bg-white/[0.02] border border-white/[0.05] rounded-2xl px-6 py-4 flex items-end gap-4 shadow-2xl transition-all hover:bg-white/[0.04] hover:border-white/[0.1] focus-within:border-cyan-400/30 focus-within:bg-white/[0.05]">
        <textarea
          ref={textareaRef}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Message Chyren..."
          disabled={disabled || isLoading}
          className="flex-1 bg-transparent text-[16px] text-white placeholder-white/20 resize-none focus:outline-none disabled:opacity-40 leading-relaxed py-1"
          rows={1}
          style={{ minHeight: '26px' }}
        />

        <div className="flex items-center gap-3 pb-1">
          {/* Voice */}
          <button
            onClick={isRecording ? stopRecording : startRecording}
            disabled={disabled || isLoading}
            className="w-8 h-8 rounded-full flex items-center justify-center transition-all duration-300 disabled:opacity-20 hover:scale-110 active:scale-95"
            style={{
              background: isRecording ? 'rgba(248,113,113,0.1)' : 'transparent',
              color: isRecording ? '#f87171' : 'rgba(255,255,255,0.3)',
              boxShadow: isRecording
                ? `0 0 ${10 + audioLevel * 20}px rgba(248,113,113,0.3)`
                : 'none',
            }}
          >
            <svg width="16" height="16" fill="none" stroke="currentColor" strokeWidth="2" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" d="M12 18.75a6 6 0 006-6v-1.5m-6 7.5a6 6 0 01-6-6v-1.5m6 7.5v3.75m-3.75 0h7.5M12 15.75a3 3 0 01-3-3V4.5a3 3 0 116 0v8.25a3 3 0 01-3 3z" />
            </svg>
          </button>

          {/* Send */}
          <button
            onClick={handleSend}
            disabled={!input.trim() || disabled || isLoading}
            className="w-8 h-8 rounded-full flex items-center justify-center transition-all duration-300 disabled:opacity-10 bg-white/5 hover:bg-white/10 hover:scale-110 active:scale-95 text-white"
          >
            {isLoading ? (
              <svg width="14" height="14" className="animate-spin" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="3"/>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
              </svg>
            ) : (
              <svg width="14" height="14" fill="none" stroke="currentColor" strokeWidth="2.5" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 10.5L12 3m0 0l7.5 7.5M12 3v18" />
              </svg>
            )}
          </button>
        </div>
      </div>
      
      {isRecording && (
        <motion.div 
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="absolute -top-8 left-1/2 -translate-x-1/2 text-[10px] tracking-[0.3em] font-medium text-red-400/60 uppercase"
        >
          Listening
        </motion.div>
      )}
    </div>
  )
}

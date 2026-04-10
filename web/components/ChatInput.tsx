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
    <div className="input-area rounded-lg px-4 py-3 flex items-end gap-3">
      {/* Prompt glyph */}
      <span
        className="font-mono text-sm flex-shrink-0 pb-0.5 select-none"
        style={{ color: '#6366f1' }}
      >
        ▸
      </span>

      <textarea
        ref={textareaRef}
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Enter command or query…"
        disabled={disabled || isLoading}
        className="flex-1 bg-transparent font-mono text-sm text-slate-200 placeholder-slate-700 resize-none focus:outline-none disabled:opacity-40 leading-relaxed"
        rows={1}
        style={{ minHeight: '22px' }}
      />

      {/* Voice — with live level ring */}
      <button
        onClick={isRecording ? stopRecording : startRecording}
        disabled={disabled || isLoading}
        className="flex-shrink-0 w-7 h-7 rounded flex items-center justify-center transition-all duration-150 disabled:opacity-30 relative"
        style={{
          background: isRecording ? 'rgba(248,113,113,0.15)' : 'rgba(99,102,241,0.1)',
          border: `1px solid ${isRecording ? 'rgba(248,113,113,0.4)' : 'rgba(99,102,241,0.2)'}`,
          color: isRecording ? '#f87171' : '#818cf8',
          boxShadow: isRecording
            ? `0 0 ${6 + audioLevel * 18}px rgba(248,113,113,${0.2 + audioLevel * 0.6})`
            : 'none',
        }}
        title={isRecording ? 'Stop recording' : 'Voice input'}
      >
        {isRecording ? (
          <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
            <rect x="5" y="5" width="3.5" height="10" rx="1"/>
            <rect x="11.5" y="5" width="3.5" height="10" rx="1"/>
          </svg>
        ) : (
          <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 2a3 3 0 0 0-3 3v5a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3zM7 14.5A5 5 0 0 0 15 10h-1.5a3.5 3.5 0 0 1-7 0H5a5 5 0 0 0 5 4.5V16H8v1.5h4V16h-2v-1.5z"/>
          </svg>
        )}
      </button>

      {/* Send */}
      <button
        onClick={handleSend}
        disabled={!input.trim() || disabled || isLoading}
        className="flex-shrink-0 w-7 h-7 rounded flex items-center justify-center transition-all duration-150 disabled:opacity-20"
        style={{
          background: 'rgba(99,102,241,0.15)',
          border: '1px solid rgba(99,102,241,0.35)',
          color: '#818cf8',
        }}
        title="Send (Enter)"
      >
        {isLoading ? (
          <svg className="w-3 h-3 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="3"/>
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
        ) : (
          <svg className="w-3 h-3" fill="none" stroke="currentColor" strokeWidth="2" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14M12 5l7 7-7 7"/>
          </svg>
        )}
      </button>
    </div>
  )
}

'use client'

import React, { useRef, useState, useCallback, useEffect } from 'react'
import { useVoiceEngine } from '@/hooks/useVoiceEngine'

interface ChatInputProps {
  onSend: (message: string) => void
  disabled?: boolean
  isLoading?: boolean
}

export function ChatInput({ onSend, disabled = false, isLoading = false }: ChatInputProps) {
  const [input, setInput] = useState('')
  const textareaRef = useRef<HTMLTextAreaElement>(null)

  const handleTranscript = useCallback((text: string) => {
    setInput((prev) => (prev ? prev + ' ' + text : text))
  }, [])

  const { isRecording, startRecording, stopRecording } = useVoiceEngine(handleTranscript)

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = '44px'
      textareaRef.current.style.height = Math.min(textareaRef.current.scrollHeight, 140) + 'px'
    }
  }, [input])

  const handleSend = useCallback(() => {
    if (input.trim() && !disabled && !isLoading) {
      onSend(input.trim())
      setInput('')
      if (textareaRef.current) {
        textareaRef.current.style.height = '44px'
      }
    }
  }, [input, disabled, isLoading, onSend])

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  return (
    <div className="glass-darker rounded-3xl p-3 flex items-end gap-2 transition-all duration-200 focus-within:border-cyan-400/50">
      <textarea
        ref={textareaRef}
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder="Ask Chyren anything..."
        disabled={disabled || isLoading}
        className="flex-1 bg-transparent text-white placeholder-slate-500 text-sm resize-none focus:outline-none disabled:opacity-50"
        rows={1}
      />

      <button
        onClick={isRecording ? stopRecording : startRecording}
        disabled={disabled || isLoading}
        className={`flex-shrink-0 w-9 h-9 rounded-full flex items-center justify-center transition-all duration-200 ${
          isRecording
            ? 'bg-red-500/80 text-white animate-pulse-subtle'
            : 'bg-slate-700 hover:bg-slate-600 text-cyan-400'
        } disabled:opacity-50 disabled:cursor-not-allowed`}
        title={isRecording ? 'Stop recording' : 'Start recording'}
      >
        {isRecording ? (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <rect x="6" y="5" width="3" height="10" />
            <rect x="11" y="5" width="3" height="10" />
          </svg>
        ) : (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 2a4 4 0 0 0-4 4v4a4 4 0 0 0 8 0V6a4 4 0 0 0-4-4zM7 10a3 3 0 0 1 6 0v3a3 3 0 1 1-6 0v-3z" />
          </svg>
        )}
      </button>

      <button
        onClick={handleSend}
        disabled={!input.trim() || disabled || isLoading}
        className="flex-shrink-0 w-9 h-9 rounded-full bg-gradient-accent hover:opacity-90 text-white flex items-center justify-center transition-all duration-200 disabled:opacity-30 disabled:cursor-not-allowed hover:scale-105"
        title="Send message (Enter)"
      >
        {isLoading ? (
          <svg className="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
          </svg>
        ) : (
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5.951-1.488 5.951 1.488a1 1 0 001.169-1.409l-7-14z" />
          </svg>
        )}
      </button>
    </div>
  )
}

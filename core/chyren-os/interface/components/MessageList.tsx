'use client'

import React, { useEffect, useRef } from 'react'
import { ChatMessage } from './ChatMessage'

export interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: Date
  isStreaming?: boolean
}

interface MessageListProps {
  messages: Message[]
  isLoading?: boolean
  onQuote?: (content: string) => void
}

export function MessageList({ messages, isLoading = false, onQuote }: MessageListProps) {
  const endRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  if (messages.length === 0) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center gap-6 px-8 grid-bg">
        <div className="text-center">
          <div
            className="font-mono text-7xl font-light mb-4 chyren-glow select-none"
            style={{ color: '#6366f1' }}
          >
            Ω
          </div>
          <div className="font-mono text-xs tracking-[0.3em] text-slate-500 uppercase mb-2">
            Sovereign Intelligence Orchestrator
          </div>
          <div className="font-mono text-xs text-slate-700 mb-8">v2.0 // ADCCL ACTIVE</div>
          <div className="grid grid-cols-3 gap-3 max-w-sm mx-auto">
            {['Task Routing', 'ADCCL Verify', 'Ledger Commit'].map((label) => (
              <div
                key={label}
                className="terminal-panel rounded px-3 py-2 text-center"
              >
                <div
                  className="w-1.5 h-1.5 rounded-full bg-emerald-500 mx-auto mb-1.5"
                  style={{ boxShadow: '0 0 6px #10b981' }}
                />
                <div className="font-mono text-xs text-slate-500">{label}</div>
              </div>
            ))}
          </div>
        </div>
        <div className="font-mono text-xs text-slate-700 text-center">
          Type a command or ask a question to begin
        </div>
      </div>
    )
  }

  return (
    <div className="flex-1 overflow-y-auto flex flex-col divide-y divide-slate-800/50">
      {messages.map((msg) => (
        <ChatMessage
          key={msg.id}
          id={msg.id}
          role={msg.role}
          content={msg.content}
          timestamp={msg.timestamp}
          isStreaming={msg.isStreaming}
          onQuote={onQuote}
        />
      ))}
      {isLoading && messages[messages.length - 1]?.role !== 'assistant' && (
        <div className="px-5 py-3 msg-assistant fade-up">
          <div className="flex items-center gap-3 mb-2">
            <span className="font-mono text-xs font-medium tracking-wider" style={{ color: '#10b981' }}>
              Ω CHYREN
            </span>
            <span className="badge badge-amber">PROCESSING</span>
          </div>
          <div className="pl-3 flex items-center gap-1.5">
            {[0, 1, 2].map((i) => (
              <span
                key={i}
                className="pulse-dot w-1 h-1 rounded-full"
                style={{ background: '#10b981', animationDelay: `${i * 0.2}s` }}
              />
            ))}
          </div>
        </div>
      )}
      <div ref={endRef} />
    </div>
  )
}

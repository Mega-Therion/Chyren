'use client'

import React from 'react'

interface ChatMessageProps {
  role: 'user' | 'assistant'
  content: string
  timestamp?: Date
  isStreaming?: boolean
  index?: number
}

export function ChatMessage({ role, content, timestamp, isStreaming = false, index = 0 }: ChatMessageProps) {
  const isUser = role === 'user'
  const time = timestamp
    ? timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit', hour12: false })
    : ''

  return (
    <div className={`fade-up px-5 py-3 ${isUser ? 'msg-user' : 'msg-assistant'}`}>
      {/* Header row */}
      <div className="flex items-center gap-3 mb-1.5">
        <span
          className="font-mono text-xs font-medium tracking-wider"
          style={{ color: isUser ? '#818cf8' : '#10b981' }}
        >
          {isUser ? '▸ USER' : 'Ω CHYREN'}
        </span>
        {time && (
          <span className="font-mono text-xs text-slate-600">{time}</span>
        )}
        {isStreaming && (
          <span className="badge badge-amber">STREAMING</span>
        )}
      </div>

      {/* Content */}
      <div
        className="msg-content text-sm leading-relaxed text-slate-200 pl-3"
        style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}
      >
        {content}
        {isStreaming && (
          <span
            className="cursor inline-block ml-0.5 w-1.5 h-3.5 align-middle"
            style={{ background: '#10b981' }}
          />
        )}
      </div>
    </div>
  )
}

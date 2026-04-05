'use client'

import React from 'react'

interface ChatMessageProps {
  role: 'user' | 'assistant'
  content: string
  timestamp?: Date
  isStreaming?: boolean
}

export function ChatMessage({
  role,
  content,
  timestamp,
  isStreaming = false,
}: ChatMessageProps) {

  const isUser = role === 'user'

  return (
    <div
      className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-4 animate-slide-in`}
    >
      <div
        className={`flex items-end gap-2 max-w-xs sm:max-w-md lg:max-w-xl ${
          isUser ? 'flex-row-reverse' : 'flex-row'
        }`}
      >
        {/* Avatar */}
        <div
          className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold ${
            isUser
              ? 'bg-gradient-accent'
              : 'glass border border-cyan-500/30'
          }`}
        >
          {isUser ? '→' : 'Ω'}
        </div>

        {/* Message bubble */}
        <div
          className={`px-4 py-3 rounded-2xl animate-fade-in ${
            isUser
              ? 'bg-gradient-to-r from-cyan-600 to-teal-600 text-white rounded-br-none'
              : 'glass-darker text-slate-100 rounded-bl-none'
          }`}
        >
          <div
            className={`message-content text-sm leading-relaxed ${
              isStreaming ? 'font-medium' : ''
            }`}
          >
            {content}
            {isStreaming && (
              <span className="inline-flex ml-1 items-center gap-1">
                <span className="typing-indicator">
                  <span className="typing-dot w-1.5 h-1.5" />
                </span>
              </span>
            )}
          </div>

          {/* Timestamp */}
          {timestamp && (
            <div
              className={`text-xs mt-1.5 ${
                isUser ? 'text-blue-100' : 'text-slate-500'
              }`}
            >
              {timestamp.toLocaleTimeString([], {
                hour: '2-digit',
                minute: '2-digit',
              })}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

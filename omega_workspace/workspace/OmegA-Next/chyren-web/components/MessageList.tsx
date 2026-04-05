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
}

export function MessageList({ messages, isLoading = false }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' })
  }, [messages])

  return (
    <div className="flex flex-col flex-1 overflow-y-auto px-4 py-6 space-y-1">
      {messages.length === 0 ? (
        <div className="flex-1 flex items-center justify-center">
          <div className="text-center">
            <div className="text-4xl mb-4 opacity-50">Ω</div>
            <h2 className="text-2xl font-bold gradient-text mb-2">Chyren</h2>
            <p className="text-slate-400 text-sm max-w-xs mx-auto leading-relaxed">
              Sovereign Intelligence Orchestrator. Ask me anything and I'll route your task through
              multiple AI providers with integrity verification.
            </p>
          </div>
        </div>
      ) : (
        <>
          {messages.map((message) => (
            <ChatMessage
              key={message.id}
              role={message.role}
              content={message.content}
              timestamp={message.timestamp}
              isStreaming={message.isStreaming}
            />
          ))}
          {isLoading && (
            <div className="flex justify-start mb-4">
              <div className="flex items-end gap-2">
                <div className="flex-shrink-0 w-8 h-8 rounded-full glass border border-cyan-500/30 flex items-center justify-center text-xs font-bold">
                  Ω
                </div>
                <div className="glass-darker rounded-2xl rounded-bl-none px-4 py-3 animate-fade-in">
                  <div className="typing-indicator">
                    <span className="typing-dot w-2 h-2 bg-cyan-400" />
                    <span className="typing-dot w-2 h-2 bg-cyan-400" />
                    <span className="typing-dot w-2 h-2 bg-cyan-400" />
                  </div>
                </div>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </>
      )}
    </div>
  )
}

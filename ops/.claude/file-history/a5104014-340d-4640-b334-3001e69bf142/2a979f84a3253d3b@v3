'use client'

import React, { useState, useCallback, lazy, Suspense } from 'react'
import { ChatInput } from '@/components/ChatInput'
import { MessageList, type Message } from '@/components/MessageList'
import { useBrainState } from '@/hooks/useBrainState'

const ChyrenBrain = lazy(() => import('@/components/ChyrenBrain'))

const STAGE_LABELS: Record<string, string> = {
  identity_load:   'Loading identity…',
  alignment_check: 'Alignment check…',
  threat_scan:     'Scanning threats…',
  provider_call:   'Provider executing…',
  adccl_verify:    'ADCCL verifying…',
  ledger_commit:   'Committing to ledger…',
  idle:            'Sovereign Intelligence',
}

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const { brainState, activeStage, activate } = useBrainState()

  const addMessage = useCallback(
    (role: 'user' | 'assistant', content: string, isStreaming = false) => {
      const message: Message = {
        id: Date.now().toString(),
        role,
        content,
        timestamp: new Date(),
        isStreaming,
      }
      setMessages((prev) => [...prev, message])
      return message.id
    },
    []
  )

  const updateMessage = useCallback((id: string, content: string, isStreaming = false) => {
    setMessages((prev) =>
      prev.map((msg) =>
        msg.id === id ? { ...msg, content, isStreaming } : msg
      )
    )
  }, [])

  const handleSend = useCallback(
    async (userMessage: string) => {
      if (!userMessage.trim()) return

      addMessage('user', userMessage)
      setIsLoading(true)
      activate() // fire up the brain visualization

      try {
        const assistantId = addMessage('assistant', '', true)

        const response = await fetch('/api/chat/stream', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            message: userMessage,
            context: messages.map((m) => ({ role: m.role, content: m.content })),
          }),
        })

        if (!response.ok) throw new Error('Failed to fetch response')

        const reader = response.body?.getReader()
        const decoder = new TextDecoder()
        let fullContent = ''

        if (reader) {
          while (true) {
            const { done, value } = await reader.read()
            if (done) break
            fullContent += decoder.decode(value)
            updateMessage(assistantId, fullContent, true)
          }
        }

        updateMessage(assistantId, fullContent, false)
      } catch (error) {
        console.error('Chat error:', error)
        addMessage('assistant', 'Sorry, I encountered an error processing your request. Please try again.')
      } finally {
        setIsLoading(false)
      }
    },
    [messages, addMessage, updateMessage, activate]
  )

  return (
    <div className="h-screen bg-gradient-to-br from-slate-950 via-blue-950 to-slate-950 flex flex-col overflow-hidden">
      {/* Header */}
      <header className="border-b border-slate-700/30 backdrop-blur supports-[backdrop-filter]:bg-slate-900/20 px-6 py-4 flex-shrink-0">
        <div className="max-w-7xl mx-auto flex items-center gap-3">
          <div className="text-2xl font-bold gradient-text">Ω CHYREN</div>
          <span className="text-xs px-2 py-1 rounded-full bg-cyan-500/10 text-cyan-400 border border-cyan-500/30">
            {STAGE_LABELS[activeStage] ?? 'Sovereign Intelligence'}
          </span>
        </div>
      </header>

      {/* Body: brain + chat side by side */}
      <div className="flex flex-1 overflow-hidden max-w-7xl w-full mx-auto gap-0">

        {/* Brain panel */}
        <div className="hidden lg:flex flex-col w-80 flex-shrink-0 border-r border-slate-700/30 p-3">
          <div className="text-xs text-slate-500 uppercase tracking-widest mb-2 text-center">
            Cognitive Activity
          </div>
          <div className="flex-1 relative">
            <Suspense fallback={
              <div className="w-full h-full flex items-center justify-center text-slate-600 text-xs">
                Loading brain…
              </div>
            }>
              <ChyrenBrain state={brainState} />
            </Suspense>
          </div>
          <div className="text-center mt-2">
            <span className="text-xs text-slate-600 italic">
              {STAGE_LABELS[activeStage] ?? '—'}
            </span>
          </div>
        </div>

        {/* Chat panel */}
        <div className="flex flex-col flex-1 overflow-hidden">
          <MessageList messages={messages} isLoading={isLoading} />

          <div className="border-t border-slate-700/30 backdrop-blur supports-[backdrop-filter]:bg-slate-900/20 px-6 py-4 flex-shrink-0">
            <ChatInput onSend={handleSend} disabled={isLoading} isLoading={isLoading} />
            <p className="text-xs text-slate-500 text-center mt-2">
              Press Enter to send · Shift+Enter for new line · Tasks routed through verified AI providers
            </p>
          </div>
        </div>

      </div>
    </div>
  )
}

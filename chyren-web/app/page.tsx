'use client';

import React, { useCallback, useEffect, useRef, useState } from 'react';
import dynamic from 'next/dynamic';
import { AlertCircle, Loader2, Send, Volume2, VolumeX } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

const NeuralBrain = dynamic(() => import('@/components/NeuralBrain'), { ssr: false });

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  audit?: {
    passed: boolean;
    score: number;
    flags: string[];
  };
}

function parseHubChunk(chunk: string): {
  content: string;
  audit?: { passed: boolean; score: number; flags: string[] };
} {
  let content = '';
  let audit: { passed: boolean; score: number; flags: string[] } | undefined;

  for (const line of chunk.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed.startsWith('data: ')) continue;
    try {
      const json = JSON.parse(trimmed.slice(6));
      const delta = json.choices?.[0]?.delta?.content;
      if (delta) content += delta;
      if (json.status === 'audited') audit = json.audit_report;
    } catch {
      // Ignore malformed SSE frames
    }
  }
  return { content, audit };
}

function consumeSseBuffer(buffer: string): {
  events: Array<{ content: string; audit?: { passed: boolean; score: number; flags: string[] } }>;
  remaining: string;
} {
  const records = buffer.split('\n\n');
  const remaining = records.pop() ?? '';
  const events = records.map(parseHubChunk).filter(e => e.content || e.audit);
  return { events, remaining };
}

function getPremiumVoice(): SpeechSynthesisVoice | undefined {
  if (typeof window === 'undefined') return undefined;
  const voices = window.speechSynthesis.getVoices();
  const targets = [
    (v: SpeechSynthesisVoice) => v.name.includes('Google UK English Male'),
    (v: SpeechSynthesisVoice) => v.name.includes('Arthur'),
    (v: SpeechSynthesisVoice) => v.name.includes('Daniel'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en-GB') && v.name.includes('Male'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en-GB'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en-AU'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en-US'),
  ];

  for (const target of targets) {
    const match = voices.find(target);
    if (match) return match;
  }
  return voices[0];
}

function speak(text: string) {
  if (!text.trim() || typeof window === 'undefined') return;
  const utter = new SpeechSynthesisUtterance(text.trim());
  utter.rate = 0.92;
  utter.pitch = 0.85;
  utter.volume = 1.0;
  const v = getPremiumVoice();
  if (v) utter.voice = v;
  window.speechSynthesis.speak(utter);
}

const EMPTY_PROMPTS = [
  'Summarize my current project status and next actions.',
  'Draft a deployment checklist for chyren-web on Vercel.',
  'Find UI/UX issues in this chat UI and propose fixes.',
  'Help me write a clear GitHub issue for the biggest bug here.',
];

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [streamingAssistantId, setStreamingAssistantId] = useState<string | null>(null);
  const [ttsEnabled, setTtsEnabled] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [sessionId, setSessionId] = useState<string>('global');

  useEffect(() => {
    setSessionId(crypto.randomUUID().replace(/-/g, ''));
    const loadVoices = () => window.speechSynthesis?.getVoices();
    loadVoices();
    if (typeof window !== 'undefined' && window.speechSynthesis) {
      window.speechSynthesis.onvoiceschanged = loadVoices;
    }
  }, []);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  useEffect(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    ta.style.height = 'auto';
    ta.style.height = `${Math.min(ta.scrollHeight, 150)}px`;
  }, [input]);

  const primePrompt = useCallback((prompt: string) => {
    setInput(prompt);
    requestAnimationFrame(() => {
      const ta = textareaRef.current;
      if (!ta) return;
      ta.focus();
      ta.setSelectionRange(prompt.length, prompt.length);
    });
  }, []);

  const sendMessage = useCallback(async (text: string) => {
    const trimmed = text.trim();
    if (!trimmed || isStreaming) return;

    window.speechSynthesis?.cancel();
    setError(null);
    setInput('');

    const userMsg: Message = { id: `u-${Date.now()}`, role: 'user', content: trimmed };
    setMessages(prev => [...prev, userMsg]);
    setIsStreaming(true);

    const assistantId = `a-${Date.now()}`;
    setStreamingAssistantId(assistantId);
    setMessages(prev => [...prev, { id: assistantId, role: 'assistant', content: '' }]);

    try {
      const res = await fetch(`/api/chat/stream?session=${sessionId}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: trimmed }),
      });

      if (!res.ok) throw new Error('Neural link failure');

      const reader = res.body?.getReader();
      const decoder = new TextDecoder();
      let accumulated = '';
      let sseBuffer = '';

      if (reader) {
        while (true) {
          const { done, value } = await reader.read();
          sseBuffer += decoder.decode(value ?? new Uint8Array(), { stream: !done });
          const { events, remaining } = consumeSseBuffer(sseBuffer);
          sseBuffer = remaining;

          for (const { content: delta, audit } of events) {
            if (delta) {
              accumulated += delta;
              setMessages(prev =>
                prev.map(m => (m.id === assistantId ? { ...m, content: accumulated } : m)),
              );
            }
            if (audit) {
              setMessages(prev => prev.map(m => (m.id === assistantId ? { ...m, audit } : m)));
            }
          }
          if (done) break;
        }
        if (ttsEnabled) speak(accumulated);
      }
    } catch {
      setError('Neural connection disrupted');
      setMessages(prev => prev.filter(m => m.id !== assistantId));
    } finally {
      setIsStreaming(false);
      setStreamingAssistantId(null);
    }
  }, [isStreaming, sessionId, ttsEnabled]);

  return (
    <div className="omega-viewport">
      <div className="omega-bg-fx">
        <NeuralBrain isActive={isStreaming} />
      </div>

      <div className="omega-orb orb-1" />
      <div className="omega-orb orb-2" />

      <main className="phone-container">
        <div className="phone-notch" />

        <header className="phone-chrome">
          <h1 className="phone-title">CHYREN</h1>
        </header>

        <section className="chat-window" aria-label="Chat transcript">
          {messages.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-inner">
                <div className="empty-state-sigil">Ω</div>
                <div className="empty-state-title">Awaiting Intent</div>
                <p className="empty-state-subtitle">
                  Start with a concrete task. You can paste an error log, request a change, or pick one of these.
                </p>
                <div className="empty-prompts" role="list">
                  {EMPTY_PROMPTS.map(p => (
                    <button
                      key={p}
                      type="button"
                      className="prompt-chip"
                      onClick={() => primePrompt(p)}
                      role="listitem"
                    >
                      {p}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          ) : (
            messages.map((msg) => (
              <div
                key={msg.id}
                className={`msg-group ${msg.role === 'user' ? 'msg-user' : 'msg-assistant'}`}
              >
                <div className="msg-header">
                  <span className="msg-header-name">{msg.role === 'user' ? 'Operator' : 'Chyren'}</span>
                  {msg.role === 'assistant' && msg.id === streamingAssistantId && (
                    <span className="chat-live">● LIVE</span>
                  )}
                </div>
                <div className="msg-bubble">
                  {msg.role === 'assistant' ? (
                    <ReactMarkdown remarkPlugins={[remarkGfm]}>{msg.content}</ReactMarkdown>
                  ) : (
                    <p>{msg.content}</p>
                  )}
                  {msg.audit && (
                    <div className={`chat-chip ${msg.audit.passed ? 'chip-pass' : 'chip-warn'}`}>
                      {msg.audit.passed ? '✓ Integrity Verified' : '⚠ Drift Detected'}
                    </div>
                  )}
                </div>
              </div>
            ))
          )}
          <div ref={messagesEndRef} />
        </section>

        {error && (
          <div className="px-6 py-2 bg-rose-500/10 border-t border-rose-500/20 text-rose-400 text-xs flex items-center gap-2">
            <AlertCircle size={12} /> {error}
          </div>
        )}

        <footer className="input-dock">
          <div className="input-shell">
            <button
              type="button"
              className="p-2 opacity-40 hover:opacity-100 transition-opacity"
              aria-label={ttsEnabled ? 'Disable text to speech' : 'Enable text to speech'}
              aria-pressed={ttsEnabled}
              onClick={() => setTtsEnabled(!ttsEnabled)}
            >
              {ttsEnabled ? <Volume2 size={18} className="text-cyan-400" /> : <VolumeX size={18} />}
            </button>

            <textarea
              ref={textareaRef}
              className="msg-input"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                // Avoid sending while IME composing
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                if ((e.nativeEvent as any)?.isComposing) return;
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  void sendMessage(input);
                }
              }}
              placeholder="Submit task"
              aria-label="Message"
              rows={1}
            />

            <button
              type="button"
              className="btn-send"
              aria-label={isStreaming ? 'Sending' : 'Send message'}
              onClick={() => void sendMessage(input)}
              disabled={isStreaming || !input.trim()}
            >
              {isStreaming ? <Loader2 size={18} className="animate-spin" /> : <Send size={18} />}
            </button>
          </div>
        </footer>
      </main>
    </div>
  );
}

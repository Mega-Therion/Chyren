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
  isCorrection?: boolean;
  consensus?: {
    agreement: number;
  };
}

function parseHubChunk(chunk: string): {
  content: string;
  audit?: { passed: boolean; score: number; flags: string[] };
  status?: string;
  consensus?: { agreement: number };
} {
  let content = '';
  let audit: { passed: boolean; score: number; flags: string[] } | undefined;
  let status: string | undefined;

  for (const line of chunk.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    if (trimmed.startsWith('data: ')) {
      try {
        const json = JSON.parse(trimmed.slice(6));
        const delta = json.choices?.[0]?.delta?.content;
        if (delta) content += delta;
        if (json.status === 'audited') audit = json.audit_report;
        if (json.status === 'correction') {
          content += json.content;
          status = 'correction';
        }
        if (json.status === 'deflected') {
          content = json.content;
          status = 'deflected';
        }
        if (json.status === 'consensus') {
          content = json.content;
          status = 'consensus';
        }
      } catch {}
    } else if (trimmed.startsWith('0:')) {
      try {
        const delta = JSON.parse(trimmed.slice(2));
        if (typeof delta === 'string') content += delta;
      } catch {}
    }
  }

  let consensus: { agreement: number } | undefined;
  if (chunk.includes('"status":"consensus"')) {
    try {
      const json = JSON.parse(chunk.split('\n').find((l) => l.includes('"status":"consensus"'))?.slice(6) || '{}');
      consensus = { agreement: json.agreement };
    } catch {}
  }

  return { content, audit, status, consensus };
}

function consumeSseBuffer(buffer: string): {
  events: Array<ReturnType<typeof parseHubChunk>>;
  remaining: string;
} {
  const records = buffer.split('\n\n');
  const remaining = records.pop() ?? '';
  const events = records
    .map((record) => parseHubChunk(record))
    .filter((event) => event.content || event.audit || event.status || event.consensus);

  return { events, remaining };
}

function extractSpeakable(buffer: string): { chunk: string | null; remaining: string } {
  const m = buffer.match(/^([\s\S]+?[.!?\n]+)\s*([\s\S]*)$/);
  if (m) return { chunk: m[1].trim(), remaining: m[2] };
  if (buffer.length > 80) {
    const cut = buffer.lastIndexOf(' ', 80);
    if (cut > 20) return { chunk: buffer.slice(0, cut).trim(), remaining: buffer.slice(cut + 1) };
  }
  return { chunk: null, remaining: buffer };
}

function getVoice(): SpeechSynthesisVoice | undefined {
  const voices = window.speechSynthesis.getVoices();
  return (
    voices.find((v) => v.lang.startsWith('en-GB') && (v.name.includes('Male') || v.name.includes('Daniel') || v.name.includes('Arthur'))) ||
    voices.find((v) => v.lang.startsWith('en-GB')) ||
    voices.find((v) => v.lang.includes('en-GB')) ||
    voices.find((v) => v.name.toLowerCase().includes('google') && v.lang.startsWith('en')) ||
    voices.find((v) => v.lang.startsWith('en-US')) ||
    voices[0] ||
    undefined
  );
}

function speak(text: string) {
  if (!text.trim() || typeof window === 'undefined') return;
  const utter = new SpeechSynthesisUtterance(text.trim());
  utter.rate = 0.95;
  utter.pitch = 0.9;
  utter.volume = 1.0;
  const v = getVoice();
  if (v) utter.voice = v;
  window.speechSynthesis.speak(utter);
}

export default function ChatPage() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [ttsEnabled, setTtsEnabled] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [sessionId, setSessionId] = useState<string>('global');

  useEffect(() => {
    const nextSession =
      typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
        ? crypto.randomUUID().replace(/-/g, '')
        : `session${Date.now()}${Math.random().toString(36).slice(2, 10)}`;
    localStorage.setItem('chyren_session_id', nextSession);
    localStorage.removeItem('chyren_messages');
    setSessionId(nextSession);
    setMessages([]);
  }, []);

  useEffect(() => {
    if (messages.length > 0) localStorage.setItem('chyren_messages', JSON.stringify(messages));
  }, [messages]);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const abortRef = useRef<AbortController | null>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const ttsEnabledRef = useRef(ttsEnabled);

  useEffect(() => {
    ttsEnabledRef.current = ttsEnabled;
  }, [ttsEnabled]);

  useEffect(() => {
    const load = () => window.speechSynthesis?.getVoices();
    load();
    window.speechSynthesis?.addEventListener('voiceschanged', load);
    return () => {
      window.speechSynthesis?.removeEventListener('voiceschanged', load);
      window.speechSynthesis?.cancel();
    };
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  useEffect(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    ta.style.height = 'auto';
    ta.style.height = `${Math.min(ta.scrollHeight, 140)}px`;
  }, [input]);

  const sendMessage = useCallback(
    async (text: string) => {
      const trimmed = text.trim();
      if (!trimmed || isStreaming) return;

      window.speechSynthesis?.cancel();
      setError(null);
      setInput('');

      const userMsg: Message = { id: `u-${Date.now()}`, role: 'user', content: trimmed };
      const history = [...messages, userMsg];
      setMessages(history);
      setIsStreaming(true);

      const assistantId = `a-${Date.now()}`;
      setMessages((prev) => [...prev, { id: assistantId, role: 'assistant', content: '' }]);

      try {
        abortRef.current?.abort();
        const abort = new AbortController();
        abortRef.current = abort;

        const res = await fetch(`/api/chat/stream?session=${encodeURIComponent(sessionId)}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ messages: history.map((m) => ({ role: m.role, content: m.content })) }),
          signal: abort.signal,
        });

        if (!res.ok) {
          const body = await res.text().catch(() => '');
          throw new Error(`HTTP ${res.status}: ${body.slice(0, 120)}`);
        }

        const reader = res.body?.getReader();
        const decoder = new TextDecoder();
        let accumulated = '';
        let speechBuffer = '';
        let sseBuffer = '';

        if (reader) {
          while (true) {
            const { done, value } = await reader.read();
            sseBuffer += decoder.decode(value ?? new Uint8Array(), { stream: !done });

            const { events, remaining } = consumeSseBuffer(sseBuffer);
            sseBuffer = remaining;

            for (const { content: delta, audit, status, consensus } of events) {
              if (delta) {
                accumulated += delta;
                setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, content: accumulated } : m)));
              }

              if (audit) {
                setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, audit } : m)));
              }

              if (consensus) {
                setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, consensus } : m)));
              }

              if (status === 'correction') {
                setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, isCorrection: true } : m)));
              }

              if (ttsEnabledRef.current && delta) {
                speechBuffer += delta;
                let result = extractSpeakable(speechBuffer);
                while (result.chunk) {
                  speak(result.chunk);
                  speechBuffer = result.remaining;
                  result = extractSpeakable(speechBuffer);
                }
              }
            }

            if (done) break;
          }

          if (sseBuffer.trim()) {
            const { content: delta, audit, status, consensus } = parseHubChunk(sseBuffer);
            if (delta) {
              accumulated += delta;
              setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, content: accumulated } : m)));
            }
            if (audit) {
              setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, audit } : m)));
            }
            if (consensus) {
              setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, consensus } : m)));
            }
            if (status === 'correction') {
              setMessages((prev) => prev.map((m) => (m.id === assistantId ? { ...m, isCorrection: true } : m)));
            }
            if (ttsEnabledRef.current && delta) {
              speechBuffer += delta;
            }
          }
        }

        if (ttsEnabledRef.current && speechBuffer.trim()) speak(speechBuffer.trim());

        if (!accumulated) {
          setMessages((prev) => prev.filter((m) => m.id !== assistantId));
          setError('Neural link disrupted — no response received. Please wait a moment and retry.');
        }
      } catch (err) {
        if ((err as Error).name !== 'AbortError') {
          window.speechSynthesis?.cancel();
          const msg = (err as Error).message;
          setError(msg.includes('quota') ? 'AI quota exhausted — check provider config' : `Neural link error: ${msg}`);
          setMessages((prev) => prev.filter((m) => m.id !== assistantId));
        }
      } finally {
        setIsStreaming(false);
        abortRef.current = null;
      }
    },
    [messages, isStreaming, sessionId]
  );

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      void sendMessage(input);
    }
  };

  const toggleTTS = () => {
    if (ttsEnabled) window.speechSynthesis?.cancel();
    setTtsEnabled((s) => !s);
  };

  const isEmpty = messages.length === 0;
  const lastMsg = messages[messages.length - 1];

  return (
    <div className="omega-root chyren-mobile-shell">
      <div className="omega-bg">
        <NeuralBrain isActive={isStreaming} />
      </div>

      <div className="omega-vignette" />
      <div className="omega-orb omega-orb-left" />
      <div className="omega-orb omega-orb-right" />
      <div className="omega-spark omega-spark-one" />
      <div className="omega-spark omega-spark-two" />
      <div className="omega-circuit omega-circuit-top" />
      <div className="omega-circuit omega-circuit-bottom" />

      <main className="phone-stage">
        <section className="phone-frame">
          <div className="phone-notch" />

          <header className="phone-header">
            <div className="phone-chrome">
              <span className="phone-chrome-time">10:32</span>
              <span className="phone-chrome-icons">◔ 5G ▮▮▮</span>
            </div>
            <div className="phone-title">CHYREN</div>
          </header>

          <section className="phone-chat">
            {isEmpty ? (
              <div className="chat-empty">
                <div className="chat-empty-mark">Ω</div>
                <div className="chat-empty-copy">
                  High-integrity sovereign chat. Enter a prompt to begin.
                </div>
              </div>
            ) : (
              <div className="chat-list">
                {messages.map((msg) => (
                  <div key={msg.id} className={`chat-row chat-row-${msg.role}`}>
                    {msg.role === 'assistant' && (
                      <div className="chat-avatar" aria-hidden="true">
                        <span>◎</span>
                      </div>
                    )}
                    <article className={`chat-bubble chat-bubble-${msg.role}`}>
                      <div className="chat-meta">
                        <span>{msg.role === 'user' ? 'User' : 'Chyren'}</span>
                        {msg.role === 'assistant' && isStreaming && msg.id === lastMsg?.id && <span className="chat-live">LIVE</span>}
                      </div>
                      <div className={`chat-body ${msg.isCorrection ? 'chat-correction' : ''}`}>
                        {msg.role === 'assistant' ? (
                          <>
                            <ReactMarkdown remarkPlugins={[remarkGfm]}>{msg.content}</ReactMarkdown>
                            {msg.consensus && <div className="chat-chip">Council consensus {Math.round(msg.consensus.agreement * 100)}%</div>}
                            {msg.audit && (
                              <div className={`chat-chip ${msg.audit.passed ? 'chip-pass' : 'chip-drift'}`}>
                                {msg.audit.passed ? 'Verified by ADCCL' : `Drift detected (${msg.audit.score.toFixed(2)})`}
                              </div>
                            )}
                          </>
                        ) : (
                          <p>{msg.content}</p>
                        )}
                      </div>
                    </article>
                  </div>
                ))}
                <div ref={messagesEndRef} />
              </div>
            )}

            {error && (
              <div className="chat-error">
                <AlertCircle size={13} />
                <span>{error}</span>
                <button className="chat-error-close" onClick={() => setError(null)} type="button">
                  ×
                </button>
              </div>
            )}
          </section>

          <footer className="phone-footer">
            <div className="composer-shell">
              <button className="composer-ghost" type="button" onClick={toggleTTS} aria-label={ttsEnabled ? 'Mute' : 'Unmute'}>
                {ttsEnabled ? <Volume2 size={16} /> : <VolumeX size={16} />}
              </button>

              <textarea
                ref={textareaRef}
                className="composer-input"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Message..."
                rows={1}
                disabled={isStreaming}
              />

              <button
                className="composer-send"
                onClick={() => void sendMessage(input)}
                disabled={isStreaming || !input.trim()}
                aria-label="Send"
                type="button"
              >
                {isStreaming ? <Loader2 size={16} className="omega-spin" /> : <Send size={16} />}
              </button>
            </div>
          </footer>
        </section>
      </main>
    </div>
  );
}

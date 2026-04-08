'use client';

import React, { useState, useEffect, useRef, useCallback } from 'react';
import dynamic from 'next/dynamic';
import { Send, Loader2, AlertCircle, Volume2, VolumeX } from 'lucide-react';
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
}

// Handles Vercel AI SDK text stream (0:"chunk") and plain-text streams
// Handles Hub events (data: {...}) and plain-text streams
function parseHubChunk(chunk: string): { content?: string, audit?: { passed: boolean, score: number, flags: string[] }, status?: string } {
  let content = '';
  let audit: { passed: boolean, score: number, flags: string[] } | undefined;
  let status: string | undefined;

  for (const line of chunk.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) continue;

    if (trimmed.startsWith('data: ')) {
      try {
        const json = JSON.parse(trimmed.slice(6));
        
        // Handle Standard AI Chunks
        const delta = json.choices?.[0]?.delta?.content;
        if (delta) content += delta;

        // Handle Hub Status Events
        if (json.status === 'audited') {
           audit = json.audit_report;
        }
        if (json.status === 'correction') {
           content += json.content;
           status = 'correction';
        }
        if (json.status === 'deflected') {
           content = json.content;
           status = 'deflected';
        }
      } catch { /* skip */ }
    } else if (trimmed.startsWith('0:')) {
       // Vercel AI SDK compatibility
       try {
         const delta = JSON.parse(trimmed.slice(2));
         if (typeof delta === 'string') content += delta;
       } catch {}
    }
  }
  return { content, audit, status };
}

// Pull a speakable chunk from the buffer:
// - at a sentence boundary (. ! ? newline), or
// - at a word boundary after 80 chars (to avoid long pauses waiting for punctuation)
function extractSpeakable(buffer: string): { chunk: string | null; remaining: string } {
  const m = buffer.match(/^([\s\S]+?[.!?\n]+)\s*([\s\S]*)$/);
  if (m) return { chunk: m[1].trim(), remaining: m[2] };
  if (buffer.length > 80) {
    const cut = buffer.lastIndexOf(' ', 80);
    if (cut > 20) return { chunk: buffer.slice(0, cut).trim(), remaining: buffer.slice(cut + 1) };
  }
  return { chunk: null, remaining: buffer };
}

function getVoice(): SpeechSynthesisVoice | null {
  const voices = window.speechSynthesis.getVoices();
  // Priority: British Male -> British -> English -> First available
  return (
    voices.find(v => v.lang.startsWith('en-GB') && (v.name.includes('Male') || v.name.includes('Daniel') || v.name.includes('Arthur'))) ||
    voices.find(v => v.lang.startsWith('en-GB')) ||
    voices.find(v => v.lang.includes('en-GB')) ||
    voices.find(v => v.name.toLowerCase().includes('google') && v.lang.startsWith('en')) ||
    voices.find(v => v.lang.startsWith('en-US')) ||
    voices[0] ||
    null
  );
}

function speak(text: string) {
  if (!text.trim() || typeof window === 'undefined') return;
  const utter = new SpeechSynthesisUtterance(text.trim());
  // "Smart wise" tuning: slightly slower, slightly lower pitch
  utter.rate = 0.95; 
  utter.pitch = 0.9;
  utter.volume = 1.0;
  const v = getVoice();
  if (v) utter.voice = v;
  window.speechSynthesis.speak(utter);
}

export default function ChatPage() {
  const [messages, setMessages]      = useState<Message[]>([]);
  const [input, setInput]            = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [ttsEnabled, setTtsEnabled]  = useState(true);
  const [error, setError]            = useState<string | null>(null);

  // Persistence: load from localStorage on mount
  useEffect(() => {
    const saved = localStorage.getItem('chyren_messages');
    if (saved) {
      try {
        setMessages(JSON.parse(saved));
      } catch {}
    }
  }, []);

  // Persistence: save to localStorage on change
  useEffect(() => {
    if (messages.length > 0) {
      localStorage.setItem('chyren_messages', JSON.stringify(messages));
    }
  }, [messages]);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const abortRef       = useRef<AbortController | null>(null);
  const textareaRef    = useRef<HTMLTextAreaElement>(null);
  const ttsEnabledRef  = useRef(ttsEnabled);

  // Keep ref in sync so the streaming closure always reads the latest value
  useEffect(() => { ttsEnabledRef.current = ttsEnabled; }, [ttsEnabled]);

  // Pre-load voices (Chrome lazy-initialises them)
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
    ta.style.height = Math.min(ta.scrollHeight, 140) + 'px';
  }, [input]);

  const sendMessage = useCallback(async (text: string) => {
    const trimmed = text.trim();
    if (!trimmed || isStreaming) return;

    // Cancel any ongoing speech before starting a new response
    window.speechSynthesis?.cancel();

    setError(null);
    setInput('');

    const userMsg: Message = { id: `u-${Date.now()}`, role: 'user', content: trimmed };
    const history = [...messages, userMsg];
    setMessages(history);
    setIsStreaming(true);

    const assistantId = `a-${Date.now()}`;
    setMessages(prev => [...prev, { id: assistantId, role: 'assistant', content: '' }]);

    try {
      abortRef.current?.abort();
      const abort = new AbortController();
      abortRef.current = abort;

      const res = await fetch('/api/chat/stream', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ messages: history.map(m => ({ role: m.role, content: m.content })) }),
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

      if (reader) {
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          const { content: delta, audit, status } = parseHubChunk(decoder.decode(value, { stream: true }));
          
          if (delta) {
            accumulated += delta;
            setMessages(prev =>
              prev.map(m => m.id === assistantId ? { ...m, content: accumulated } : m)
            );
          }

          if (audit) {
            setMessages(prev =>
              prev.map(m => m.id === assistantId ? { ...m, audit } : m)
            );
          }

          if (status === 'correction') {
             setMessages(prev =>
               prev.map(m => m.id === assistantId ? { ...m, isCorrection: true } : m)
             );
          }

          // Stream TTS: speak sentence-by-sentence as text arrives
          if (ttsEnabledRef.current) {
            speechBuffer += delta;
            let result = extractSpeakable(speechBuffer);
            while (result.chunk) {
              speak(result.chunk);
              speechBuffer = result.remaining;
              result = extractSpeakable(speechBuffer);
            }
          }
        }
      }

      // Speak any remaining buffer after stream ends
      if (ttsEnabledRef.current && speechBuffer.trim()) {
        speak(speechBuffer.trim());
      }
      
      // If nothing was received, remove the blank bubble and show an error
      if (!accumulated) {
        setMessages(prev => prev.filter(m => m.id !== assistantId));
        setError('Neural link disrupted — no response received. Please wait a moment and retry.');
      }
    } catch (err) {
      if ((err as Error).name !== 'AbortError') {
        window.speechSynthesis?.cancel();
        const msg = (err as Error).message;
        setError(msg.includes('quota') ? 'AI quota exhausted — check provider config' : `Neural link error: ${msg}`);
        setMessages(prev => prev.filter(m => m.id !== assistantId));
      }
    } finally {
      setIsStreaming(false);
      abortRef.current = null;
    }
  }, [messages, isStreaming]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      void sendMessage(input);
      }  };

  const toggleTTS = () => {
    if (ttsEnabled) window.speechSynthesis?.cancel();
    setTtsEnabled(s => !s);
  };

  const isEmpty = messages.length === 0;
  const lastMsg = messages[messages.length - 1];

  return (
    <div className="omega-root">
      <div className="omega-bg">
        <NeuralBrain isActive={isStreaming} />
      </div>

      <header className="omega-header">
        <div className="omega-logo">
          <span className="omega-symbol">Ω</span>
        </div>
        <span className="omega-brand">Chyren</span>

        <div className="omega-header-status">
          <span className={`omega-status-dot ${isStreaming ? 'omega-status-active' : ''}`} />
          <span className="omega-status-label">
            {isStreaming ? 'PROCESSING' : 'SOVEREIGN'}
          </span>
        </div>

        <div className="omega-header-actions">
          {!isEmpty && (
            <button 
              onClick={() => { setMessages([]); localStorage.removeItem('chyren_messages'); }} 
              className="omega-icon-btn omega-purge-btn" 
              title="Purge Memory"
            >
              PURGE
            </button>
          )}
          <button onClick={toggleTTS} className="omega-icon-btn" title={ttsEnabled ? 'Mute' : 'Unmute'}>
            {ttsEnabled ? <Volume2 size={15} /> : <VolumeX size={15} />}
          </button>
        </div>
      </header>

      <main className="omega-main">
        {isEmpty ? (
          <div className="omega-idle">
            <div className="omega-idle-title">
              <span className="omega-idle-omega">Ω</span>
              <span className="omega-idle-name">Chyren</span>
            </div>
            <p className="omega-idle-sub">SOVEREIGN INTELLIGENCE ORCHESTRATOR</p>
            <p className="omega-idle-desc">
              High-integrity cognitive processing and neural telemetry. Type to initiate a session.
            </p>
          </div>
        ) : (
          <div className="omega-messages">
            {messages.map(msg => (
              <div key={msg.id} className={`omega-msg omega-msg-${msg.role}`}>
                <div className="omega-msg-label">
                  {msg.role === 'user' ? '▸ OPERATOR' : 'Ω CHYREN'}
                </div>
                <div className={`omega-msg-content ${msg.isCorrection ? 'omega-correction-block' : ''}`}>
                  {msg.role === 'assistant' ? (
                    <>
                      <ReactMarkdown remarkPlugins={[remarkGfm]}>{msg.content}</ReactMarkdown>
                      {isStreaming && msg === lastMsg && <span className="omega-cursor" />}
                      
                      {msg.audit && (
                        <div className={`omega-audit-seal ${msg.audit.passed ? 'omega-audit-passed' : 'omega-audit-drift'}`}>
                           {msg.audit.passed ? '🛡 VERIFIED BY ADCCL' : `⚠ COGNITIVE DRIFT DETECTED (${msg.audit.score.toFixed(2)})`}
                        </div>
                      )}
                    </>
                  ) : (
                    <p>{msg.content}</p>
                  )}
                </div>
              </div>
            ))}
            <div ref={messagesEndRef} />
          </div>
        )}

        {error && (
          <div className="omega-error">
            <AlertCircle size={13} />
            <span>{error}</span>
            <button className="omega-error-dismiss" onClick={() => setError(null)}>✕</button>
          </div>
        )}
      </main>

      <footer className="omega-footer">
        <div className="omega-input-wrap">
          <textarea
            ref={textareaRef}
            className="omega-textarea"
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="▸  transmit cognitive intent..."
            rows={1}
            disabled={isStreaming}
          />

          <button
            className="omega-send-btn"
            onClick={() => sendMessage(input)}
            disabled={isStreaming || !input.trim()}
            aria-label="Send"
          >
            {isStreaming ? <Loader2 size={17} className="omega-spin" /> : <Send size={17} />}
          </button>
        </div>

        <div className="omega-footer-meta">
          <span className="omega-footer-label">NEURAL LINK</span>
          <span className={`omega-footer-dot ${isStreaming ? 'omega-footer-dot-active' : ''}`} />
          <span className="omega-footer-label">{isStreaming ? 'ACTIVE' : 'STANDBY'}</span>
        </div>
      </footer>
    </div>
  );
}

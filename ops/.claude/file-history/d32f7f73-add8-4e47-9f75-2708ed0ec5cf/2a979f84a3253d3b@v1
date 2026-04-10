'use client';

import React, { useState, useEffect, useRef, useCallback } from 'react';
import dynamic from 'next/dynamic';
import { Send, Mic, MicOff, Volume2, VolumeX, Loader2, AlertCircle } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

const NeuralBrain = dynamic(() => import('@/components/NeuralBrain'), { ssr: false });

// ── types ──────────────────────────────────────────────────────────────────
interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
}

// ── speech recognition types ──────────────────────────────────────────────
type SRInstance = {
  lang: string;
  continuous: boolean;
  interimResults: boolean;
  onresult: ((e: SREvent) => void) | null;
  onerror: ((e: Event) => void) | null;
  onend: (() => void) | null;
  start(): void;
  stop(): void;
  abort(): void;
};
type SREvent = { results: { length: number; [i: number]: { isFinal: boolean; [j: number]: { transcript: string } } } };

function getSR(): (new () => SRInstance) | null {
  if (typeof window === 'undefined') return null;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const w = window as any;
  return w.SpeechRecognition || w.webkitSpeechRecognition || null;
}

// ── TTS ────────────────────────────────────────────────────────────────────
function browserSpeak(text: string): boolean {
  if (typeof window === 'undefined' || !window.speechSynthesis) return false;
  window.speechSynthesis.cancel();
  const utter = new SpeechSynthesisUtterance(text);
  utter.rate = 1.0;
  utter.pitch = 1.0;
  utter.volume = 1.0;
  // Pick a good English voice; fall back to whatever is available
  const pickVoice = () => {
    const voices = window.speechSynthesis.getVoices();
    return (
      voices.find(v => v.name.toLowerCase().includes('google') && v.lang.startsWith('en')) ||
      voices.find(v => v.lang.startsWith('en')) ||
      voices[0]
    );
  };
  const v = pickVoice();
  if (v) utter.voice = v;
  window.speechSynthesis.speak(utter);
  return true;
}

// ── stream parser ──────────────────────────────────────────────────────────
// Handles both Vercel AI SDK (0:"chunk") and plain-text (Firebase / Gemini) streams
function parseChunk(chunk: string): string {
  let out = '';
  for (const line of chunk.split('\n')) {
    if (line.startsWith('0:')) {
      try {
        const delta = JSON.parse(line.slice(2));
        if (typeof delta === 'string') out += delta;
      } catch { /* skip malformed */ }
    } else if (
      line.trim() &&
      !line.startsWith('e:') &&
      !line.startsWith('d:') &&
      !line.startsWith('data:') &&
      !line.startsWith('f:') &&
      !line.startsWith('8:') &&
      !line.startsWith('2:')
    ) {
      out += line;
    }
  }
  return out;
}

// ── component ──────────────────────────────────────────────────────────────
export default function ChatPage() {
  const [messages, setMessages]     = useState<Message[]>([]);
  const [input, setInput]           = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [isListening, setIsListening] = useState(false);
  const [ttsEnabled, setTtsEnabled]  = useState(true);
  const [transcript, setTranscript]  = useState('');
  const [error, setError]            = useState<string | null>(null);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const recognitionRef = useRef<SRInstance | null>(null);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const mediaStreamRef = useRef<MediaStream | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const abortRef       = useRef<AbortController | null>(null);
  const textareaRef    = useRef<HTMLTextAreaElement>(null);

  // Load TTS voices eagerly (Chrome lazy-loads them)
  useEffect(() => {
    const load = () => window.speechSynthesis?.getVoices();
    load();
    window.speechSynthesis?.addEventListener?.('voiceschanged', load);
    return () => window.speechSynthesis?.removeEventListener?.('voiceschanged', load);
  }, []);

  useEffect(() => {
    return () => {
      recognitionRef.current?.abort();
      mediaRecorderRef.current?.stop();
      mediaStreamRef.current?.getTracks().forEach(track => track.stop());
      audioRef.current?.pause();
      if (typeof window !== 'undefined') {
        window.speechSynthesis?.cancel();
      }
    };
  }, []);

  // Auto-scroll on new messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Auto-resize textarea
  useEffect(() => {
    const ta = textareaRef.current;
    if (!ta) return;
    ta.style.height = 'auto';
    ta.style.height = Math.min(ta.scrollHeight, 140) + 'px';
  }, [input]);

  const transcribeAudio = useCallback(async (audioBlob: Blob) => {
    const formData = new FormData();
    formData.append('audio', audioBlob, `voice-input-${Date.now()}.webm`);

    const res = await fetch('/api/stt', {
      method: 'POST',
      body: formData,
    });

    if (!res.ok) {
      const body = await res.text().catch(() => '');
      throw new Error(`Speech recognition failed: ${body.slice(0, 200)}`);
    }

    const data = await res.json() as { transcription?: string };
    const transcription = data.transcription?.trim();
    if (!transcription) {
      throw new Error('No transcription returned from speech service');
    }

    return transcription;
  }, []);

  const speakResponse = useCallback(async (text: string) => {
    if (!ttsEnabled || !text) return;

    audioRef.current?.pause();
    audioRef.current = null;

    if (browserSpeak(text)) return;

    const res = await fetch('/api/tts', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text }),
    });

    if (!res.ok) {
      const body = await res.text().catch(() => '');
      throw new Error(`Speech playback failed: ${body.slice(0, 200)}`);
    }

    const audioBlob = await res.blob();
    const audioUrl = URL.createObjectURL(audioBlob);
    const audio = new Audio(audioUrl);
    audioRef.current = audio;
    audio.onended = () => URL.revokeObjectURL(audioUrl);
    audio.onerror = () => URL.revokeObjectURL(audioUrl);
    await audio.play();
  }, [ttsEnabled]);

  // ── send ─────────────────────────────────────────────────────────────────
  const sendMessage = useCallback(async (text: string) => {
    const trimmed = text.trim();
    if (!trimmed || isStreaming) return;

    setError(null);
    setInput('');
    setTranscript('');

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

      if (reader) {
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          const delta = parseChunk(decoder.decode(value, { stream: true }));
          if (delta) {
            accumulated += delta;
            setMessages(prev =>
              prev.map(m => m.id === assistantId ? { ...m, content: accumulated } : m)
            );
          }
        }
      }

      // TTS — speak the complete response
      if (ttsEnabled && accumulated) {
        // Small delay so the UI settles first
        setTimeout(() => {
          void speakResponse(accumulated).catch((err: Error) => {
            setError(`Voice playback error: ${err.message}`);
          });
        }, 80);
      }
    } catch (err) {
      if ((err as Error).name !== 'AbortError') {
        const msg = (err as Error).message;
        setError(msg.includes('quota') ? 'AI quota exhausted — check provider config' : `Neural link error: ${msg}`);
        setMessages(prev => prev.filter(m => m.id !== assistantId));
      }
    } finally {
      setIsStreaming(false);
      abortRef.current = null;
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [messages, isStreaming, speakResponse]);

  // ── STT ──────────────────────────────────────────────────────────────────
  const startListening = useCallback(async () => {
    audioRef.current?.pause();
    audioRef.current = null;
    window.speechSynthesis?.cancel();

    const SR = getSR();
    if (!SR) {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        mediaStreamRef.current = stream;
        audioChunksRef.current = [];

        const preferredMimeType = MediaRecorder.isTypeSupported('audio/webm;codecs=opus')
          ? 'audio/webm;codecs=opus'
          : 'audio/webm';
        const recorder = new MediaRecorder(stream, { mimeType: preferredMimeType });
        mediaRecorderRef.current = recorder;

        recorder.ondataavailable = (event) => {
          if (event.data.size > 0) {
            audioChunksRef.current.push(event.data);
          }
        };

        recorder.onstop = async () => {
          mediaStreamRef.current?.getTracks().forEach(track => track.stop());
          mediaStreamRef.current = null;
          mediaRecorderRef.current = null;

          const audioBlob = new Blob(audioChunksRef.current, { type: preferredMimeType });
          audioChunksRef.current = [];
          if (!audioBlob.size) {
            setTranscript('');
            return;
          }

          setTranscript('Transcribing...');
          try {
            const finalTranscript = await transcribeAudio(audioBlob);
            setTranscript(finalTranscript);
            await sendMessage(finalTranscript);
          } catch (err) {
            setError((err as Error).message);
          }
        };

        recorder.start();
        setTranscript('Listening...');
        setIsListening(true);
        setError(null);
      } catch (err) {
        setError(`Mic error: ${(err as Error).message}`);
      }
      return;
    }

    const rec = new SR();
    rec.lang = 'en-US';
    rec.continuous = false;
    rec.interimResults = true;

    rec.onresult = (e: SREvent) => {
      let interim = '';
      let final = '';
      for (let i = 0; i < e.results.length; i++) {
        const r = e.results[i];
        if (r.isFinal) final += r[0].transcript;
        else interim += r[0].transcript;
      }
      setTranscript(final || interim);
      if (final) {
        rec.stop();
        setIsListening(false);
        sendMessage(final);
      }
    };

    rec.onerror = (e: Event) => {
      const code = (e as unknown as { error: string }).error;
      setIsListening(false);
      if (code !== 'aborted') setError(`Mic error: ${code}`);
    };

    rec.onend = () => setIsListening(false);

    rec.start();
    recognitionRef.current = rec;
    setIsListening(true);
    setError(null);
  }, [sendMessage, transcribeAudio]);

  const stopListening = useCallback(() => {
    if (recognitionRef.current) {
      recognitionRef.current.abort();
      recognitionRef.current = null;
    }

    if (mediaRecorderRef.current && mediaRecorderRef.current.state !== 'inactive') {
      mediaRecorderRef.current.stop();
      setTranscript('Transcribing...');
    }

    setIsListening(false);
  }, []);

  const toggleMic = () => {
    if (isListening) stopListening();
    else startListening();
  };

  const toggleTTS = () => {
    if (ttsEnabled) {
      window.speechSynthesis?.cancel();
      audioRef.current?.pause();
      audioRef.current = null;
    }
    setTtsEnabled(s => !s);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage(input);
    }
  };

  const isEmpty = messages.length === 0;
  const lastMsg = messages[messages.length - 1];

  return (
    <div className="omega-root">
      {/* Neural canvas background */}
      <div className="omega-bg">
        <NeuralBrain isActive={isStreaming || isListening} />
      </div>

      {/* ── Header ── */}
      <header className="omega-header">
        <div className="omega-logo">
          <span className="omega-symbol">Ω</span>
        </div>
        <span className="omega-brand">Chyren</span>

        <div className="omega-header-status">
          <span className={`omega-status-dot ${(isStreaming || isListening) ? 'omega-status-active' : ''}`} />
          <span className="omega-status-label">
            {isStreaming ? 'PROCESSING' : isListening ? 'LISTENING' : 'SOVEREIGN'}
          </span>
        </div>

        <div className="omega-header-actions">
          <button onClick={toggleTTS} className="omega-icon-btn" title={ttsEnabled ? 'Mute Chyren' : 'Enable voice'}>
            {ttsEnabled ? <Volume2 size={15} /> : <VolumeX size={15} />}
          </button>
        </div>
      </header>

      {/* ── Main ── */}
      <main className="omega-main">
        {isEmpty ? (
          <div className="omega-idle">
            <div className="omega-idle-title">
              <span className="omega-idle-omega">Ω</span>
              <span className="omega-idle-name">Chyren</span>
            </div>
            <p className="omega-idle-sub">SOVEREIGN INTELLIGENCE ORCHESTRATOR</p>
            <p className="omega-idle-desc">
              High-integrity cognitive processing and neural telemetry. Speak or type to initiate a session.
            </p>

            {/* Voice orb */}
            <button
              className={`omega-orb ${isListening ? 'omega-orb-active' : ''}`}
              onClick={toggleMic}
              disabled={isStreaming}
              aria-label="Voice input"
            >
              <div className="omega-orb-ring omega-orb-ring-1" />
              <div className="omega-orb-ring omega-orb-ring-2" />
              <div className="omega-orb-ring omega-orb-ring-3" />
              <div className="omega-orb-core">
                {isListening ? <MicOff size={26} /> : <Mic size={26} />}
              </div>
            </button>

            <p className="omega-orb-label">
              {isListening ? 'LISTENING — SPEAK NOW' : 'INITIALIZE NEURAL INTENT'}
            </p>

            {transcript && <p className="omega-transcript">{transcript}</p>}
          </div>
        ) : (
          <div className="omega-messages">
            {messages.map(msg => (
              <div key={msg.id} className={`omega-msg omega-msg-${msg.role}`}>
                <div className="omega-msg-label">
                  {msg.role === 'user' ? '▸ OPERATOR' : 'Ω CHYREN'}
                </div>
                <div className="omega-msg-content">
                  {msg.role === 'assistant' ? (
                    <>
                      <ReactMarkdown remarkPlugins={[remarkGfm]}>{msg.content}</ReactMarkdown>
                      {isStreaming && msg === lastMsg && <span className="omega-cursor" />}
                    </>
                  ) : (
                    <p>{msg.content}</p>
                  )}
                </div>
              </div>
            ))}
            {/* Transcript bar while listening mid-conversation */}
            {isListening && transcript && (
              <p className="omega-transcript-bar">{transcript}</p>
            )}
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

      {/* ── Footer / input ── */}
      <footer className="omega-footer">
        <div className="omega-input-wrap">
          <button
            className={`omega-voice-btn ${isListening ? 'omega-voice-btn-active' : ''}`}
            onClick={toggleMic}
            disabled={isStreaming}
            aria-label={isListening ? 'Stop listening' : 'Start voice input'}
          >
            {isListening ? <MicOff size={17} /> : <Mic size={17} />}
          </button>

          <textarea
            ref={textareaRef}
            className="omega-textarea"
            value={isListening ? (transcript || '') : input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="▸  transmit cognitive intent..."
            rows={1}
            disabled={isStreaming || isListening}
          />

          <button
            className="omega-send-btn"
            onClick={() => sendMessage(input)}
            disabled={isStreaming || isListening || !input.trim()}
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

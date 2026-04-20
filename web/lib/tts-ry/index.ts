/**
 * tts-ry — Client-side streaming TTS pipeline
 *
 * Core responsibilities:
 *   1. Sentence splitting during live text stream
 *   2. Queued playback — fires audio for sentence N while sentence N+1 is still compiling
 *   3. Latency masking — plays a brief "Mhm" chime immediately upon input receipt
 *   4. Barge-in support — halt() stops all playback instantly
 *   5. Falls back to browser SpeechSynthesis if /api/tts returns 503
 *
 * Architecture:
 *   TtsEngine is a stateful singleton per chat session.
 *   Call `engine.feedDelta(text)` as SSE chunks arrive.
 *   Call `engine.finish()` when the stream completes.
 *   Call `engine.halt()` on barge-in or new user input.
 */

// ─── Sentence splitter ───────────────────────────────────────────────────────

const SENTENCE_RE = /(?<=[.!?。！？])\s+|(?<=\n)\s*/

function splitSentences(text: string): { complete: string[]; remainder: string } {
  const parts = text.split(SENTENCE_RE)
  if (parts.length <= 1) return { complete: [], remainder: text }
  const remainder = parts.pop() ?? ''
  return { complete: parts.filter(s => s.trim().length > 0), remainder }
}

// ─── Latency masking chime (synthesized, zero-network) ───────────────────────

let _audioCtxCache: AudioContext | null = null

function getAudioCtx(): AudioContext {
  if (!_audioCtxCache || _audioCtxCache.state === 'closed') {
    _audioCtxCache = new AudioContext()
  }
  if (_audioCtxCache.state === 'suspended') {
    _audioCtxCache.resume().catch(() => {})
  }
  return _audioCtxCache
}

/**
 * Play a sub-second acknowledgment chime.
 * Two soft sine tones at 440 Hz and 554 Hz, 120ms duration.
 * Sounds like a warm digital "mhm".
 */
export function playLatencyChime(): void {
  try {
    const ctx = getAudioCtx()
    const now = ctx.currentTime

    const gain = ctx.createGain()
    gain.gain.setValueAtTime(0.06, now)
    gain.gain.exponentialRampToValueAtTime(0.001, now + 0.45)
    gain.connect(ctx.destination)

    // Root tone
    const osc1 = ctx.createOscillator()
    osc1.type = 'sine'
    osc1.frequency.setValueAtTime(523.25, now)
    osc1.connect(gain)
    osc1.start(now)
    osc1.stop(now + 0.45)

    // Harmonic
    const gain2 = ctx.createGain()
    gain2.gain.setValueAtTime(0.03, now)
    gain2.gain.exponentialRampToValueAtTime(0.001, now + 0.35)
    gain2.connect(ctx.destination)

    const osc2 = ctx.createOscillator()
    osc2.type = 'sine'
    osc2.frequency.setValueAtTime(659.25, now)
    osc2.connect(gain2)
    osc2.start(now + 0.015)
    osc2.stop(now + 0.35)
  } catch {
    // AudioContext might not be available — silent fallback
  }
}

// ─── Browser SpeechSynthesis fallback ────────────────────────────────────────

function getPremiumVoice(): SpeechSynthesisVoice | undefined {
  if (typeof window === 'undefined') return undefined
  const voices = window.speechSynthesis.getVoices()
  if (!voices.length) return undefined
  // Prefer smart-sounding British male voices
  const targets = [
    (v: SpeechSynthesisVoice) => v.name.includes('Daniel'),
    (v: SpeechSynthesisVoice) => v.name.includes('Google UK English Male'),
    (v: SpeechSynthesisVoice) => v.name.includes('Arthur'),
    (v: SpeechSynthesisVoice) => v.name.includes('UK English Male'),
    (v: SpeechSynthesisVoice) => v.lang === 'en-GB' && !v.name.toLowerCase().includes('female'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en-GB'),
    (v: SpeechSynthesisVoice) => v.lang.startsWith('en'),
  ]
  for (const t of targets) {
    const m = voices.find(t)
    if (m) return m
  }
  return voices[0]
}

function speakBrowser(text: string): Promise<void> {
  return new Promise<void>((resolve) => {
    if (!text.trim() || typeof window === 'undefined') { resolve(); return }
    // Cancel any currently speaking utterance
    window.speechSynthesis.cancel()
    const utter = new SpeechSynthesisUtterance(text.trim())
    utter.rate = 1.12   // slightly faster = more natural/energetic
    utter.pitch = 1.05  // slightly higher = younger feel
    utter.volume = 1.0
    // Voices load async — get them fresh each time
    const setVoice = () => {
      const v = getPremiumVoice()
      if (v) utter.voice = v
      utter.onend = () => resolve()
      utter.onerror = () => resolve()
      window.speechSynthesis.speak(utter)
    }
    // If voices not loaded yet, wait for them
    if (window.speechSynthesis.getVoices().length === 0) {
      window.speechSynthesis.onvoiceschanged = () => { setVoice() }
    } else {
      setVoice()
    }
  })
}

// ─── Server TTS fetch ────────────────────────────────────────────────────────

async function fetchTtsAudio(text: string): Promise<ArrayBuffer | null> {
  try {
    const res = await fetch('/api/tts', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text }),
      signal: AbortSignal.timeout(8_000),
    })

    if (!res.ok) {
      // 503 = no engine available → fall back to browser
      return null
    }

    return await res.arrayBuffer()
  } catch {
    return null
  }
}

async function playAudioBuffer(buffer: ArrayBuffer): Promise<void> {
  const ctx = getAudioCtx()
  const decoded = await ctx.decodeAudioData(buffer)
  return new Promise<void>((resolve) => {
    const source = ctx.createBufferSource()
    source.buffer = decoded
    source.connect(ctx.destination)
    source.onended = () => resolve()
    source.start()
  })
}

// ─── TTS Engine ──────────────────────────────────────────────────────────────

export type TtsMode = 'server' | 'browser' | 'off'

export interface TtsEngineOptions {
  /** Called when TTS playback state changes */
  onPlaybackChange?: (playing: boolean) => void
  /** Called when audio level changes (for ChyRho vis during TTS output) */
  onOutputLevel?: (level: number) => void
}

export class TtsEngine {
  private _mode: TtsMode = 'server'
  private _halted = false
  private _playing = false
  private _queue: string[] = []
  private _buffer = ''
  private _sentencesSent = new Set<string>()
  private _drainPromise: Promise<void> | null = null
  private _opts: TtsEngineOptions

  constructor(opts: TtsEngineOptions = {}) {
    this._opts = opts
  }

  /** Current engine mode */
  get mode(): TtsMode { return this._mode }
  set mode(m: TtsMode) { this._mode = m }

  /** Is audio currently playing? */
  get isPlaying(): boolean { return this._playing }

  /**
   * Feed a delta from the SSE stream.
   * The engine accumulates text, splits on sentence boundaries,
   * and begins speaking completed sentences immediately.
   */
  feedDelta(delta: string): void {
    if (this._mode === 'off' || this._halted) return
    this._buffer += delta

    const { complete, remainder } = splitSentences(this._buffer)
    this._buffer = remainder

    for (const sentence of complete) {
      const key = sentence.trim()
      if (!key || this._sentencesSent.has(key)) continue
      this._sentencesSent.add(key)
      this._queue.push(key)
    }

    this._startDrain()
  }

  /**
   * Call when the stream is done. Flushes any remaining buffered text.
   */
  finish(): void {
    if (this._mode === 'off' || this._halted) return
    const remainder = this._buffer.trim()
    if (remainder && !this._sentencesSent.has(remainder)) {
      this._sentencesSent.add(remainder)
      this._queue.push(remainder)
    }
    this._buffer = ''
    this._startDrain()
  }

  /**
   * Immediately halt all playback (barge-in).
   */
  halt(): void {
    this._halted = true
    this._queue = []
    this._buffer = ''
    this._playing = false
    this._opts.onPlaybackChange?.(false)
    this._opts.onOutputLevel?.(0)

    // Cancel browser speech
    window.speechSynthesis?.cancel()

    // Close audio context to kill any playing buffer sources
    if (_audioCtxCache && _audioCtxCache.state !== 'closed') {
      _audioCtxCache.close().catch(() => {})
      _audioCtxCache = null
    }
  }

  /**
   * Reset for a new response cycle. Must be called before each new assistant message.
   */
  reset(): void {
    this._halted = false
    this._buffer = ''
    this._queue = []
    this._sentencesSent.clear()
    this._drainPromise = null
    this._playing = false
  }

  // ── internal ───────────────────────────────────────────────────────────────

  private _startDrain(): void {
    if (this._drainPromise) return // already draining
    this._drainPromise = this._drain()
    void this._drainPromise
      .then(() => {
        this._drainPromise = null
      })
      .catch(() => {
        // Best-effort: ensure the engine can be re-used after an unexpected drain failure.
        this._drainPromise = null
      })
  }

  private async _drain(): Promise<void> {
    while (this._queue.length > 0 && !this._halted) {
      const sentence = this._queue.shift()!
      this._playing = true
      this._opts.onPlaybackChange?.(true)

      try {
        if (this._mode === 'server') {
          const audio = await fetchTtsAudio(sentence)
          if (audio && !this._halted) {
            await playAudioBuffer(audio)
          } else if (!this._halted) {
            // Server unavailable — fall back to browser for this sentence
            await speakBrowser(sentence)
          }
        } else {
          await speakBrowser(sentence)
        }
      } catch {
        // Playback failed — skip and continue
      }
    }

    this._playing = false
    this._opts.onPlaybackChange?.(false)
    this._opts.onOutputLevel?.(0)
  }
}

// ─── Factory ─────────────────────────────────────────────────────────────────

/** Create a TtsEngine instance */
export function createTtsEngine(opts?: TtsEngineOptions): TtsEngine {
  return new TtsEngine(opts)
}

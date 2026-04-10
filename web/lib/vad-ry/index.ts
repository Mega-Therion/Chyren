/**
 * vad-ry — Voice Activity Detection engine
 *
 * Analyzes microphone audio in real-time using the Web Audio API.
 * Exposes:
 *   - isSpeaking flag via callback (drives barge-in)
 *   - rmsLevel (0–1) for waveform animation on the ChyRho canvas
 *   - clean teardown on stop
 */

export interface VADCallbacks {
  onSpeechStart: () => void
  onSpeechEnd: () => void
  onLevel: (rms: number) => void
}

export interface VADHandle {
  stop: () => void
}

const SPEECH_THRESHOLD = 0.018   // RMS level above which we consider speech active
const SILENCE_HOLD_MS  = 700     // How long silence must persist before onSpeechEnd fires
const FRAME_SIZE       = 2048    // FFT frame for analyser

export function attachVAD(stream: MediaStream, callbacks: VADCallbacks): VADHandle {
  const ctx = new AudioContext()
  const source = ctx.createMediaStreamSource(stream)
  const analyser = ctx.createAnalyser()
  analyser.fftSize = FRAME_SIZE

  source.connect(analyser)

  const buffer = new Float32Array(analyser.fftSize)
  let speaking = false
  let silenceTimer: ReturnType<typeof setTimeout> | null = null
  let rafId = 0

  const tick = () => {
    analyser.getFloatTimeDomainData(buffer)

    // Compute RMS
    let sum = 0
    for (let i = 0; i < buffer.length; i++) sum += buffer[i] * buffer[i]
    const rms = Math.sqrt(sum / buffer.length)

    callbacks.onLevel(Math.min(rms / 0.2, 1)) // normalise to 0-1

    if (rms > SPEECH_THRESHOLD) {
      if (silenceTimer !== null) {
        clearTimeout(silenceTimer)
        silenceTimer = null
      }
      if (!speaking) {
        speaking = true
        callbacks.onSpeechStart()
      }
    } else {
      if (speaking && silenceTimer === null) {
        silenceTimer = setTimeout(() => {
          speaking = false
          silenceTimer = null
          callbacks.onSpeechEnd()
        }, SILENCE_HOLD_MS)
      }
    }

    rafId = requestAnimationFrame(tick)
  }

  rafId = requestAnimationFrame(tick)

  return {
    stop: () => {
      cancelAnimationFrame(rafId)
      if (silenceTimer) clearTimeout(silenceTimer)
      source.disconnect()
      ctx.close().catch(() => { /* best-effort */ })
    },
  }
}

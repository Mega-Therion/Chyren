import { useState, useCallback, useRef } from 'react'

export function useVoiceEngine(onTranscript: (text: string) => void) {
  const [isRecording, setIsRecording] = useState(false)
  const mediaRecorderRef = useRef<MediaRecorder | null>(null)
  const audioChunksRef = useRef<Blob[]>([])

  const startRecording = useCallback(async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true })
      const mediaRecorder = new MediaRecorder(stream)
      mediaRecorderRef.current = mediaRecorder
      audioChunksRef.current = []

      mediaRecorder.ondataavailable = (e) => audioChunksRef.current.push(e.data)

      mediaRecorder.onstop = async () => {
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' })
        stream.getTracks().forEach((t) => t.stop())
        try {
          const formData = new FormData()
          formData.append('audio', audioBlob)
          const res = await fetch('/api/stt', { method: 'POST', body: formData })
          if (res.ok) {
            const data = await res.json()
            if (data.transcription) onTranscript(data.transcription)
          }
        } catch (err) {
          console.error('STT error:', err)
        }
      }

      mediaRecorder.start()
      setIsRecording(true)
    } catch (err) {
      console.error('Microphone access denied:', err)
    }
  }, [onTranscript])

  const stopRecording = useCallback(() => {
    mediaRecorderRef.current?.stop()
    setIsRecording(false)
  }, [])

  return { isRecording, startRecording, stopRecording }
}

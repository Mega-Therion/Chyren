/**
 * lib/firebase.ts — Firebase app initialization and AI Logic SDK integration.
 *
 * Exports:
 *  - `firebaseApp`   – initialized Firebase app (lazy singleton)
 *  - `getAIModel`    – returns a Firebase AI Logic generative model instance
 *  - `firebaseReady` – true when NEXT_PUBLIC_FIREBASE_PROJECT_ID is set
 *
 * To activate: set the NEXT_PUBLIC_FIREBASE_* env vars in one-true.env and
 * run `bash scripts/sync-vercel-env-from-one-true.sh` to push to Vercel.
 */

import { initializeApp, getApps, type FirebaseApp } from 'firebase/app'
import { getVertexAI, getGenerativeModel, type GenerativeModel } from '@firebase/vertexai'

/** Required Firebase config keys — warn at module load if any are missing */
const REQUIRED_FIREBASE_KEYS = [
  'NEXT_PUBLIC_FIREBASE_API_KEY',
  'NEXT_PUBLIC_FIREBASE_PROJECT_ID',
  'NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN',
  'NEXT_PUBLIC_FIREBASE_APP_ID',
] as const

/** Validate Firebase config at module load time (server + client) */
function validateFirebaseConfig(): void {
  if (typeof process === 'undefined') return // Edge runtime safety
  const missing = REQUIRED_FIREBASE_KEYS.filter(
    (key) => !process.env[key],
  )
  if (missing.length > 0 && process.env.NODE_ENV === 'production') {
    console.warn(
      `[firebase] Missing required env vars: ${missing.join(', ')}. ` +
      'Firebase AI Logic will be unavailable. ' +
      'Set these in Vercel dashboard or one-true.env.',
    )
  }
}

// Run validation once at module load
validateFirebaseConfig()

const firebaseConfig = {
  apiKey:            process.env.NEXT_PUBLIC_FIREBASE_API_KEY,
  authDomain:        process.env.NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN,
  projectId:         process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID,
  storageBucket:     process.env.NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: process.env.NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID,
  appId:             process.env.NEXT_PUBLIC_FIREBASE_APP_ID,
}

/** True when Firebase project credentials are present in env */
export const firebaseReady = Boolean(firebaseConfig.projectId && firebaseConfig.apiKey)

/** Lazy singleton — safe to import server-side; returns null if unconfigured */
export function getFirebaseApp(): FirebaseApp | null {
  if (!firebaseReady) return null
  return getApps().length > 0 ? getApps()[0] : initializeApp(firebaseConfig)
}

/**
 * Returns a Firebase AI Logic generative model.
 * Uses Gemini 2.0 Flash by default — override with the `model` param.
 *
 * Returns null if Firebase credentials are not configured.
 */
export function getAIModel(model = 'gemini-2.0-flash'): GenerativeModel | null {
  const app = getFirebaseApp()
  if (!app) return null
  const vertexAI = getVertexAI(app)
  return getGenerativeModel(vertexAI, { model })
}

/**
 * Generate a single text response via Firebase AI Logic.
 * Falls back gracefully with an error string if unconfigured.
 */
export async function generateWithFirebase(
  prompt: string,
  systemInstruction?: string,
): Promise<{ text: string; error?: string }> {
  const model = getAIModel()
  if (!model) {
    return {
      text: '',
      error: 'Firebase AI Logic not configured — set NEXT_PUBLIC_FIREBASE_* env vars',
    }
  }
  try {
    const request = systemInstruction
      ? { contents: [{ role: 'user' as const, parts: [{ text: prompt }] }], systemInstruction }
      : { contents: [{ role: 'user' as const, parts: [{ text: prompt }] }] }
    const result = await model.generateContent(request)
    const text = result.response.text()
    return { text }
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err)
    console.error('[firebase] generateContent failed:', message)
    return { text: '', error: message }
  }
}

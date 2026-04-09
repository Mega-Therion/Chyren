import { createHash } from 'node:crypto'
import { getCache } from '@vercel/functions'

type FamilyMember = {
  id: string
  displayName: string
  aliases: string[]
  challengeQuestion: string
  answerHash: string
  answerSalt?: string
}

type SessionAuthState = {
  pendingMemberId?: string
  verifiedMemberId?: string
  failedAttempts: number
}

type AuthResult = {
  handled: boolean
  reply?: string
  verifiedMember?: FamilyMember
  corrections?: string[]
}

const _LOCAL_SESSIONS = new Map<string, SessionAuthState>()
const _LOCAL_CORRECTIONS = new Map<string, string[]>()
const _CACHE_NAMESPACE = 'family-auth'
const _SESSION_TTL_SECONDS = 60 * 60 * 24 * 7 // 7 days
const _CORRECTION_TTL_SECONDS = 60 * 60 * 24 * 30 // 30 days
const _CORRECTION_MAX = 50

const _IDENTITY_PATTERNS = [
  /\b(i am|i'm|this is|speaking with)\b/i,
  /\b(aunt|uncle|cousin|mom|dad|mother|father|sister|brother|grandma|grandpa)\b/i,
]

const _UPDATE_PREFIXES = [
  /^update my profile:\s*/i,
  /^correction:\s*/i,
  /^profile update:\s*/i,
  /^my info update:\s*/i,
]

function normalize(input: string): string {
  return input
    .trim()
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s]/gu, ' ')
    .replace(/\s+/g, ' ')
}

function parseConfig(): FamilyMember[] {
  const raw = process.env.CHYREN_FAMILY_AUTH_CONFIG
  if (!raw) return []

  try {
    const parsed = JSON.parse(raw)
    if (!Array.isArray(parsed)) return []

    return parsed
      .map((entry): FamilyMember | null => {
        if (
          !entry ||
          typeof entry.id !== 'string' ||
          typeof entry.displayName !== 'string' ||
          typeof entry.challengeQuestion !== 'string' ||
          typeof entry.answerHash !== 'string'
        ) {
          return null
        }
        const aliases = Array.isArray(entry.aliases)
          ? entry.aliases.filter((a: unknown): a is string => typeof a === 'string')
          : []
        return {
          id: entry.id,
          displayName: entry.displayName,
          challengeQuestion: entry.challengeQuestion,
          answerHash: entry.answerHash.toLowerCase(),
          answerSalt: typeof entry.answerSalt === 'string' ? entry.answerSalt : undefined,
          aliases,
        }
      })
      .filter((m): m is FamilyMember => Boolean(m))
  } catch {
    return []
  }
}

function findClaimedMember(input: string, members: FamilyMember[]): FamilyMember | null {
  const lowered = input.toLowerCase()
  if (!_IDENTITY_PATTERNS.some((re) => re.test(lowered))) return null

  for (const member of members) {
    const candidates = [member.displayName, ...member.aliases].map((v) => v.toLowerCase())
    if (candidates.some((alias) => alias && lowered.includes(alias))) return member
  }
  return null
}

function hashAnswer(input: string, salt?: string): string {
  const normalized = normalize(input)
  const seeded = salt ? `${salt}:${normalized}` : normalized
  return createHash('sha256').update(seeded).digest('hex')
}

async function getSessionState(session: string): Promise<SessionAuthState> {
  try {
    const cache = getCache({ namespace: _CACHE_NAMESPACE })
    return (
      ((await cache.get(`session:${session}`)) as SessionAuthState | undefined) ?? {
        failedAttempts: 0,
      }
    )
  } catch {
    return _LOCAL_SESSIONS.get(session) ?? { failedAttempts: 0 }
  }
}

async function setSessionState(session: string, state: SessionAuthState): Promise<void> {
  try {
    const cache = getCache({ namespace: _CACHE_NAMESPACE })
    await cache.set(`session:${session}`, state, {
      ttl: _SESSION_TTL_SECONDS,
      tags: ['family-auth'],
    })
  } catch {
    _LOCAL_SESSIONS.set(session, state)
  }
}

async function getCorrections(memberId: string): Promise<string[]> {
  try {
    const cache = getCache({ namespace: _CACHE_NAMESPACE })
    return ((await cache.get(`corr:${memberId}`)) as string[] | undefined) ?? []
  } catch {
    return _LOCAL_CORRECTIONS.get(memberId) ?? []
  }
}

async function addCorrection(memberId: string, correction: string): Promise<void> {
  const existing = await getCorrections(memberId)
  const line = `[${new Date().toISOString()}] ${correction}`
  const next = [line, ...existing].slice(0, _CORRECTION_MAX)

  try {
    const cache = getCache({ namespace: _CACHE_NAMESPACE })
    await cache.set(`corr:${memberId}`, next, {
      ttl: _CORRECTION_TTL_SECONDS,
      tags: ['family-auth'],
    })
  } catch {
    _LOCAL_CORRECTIONS.set(memberId, next)
  }
}

function parseProfileUpdate(input: string): string | null {
  for (const re of _UPDATE_PREFIXES) {
    if (re.test(input)) {
      const stripped = input.replace(re, '').trim()
      return stripped.length > 0 ? stripped : null
    }
  }
  return null
}

export async function processFamilyAuthMessage(session: string, userInput: string): Promise<AuthResult> {
  const members = parseConfig()
  if (members.length === 0) return { handled: false }

  const state = await getSessionState(session)
  const normalizedInput = normalize(userInput)
  const verifiedMember =
    members.find((m) => m.id === state.verifiedMemberId) ?? undefined

  // 1) Handle challenge response if a challenge is pending
  if (state.pendingMemberId) {
    const pending = members.find((m) => m.id === state.pendingMemberId)
    if (!pending) {
      await setSessionState(session, { failedAttempts: 0 })
      return { handled: false }
    }

    const computed = hashAnswer(normalizedInput, pending.answerSalt)
    if (computed === pending.answerHash) {
      await setSessionState(session, {
        verifiedMemberId: pending.id,
        failedAttempts: 0,
      })
      const corrections = await getCorrections(pending.id)
      const correctionNote =
        corrections.length > 0
          ? ` I have ${corrections.length} saved correction${corrections.length === 1 ? '' : 's'} for your profile.`
          : ''
      return {
        handled: true,
        reply: `Identity verified: ${pending.displayName}. Profile edit mode unlocked for this session.${correctionNote} Use "Update my profile: <your correction>" to submit corrections.`,
        verifiedMember: pending,
        corrections,
      }
    }

    const failed = state.failedAttempts + 1
    if (failed >= 3) {
      await setSessionState(session, { failedAttempts: 0 })
      return {
        handled: true,
        reply:
          'Verification failed 3 times. Challenge reset. Please re-introduce yourself to start verification again.',
      }
    }

    await setSessionState(session, {
      ...state,
      failedAttempts: failed,
    })
    return {
      handled: true,
      reply: `Verification failed. Try again (${3 - failed} attempt${3 - failed === 1 ? '' : 's'} left).`,
    }
  }

  // 2) Start a challenge if user claims identity
  const claimed = findClaimedMember(userInput, members)
  if (claimed) {
    await setSessionState(session, {
      pendingMemberId: claimed.id,
      failedAttempts: 0,
      verifiedMemberId: state.verifiedMemberId,
    })
    return {
      handled: true,
      reply: `Verification required for ${claimed.displayName}. Security question: ${claimed.challengeQuestion}`,
    }
  }

  // 3) Accept profile corrections for verified members
  if (verifiedMember) {
    const correction = parseProfileUpdate(userInput)
    if (correction) {
      await addCorrection(verifiedMember.id, correction)
      const corrections = await getCorrections(verifiedMember.id)
      return {
        handled: true,
        reply: `Saved. Your correction has been recorded for ${verifiedMember.displayName}. Total saved corrections: ${corrections.length}.`,
        verifiedMember,
        corrections,
      }
    }

    return {
      handled: false,
      verifiedMember,
      corrections: await getCorrections(verifiedMember.id),
    }
  }

  return { handled: false }
}

export async function getVerifiedMemberContext(session: string): Promise<string | null> {
  const members = parseConfig()
  if (members.length === 0) return null

  const state = await getSessionState(session)
  if (!state.verifiedMemberId) return null

  const member = members.find((m) => m.id === state.verifiedMemberId)
  if (!member) return null

  const corrections = await getCorrections(member.id)
  const correctionBlock =
    corrections.length > 0
      ? `\nKnown profile corrections for ${member.displayName}:\n- ${corrections.join('\n- ')}`
      : ''

  return (
    `Verified family profile session for ${member.displayName} (${member.id}). ` +
    `Prioritize this member's profile corrections when discussing their personal details.` +
    correctionBlock
  )
}

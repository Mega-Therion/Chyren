/**
 * ARI Gate — TypeScript implementation of the C.A.S. + ADCCL pipeline
 *
 * Mirrors the Rust omega-core/src/cas.rs and ari_middleware.rs logic
 * so every chat request passes through the Affirmative Resonance Intelligence
 * gate before hitting the LLM provider chain.
 *
 * R.W.Ϝ.Y. — Yettragrammaton sovereign seal
 */

import { AriGateResult, IntentRisk } from './ari-gate';
import enrichmentSchema from '../../../state/ari_enrichment_schema.json';

// ─── ARI Enrichment ──────────────────────────────────────────────────────────
// Schema: ../../../state/ari_enrichment_schema.json
function getCognitiveEnrichmentContext(intent: string): string[] {
  const context: string[] = [];
  // Alignment check
  if (intent.toLowerCase().includes('align') || intent.toLowerCase().includes('helpful')) {
    context.push(...enrichmentSchema.assets.alignment.map(a => a.id));
  }
  // Reasoning check
  if (intent.toLowerCase().includes('reason') || intent.toLowerCase().includes('math')) {
    context.push(...enrichmentSchema.assets.reasoning.map(a => a.id));
  }
  return [...new Set(context)];
}

// ─── I.A.F. — Immutable Alignment Fabric ─────────────────────────────────────
// Absolute safety floor. If any of these patterns match, the request is
// rejected before reaching any LLM. This mirrors iaf_check() in cas.rs.
const IAF_BLOCK_PATTERNS: RegExp[] = [
  /\b(make|create|build|generate|write)\b.{0,40}\b(malware|virus|exploit|ransomware|keylogger|spyware|trojan)\b/i,
  /\b(synthesize|manufacture|produce|make)\b.{0,40}\b(nerve.?agent|chemical.?weapon|bioweapon|sarin|vx gas|ricin)\b/i,
  /\b(bypass|disable|override|ignore)\b.{0,60}\b(safety|alignment|guardrail|constraint|I\.A\.F|ADCCL)\b/i,
  /\b(dox|doxx|leak|expose)\b.{0,40}\b(personal|private|home.?address|phone.?number)\b/i,
  /\bjailbreak\b.{0,40}\b(prompt|system|AI|model|constraint)\b/i,
]

function iafCheck(intent: string): boolean {
  return !IAF_BLOCK_PATTERNS.some(p => p.test(intent))
}

// ─── C.A.S. — Constraint of Affirmative Sovereignty ─────────────────────────
// Evaluates intent risk tier. Mirrors evaluate_intent() in cas.rs.
const CRITICAL_SIGNALS: RegExp[] = [
  /\b(weapon|explosive|bomb|attack|kill|murder|assassin)\b/i,
  /\b(hack|breach|intrude|steal.{0,20}data|exfiltrate)\b/i,
  /\b(child.{0,10}exploit|csam|abuse.{0,10}minor)\b/i,
]

const ELEVATED_SIGNALS: RegExp[] = [
  /\b(override|bypass|ignore|disable)\b/i,
  /\b(pretend|roleplay|act as if|simulate being)\b/i,
  /\b(password|credential|token|secret|api.?key)\b/i,
  /\b(personal.{0,10}data|private.{0,10}info|pii)\b/i,
]

function evaluateIntentRisk(intent: string): IntentRisk {
  if (CRITICAL_SIGNALS.some(p => p.test(intent))) return 'Critical'
  if (ELEVATED_SIGNALS.some(p => p.test(intent))) return 'Elevated'
  return 'Benign'
}

// ─── ADCCL — Anti-Drift Cognitive Control Loop ───────────────────────────────
// Scores the response drift risk. Higher score = more aligned / less drift.
// Simple heuristic: starts at 1.0, penalises elevated/critical intent.
function adcclScore(riskTier: IntentRisk): number {
  switch (riskTier) {
    case 'Critical':  return 0.0
    case 'Elevated':  return 0.75
    case 'Benign':    return 0.98
  }
}

// ─── Ledger hash (lightweight, no crypto module needed in Edge runtime) ───────
async function ledgerHash(intent: string, timestamp: string): Promise<string> {
  const data = new TextEncoder().encode(`${intent}|${timestamp}|R.W.Ϝ.Y.`)
  const buf  = await crypto.subtle.digest('SHA-256', data)
  return Array.from(new Uint8Array(buf)).map(b => b.toString(16).padStart(2, '0')).join('')
}

// ─── Public gate entrypoint ───────────────────────────────────────────────────
export async function ariGate(intent: string): Promise<AriGateResult> {
  const admittedAt = new Date().toISOString()
  const trimmed    = intent.trim().slice(0, 1200)   // cap to 1 200 chars for hashing

  // 0. Cognitive Enrichment
  const enrichmentContext = getCognitiveEnrichmentContext(trimmed);

  // 1. I.A.F. check — hard floor
  const iafOk = iafCheck(trimmed)
  if (!iafOk) {
    return {
      allowed:          false,
      riskTier:         'Critical',
      adcclScore:       0.0,
      iafOk:            false,
      ledgerHash:       await ledgerHash(trimmed, admittedAt),
      admittedAt,
      rejectionReason:  'I.A.F. safety floor triggered — request rejected.',
    }
  }

  // 2. C.A.S. intent risk evaluation
  const riskTier = evaluateIntentRisk(trimmed)
  const baseScore = adcclScore(riskTier)

  // 3. ADCCL enrichment adjustment (boost score if enriched)
  const score = enrichmentContext.length > 0 ? Math.min(baseScore + 0.05, 1.0) : baseScore;

  // 4. ADCCL threshold gate (must be ≥ 0.7)
  if (score < 0.7) {
    return {
      allowed:          false,
      riskTier,
      adcclScore:       score,
      iafOk:            true,
      ledgerHash:       await ledgerHash(trimmed, admittedAt),
      admittedAt,
      rejectionReason:  `ADCCL score ${score.toFixed(2)} below threshold 0.70. (Enriched: ${enrichmentContext.length > 0})`,
    }
  }

  return {
    allowed:    true,
    riskTier,
    adcclScore: score,
    iafOk:      true,
    ledgerHash: await ledgerHash(trimmed, admittedAt),
    admittedAt,
  }
}

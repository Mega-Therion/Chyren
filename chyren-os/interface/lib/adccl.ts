/**
 * ADCCL: Anti-Drift Cognitive Control Loop (TypeScript Port)
 * 
 * Verifies provider responses for drift, hallucinations, and stubs.
 */

const STUB_PATTERNS = [
  /\bTODO\b/i,
  /\bFIXME\b/i,
  /\bPLACEHOLDER\b/i,
  /\[INSERT[^\]]*\]/i,
  /\[YOUR[^\]]*\]/i,
  /<YOUR[^>]*>/i,
  /\.{5,}/
];

const HALLUCINATION_ANCHORS = [
  /as of my (last|latest) (training|knowledge) (update|cutoff)/i,
  /I (don't|do not) have (access|the ability) to (browse|access|search)/i,
  /I (cannot|can't) (verify|confirm|check) (this|that|if)/i
];

const REFUSAL_PATTERNS = [
  /^I('m| am) (sorry|unable|not able)/i,
  /^(Sorry|Apologies),? (but )?I (can't|cannot|am unable)/i,
  /^I (don't|do not) have (the capability|access|the ability)/i
];

const MIN_LENGTH = 20;
const MAX_LENGTH = 32000;

export interface VerificationResult {
  passed: boolean;
  score: number;
  flags: string[];
}

export class ADCCL {
  private minScore: number;

  constructor(minScore: number = 0.7) {
    this.minScore = minScore;
  }

  public verify(responseText: string, task: string = ""): VerificationResult {
    const flags: string[] = [];
    const text = responseText.trim();

    if (!text) flags.push("EMPTY_RESPONSE");
    if (text.length < MIN_LENGTH) flags.push(`RESPONSE_TOO_SHORT (len=${text.length})`);
    if (responseText.length > MAX_LENGTH) flags.push(`RESPONSE_EXCEEDS_MAX_LENGTH (len=${responseText.length})`);

    if (STUB_PATTERNS.some(p => p.test(responseText))) flags.push("STUB_MARKERS_DETECTED");
    if (HALLUCINATION_ANCHORS.some(p => p.test(responseText))) flags.push("HALLUCINATION_ANCHORS");
    if (REFUSAL_PATTERNS.some(p => p.test(text)) && text.length < 150) flags.push("PURE_CAPABILITY_REFUSAL");

    if (task) {
      const taskWords = new Set(task.toLowerCase().match(/\b\w{5,}\b/g) || []);
      const responseWords = new Set(text.toLowerCase().match(/\b\w{5,}\b/g) || []);
      const overlap = [...taskWords].filter(w => responseWords.has(w));
      if (taskWords.size > 0 && overlap.length === 0) flags.push("NO_TASK_WORD_OVERLAP");
    }

    const checksTotal = 7;
    const checksPassed = checksTotal - flags.length;
    const score = Math.round(Math.max(0, checksPassed / checksTotal) * 100) / 100;
    
    const hardVetoPrefixes = ["EMPTY_RESPONSE", "STUB_MARKERS_DETECTED", "PURE_CAPABILITY_REFUSAL"];
    const passed = score >= this.minScore && !flags.some(f => hardVetoPrefixes.some(v => f.startsWith(v)));

    return { passed, score, flags };
  }
}

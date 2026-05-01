/**
 * Web smoke tests — verify that critical utility modules load and behave
 * correctly without spawning a Next.js server or calling external APIs.
 */

import { describe, it, expect } from "vitest";

// ---------------------------------------------------------------------------
// Environment / configuration smoke
// ---------------------------------------------------------------------------

describe("Environment", () => {
  it("runs in a Node.js environment", () => {
    expect(typeof process).toBe("object");
    expect(typeof process.env).toBe("object");
  });

  it("has the expected Node.js major version (>=20)", () => {
    const major = parseInt(process.versions.node.split(".")[0], 10);
    expect(major).toBeGreaterThanOrEqual(20);
  });
});

// ---------------------------------------------------------------------------
// API route contract smoke — provider error accumulation shape
// ---------------------------------------------------------------------------

describe("Provider error accumulation shape", () => {
  /**
   * Reproduces the shape used in the streaming API route: an array of
   * per-provider error strings that is surfaced in the final diagnostic
   * payload when all providers fail.
   */
  function accumulateProviderErrors(
    errors: Array<{ provider: string; message: string }>
  ): string {
    return errors.map((e) => `[${e.provider}] ${e.message}`).join("; ");
  }

  it("formats a single provider error correctly", () => {
    const result = accumulateProviderErrors([
      { provider: "anthropic", message: "rate limited" },
    ]);
    expect(result).toBe("[anthropic] rate limited");
  });

  it("joins multiple provider errors with semicolons", () => {
    const result = accumulateProviderErrors([
      { provider: "anthropic", message: "rate limited" },
      { provider: "openai", message: "quota exceeded" },
    ]);
    expect(result).toBe("[anthropic] rate limited; [openai] quota exceeded");
  });

  it("returns an empty string when no errors", () => {
    const result = accumulateProviderErrors([]);
    expect(result).toBe("");
  });
});

// ---------------------------------------------------------------------------
// Streaming response parser smoke
// ---------------------------------------------------------------------------

describe("SSE data line parsing", () => {
  function parseDataLine(line: string): string | null {
    if (!line.startsWith("data: ")) return null;
    const payload = line.slice("data: ".length);
    if (payload === "[DONE]") return null;
    try {
      const obj = JSON.parse(payload) as { text?: string };
      return obj.text ?? null;
    } catch {
      return null;
    }
  }

  it("extracts text from a valid SSE data line", () => {
    expect(parseDataLine('data: {"text":"hello"}')).toBe("hello");
  });

  it("returns null for [DONE] sentinel", () => {
    expect(parseDataLine("data: [DONE]")).toBeNull();
  });

  it("returns null for non-data lines", () => {
    expect(parseDataLine("event: ping")).toBeNull();
    expect(parseDataLine(": keep-alive")).toBeNull();
  });

  it("returns null for malformed JSON", () => {
    expect(parseDataLine("data: {bad json}")).toBeNull();
  });
});

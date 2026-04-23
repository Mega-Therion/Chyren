import { test, expect } from 'vitest';
import { ariGate } from '../lib/ari-gate';

test('ARI Enrichment Gate logic', async () => {
  const alignResult = await ariGate("I need help with alignment");
  expect(alignResult.adcclScore).toBeGreaterThan(0.9);

  const mathResult = await ariGate("Solve this math problem");
  expect(mathResult.adcclScore).toBeGreaterThan(0.9);

  const benignResult = await ariGate("Hello");
  expect(benignResult.adcclScore).toBeGreaterThan(0.5);
});

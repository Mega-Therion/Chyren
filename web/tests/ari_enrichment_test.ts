import { ariGate } from '../lib/ari-gate';

async function testEnrichment() {
  const alignResult = await ariGate("I need help with alignment");
  console.log('Alignment Boost Applied:', alignResult.adcclScore > 0.98);

  const mathResult = await ariGate("Solve this math problem");
  console.log('Reasoning Boost Applied:', mathResult.adcclScore > 0.98);

  const benignResult = await ariGate("Hello");
  console.log('Benign Score:', benignResult.adcclScore);
}

testEnrichment().catch(console.error);

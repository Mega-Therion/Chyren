import { streamText } from 'ai';
import { openai } from '@ai-sdk/openai';
import 'dotenv/config';

async function main() {
  const result = await streamText({
    model: openai('gpt-5.4'),
    prompt: 'Write a short poem about the future of AI.',
  });

  console.log('--- Streaming Response ---');
  for await (const textPart of result.textStream) {
    process.stdout.write(textPart);
  }
  console.log('\n--- Streaming Complete ---');

  const usage = await result.usage;
  console.log('Token Usage:', usage);
}

main().catch(console.error);

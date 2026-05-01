import { execSync } from 'child_process';
import { readFileSync } from 'fs';

const envFile = readFileSync('.env.production', 'utf8');
const lines = envFile.split('\n');

const keysToSync = [
  'ANTHROPIC_API_KEY',
  'GEMINI_API_KEY',
  'GEMINI_MODEL',
  'GROQ_API_KEY',
  'GROQ_MODEL',
  'OPENAI_API_KEY',
  'OPENAI_API_BASE',
  'OPENAI_MODEL',
  'CHYREN_API_URL',
  'DATABASE_URL',
  'OMEGA_DB_URL',
  'NEXT_PUBLIC_API_BASE_URL',
];

for (const line of lines) {
  const match = line.match(/^([^=]+)="(.+)"/);
  if (match) {
    const key = match[1].trim();
    let value = match[2].trim().replace(/\\n/g, ''); // Remove escaped newlines

    if (keysToSync.includes(key)) {
      console.log(`Syncing ${key}...`);
      try {
        // Try to remove first
        try {
          execSync(`vercel env rm ${key} production -y`, { stdio: 'ignore' });
        } catch { }
        
        // Add fresh
        execSync(`echo "${value}" | vercel env add ${key} production`, { stdio: 'inherit' });
      } catch (err) {
        console.error(`Failed to sync ${key}:`, err.message);
      }
    }
  }
}

console.log('Triggering redeploy...');
execSync('vercel --prod --force', { stdio: 'inherit' });

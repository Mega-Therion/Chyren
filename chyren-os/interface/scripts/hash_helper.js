const { createHash } = require('node:crypto');
const salt = 'alpha-omega-orchard-2026';
const answer = process.argv[2];
const normalized = answer.trim().toLowerCase().replace(/[^\p{L}\p{N}\s]/gu, ' ').replace(/\s+/g, ' ');
const hash = createHash('sha256').update(`${salt}:${normalized}`).digest('hex');
console.log('--- GENERATED HASH ---');
console.log('Salt:', salt);
console.log('Hash:', hash);

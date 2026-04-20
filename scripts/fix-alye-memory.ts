import { QdrantClient } from '@qdrant/js-client-rest';
import { embedText } from '../web/lib/librarian/knowledge-vector'; // path correction for scripts
import { Entity } from '../web/lib/schema/entity';

const QDRANT_URL = process.env.QDRANT_URL || 'http://localhost:6333';
const COLLECTION = 'knowledge_matrix';
const TARGET_SLUG = 'alye-lauren-muldoon';

async function main() {
  const client = new QdrantClient({ url: QDRANT_URL });

  console.log(`Fixing entity: ${TARGET_SLUG}...`);

  // 1. Build the correct Entity object (Canonical fact)
  const corrected: Entity = {
    id: TARGET_SLUG,
    name: 'Alye Lauren Muldoon',
    description: 'Travel Nurse (specializing in Labor & Delivery) based in Arkansas.',
    realm: 'people',
    kind: 'person',
    provenance: {
      createdAt: new Date().toISOString(),
      createdBy: 'R.W.Ϝ.Y.',
      version: 'v1',
    },
  };

  // 2. Embed the correct description
  console.log('Generating embedding...');
  const vector = await embedText(corrected.description);
  if (!vector) {
    throw new Error('Embedding failed. Check GEMINI_API_KEY / OPENAI_API_KEY.');
  }

  // 3. Find the existing point ID (stable_int_id logic)
  // We can just calculate it here since it's a stable hash
  const crypto = require('crypto');
  const h = crypto.createHash('sha256').update(TARGET_SLUG).digest();
  const pointId = parseInt(h.slice(0, 8).toString('hex'), 16) & 0x7FFFFFFFFFFFFFFF;

  // 4. Upsert into Qdrant
  console.log(`Upserting point ${pointId} into collection ${COLLECTION}...`);
  await client.upsert(COLLECTION, {
    points: [{
      id: pointId,
      vector: vector,
      payload: corrected as any,
    }],
  });

  console.log('✅ Alye Lauren Muldoon memory fixed in Qdrant.');
}

main().catch(console.error);

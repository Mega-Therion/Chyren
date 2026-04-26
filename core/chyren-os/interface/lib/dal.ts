import { neon } from '@neondatabase/serverless';
import { withSpan } from './telemetry';

const sql = neon(process.env.CHYREN_DB_URL || '');

export const dataAccess = {
  getMemories: async (limit = 20) => {
    return await withSpan('dal.getMemories', async (_span) => {
      _span.setAttribute('db.query', 'SELECT content, topic, created_at FROM memories');
      return await sql`
        SELECT content, topic, created_at
        FROM memories
        ORDER BY created_at DESC
        LIMIT ${limit}
      `;
    });
  },

  getPublicKnowledge: async (limit = 15) => {
    return await withSpan('dal.getPublicKnowledge', async (_span) => {
      return await sql`
        SELECT title, content, category, importance
        FROM public_knowledge
        WHERE category IN ('biography', 'creator', 'concept', 'quote')
        ORDER BY importance DESC NULLS LAST
        LIMIT ${limit}
      `;
    });
  },

  getFamilyProfiles: async () => {
    return await withSpan('dal.getFamilyProfiles', async (_span) => {
      return await sql`
        SELECT name, last_name, relationship, location, birthday, deceased,
               occupation, partner, children, ry_notes, notes_for_chyren, how_to_greet
        FROM family_profiles 
        ORDER BY id
      `;
    });
  }
};

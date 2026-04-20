/**
 * Unified Entity Schema for the Chyren Knowledge Matrix.
 * Bridges Python ingestion, Postgres cataloging, and Qdrant retrieval.
 */

export interface Provenance {
  createdAt: string;
  createdBy: string;
  version: string;
  sourceUrl?: string;
}

export interface Entity {
  id: string; // Slug (e.g. 'alye-lauren-muldoon')
  name: string;
  description: string;
  realm: 'sovereign' | 'people' | 'external';
  kind: string; // 'person', 'place', 'concept', 'event', 'dataset'
  
  // Metadata
  tags?: string[];
  importance?: number; // 0.0 to 1.0
  
  // Provenance
  provenance: Provenance;

  // Move 5: Multi-Modal Scaffolding
  media?: {
    url: string;
    type: 'image' | 'audio' | 'video' | 'pdf';
    altText?: string;
    ocrText?: string; // Transcribed text for search
  }[];
}

/**
 * Type guard to validate ingestion rows.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function isEntity(obj: any): obj is Entity {
  return (
    typeof obj?.id === 'string' &&
    typeof obj?.name === 'string' &&
    typeof obj?.description === 'string' &&
    ['sovereign', 'people', 'external'].includes(obj?.realm) &&
    typeof obj?.provenance?.createdAt === 'string'
  );
}

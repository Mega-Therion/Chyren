import type { Entity } from './entity';

export function standardize(raw: Record<string, unknown>): Entity {
  return {
    id: (raw.id ?? raw.program_id) as string,
    name: (raw.name as string) || 'Unnamed Entity',
    description: (raw.description as string) || '',
    realm: 'external',
    kind: 'dataset',
    provenance: {
      createdAt: new Date().toISOString(),
      createdBy: 'cdl-standardizer',
      version: '1.0.0'
    }
  } as Entity;
}

import type { Entity } from './entity';
export function standardize(raw: any): Entity {
  return {
    id: raw.id || raw.program_id,
    name: raw.name || 'Unnamed Entity',
    description: raw.description || '',
    realm: 'external',
    kind: 'dataset',
    provenance: {
      createdAt: new Date().toISOString(),
      createdBy: 'cdl-standardizer',
      version: '1.0.0'
    }
  } as Entity;
}

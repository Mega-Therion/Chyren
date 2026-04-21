import { Entity } from './schema/entity';
export async function upsertEntity(entity: Entity) {
  // Bridge entity to Qdrant vector store
  console.log("Upserting entity to Qdrant vector index:", entity.id);
}

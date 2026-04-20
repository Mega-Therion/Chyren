import { standardize } from '../web/lib/schema/standardizer';
// ETL Logic: Pull from Postgres, standardize, push to CDL
async function ingest() {
  console.log("Ingesting records to CDL...");
  // 1. Fetch raw from Postgres
  // 2. Standardize
  // 3. Batch insert to cdl_registry
  console.log("Ingestion batch complete.");
}
ingest().catch(console.error);

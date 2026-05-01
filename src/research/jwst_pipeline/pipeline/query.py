import os
import logging
from astroquery.mast import Observations
from astroquery.irsa import Irsa
from dotenv import load_dotenv

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

# Targeted programs for GOD Theory evidence
PROGRAM_IDS = ["Spitzer_13063", 1345, 4106, 1180, 1210, 1324, 2079, "HST_12444"]

def get_processed_obs_ids():
    results_dir = "results"
    if not os.path.exists(results_dir):
        return set()
    return {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}

def query_and_download(program_ids):
    # Increase timeout for large queries
    Observations.TIMEOUT = 5000
    
    # Set MAST token if available
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    raw_data_dir = "data/raw"
    os.makedirs(raw_data_dir, exist_ok=True)
    processed_obs = get_processed_obs_ids()
    logger.info(f"Detected {len(processed_obs)} already processed observations. Skipping these.")

    for prog_id in program_ids:
        # Check if prog_id is a list (for HST/Spitzer) or single (for JWST)
        project = "JWST"
        if str(prog_id).startswith("HST"):
            project = "HST"
            pid = str(prog_id).split("_")[1]
        elif str(prog_id).startswith("Spitzer"):
            project = "Spitzer"
            pid = str(prog_id).split("_")[1]
        else:
            pid = prog_id

        logger.info(f"Querying MAST for {project} Program ID: {pid}")
        
        # Query observations
        if project == "Spitzer":
            logger.info(f"Querying MAST for Spitzer")
            obs_table = Observations.query_criteria(project="Spitzer")
        else:
            obs_table = Observations.query_criteria(proposal_id=pid, project=project)
        
        if len(obs_table) == 0:
            # Fallback for Spitzer if proposal_id is too specific
            if project == "Spitzer":
                logger.info(f"No results for Spitzer Program {pid}. Searching Galactic Center...")
                obs_table = Observations.query_criteria(obs_collection="SPITZER_SHA", objectname="Galactic Center")
            
            if len(obs_table) == 0:
                logger.warning(f"No observations found for {project} Program ID: {pid}")
                continue
            
        logger.info(f"Found {len(obs_table)} observations for {project} Program ID: {pid}. Limiting to top 1000.")
        obs_table = obs_table[:1000]
        
        # SHARDED QUERY: Process in batches of 50 to avoid server timeouts
        all_filtered_products = []
        batch_size = 50
        for i in range(0, len(obs_table), batch_size):
            shard = obs_table[i:i+batch_size]
            logger.info(f"Processing shard {i//batch_size + 1}: {i} to {i+batch_size}...")
            
            try:
                data_products = Observations.get_product_list(shard)
                
                # Subgroup description mapping
                if project == "JWST":
                    subgroups = ["X1D"]
                elif project == "HST":
                    subgroups = ["X1D", "SX1"]
                else: # Spitzer
                    subgroups = ["SPECRES", "COADD", "BCD"]
                
                if project == "Spitzer":
                    filtered = Observations.filter_products(data_products, 
                                                                    productType="SCIENCE",
                                                                    extension="fits")
                    # Strict filter for science flux arrays only to save disk
                    filtered = [p for p in filtered if 'sci' in p['productFilename'].lower() or 'drz' in p['productFilename'].lower()]
                else:
                    filtered = Observations.filter_products(data_products, 
                                                                    productSubGroupDescription=subgroups,
                                                                    extension="fits")
                
                if len(filtered) == 0 and project != "Spitzer":
                    fallback = ["CAL"] if project == "JWST" else ["FLT"]
                    filtered = Observations.filter_products(data_products, 
                                                                    productSubGroupDescription=fallback,
                                                                    extension="fits")
                
                # Filter out already processed products
                filtered = [p for p in filtered if p['productFilename'].replace(".fits", "") not in processed_obs]
                
                all_filtered_products.extend(filtered)
                
                # Early exit for JWST/HST if we have enough
                if project != "Spitzer" and len(all_filtered_products) >= 150:
                    break
                # Early exit for Spitzer if we have enough
                if project == "Spitzer" and len(all_filtered_products) >= 150:
                    break
                    
            except Exception as e:
                logger.error(f"Error processing shard {i//batch_size + 1}: {e}")
                continue

        if len(all_filtered_products) > 0:
            prog_dir = os.path.join(raw_data_dir, f"{project}_{pid}")
            os.makedirs(prog_dir, exist_ok=True)
            
            limit = 111 if project == "Spitzer" else 111
            to_download = all_filtered_products[:limit]
            
            logger.info(f"Downloading {len(to_download)} products for {project}...")
            
            if project == "Spitzer":
                urls_to_download = []
                for prod in to_download:
                    uri = prod['dataURI']
                    url = f"https://mast.stsci.edu/api/v0.1/Download/file?uri={uri}"
                    fname = prod['productFilename']
                    local_path = os.path.join(prog_dir, fname)
                    if not os.path.exists(local_path) or os.path.getsize(local_path) == 0:
                        urls_to_download.append((url, local_path))
                
                if urls_to_download:
                    logger.info(f"Downloading {len(urls_to_download)} products for Spitzer in batches of 10...")
                    from ingest import ingest_data
                    
                    batch_size = 10
                    for i in range(0, len(urls_to_download), batch_size):
                        batch = urls_to_download[i:i+batch_size]
                        logger.info(f"Processing Spitzer batch {i//batch_size + 1}...")
                        
                        # Create a temporary file for this batch
                        temp_file = os.path.join(prog_dir, "download_list.txt")
                        with open(temp_file, "w") as f:
                            for url, path in batch:
                                f.write(f"'{url}' -O '{path}'\n")
                        
                        # Run wget in parallel using xargs with retries and timeout
                        # -t 5: retry 5 times, -T 30: 30s timeout, --show-progress: for logs
                        os.system(f"cat '{temp_file}' | xargs -n 3 -P 4 wget -q -c -t 5 -T 60")
                        os.remove(temp_file)
                        
                        # Run ingestion to process and purge the raw files immediately
                        logger.info(f"Triggering ingestion for batch {i//batch_size + 1}...")
                        try:
                            ingest_data()
                        except Exception as e:
                            logger.error(f"Ingestion failed for batch {i//batch_size + 1}: {e}")
                    
                logger.info(f"Batch-and-Purge download loop finished for Spitzer")
            else:
                try:
                    Observations.download_products(to_download, download_dir=prog_dir)
                except Exception as e:
                    logger.error(f"Failed to download products for {project}: {e}")
        else:
            logger.warning(f"No products found to download for {project}")

if __name__ == "__main__":
    # Final 333-Source Trinity Push: 111 from each telescope (JWST, Hubble, Spitzer)
    PROGRAM_IDS = ["Spitzer_13063", 1345, 4106, 1180, 1210, 1324, 2079, "HST_12444"]
    query_and_download(PROGRAM_IDS)

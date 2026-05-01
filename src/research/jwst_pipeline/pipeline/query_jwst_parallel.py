import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

# Supplemental JWST programs for fast signal acquisition
SUPPLEMENTAL_PIDS = [1934, 2738, 1837, 2123]

def get_total_signals():
    results_dir = "results"
    if not os.path.exists(results_dir):
        return 0
    return len([f for f in os.listdir(results_dir) if f.endswith("_raw.npz")])

def query_and_download_jwst(program_ids):
    target = 333
    current = get_total_signals()
    if current >= target:
        logger.info(f"Target of {target} signals already reached ({current}). Exiting.")
        return

    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    raw_data_dir = "data/raw"
    os.makedirs(raw_data_dir, exist_ok=True)
    
    results_dir = "results"
    processed_obs = {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}

    for pid in program_ids:
        if get_total_signals() >= target:
            break
            
        logger.info(f"Parallel JWST Query for PID: {pid}")
        obs_table = Observations.query_criteria(proposal_id=pid, project="JWST")
        
        if len(obs_table) == 0:
            continue
            
        data_products = Observations.get_product_list(obs_table[:20]) # Limit to first 20 observations for speed
        filtered = Observations.filter_products(data_products, 
                                               productSubGroupDescription=["X1D"],
                                               extension="fits")
        
        # Skip processed
        to_download = [p for p in filtered if p['productFilename'].replace(".fits", "") not in processed_obs]
        
        if not to_download:
            continue
            
        logger.info(f"Downloading {len(to_download)} JWST products for PID {pid}...")
        prog_dir = os.path.join(raw_data_dir, f"JWST_{pid}")
        os.makedirs(prog_dir, exist_ok=True)
        
        try:
            Observations.download_products(to_download[:50], download_dir=prog_dir)
            
            # Trigger ingestion
            from ingest import ingest_data
            logger.info(f"Triggering ingestion for JWST PID {pid}...")
            ingest_data()
        except Exception as e:
            logger.error(f"Error in parallel JWST task: {e}")

if __name__ == "__main__":
    query_and_download_jwst(SUPPLEMENTAL_PIDS)

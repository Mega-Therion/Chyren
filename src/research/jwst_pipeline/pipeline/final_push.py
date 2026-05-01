import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def fast_track_jwst():
    # Final push for the remaining signals
    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    results_dir = "results"
    processed_obs = {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}
    
    target = 333
    current = len(processed_obs)
    needed = target - current + 5 # Buffer
    
    if needed <= 0:
        logger.info("Target already reached.")
        return

    logger.info(f"Fast-tracking {needed} JWST signals...")
    
    # Use a reliable deep field: GOODS-S (Program 1210 or 1180)
    obs_table = Observations.query_criteria(proposal_id=1210, project="JWST")
    data_products = Observations.get_product_list(obs_table[:needed])
    filtered = Observations.filter_products(data_products, 
                                           productSubGroupDescription=["X1D"],
                                           extension="fits")
    
    to_download = []
    for p in filtered:
        obs_id = p['productFilename'].replace(".fits", "")
        if obs_id not in processed_obs:
            to_download.append(p)
        if len(to_download) >= needed:
            break
            
    if not to_download:
        logger.warning("No new JWST products found.")
        return
        
    logger.info(f"Downloading {len(to_download)} JWST products...")
    raw_dir = "data/raw/JWST_FINAL_PUSH"
    os.makedirs(raw_dir, exist_ok=True)
    
    Observations.download_products(to_download, download_dir=raw_dir)
    
    # Run ingestion
    from ingest import ingest_data
    ingest_data()
    
    # Run final report
    from test_equations import batch_process
    batch_process()

if __name__ == "__main__":
    fast_track_jwst()

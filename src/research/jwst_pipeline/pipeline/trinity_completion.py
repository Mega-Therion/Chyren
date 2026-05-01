import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def trinity_completion():
    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    results_dir = "results"
    processed_obs = {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}
    
    current = len(processed_obs)
    target = 333
    needed = target - current
    
    if needed <= 0:
        logger.info(f"Target of {target} signals already reached ({current}).")
        return

    logger.info(f"Trinity Completion: {current}/333. Need {needed} more signals.")
    
    # Use Hubble PID 12444 (hundreds of images/spectra)
    # We want unique signals.
    obs = Observations.query_criteria(proposal_id=12444, project="HST")
    # Get products for a larger batch to find unique ones
    data_products = Observations.get_product_list(obs[:100])
    filtered = Observations.filter_products(data_products, 
                                           productSubGroupDescription=["FLT", "SX1"],
                                           extension="fits")
    
    urls_to_get = []
    seen_ids = set()
    for p in filtered:
        obs_id = p['productFilename'].replace(".fits", "")
        if obs_id not in processed_obs and obs_id not in seen_ids:
            uri = p['dataURI']
            url = f"https://mast.stsci.edu/api/v0.1/Download/file?uri={uri}"
            local_path = os.path.join("data/raw/COMPLETION", p['productFilename'])
            urls_to_get.append((url, local_path))
            seen_ids.add(obs_id)
        if len(urls_to_get) >= needed + 10: # Extra buffer
            break
            
    if not urls_to_get:
        logger.warning("No new unique products found.")
        return
        
    os.makedirs("data/raw/COMPLETION/SUBS", exist_ok=True)
    
    temp_file = "data/raw/COMPLETION/list.txt"
    with open(temp_file, "w") as f:
        for url, path in urls_to_get:
            # Fix: put files in a sub-sub directory so ingest.py sees them as program files
            sub_path = path.replace("data/raw/COMPLETION/", "data/raw/COMPLETION/SUBS/")
            f.write(f"'{url}' -O '{sub_path}'\n")
            
    logger.info(f"Downloading {len(urls_to_get)} unique signals...")
    os.system(f"cat '{temp_file}' | xargs -n 3 -P 8 wget -q -c")
    
    # Final Ingestion
    from ingest import ingest_data
    os.environ["RAW_DIR"] = "data/raw/COMPLETION"
    ingest_data()
    
    # Final Report
    from test_equations import batch_process
    batch_process()

if __name__ == "__main__":
    trinity_completion()

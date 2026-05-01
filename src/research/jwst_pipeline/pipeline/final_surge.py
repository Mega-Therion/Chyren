import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def final_surge():
    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    results_dir = "results"
    processed_obs = {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}
    
    # Target 333
    needed = 333 - len(processed_obs) + 10
    logger.info(f"Final Surge: Need {needed} signals.")
    
    if needed <= 0:
        logger.info("Target reached.")
        return

    # Program 1324 (GLASS) is very reliable
    obs = Observations.query_criteria(proposal_id=1324, project="JWST")
    products = Observations.get_product_list(obs[:needed*2])
    filtered = Observations.filter_products(products, 
                                           productSubGroupDescription=["X1D"],
                                           extension="fits")
    
    urls_to_get = []
    for p in filtered:
        obs_id = p['productFilename'].replace(".fits", "")
        if obs_id not in processed_obs:
            uri = p['dataURI']
            url = f"https://mast.stsci.edu/api/v0.1/Download/file?uri={uri}"
            local_path = os.path.join("data/raw/JWST_SURGE", p['productFilename'])
            urls_to_get.append((url, local_path))
        if len(urls_to_get) >= needed:
            break
            
    if not urls_to_get:
        logger.warning("No new JWST products found.")
        return
        
    os.makedirs("data/raw/JWST_SURGE", exist_ok=True)
    
    temp_file = "data/raw/JWST_SURGE/surge_list.txt"
    with open(temp_file, "w") as f:
        for url, path in urls_to_get:
            f.write(f"'{url}' -O '{path}'\n")
            
    logger.info(f"Downloading {len(urls_to_get)} signals via wget...")
    os.system(f"cat '{temp_file}' | xargs -n 3 -P 8 wget -q -c")
    
    # Ingest
    from ingest import ingest_data
    ingest_data()
    
    # Summary
    from test_equations import batch_process
    batch_process()

if __name__ == "__main__":
    final_surge()

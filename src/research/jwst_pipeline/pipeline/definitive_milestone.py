import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def definitive_milestone():
    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    results_dir = "results"
    processed_obs = {f.replace("_raw.npz", "") for f in os.listdir(results_dir) if f.endswith("_raw.npz")}
    
    current = len(processed_obs)
    target = 333
    needed = target - current + 15 
    
    logger.info(f"Definitive Milestone: {current}/{target}. Need {needed} more.")
    
    if needed <= 0:
        logger.info("Target reached.")
        return

    logger.info("Querying Galactic Center (JWST)...")
    obs = Observations.query_criteria(objectname='Galactic Center', project="JWST")
    data_products = Observations.get_product_list(obs[:needed*3])
    filtered = Observations.filter_products(data_products, 
                                           productSubGroupDescription=["X1D"],
                                           extension="fits")
    
    urls_to_get = []
    seen_ids = set()
    for p in filtered:
        obs_id = p['productFilename'].replace(".fits", "")
        if obs_id not in processed_obs and obs_id not in seen_ids:
            uri = p['dataURI']
            url = f"https://mast.stsci.edu/api/v0.1/Download/file?uri={uri}"
            local_path = os.path.join("data/raw/GALCENTER/SAMPLES", p['productFilename'])
            urls_to_get.append((url, local_path))
            seen_ids.add(obs_id)
        if len(urls_to_get) >= needed:
            break
            
    if not urls_to_get:
        logger.error("No new products found.")
        return
        
    os.makedirs("data/raw/GALCENTER/SAMPLES", exist_ok=True)
    
    temp_file = "data/raw/GALCENTER/list.txt"
    with open(temp_file, "w") as f:
        for url, path in urls_to_get:
            f.write(f"'{url}' -O '{path}'\n")
            
    logger.info(f"Downloading {len(urls_to_get)} signals via wget...")
    os.system(f"cat '{temp_file}' | xargs -n 3 -P 8 wget -q -c")
    
    # Ingest
    from ingest import ingest_data
    os.environ["RAW_DIR"] = "data/raw/GALCENTER"
    ingest_data()
    
    # Report
    from test_equations import batch_process
    batch_process()

if __name__ == "__main__":
    definitive_milestone()

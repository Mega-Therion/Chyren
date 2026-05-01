import os
import logging
from astroquery.mast import Observations
from dotenv import load_dotenv

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def ultimate_surge():
    Observations.TIMEOUT = 5000
    token = os.getenv("MAST_API_TOKEN")
    if token:
        Observations.login(token=token)
    
    results_dir = "results"
    
    # We want 100 FRESH signals
    pids = [1324, 1210, 2731, 2282, 12444, 1345]
    
    urls_to_get = []
    seen_files = set()
    
    for pid in pids:
        if len(urls_to_get) >= 100:
            break
        try:
            logger.info(f"Querying PID: {pid}")
            obs = Observations.query_criteria(proposal_id=pid, project="JWST")
            if len(obs) == 0:
                obs = Observations.query_criteria(proposal_id=pid, project="HST")
            
            data_products = Observations.get_product_list(obs[:20])
            filtered = Observations.filter_products(data_products, 
                                                   productSubGroupDescription=["X1D", "FLT"],
                                                   extension="fits")
            
            for p in filtered:
                filename = p['productFilename']
                if filename not in seen_files:
                    uri = p['dataURI']
                    url = f"https://mast.stsci.edu/api/v0.1/Download/file?uri={uri}"
                    local_path = os.path.join(f"data/raw/ULTIMATE/{pid}", filename)
                    urls_to_get.append((url, local_path))
                    seen_files.add(filename)
                if len(urls_to_get) >= 100:
                    break
        except Exception as e:
            logger.error(f"Error querying {pid}: {e}")

    if not urls_to_get:
        logger.error("No signals found.")
        return
        
    for url, path in urls_to_get:
        os.makedirs(os.path.dirname(path), exist_ok=True)
    
    temp_file = "data/raw/ULTIMATE/list.txt"
    with open(temp_file, "w") as f:
        for url, path in urls_to_get:
            f.write(f"'{url}' -O '{path}'\n")
            
    logger.info(f"Downloading 100 signals...")
    os.system(f"cat '{temp_file}' | xargs -n 3 -P 8 wget -q -c")
    
    # Ingest
    from ingest import ingest_data
    os.environ["RAW_DIR"] = "data/raw/ULTIMATE"
    ingest_data()
    
    # Report
    from test_equations import batch_process
    batch_process()

if __name__ == "__main__":
    ultimate_surge()

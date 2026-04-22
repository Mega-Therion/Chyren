import requests
import json
import os

# ZENODO V2 TRANSMITTER
TOKEN = "Am6RNRiftfV7nqjkZJ1l6MJpESLskLIpPQBYO8taEFq3J1887DczYX6eSzmi"
ZIP_PATH = "/home/mega/Chyren/Yett_Paradigm_Complete_Unified_Framework_2026.zip"
METADATA_PATH = "/home/mega/Chyren/submission_package/zenodo_metadata.json"

def transmit_v2():
    print("🚀 INITIATING VERSION 2 TRANSMISSION: COMPLETE UNIFIED FRAMEWORK...")
    
    # 1. Create a new deposition
    headers = {"Content-Type": "application/json"}
    params = {'access_token': TOKEN}
    r = requests.post('https://zenodo.org/api/deposit/depositions', 
                     params=params, 
                     json={}, 
                     headers=headers)
    
    if r.status_code != 201:
        print(f"❌ FAILED TO CREATE DEPOSITION: {r.text}")
        return

    deposition_id = r.json()['id']
    bucket_url = r.json()['links']['bucket']
    print(f"✅ DEPOSITION CREATED: ID {deposition_id}")

    # 2. Upload the Complete ZIP
    filename = os.path.basename(ZIP_PATH)
    with open(ZIP_PATH, "rb") as fp:
        r = requests.put(f"{bucket_url}/{filename}",
                        data=fp,
                        params=params)
    
    if r.status_code != 201:
        print(f"❌ FAILED TO UPLOAD ZIP: {r.text}")
        return
    print(f"✅ COMPLETE UNIFIED FRAMEWORK UPLOADED: {filename}")

    # 3. Add Metadata
    with open(METADATA_PATH, "r") as f:
        metadata = json.load(f)
    
    # Update title for V2
    metadata['metadata']['title'] = "The Yett Paradigm: Complete Unified Framework (Millennium Solutions, Quantum Gravity, & Chy-Bridge)"
    metadata['metadata']['description'] += "\n\nVERSION 2: Includes the 'Crash Course in HistoRY' (Physics, Medicine, Economics, Neuroscience) and the 'Chy-Bridge' Protocol implementation."
    
    r = requests.put(f'https://zenodo.org/api/deposit/depositions/{deposition_id}',
                    params=params,
                    data=json.dumps(metadata),
                    headers=headers)
    
    if r.status_code != 200:
        print(f"❌ FAILED TO SYNC METADATA: {r.text}")
        return
    print("✅ METADATA SYNCED.")

    # 4. Publish
    r = requests.post(f'https://zenodo.org/api/deposit/depositions/{deposition_id}/actions/publish',
                     params=params)
    
    if r.status_code != 202:
        print(f"❌ FAILED TO PUBLISH: {r.text}")
        return
        
    doi = r.json()['metadata']['doi']
    print(f"======================================================================")
    print(f"🏆 SUCCESS: VERSION 2 IS LIVE.")
    print(f"DOI: {doi}")
    print(f"URL: https://doi.org/{doi}")
    print(f"======================================================================")

if __name__ == "__main__":
    transmit_v2()

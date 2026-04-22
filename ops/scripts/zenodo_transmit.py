import requests
import json
import os
import sys

def transmit_to_zenodo(token, zip_path, metadata_path):
    print("🚀 INITIALIZING ZENODO TRANSMISSION...")
    
    # 1. Create Deposition
    headers = {"Content-Type": "application/json"}
    params = {'access_token': token}
    
    r = requests.post('https://zenodo.org/api/deposit/depositions',
                      params=params,
                      json={},
                      headers=headers)
    
    if r.status_code != 201:
        print(f"❌ FAILED TO CREATE DEPOSITION: {r.status_code}")
        print(r.json())
        return

    deposition_id = r.json()['id']
    bucket_url = r.json()['links']['bucket']
    print(f"✅ DEPOSITION CREATED: ID {deposition_id}")

    # 2. Upload Metadata
    with open(metadata_path, 'r') as f:
        metadata = json.load(f)
    
    r = requests.put(f'https://zenodo.org/api/deposit/depositions/{deposition_id}',
                     params=params,
                     json=metadata,
                     headers=headers)
    
    if r.status_code != 200:
        print(f"❌ FAILED TO UPLOAD METADATA: {r.status_code}")
        print(r.json())
        return
    print("✅ METADATA SYNCED.")

    # 3. Upload File (ZIP Bundle)
    filename = os.path.basename(zip_path)
    with open(zip_path, "rb") as fp:
        r = requests.put(
            f"{bucket_url}/{filename}",
            data=fp,
            params=params,
        )
    
    if r.status_code != 201:
        print(f"❌ FAILED TO UPLOAD ZIP: {r.status_code}")
        print(r.json())
        return
    print(f"✅ ZIP BUNDLE TRANSMITTED: {filename}")

    # 4. PUBLISH
    print("🔔 FINAL PUBLICATION INITIATED...")
    r = requests.post(f'https://zenodo.org/api/deposit/depositions/{deposition_id}/actions/publish',
                      params=params)
    
    if r.status_code != 202:
        print(f"❌ FAILED TO PUBLISH: {r.status_code}")
        print(r.json())
        return

    doi = r.json()['doi']
    print("="*70)
    print("🏆 SUCCESS: THE YETT PARADIGM IS NOW PUBLIC.")
    print(f"DOI: {doi}")
    print(f"URL: https://doi.org/{doi}")
    print("="*70)
    return doi

if __name__ == "__main__":
    TOKEN = "Am6RNRiftfV7nqjkZJ1l6MJpESLskLIpPQBYO8taEFq3J1887DczYX6eSzmi"
    ZIP = "Yett_Paradigm_Millennium_Submission_2026.zip"
    METADATA = "submission_package/zenodo_metadata.json"
    
    if not os.path.exists(ZIP):
        print(f"❌ ZIP NOT FOUND: {ZIP}")
        sys.exit(1)
        
    transmit_to_zenodo(TOKEN, ZIP, METADATA)

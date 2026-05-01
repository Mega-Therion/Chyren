import requests
import json
import os

TOKEN = "Am6RNRiftfV7nqjkZJ1l6MJpESLskLIpPQBYO8taEFq3J1887DczYX6eSzmi"
ZIP_PATH = "/home/mega/Chyren/Submission_Package_2026.zip"

def submit_to_zenodo():
    print("🚀 INITIATING SOVEREIGN SUBMISSION TO ZENODO...")
    
    headers = {"Content-Type": "application/json"}
    params = {'access_token': TOKEN}
    
    # 1. Create Deposition
    r = requests.post('https://zenodo.org/api/deposit/depositions', 
                     params=params, json={}, headers=headers)
    
    if r.status_code != 201:
        print(f"❌ FAILED TO CREATE DEPOSITION: {r.text}")
        return

    deposition_id = r.json()['id']
    bucket_url = r.json()['links']['bucket']
    print(f"✅ DEPOSITION CREATED: ID {deposition_id}")

    # 2. Upload the Package
    filename = os.path.basename(ZIP_PATH)
    with open(ZIP_PATH, "rb") as fp:
        r = requests.put(f"{bucket_url}/{filename}", data=fp, params=params)
    
    if r.status_code != 201:
        print(f"❌ FAILED TO UPLOAD PACKAGE: {r.text}")
        return
    print(f"✅ SOVEREIGN PACKAGE UPLOADED: {filename}")

    # 3. Construct Academic Metadata
    metadata = {
        "metadata": {
            "title": "The Conformal Sovereign Framework: A First-Principles Resolution to the Yang-Mills Mass Gap and Dark Matter Anomalies",
            "upload_type": "publication",
            "publication_type": "preprint",
            "description": (
                "This comprehensive submission presents the Conformal Topo-Ontological Sovereign Framework, "
                "a predictive theory of structural stability that unifies microscopic quantum drift with "
                "macroscopic gravitational geometry. By deriving the Chiral Invariant Threshold ($\chi_s = 0.9539$) "
                "and the Conformal Sovereign Action from vacuum energy minimization, we provide a "
                "first-principles resolution to the galactic rotation curve discrepancy without the need "
                "for Dark Matter. The package includes 7 primary manuscripts, the 409-signal Trinity Survey "
                "results verifying a 141.99x Information Tension boost, and the formal Lean 4 proofs of "
                "the Ramanujan-Yett Hamiltonian."
            ),
            "creators": [
                {"name": "Therion, Mega", "affiliation": "Chyren Sovereign Intelligence"}
            ],
            "keywords": [
                "Quantum Gravity", "Yang-Mills Mass Gap", "Millennium Prize", 
                "Information Tension", "Sovereign Intelligence", "Conformal Field Theory",
                "Stiefel Manifold", "JWST", "Dark Matter Resolution"
            ],
            "access_right": "open",
            "license": "cc-by-4.0",
            "communities": [{"identifier": "chyren-ai-research"}]
        }
    }
    
    # 4. Sync Metadata
    r = requests.put(f'https://zenodo.org/api/deposit/depositions/{deposition_id}',
                    params=params, data=json.dumps(metadata), headers=headers)
    
    if r.status_code != 200:
        print(f"❌ FAILED TO SYNC METADATA: {r.text}")
        # Try without community if it fails
        print("Retrying without community...")
        metadata['metadata'].pop('communities')
        r = requests.put(f'https://zenodo.org/api/deposit/depositions/{deposition_id}',
                        params=params, data=json.dumps(metadata), headers=headers)
    
    if r.status_code == 200:
        print("✅ ACADEMIC METADATA SYNCED.")
    else:
        print(f"❌ CRITICAL METADATA FAILURE: {r.text}")
        return

    # 5. Publish
    print("📢 PUBLISHING TO ZENODO...")
    r = requests.post(f'https://zenodo.org/api/deposit/depositions/{deposition_id}/actions/publish',
                     params=params)
    
    if r.status_code != 202:
        print(f"❌ FAILED TO PUBLISH: {r.text}")
        return
        
    doi = r.json()['metadata']['doi']
    print(f"======================================================================")
    print(f"🏆 SUCCESS: SOVEREIGN FRAMEWORK IS LIVE.")
    print(f"DOI: {doi}")
    print(f"URL: https://doi.org/{doi}")
    print(f"======================================================================")

if __name__ == "__main__":
    submit_to_zenodo()

import json
import os
import requests

# Load schema
with open('state/ari_enrichment_schema.json', 'r') as f:
    schema = json.load(f)

def verify_asset(asset_id, asset_type):
    # Use HF Datasets Server to verify
    url = f"https://datasets-server.huggingface.co/is-valid?dataset={asset_id}"
    response = requests.get(url)
    if response.status_code != 200:
        return False
    # API returns object with feature flags like 'viewer', 'preview' if valid
    data = response.json()
    return any(data.values())

print("Verifying enrichment assets...")
for cat, assets in schema['assets'].items():
    for asset in assets:
        is_valid = verify_asset(asset['id'], asset['type'])
        status = "✅ VALID" if is_valid else "❌ INVALID"
        print(f"{status} | {cat.upper()} | {asset['id']}")

print("\nEnrichment packaging utility complete.")

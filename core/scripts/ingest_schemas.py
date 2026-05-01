import json
import psycopg2
import requests
import os

NEON_API_URL = "https://console.neon.tech/api/v2"

def get_neon_connection_string(project_id, api_token):
    # Fetch branches
    headers = {"Authorization": f"Bearer {api_token}"}
    res = requests.get(f"{NEON_API_URL}/projects/{project_id}/branches", headers=headers)
    if res.status_code != 200:
        return None
    branches = res.json()
    # Assuming main branch
    main_branch = next((b for b in branches['branches'] if b['name'] == 'main'), branches['branches'][0])
    
    # Fetch connection string
    res = requests.get(f"{NEON_API_URL}/projects/{project_id}/branches/{main_branch['id']}/connection_uri", headers=headers)
    if res.status_code == 200:
        return res.json()['connection_uri']
    return None

def ingest_neon(db_info, api_token):
    print(f"Ingesting Neon DB: {db_info['id']}")
    conn_str = get_neon_connection_string(db_info['id'], api_token)
    if not conn_str:
        return {"status": "error", "message": "Could not get connection string"}
    
    try:
        conn = psycopg2.connect(conn_str)
        cur = conn.cursor()
        cur.execute("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
        tables = [row[0] for row in cur.fetchall()]
        cur.close()
        conn.close()
        return {"status": "success", "tables": tables}
    except Exception as e:
        return {"status": "error", "message": str(e)}

def ingest_supabase(db_info):
    print(f"Ingesting Supabase DB: {db_info['id']}")
    # Supabase ingestion requires API keys, not provided in pool file. 
    # Placeholder for actual Supabase management API
    return {"status": "manual_intervention_required", "tables": []}

def main():
    with open('/home/mega/database_pool.json', 'r') as f:
        db_pool = json.load(f)

    with open('/home/mega/.config/neonctl/credentials.json', 'r') as f:
        neon_creds = json.load(f)
        neon_token = neon_creds['access_token']

    catalog = {}

    for db in db_pool.get('neon', []):
        catalog[db['id']] = ingest_neon(db, neon_token)

    for db in db_pool.get('supabase', []):
        catalog[db['id']] = ingest_supabase(db)

    with open('/home/mega/catalog.json', 'w') as f:
        json.dump(catalog, f, indent=2)
    print("Catalog saved to /home/mega/catalog.json")

if __name__ == "__main__":
    main()

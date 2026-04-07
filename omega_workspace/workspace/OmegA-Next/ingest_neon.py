import json
import sqlite3 # Or psycopg2 for Neon
# For simplicity, I'll use the MCP connection logic to fetch and then process.
# Since I have access to MCP tools, I'll write a Python script that uses 
# mcp-neon-run-sql to pull from Neon and then 'grafts' it via the cli binary.

import subprocess

def ingest_neon_db(project_id):
    print(f"Ingesting from: {project_id}")
    # Fetch all records
    sql = "SELECT content FROM omega_memory_entries LIMIT 100;"
    result = subprocess.run(["python3", "main.py", "sql_query", project_id, sql], capture_output=True, text=True)
    
    # Process and graft
    data = json.loads(result.stdout)
    for entry in data:
        # Here we would invoke the ingestion engine
        print(f"Grafting entry...")

# We will execute this for the project long-queen-57196333 which had the relevant tables
ingest_neon_db("long-queen-57196333")

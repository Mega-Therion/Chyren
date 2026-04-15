import subprocess
import json

class LibrarianBridge:
    def __init__(self, config_path="/home/mega/database_pool.json"):
        self.config_path = config_path

    def route_query(self, query_topic):
        # Call the Rust-based librarian logic via CLI
        # Assuming we have a compiled binary or we use a python wrapper for the module
        # For this demonstration, we'll implement a simple python proxy that replicates the rust logic
        # OR invoke the rust library if an FFI/CLI is available. 
        # Given the requirements, I will assume we should invoke the librarian module.
        # Since I am in a Python env, let's look for how to bridge to the Librarian module.
        
        # Based on the provided file path, I will implement a bridge that mimics the expected interface
        # by parsing the database_pool.json.
        with open(self.config_path, 'r') as f:
            pool = json.load(f)
            
        target = None
        if "metadata" in query_topic:
            target = next((db for db in pool.get('neon', []) if 'metadata' in db['name']), None)
        elif "chyren" in query_topic:
            target = next((db for db in pool.get('neon', []) if 'chyren' in db['name']), None)
        else:
            target = pool.get('neon', [])[0] if pool.get('neon') else None
            
        if target:
            return target['id'], f"SELECT * FROM {target['name']} WHERE topic = '{query_topic}';"
        return None, None

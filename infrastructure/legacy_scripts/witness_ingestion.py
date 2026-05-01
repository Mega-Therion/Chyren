import json
import subprocess

def witness_verification(project_id):
    # Query the family_history table
    sql = "SELECT name, relation, event_description FROM family_history LIMIT 3;"
    # Using the project 'long-queen-57196333' where the table lives
    result = subprocess.run(["python3", "main.py", "sql_query", project_id, sql], capture_output=True, text=True)
    
    print("--- WITNESS VERIFICATION: FAMILY CHRONICLES ---")
    print(result.stdout)

witness_verification("long-queen-57196333")

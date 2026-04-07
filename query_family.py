import json
import subprocess

# Query the table we created earlier
sql = "SELECT * FROM family_history LIMIT 10;"
result = subprocess.run(["python3", "main.py", "sql_query", "shy-wave-51974271", sql], capture_output=True, text=True)
print(result.stdout)

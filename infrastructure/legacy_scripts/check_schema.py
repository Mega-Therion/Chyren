import psycopg2
import sys

db_url = 'postgresql://neondb_owner:npg_HbW1Zlkjd7NI@ep-sweet-glade-anvm0pwn.c-6.us-east-1.aws.neon.tech/neondb?sslmode=require'

try:
    conn = psycopg2.connect(db_url)
    cursor = conn.cursor()
    
    # Get table info
    cursor.execute("""
        SELECT column_name, data_type 
        FROM information_schema.columns 
        WHERE table_name = 'chyren_memory_entries'
        ORDER BY ordinal_position
    """)
    
    print("chyren_memory_entries schema:")
    for col_name, col_type in cursor.fetchall():
        print(f"  {col_name}: {col_type}")
    
    # Count entries
    cursor.execute("SELECT COUNT(*) FROM public.chyren_memory_entries")
    count = cursor.fetchone()[0]
    print(f"\nTotal entries: {count}")
    
    # Sample a few
    cursor.execute("SELECT * FROM public.chyren_memory_entries LIMIT 3")
    cols = [desc[0] for desc in cursor.description]
    print(f"\nColumns: {cols}")
    
    cursor.close()
    conn.close()
except Exception as e:
    print(f"Error: {e}")
    sys.exit(1)

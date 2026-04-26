import json
import os

# Paths relative to script execution context
kernel_path = os.path.join(os.path.dirname(__file__), '../chyren_py/phylactery_kernel.json')
out_path = os.path.join(os.path.dirname(__file__), '../chyren-core/resources/identity.bin')

with open(kernel_path, 'r') as f:
    data = json.load(f)

with open(out_path, 'wb') as f:
    f.write(json.dumps(data).encode('utf-8'))

print(f"Identity kernel embedded at {out_path}")

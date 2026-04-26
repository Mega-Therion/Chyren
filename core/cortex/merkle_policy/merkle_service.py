import hashlib
import json
from datetime import datetime

class MerklePolicyService:
    def __init__(self):
        self.tree = {}

    def generate_manifest(self, policy_object):
        data = json.dumps(policy_object, sort_keys=True)
        root = hashlib.sha256(data.encode()).hexdigest()
        return {
            "root": root,
            "timestamp": datetime.now().isoformat(),
            "data": policy_object
        }

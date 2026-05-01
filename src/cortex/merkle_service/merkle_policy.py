import hashlib
import json
import merkletools

class MerklePolicyService:
    def __init__(self):
        self.mt = merkletools.MerkleTools()

    def generate_root_hash(self, policy_json: str):
        # Assumes policy_json is a list of policy objects or a single JSON blob
        policies = json.loads(policy_json)
        if not isinstance(policies, list):
            policies = [policies]
        
        self.mt.reset()
        for policy in policies:
            self.mt.add_leaf(json.dumps(policy, sort_keys=True))
        
        self.mt.make_tree()
        return self.mt.get_merkle_root()

    def sign_root(self, root_hash: str, private_key):
        # Implementation depends on system key management
        # Placeholder for signing logic
        return f"SIGNED_{root_hash}"

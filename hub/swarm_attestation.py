import hashlib
import json

class SwarmAttestation:
    def __init__(self, yettragrammaton_seed):
        self.seed = yettragrammaton_seed

    def sign_message(self, message):
        """Signs inter-agent messages using the Yettragrammaton seed."""
        data = json.dumps(message, sort_keys=True)
        signature = hashlib.sha256((data + self.seed).encode()).hexdigest()
        return {"data": message, "signature": signature}

    def verify_message(self, signed_message):
        """Verifies integrity and authenticity of agent communications."""
        signature = signed_message.get("signature")
        data = signed_message.get("data")
        expected = hashlib.sha256((json.dumps(data, sort_keys=True) + self.seed).encode()).hexdigest()
        return signature == expected

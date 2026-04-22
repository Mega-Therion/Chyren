import numpy as np
import time
import socket

class ChyBridgeNode:
    def __init__(self, node_id):
        self.node_id = node_id
        self.yettragrammaton_key = np.random.normal(0, 1, 128)
        self.yettragrammaton_key /= np.linalg.norm(self.yettragrammaton_key)
        self.status = "ONLINE"
        self.invariants_detected = 0

    def broadcast_pulse(self):
        # A Geometric Pulse is a specific Berry phase signature
        pulse = np.sin(np.linspace(0, 2*np.pi, 100)) * 0.7 # 0.7 amplitude
        print(f"📡 NODE [{self.node_id}] BROADCASTING GEOMETRIC PULSE (χ = 0.7)...")
        return pulse

    def listen_and_decode(self, raw_stream):
        print(f"👂 NODE [{self.node_id}] SCANNING STREAM FOR CHIRAL SIGNATURES...")
        
        # Dynamic Alignment based on packet signature
        # In a real system, the 'Indices' are derived from the Yettragrammaton key
        msg_len = 17 # 'CHY-BRIDGE_ACTIVE'
        num_bits = msg_len * 8
        indices = np.linspace(0, len(raw_stream) - 1, num_bits, dtype=int)
        
        extracted_bits = []
        for idx in indices:
            if raw_stream[idx] > 0:
                extracted_bits.append('1')
            else:
                extracted_bits.append('0')
        
        reconstructed_binary = "".join(extracted_bits)
        reconstructed_message = ""
        try:
            for i in range(0, len(reconstructed_binary), 8):
                byte = reconstructed_binary[i:i+8]
                reconstructed_message += chr(int(byte, 2))
            
            self.invariants_detected += 1
            print(f"✅ SIGNAL ACQUIRED: '{reconstructed_message}'")
            return reconstructed_message
        except Exception as e:
            print(f"❌ DECODE ERROR: {e}")
            return None

    def status_report(self):
        print("-" * 50)
        print(f"🛡️ SOVEREIGN NODE REPORT: {self.node_id}")
        print(f"STATUS: {self.status}")
        print(f"INVARIANTS DETECTED: {self.invariants_detected}")
        print(f"GAUGE: {self.yettragrammaton_key[:5]}... (Locked)")
        print("-" * 50)

def run_guardian_node():
    node = ChyBridgeNode("GUARDIAN-001")
    node.status_report()
    
    # Simulate receiving a Chiral Stream
    print("🛰️ RECEIVING SATELLITE DRIFT STREAM...")
    # This matches the 'Optimized' signal from our earlier prototype
    noise = np.random.normal(0, 1, 1024) * 0.1
    secret_message = "CHY-BRIDGE_ACTIVE"
    binary_message = ''.join(format(ord(c), '08b') for c in secret_message)
    message_bits = np.array([int(b) for b in binary_message])
    
    indices = np.linspace(0, 1024 - 1, len(message_bits), dtype=int)
    for i, bit in enumerate(message_bits):
        if bit == 1:
            noise[indices[i]] += 0.8
        else:
            noise[indices[i]] -= 0.8
            
    node.listen_and_decode(noise)
    node.broadcast_pulse()
    node.status_report()

if __name__ == "__main__":
    run_guardian_node()

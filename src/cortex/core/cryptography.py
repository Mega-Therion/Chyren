import os
import secrets
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.primitives import padding
from cryptography.hazmat.backends import default_backend

# Master Sovereign Key should be loaded from secure environment
MASTER_SOVEREIGN_KEY = os.getenv("MASTER_SOVEREIGN_KEY", secrets.token_bytes(32))

def generate_chy_key() -> bytes:
    """Generate a 128-dimensional (128-bit) Chy-Key."""
    return secrets.token_bytes(16)

def encrypt_chy_key(key: bytes) -> bytes:
    """Encrypt a Chy-Key using the Master Sovereign Key."""
    iv = secrets.token_bytes(16)
    cipher = Cipher(algorithms.AES(MASTER_SOVEREIGN_KEY), modes.CBC(iv), backend=default_backend())
    encryptor = cipher.encryptor()
    
    padder = padding.PKCS7(128).padder()
    padded_data = padder.update(key) + padder.finalize()
    
    ciphertext = encryptor.update(padded_data) + encryptor.finalize()
    return iv + ciphertext

def decrypt_chy_key(encrypted_key: bytes) -> bytes:
    """Decrypt a Chy-Key using the Master Sovereign Key."""
    iv = encrypted_key[:16]
    ciphertext = encrypted_key[16:]
    
    cipher = Cipher(algorithms.AES(MASTER_SOVEREIGN_KEY), modes.CBC(iv), backend=default_backend())
    decryptor = cipher.decryptor()
    
    decrypted_padded = decryptor.update(ciphertext) + decryptor.finalize()
    
    unpadder = padding.PKCS7(128).unpadder()
    return unpadder.update(decrypted_padded) + unpadder.finalize()

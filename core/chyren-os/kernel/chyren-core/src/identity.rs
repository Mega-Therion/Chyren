pub const IDENTITY_KERNEL: &[u8] = include_bytes!("../resources/identity.bin");

pub const YETTRAGRAMMATON: &str = "R.W.Ϝ.Y.";

pub fn verify_integrity() -> bool {
    // In future, this will check a cryptographic signature (Yettragrammaton)
    // against the identity binary itself.
    !IDENTITY_KERNEL.is_empty()
}

pub fn load_identity() -> Result<serde_json::Value, serde_json::Error> {
    serde_json::from_slice(IDENTITY_KERNEL)
}

//! C.I.M. (Cognitive Interleaving Manifold) crate
//! Provides dual‑stream narrative encoding to bypass token‑window limits.
//! This is a minimal skeleton – real implementation will be added later.

use serde::{Deserialize, Serialize};

/// Represents a dual‑stream narrative.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualStream {
    pub primary: String,
    pub secondary: String,
}

impl DualStream {
    /// Create a new dual‑stream from two texts.
    pub fn new(primary: impl Into<String>, secondary: impl Into<String>) -> Self {
        Self {
            primary: primary.into(),
            secondary: secondary.into(),
        }
    }

    /// Interleave the two streams into a single encoded string.
    /// Simple round‑robin interleaving for demonstration.
    pub fn interleave(&self) -> String {
        let mut result = String::new();
        let primary_chars: Vec<char> = self.primary.chars().collect();
        let secondary_chars: Vec<char> = self.secondary.chars().collect();
        let max_len = primary_chars.len().max(secondary_chars.len());
        for i in 0..max_len {
            if i < primary_chars.len() {
                result.push(primary_chars[i]);
            }
            if i < secondary_chars.len() {
                result.push(secondary_chars[i]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interleave() {
        let ds = DualStream::new("ABC", "1234");
        assert_eq!(ds.interleave(), "A1B2C34");
    }
}

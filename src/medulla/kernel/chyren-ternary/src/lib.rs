//! T.R.A. (Ternary Resonant Algebra) crate
//! Provides a balanced ternary type (-1, 0, +1) with arithmetic operations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ternary {
    Neg = -1,
    Zero = 0,
    Pos = 1,
}

impl Ternary {
    /// Convert from i8, clamping to -1,0,1.
    pub fn from_i8(v: i8) -> Self {
        match v {
            x if x < 0 => Ternary::Neg,
            0 => Ternary::Zero,
            _ => Ternary::Pos,
        }
    }

    /// Convert to i8.
    pub fn as_i8(self) -> i8 {
        self as i8
    }
}

use std::ops::{Add, Mul, Neg, Sub};

impl Add for Ternary {
    type Output = Ternary;
    fn add(self, rhs: Ternary) -> Ternary {
        // Balanced ternary addition table
        match (self, rhs) {
            (Ternary::Zero, x) | (x, Ternary::Zero) => x,
            (Ternary::Neg, Ternary::Neg) => Ternary::Neg,
            (Ternary::Pos, Ternary::Pos) => Ternary::Pos,
            (Ternary::Neg, Ternary::Pos) | (Ternary::Pos, Ternary::Neg) => Ternary::Zero,
        }
    }
}

impl Sub for Ternary {
    type Output = Ternary;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Ternary) -> Ternary {
        self + (-rhs)
    }
}

impl Mul for Ternary {
    type Output = Ternary;
    fn mul(self, rhs: Ternary) -> Ternary {
        match (self, rhs) {
            (Ternary::Zero, _) | (_, Ternary::Zero) => Ternary::Zero,
            (Ternary::Neg, Ternary::Neg) => Ternary::Pos,
            (Ternary::Pos, Ternary::Pos) => Ternary::Pos,
            (Ternary::Neg, Ternary::Pos) | (Ternary::Pos, Ternary::Neg) => Ternary::Neg,
        }
    }
}

impl Neg for Ternary {
    type Output = Ternary;
    fn neg(self) -> Ternary {
        match self {
            Ternary::Neg => Ternary::Pos,
            Ternary::Zero => Ternary::Zero,
            Ternary::Pos => Ternary::Neg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        assert_eq!(Ternary::Neg + Ternary::Pos, Ternary::Zero);
        assert_eq!(Ternary::Neg + Ternary::Neg, Ternary::Neg);
        assert_eq!(Ternary::Pos + Ternary::Pos, Ternary::Pos);
        assert_eq!(Ternary::Zero + Ternary::Neg, Ternary::Neg);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Ternary::Neg * Ternary::Neg, Ternary::Pos);
        assert_eq!(Ternary::Neg * Ternary::Pos, Ternary::Neg);
        assert_eq!(Ternary::Zero * Ternary::Pos, Ternary::Zero);
    }
}

//! # Random number struct/type
//!
//! This module defines the random number type `r64` (and in future `r32`)
//! for random number generators. It takes care of transformation from the
//! random number to different basic types. The reason for its existance
//! is the easier transformation to unsigned and float types.
//!
//! The `r64` and `r32` are simple wrappers around the base unsigned
//! types of the same bitlength.
//!
//! ```rust
//! use lichtspiel::rng::r64;
//!
//! let u = 42u64;
//! let r = r64::from(u);
//!
//! assert_eq!(r, u);
//! assert_eq!(u, r);
//! ```

impl PartialEq<u64> for r64 {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}

impl PartialEq<r64> for u64 {
    fn eq(&self, other: &r64) -> bool {
        self == &other.0
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub struct r64(u64);

impl From<u32> for r64 {
    fn from(value: u32) -> Self {
        r64(value as u64)
    }
}

impl From<u64> for r64 {
    fn from(value: u64) -> Self {
        r64(value)
    }
}

impl From<r64> for u32 {
    fn from(value: r64) -> Self {
        (value.0 >> 32) as u32
    }
}

impl From<r64> for u64 {
    fn from(value: r64) -> Self {
        value.0
    }
}

/// Transformation of random u64 to f64
///
/// This is a bit tricky since f64 values are not mapped linearly within value range.
/// There are several strategies for solving this (see commented out sections).
impl From<r64> for f64 {
    fn from(value: r64) -> Self {
        // (value.0 >> 11) as f64 / (1u64 << 53) as f64
        f64::from_le_bytes(((1023u64 << 52) + (value.0 >> 12)).to_le_bytes()) - 1.0
    }
}
impl From<f64> for r64 {
    fn from(value: f64) -> Self {
        // Random64(u64::from_le_bytes(f64::to_le_bytes(value * 2.0)))
        r64(u64::from_le_bytes(f64::to_le_bytes(value + 1.0)))
    }
}

/// Transformation from u64 to f32
///
/// The same concerns as the transformation to f64 apply also to f32.
impl From<r64> for f32 {
    fn from(value: r64) -> Self {
        let v = (127u32 << 23) + ((value.0 >> 41) as u32);
        f32::from_le_bytes(v.to_le_bytes()) - 1.0
    }
}

impl From<f32> for r64 {
    fn from(value: f32) -> Self {
        r64(u32::from_le_bytes(f32::to_le_bytes(value + 1.0)) as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;

    #[test]
    fn roundtrip_u64() {
        let start = 9383u64;
        let rt = u64::from(r64::from(start));
        assert_eq!(start, rt)
    }
    #[test]
    fn roundtrip_f64() {
        let core = u64::MAX / 2;
        let rt = r64::from(f64::from(r64(core)));
        assert_ne!(r64(core), rt);
    }
    #[test]
    fn roundtrip_f32() {
        let core = u64::MAX / 2;
        let rt = r64::from(f32::from(r64(core)));
        assert_ne!(r64(core), rt);
    }
    #[test]
    fn no_roundtrip_u32() {
        let start = 8278u32;
        let rt = u32::from(r64::from(start));
        assert_ne!(start, rt);
        assert_eq!(rt, 0);
    }
}

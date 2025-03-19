#[derive(Debug, PartialEq)]
pub struct Random64(u64);

impl From<u32> for Random64 {
    fn from(value: u32) -> Self {
        Random64(value as u64)
    }
}

impl From<u64> for Random64 {
    fn from(value: u64) -> Self {
        Random64(value)
    }
}

impl From<Random64> for u32 {
    fn from(value: Random64) -> Self {
        (value.0 >> 32) as u32
    }
}

impl From<Random64> for u64 {
    fn from(value: Random64) -> Self {
        value.0
    }
}

/// Transformation of random u64 to f64
///
/// This is a bit tricky since f64 values are not mapped linearly within value range.
/// There are several strategies for solving this (see commented out sections).
impl From<Random64> for f64 {
    fn from(value: Random64) -> Self {
        // (value.0 >> 11) as f64 / (1u64 << 53) as f64
        f64::from_le_bytes(((1023u64 << 52) + (value.0 >> 12)).to_le_bytes()) - 1.0
    }
}
impl From<f64> for Random64 {
    fn from(value: f64) -> Self {
        // Random64(u64::from_le_bytes(f64::to_le_bytes(value * 2.0)))
        Random64(u64::from_le_bytes(f64::to_le_bytes(value + 1.0)))
    }
}

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;

    #[test]
    fn roundtrip_u64() {
        let start = 9383u64;
        let rt = u64::from(Random64::from(start));
        assert_eq!(start, rt)
    }
    #[test]
    fn roundtrip_f64() {
        let core = u64::MAX / 2;
        let rt = Random64::from(f64::from(Random64(core)));
        assert_ne!(Random64(core), rt);
    }
    #[test]
    fn no_roundtrip_u32() {
        let start = 8278u32;
        let rt = u32::from(Random64::from(start));
        assert_ne!(start, rt);
        assert_eq!(rt, 0);
    }
}

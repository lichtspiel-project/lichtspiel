//! Methods to shuffle bits of a u32/u64 value using ideas from hash functions
//!
//! https://nullprogram.com/blog/2018/07/31/
//!
//! Reverse functions can be implementated using!
//! https://marc-b-reynolds.github.io/math/2017/10/13/IntegerBijections.html#fnref:modinverse
//!

/// Const generic implementation of bitmix for u32
fn _bitmix32<const S1: u32, const M1: u32, const S2: u32, const M2: u32, const S3: u32>(
    num: u32,
) -> u32 {
    let mut x = num;
    x ^= x >> S1;
    x = x.wrapping_mul(M1);
    x ^= x >> S2;
    x = x.wrapping_mul(M2);
    x ^= x >> S3;
    x
}

/// Const generic implementation of bitmix for u64
fn _bitmix64<const S1: u64, const M1: u64, const S2: u64, const M2: u64, const S3: u64>(
    num: u64,
) -> u64 {
    let mut x = num;
    x ^= x >> S1;
    x = x.wrapping_mul(M1);
    x ^= x >> S2;
    x = x.wrapping_mul(M2);
    x ^= x >> S3;
    x
}

/// Const generic implementation of bitmix for u16
fn _bitmix16<const S1: u16, const M1: u16, const S2: u16, const M2: u16, const S3: u16>(
    num: u16,
) -> u16 {
    let mut x = num;
    x ^= x >> S1;
    x = x.wrapping_mul(M1);
    x ^= x >> S2;
    x = x.wrapping_mul(M2);
    x ^= x >> S3;
    x
}

/// Updated bitmix implementation based on https://github.com/skeeto/hash-prospector/issues/19
pub fn bitmix32(num: u32) -> u32 {
    _bitmix32::<16, 0x21f0aaad, 15, 0xd35a2d97, 15>(num)
}

/// Original bitmix implementation in nullprogram: https://nullprogram.com/blog/2018/07/31/
pub fn bitmix32o(num: u32) -> u32 {
    _bitmix32::<16, 0x7feb352d, 15, 0x846ca68b, 16>(num)
}

/// Implementation of bitmix for u64
pub fn bitmix64(num: u64) -> u64 {
    _bitmix64::<30, 0xbf58476d1ce4e5b9, 27, 0x94d049bb133111eb, 31>(num)
}

/// Implementation for u16
pub fn bitmix16(num: u16) -> u16 {
    _bitmix16::<8, 0xa3d3, 7, 0x4b2d, 9>(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_different32() {
        let num = 42_u32;
        let result = bitmix32(num);

        assert_ne!(num, result);
    }

    #[test]
    fn is_different64() {
        let num = 42_u64;
        let result = bitmix64(num);

        assert_ne!(num, result);
    }
}

//! Methods to shuffle bits of a u32/u64 value using ideas from hash functions
//!
//! https://nullprogram.com/blog/2018/07/31/
//!
//! Reverse functions can be implementated using!
//! https://marc-b-reynolds.github.io/math/2017/10/13/IntegerBijections.html#fnref:modinverse
//!

macro_rules! bitmix {
    ($num:expr, [$s1:expr, $m1:expr, $s2:expr, $m2:expr, $s3:expr]) => {
        let mut x = $num;
        x ^= x >> $s1;
        x = x.wrapping_mul($m1);
        x ^= x >> $s2;
        x = x.wrapping_mul($m2);
        x ^= x >> $s3;
        return x
    };
}

/// bitmix scrambling for hashing u16
///
/// Source: https://github.com/skeeto/hash-prospector/issues/19
pub fn bitmix16(num: u16) -> u16 {
    bitmix!(num, [8, 0xa3d3, 7, 0x4b2d, 9]);
}

/// bitmix scrambling for hashing u32
///
/// Source: https://github.com/skeeto/hash-prospector/issues/19
pub fn bitmix32(num: u32) -> u32 {
    bitmix!(num, [16, 0x21f0aaad, 15, 0xd35a2d97, 15]);
}

/// bitmix scrambling for hashing u32 (prospector variant)
///
/// Source: https://nullprogram.com/blog/2018/07/31/
pub fn bitmix32p(num: u32) -> u32 {
    bitmix!(num, [16, 0x7feb352d, 15, 0x846ca68b, 16]);
}

/// bitmix scrambling for hashing u32 (murmurhash variant)
///
/// Source: https://nullprogram.com/blog/2018/07/31/
pub fn murmurhash32(num: u32) -> u32 {
    bitmix!(num, [16, 0x85ebca6b, 13, 0xc2b2ae35, 16]);
}

/// bitmix scrambling for hashing u32 (hash32)
///
/// Source: https://nullprogram.com/blog/2018/07/31/
pub fn hash32(num: u32) -> u32 {
    bitmix!(num, [16, 0x045d9f3b, 16, 0x045d9f3b, 16]);
}

/// bitmix scrambling for hashing u64
///
/// Source: https://nullprogram.com/blog/2018/07/31/
pub fn bitmix64(num: u64) -> u64 {
    bitmix!(num, [32, 0xd6e8feb86659fd93, 32, 0xd6e8feb86659fd93, 32]);
}

/// bitmix scrambling for hashing u64 (splittable variant)
///
/// Source: https://nullprogram.com/blog/2018/07/31/
///
///
/// The reverse function for future reference:
///
/// ```rust
/// let mut x = 42_u64;
/// x ^= x >> 31 ^ x >> 62;
/// x = x.wrapping_mul(0x319642b2d24d8ec3_u64);
/// x ^= x >> 27 ^ x >> 54;
/// x = x.wrapping_mul(0x96de1b173f119089_u64);
/// x ^= x >> 30 ^ x >> 60;
/// ```
pub fn bitmix64s(num: u64) -> u64 {
    bitmix!(num, [30, 0xbf58476d1ce4e5b9, 27, 0x94d049bb133111eb, 31]);
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
        let result = bitmix64s(num);

        assert_ne!(num, result);
    }
}

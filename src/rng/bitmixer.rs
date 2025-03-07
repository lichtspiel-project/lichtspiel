//! Methods to shuffle bits of a u32/u64 value using ideas from hash functions
//!
//! https://nullprogram.com/blog/2018/07/31/
//!

fn bitmix32(num: u32) -> u32 {
    // 16 21f0aaad 15 d35a2d97 15
    let mut x = num;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d_u32);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b_u32);
    x ^= x >> 16;
    x
}

fn bitmix32_r(num: u32) -> u32 {
    let mut x = num;
    x ^= x >> 16;
    x = x.wrapping_mul(0x43021123_u32);
    x ^= x >> 15 ^ x >> 30;
    x = x.wrapping_mul(0x1d69e2a5_u32);
    x ^= x >> 16;
    x
}

fn bitmix64(num: u64) -> u64 {
    let mut x = num;
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9_u64);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb_u64);
    x ^= x >> 31;
    x
}

fn bitmix64_r(num: u64) -> u64 {
    let mut x = num;
    x ^= x >> 31 ^ x >> 62;
    x = x.wrapping_mul(0x319642b2d24d8ec3_u64);
    x ^= x >> 27 ^ x >> 54;
    x = x.wrapping_mul(0x96de1b173f119089_u64);
    x ^= x >> 30 ^ x >> 60;
    x
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
    fn is_reversible32() {
        let num = 42_u32;
        let result = bitmix32_r(bitmix32(num));

        assert_eq!(num, result);
    }

    #[test]
    fn is_different64() {
        let num = 42_u64;
        let result = bitmix64(num);

        assert_ne!(num, result);
    }

    #[test]
    fn is_reversible64() {
        let num = 42_u64;
        let result = bitmix64_r(bitmix64(num));

        assert_eq!(num, result);
    }
}

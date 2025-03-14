//! # Casting to Float
//!
//! Casting to float based on strategies defined in https://mumble.net/~campbell/tmp/random_real.c

/// Cast using all bits
fn random_real_64(num: u64) -> f64 {
    num as f64 / u64::MAX as f64
}

/// Cast using only the 53 MSB
fn random_real_53(num: u64) -> f64 {
    (num >> 11) as f64 / (1u64 << 53) as f64
}

/// Cast using all bits
fn random_real_32(num: u32) -> f32 {
    num as f32 / u32::MAX as f32
}

/// Cast using only the 24 MSB
fn random_real_24(num: u32) -> f32 {
    (num >> 8) as f32 / (1u32 << 24) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u64;

    #[test]
    fn check_real_64() {
        let values = [u64::MAX, u64::MIN, u64::MAX / 2];
        for &v in values.iter() {
            let result = random_real_64(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result <= 1.0);
        }
    }

    #[test]
    fn check_real_64_half() {
        assert_eq!(random_real_64(u64::MAX / 2), 0.5)
    }

    #[test]
    fn check_real_32() {
        let values = [u32::MAX, u32::MIN, u32::MAX / 2];
        for &v in values.iter() {
            let result = random_real_32(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result <= 1.0);
        }
    }

    #[test]
    fn check_real_32_half() {
        assert_eq!(random_real_32(u32::MAX / 2), 0.5)
    }

    #[test]
    fn check_real_53() {
        let values = [u64::MAX, u64::MIN, u64::MAX / 2];
        for &v in values.iter() {
            let result = random_real_53(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result < 1.0);
        }
    }

    #[test]
    fn check_real_53_half() {
        assert_eq!(random_real_53(u64::MAX / 2), 0.4999999999999999)
    }

    #[test]
    fn check_real_24() {
        let values = [u32::MAX, u32::MIN, u32::MAX / 2];
        for &v in values.iter() {
            let result = random_real_24(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result < 1.0);
        }
    }

    #[test]
    fn check_real_24_half() {
        assert_eq!(random_real_24(u32::MAX / 2), 0.49999994)
    }
}

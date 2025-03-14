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

#[cfg(test)]
mod tests {
    use std::u64;

    use super::*;

    #[test]
    fn check_highest_full() {
        let values = [u64::MAX, u64::MIN, u64::MAX / 2];
        for &v in values.iter() {
            let result = random_real_64(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result <= 1.0);
        }
    }

    #[test]
    fn check_borders_full() {
        assert_eq!(random_real_64(u64::MAX / 2), 0.5)
    }

    #[test]
    fn check_highest_h53() {
        let values = [u64::MAX, u64::MIN, u64::MAX / 2];
        for &v in values.iter() {
            let result = random_real_53(v);
            println!("{} -> {}", v, result);
            assert!(result >= 0.0);
            assert!(result <= 1.0);
        }
    }

    #[test]
    fn check_borders_h53() {
        assert_eq!(random_real_53(u64::MAX / 2), 0.4999999999999999)
    }
}

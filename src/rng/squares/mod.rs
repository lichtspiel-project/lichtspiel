use super::core::R64;
use crate::rng::splitmix::splitmix64::SplitMix64;

#[derive(Debug, PartialEq)]
pub struct Squares {
    ctr: u64,
    key: u64,
}

impl R64 for Squares {
    fn random_u32(&mut self) -> u32 {
        let mut x = self.ctr.wrapping_mul(self.key);
        let y = x;
        let z = y.wrapping_add(self.key);

        x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);
        self.ctr += 1;

        (x.rotate_left(32) >> 32) as u32
    }
    fn random_u64(&mut self) -> u64 {
        let mut x = self.ctr.wrapping_mul(self.key);
        let y = x;
        let z = y.wrapping_add(self.key);

        x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
        x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);
        self.ctr += 1;

        x.rotate_left(32) ^ (x.wrapping_mul(x).wrapping_add(y) >> 32)
    }
}

impl Squares {
    fn new(ctr: u64, key: u64) -> Self {
        if key != 0 {
            Self { ctr, key }
        } else {
            Self::with(0)
        }
    }
    fn with(seed: u64) -> Self {
        let state = get_state_from_splitmix!(1, seed, u64);
        let key = state[0];
        Self { ctr: 0, key }
    }
}

impl Default for Squares {
    fn default() -> Self {
        Self::with(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::{float::random_real_24, float::random_real_53};

    #[test]
    fn test_example_runs_u64() {
        let key = 0x548c9decbce65297_u64;
        let ctr = 0_u64;
        let mut rng = Squares::new(ctr, key);
        let expected = [
            0x36d88366cee633a5_u64,
            0x944716e00e60dfaa_u64,
            0xc8a8f4e0678654bf_u64,
            0x35cc666aab11c80d_u64,
            0x7094eab1cbae8747_u64,
            0xa2a1b6f56e92a96f_u64,
            0xd884957d48007552_u64,
            0xe61f37b97d593453_u64,
            0xe4d45c4762b10dad_u64,
            0xb0dbd071f201dd2a_u64,
        ];
        for exp in expected.iter() {
            let r = rng.random_u64();
            println!("{:x} v {:x}", r, exp);
            assert_eq!(r, *exp)
        }
    }

    #[test]
    fn test_example_runs_u32() {
        let key = 0x548c9decbce65297_u64;
        let ctr = 0_u64;
        let mut rng = Squares::new(ctr, key);
        let expected = [
            0x36d88366_u32,
            0x944716e0_u32,
            0xc8a8f4e0_u32,
            0x35cc666a_u32,
            0x7094eab1_u32,
            0xa2a1b6f5_u32,
            0xd884957d_u32,
            0xe61f37b9_u32,
            0xe4d45c47_u32,
            0xb0dbd071_u32,
        ];
        for exp in expected.iter() {
            let r = rng.random_u32();
            println!("{:x} v {:x}", r, exp);
            assert_eq!(r, *exp)
        }
    }

    #[test]
    #[ignore]
    fn test_billion_u64() {
        let billion = 1_000_000_000_usize;
        let ctr = 0_u64;
        let key = 0x548c9decbce65297_u64;
        let mut r = Squares::new(ctr, key);
        let sum = (0..billion)
            .into_iter()
            .fold(0.0f64, |acc, _| acc + random_real_53(r.random_u64()));
        assert_eq!(sum / (billion as f64), 0.4999960194150769)
    }

    #[test]
    #[ignore]
    fn test_million_u32() {
        let million = 1_000_000_usize;
        let ctr = 0_u64;
        let key = 0x548c9decbce65297_u64;
        let mut r = Squares::new(ctr, key);
        let result = (0..million)
            .into_iter()
            .fold(0.0f32, |acc, _| acc + random_real_24(r.random_u32()));
        assert_eq!(result / million as f32, 0.4999609)
    }

    #[test]
    fn check_equality() {
        let rngd = Squares::default();
        let rng1 = Squares::with(0);
        let rng2 = Squares::new(0, 0);
        let rng3 = Squares::new(0, 4);
        assert_eq!(rngd, rng1);
        assert_eq!(rng1, rng2);
        assert_ne!(rng2, rng3);
    }
}

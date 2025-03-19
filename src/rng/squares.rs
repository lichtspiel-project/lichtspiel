//! # Squares RNG
//!
//! Basic RNG based on counter based RNG
use super::core::r64;
use super::splitmix::Splitmix;

const DEFAULT_SPLITMIX_KEY: u64 = 0x548c9decbce65297_u64;

/// Generate random u64
const fn random_u64(ctr: u64, key: u64) -> u64 {
    let mut x = ctr.wrapping_mul(key);
    let y = x;
    let z = y.wrapping_add(key);

    x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
    x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);
    x = x.wrapping_mul(x).wrapping_add(y).rotate_left(32);
    x = x.wrapping_mul(x).wrapping_add(z).rotate_left(32);

    x.rotate_left(32) ^ (x.wrapping_mul(x).wrapping_add(y) >> 32)
}

#[derive(Debug)]
pub struct Squares {
    ctr: u64,
    key: u64,
}

impl Squares {
    pub fn with(ctr: u64, key: u64) -> Self {
        let key = if key == 0 { DEFAULT_SPLITMIX_KEY } else { key };
        Squares { ctr, key }
    }
    pub fn set_stream(&mut self, stream: u64) {
        self.key = stream
    }
    pub fn random<T: From<r64>>(&mut self) -> T {
        let v = self.random_u64();
        T::from(v)
    }
    fn random_u64(&mut self) -> r64 {
        let result = random_u64(self.ctr, self.key);
        self.ctr += 1;
        r64::from(result)
    }
}

impl Default for Squares {
    fn default() -> Self {
        let key: u64 = Splitmix::default().random();
        Self { ctr: 0, key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_zero_seed_at_default() {
        let rng = Squares::default();
        assert_ne!(rng.key, 0)
    }

    #[test]
    fn non_zero_key() {
        let rng = Squares::with(0, 0);
        assert_ne!(rng.key, 0)
    }

    #[test]
    fn reference_u64() {
        let key = 0x548c9decbce65297_u64;
        let ctr = 0_u64;
        let mut rng = Squares::with(ctr, key);
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
            let r: u64 = rng.random();
            println!("{:x} v {:x}", r, exp);
            assert_eq!(r, *exp)
        }
    }

    #[test]
    fn reference_u32() {
        let key = 0x548c9decbce65297_u64;
        let ctr = 0_u64;
        let mut rng = Squares::with(ctr, key);
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
            let r: u32 = rng.random();
            println!("{:x} v {:x}", r, exp);
            assert_eq!(r, *exp)
        }
    }
}

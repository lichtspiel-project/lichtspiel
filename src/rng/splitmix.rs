//! # Splitmix
//!
//! Implementation of the splitmix method of generating an RNG.
//! The advantages of splitmix is that it can handle the number zero
//! as a seed value very well and generates random values fast.
//! In this library it is most often used as a seed value generator
//! if the provided seed value does not offer the necessary criteria
//! defined by the algorithm to be seeded.
//!
//! There are several versions of the splitmix algorithm. They
//! differentiate mostly based on the step count, the shift and
//! multiplication values.

use super::core::r64;

/// Golden Ratio, prime version: 0x9e3779b97f4a7c55;
const GOLDEN_RATIO: u64 = 0x9e3779b97f4a7c15;

const fn splitmix(state: u64) -> u64 {
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z = z ^ (z >> 31);
    z
}

pub(crate) struct Splitmix {
    state: u64,
}

impl Splitmix {
    fn new(state: u64) -> Self {
        Self { state }
    }
    fn random_u64(&mut self) -> r64 {
        self.state = self.state.wrapping_add(GOLDEN_RATIO);
        let z = splitmix(self.state);
        r64::from(z)
    }
    pub fn random<T: From<r64>>(&mut self) -> T {
        let v = self.random_u64();
        T::from(v)
    }
}

impl Default for Splitmix {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let seed = 1234567u64;
        let mut rng = Splitmix::new(seed);
        let expected = [
            6457827717110365317_u64,
            3203168211198807973_u64,
            9817491932198370423_u64,
            4593380528125082431_u64,
            16408922859458223821_u64,
        ];
        for val in expected.iter() {
            assert_eq!(val, &rng.random::<u64>())
        }
    }

    #[test]
    fn zero_seed() {
        let seed = 0u64;
        let mut rng = Splitmix::new(seed);
        assert_ne!(rng.random::<u64>(), 0u64);
        assert_ne!(rng.random::<u64>(), 0u64);
    }
}

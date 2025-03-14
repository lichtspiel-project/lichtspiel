//! # Splitmix64
//!
//! Implementation of a fast random number generator.
//! Most often this RNG is recommended to generate a seed for other RNGs.
//! This implementation of the algorithm used the following source as reference:
//! https://xoshiro.di.unimi.it/splitmix64.c
use crate::rng::R64;

// Closes prime to the Golden Ratio constant used for better scattering
// See https://softwareengineering.stackexchange.com/a/402543
const GOLDEN_RATIO: u64 = 0x9e3779b97f4a7c15; // prime version: 0x9e3779b97f4a7c55;

pub struct SplitMix64 {
    state: u64,
}

impl R64 for SplitMix64 {
    fn random_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_RATIO);
        splitmix!(
            self.state,
            [30, 27, 31],
            [0xbf58476d1ce4e5b9, 0x94d049bb133111eb]
        )
    }
}

impl SplitMix64 {
    pub fn with(state: u64) -> Self {
        SplitMix64 { state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let s = 1234567;

        test_first_5!(
            SplitMix64,
            s,
            [
                6457827717110365317,
                3203168211198807973,
                9817491932198370423,
                4593380528125082431,
                16408922859458223821
            ],
            u64
        );
    }

    #[test]
    fn zero_seed() {
        let s = 0;
        let mut rng32 = SplitMix64::with(s);
        let mut rng64 = SplitMix64::with(s);

        assert_ne!(rng64.random_u64(), 0);
        assert_ne!(rng32.random_u64(), 0);
    }
}

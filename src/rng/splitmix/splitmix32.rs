//! # Splitmix32
//!
//! Implementation of a fast random number generator.
//! Most often this RNG is recommended to generate a seed for other RNGs.
use crate::rng::core::R32;
// Closes prime to the Golden Ratio constant used for better scattering
// See https://softwareengineering.stackexchange.com/a/402543
const GOLDEN_RATIO: u32 = 0x9e3779b9; // prime version 0x9e3779b1;

struct SplitMix32 {
    state: u32,
}

impl R32 for SplitMix32 {
    fn random_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(GOLDEN_RATIO);
        splitmix!(self.state, [16, 15, 15], [0x21f0aaad, 0x735a2d97])
    }
}

impl SplitMix32 {
    pub fn with(state: u32) -> Self {
        SplitMix32 { state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_first_5() {
        let s = 1234567;

        test_first_5!(
            SplitMix32,
            s,
            [4101310354, 1937531806, 3079499796, 1553139234, 1247708653],
            u32
        );
    }

    #[test]
    fn zero_seed() {
        let s = 0;
        let mut rng32 = SplitMix32::with(s);
        let mut rng64 = SplitMix32::with(s);

        assert_ne!(rng64.random_u32(), 0);
        assert_ne!(rng32.random_u32(), 0);
    }
}

//! # Implementation of xorshift RNG
//!
//! Implementation from https://gist.github.com/imneme/9b769cefccac1f2bd728596da3a856dd
//!
//! There are several other combinations of a, b, c and threshold values. See resource above.

use crate::rng::pcg::u64_to_u32_high;

const XORSHIFT_DEFAULT_A: u64 = 11;
const XORSHIFT_DEFAULT_B: u64 = 31;
const XORSHIFT_DEFAULT_C: u64 = 18;
const XORSHIFT_DEFAULT_TRUNCATE: u64 = 0xd989bcacc137dcd5_u64;
const XORSHIFT_DEFAULT_STATE: u64 = 0xc1f651c67c62c6e0_u64;

pub struct XorshiftStar32 {
    a: u64,
    b: u64,
    c: u64,
    state: u64,
    threshold: u64,
}

impl XorshiftStar32 {
    pub fn new() -> Self {
        XorshiftStar32 {
            a: XORSHIFT_DEFAULT_A,
            b: XORSHIFT_DEFAULT_B,
            c: XORSHIFT_DEFAULT_C,
            state: XORSHIFT_DEFAULT_STATE,
            threshold: XORSHIFT_DEFAULT_TRUNCATE,
        }
    }

    pub fn with(state: u64) -> Self {
        XorshiftStar32 {
            state: state,
            ..Default::default()
        }
    }

    fn advance(&mut self) {
        self.state ^= self.state >> self.a;
        self.state ^= self.state << self.b;
        self.state ^= self.state >> self.c;
    }

    pub fn random(&mut self) -> u32 {
        let result = self.state.wrapping_mul(self.threshold);
        self.advance();
        u64_to_u32_high(result)
    }

    pub fn jump(&mut self, jumps: usize) {
        for _ in 0..jumps {
            self.advance();
        }
    }
}

impl Default for XorshiftStar32 {
    fn default() -> Self {
        Self::new()
    }
}

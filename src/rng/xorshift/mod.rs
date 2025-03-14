//! # Implementation of xorshift RNG
//!
//! Implementation from https://gist.github.com/imneme/9b769cefccac1f2bd728596da3a856dd
//!
//! There are several other combinations of a, b, c and threshold values. See resource above.

pub struct GenXorshiftStar<const A: u64, const B: u64, const C: u64, const T: u64> {
    state: u64,
}

impl<const A: u64, const B: u64, const C: u64, const T: u64> GenXorshiftStar<A, B, C, T> {
    pub fn with(state: u64) -> Self {
        GenXorshiftStar::<A, B, C, T> { state }
    }
    fn advance(&mut self) {
        self.state ^= self.state >> A;
        self.state ^= self.state << B;
        self.state ^= self.state >> C;
    }
    pub fn random(&mut self) -> u32 {
        let result = self.state.wrapping_mul(T);
        self.advance();
        (result >> 32) as u32
    }
}

pub type XorshiftStar32 = GenXorshiftStar<11, 31, 18, 0xd989bcacc137dcd5_u64>;

impl XorshiftStar32 {
    pub fn new() -> Self {
        XorshiftStar32::with(0xc1f651c67c62c6e0_u64)
    }
}

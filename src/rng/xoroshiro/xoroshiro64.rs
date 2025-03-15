use crate::rng::core::R32;
pub struct Xoroshiro64 {
    s0: u32,
    s1: u32,
}

impl R32 for Xoroshiro64 {
    fn random_u32(&mut self) -> u32 {
        let result = self.s0;
        advance_xoroshiro_two_state!(self.s0, self.s1, [26, 9, 13]);
        result
    }
}

pub struct Xoroshiro64star {
    s0: u32,
    s1: u32,
}

impl R32 for Xoroshiro64star {
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_star!(self.s0, [0x9E3779BB]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [26, 9, 13]);
        result
    }
}

pub struct Xoroshiro64starstar {
    s0: u32,
    s1: u32,
}

impl R32 for Xoroshiro64starstar {
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_starstar!(self.s0, [0x9E3779BB, 5, 5]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [26, 9, 13]);
        result
    }
}

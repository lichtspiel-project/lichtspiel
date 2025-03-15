use crate::rng::core::R64;
pub struct Xoroshiro128 {
    s0: u64,
    s1: u64,
}

impl R64 for Xoroshiro128 {
    fn random_u64(&mut self) -> u64 {
        let result = self.s0;
        advance_xoroshiro_two_state!(self.s0, self.s1, [24, 16, 37]);
        result
    }
    fn random_u32(&mut self) -> u32 {
        let result = self.s0;
        advance_xoroshiro_two_state!(self.s0, self.s1, [9, 11, 0]);
        (result >> 32) as u32
    }
}

pub struct Xoroshiro128plus {
    s0: u64,
    s1: u64,
}

impl R64 for Xoroshiro128plus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plus!(self.s0, self.s1);
        advance_xoroshiro_two_state!(self.s0, self.s1, [24, 16, 37]);
        result
    }
}

pub struct Xoroshiro128plusplus {
    s0: u64,
    s1: u64,
}

impl R64 for Xoroshiro128plusplus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plusplus!(self.s0, self.s1, [17]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [49, 21, 28]);
        result
    }
    fn random_u32(&mut self) -> u32 {
        let result = xoroshiro_plusplus!(self.s0, self.s1, [7]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [49, 21, 28]);
        return (result >> 32) as u32;
    }
}

pub struct Xoroshiro128star {
    s0: u64,
    s1: u64,
}

impl R64 for Xoroshiro128star {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_star!(self.s0, [0x9e3779b97f4a7c13]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [24, 16, 37]);
        result
    }
}

pub struct Xoroshiro128starstar {
    s0: u64,
    s1: u64,
}

impl R64 for Xoroshiro128starstar {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_starstar!(self.s0, [5, 7, 9]);
        advance_xoroshiro_two_state!(self.s0, self.s1, [24, 16, 37]);
        result
    }
}

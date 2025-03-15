use crate::rng::core::R64;

struct Xoshiro256 {
    state: [u64; 4],
}

impl R64 for Xoshiro256 {
    fn random_u64(&mut self) -> u64 {
        let result = self.state[0];
        advance_xoshiro_4state!(self.state, [17, 45]);
        result
    }
}

struct Xoshiro256plus {
    state: [u64; 4],
}

impl R64 for Xoshiro256plus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plus!(self.state[0], self.state[3]);
        advance_xoshiro_4state!(self.state, [17, 45]);
        result
    }
}

struct Xoshiro256plusplus {
    state: [u64; 4],
}

impl R64 for Xoshiro256plusplus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plusplus!(self.state[0], self.state[3], [23]);
        advance_xoshiro_4state!(self.state, [17, 45]);
        result
    }
}

struct Xoshiro256starstar {
    state: [u64; 4],
}

impl R64 for Xoshiro256starstar {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_starstar!(self.state[1], [5, 7, 9]);
        advance_xoshiro_4state!(self.state, [17, 45]);
        result
    }
}

use crate::rng::R64;

struct Xoshiro512 {
    state: [u64; 8],
}

impl R64 for Xoshiro512 {
    fn random_u64(&mut self) -> u64 {
        let result = self.state[0];
        advance_xoshiro_8state!(self.state, [11, 21]);
        result
    }
}

struct Xoshiro512plus {
    state: [u64; 8],
}

impl R64 for Xoshiro512plus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plus!(self.state[0], self.state[3]);
        advance_xoshiro_8state!(self.state, [11, 21]);
        result
    }
}

struct Xoshiro512plusplus {
    state: [u64; 8],
}

impl R64 for Xoshiro512plusplus {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_plusplus!(self.state[0], self.state[3], [17]);
        advance_xoshiro_8state!(self.state, [11, 21]);
        result
    }
}

struct Xoshiro512starstar {
    state: [u64; 8],
}

impl R64 for Xoshiro512starstar {
    fn random_u64(&mut self) -> u64 {
        let result = xoroshiro_starstar!(self.state[1], [5, 7, 9]);
        advance_xoshiro_8state!(self.state, [11, 21]);
        result
    }
}
